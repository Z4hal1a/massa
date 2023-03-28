use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::thread;

use crossbeam_channel::{select, Receiver};
use tracing::{debug, info, warn};

use massa_factory_exports::{FactoryChannels, FactoryConfig};
use massa_models::block_header::SecuredHeader;
use massa_models::denunciation::Denunciation;
use massa_models::endorsement::SecureShareEndorsement;
use massa_models::slot::Slot;

// TODO: rework these values
const DENUNCIATION_FACTORY_ENDORSEMENT_CACHE_MAX_LEN: usize = 4096;
const DENUNCIATION_FACTORY_BLOCK_HEADER_CACHE_MAX_LEN: usize = 4096;

/// Structure gathering all elements needed by the factory thread
#[allow(dead_code)]
pub(crate) struct DenunciationFactoryWorker {
    cfg: FactoryConfig,
    channels: FactoryChannels,
    factory_receiver: Receiver<()>,
    consensus_receiver: Receiver<SecuredHeader>,
    endorsement_pool_receiver: Receiver<SecureShareEndorsement>,

    // TODO: optim with PreHashSet?
    // internal for endorsements (or secure share endorsements)
    /// Internal storage for endorsement -
    /// store at max 1 endorsement per entry, as soon as we have 2 we produce a Denunciation
    endorsements_by_slot_index: HashMap<(Slot, u32), Vec<SecureShareEndorsement>>,
    /// Cache to avoid processing several time the same endorsement denunciation
    seen_endorsement_denunciation: HashSet<(Slot, u32)>,
    // internal for block header (or secured header)
    /// Internal storage for endorsement -
    /// store at max 1 endorsement per entry, as soon as we have 2 we produce a Denunciation
    block_header_by_slot: HashMap<Slot, Vec<SecuredHeader>>,
    /// Cache to avoid processing several time the same endorsement denunciation
    seen_block_header_denunciation: HashSet<Slot>,
}

impl DenunciationFactoryWorker {
    /// Creates the `FactoryThread` structure to gather all data and references
    /// needed by the factory worker thread.
    pub(crate) fn spawn(
        cfg: FactoryConfig,
        channels: FactoryChannels,
        factory_receiver: Receiver<()>,
        consensus_receiver: Receiver<SecuredHeader>,
        endorsement_pool_receiver: Receiver<SecureShareEndorsement>,
    ) -> thread::JoinHandle<()> {
        thread::Builder::new()
            .name("denunciation-factory".into())
            .spawn(|| {
                let mut factory = Self {
                    cfg,
                    channels,
                    factory_receiver,
                    consensus_receiver,
                    endorsement_pool_receiver,
                    endorsements_by_slot_index: Default::default(),
                    seen_endorsement_denunciation: Default::default(),
                    block_header_by_slot: Default::default(),
                    seen_block_header_denunciation: Default::default(),
                };
                factory.run();
            })
            .expect("failed to spawn thread : denunciation-factory")
    }

    /// Process new secured header (~ block header)
    fn process_new_secured_header(&mut self, secured_header: SecuredHeader) {
        let key = secured_header.content.slot;
        if self.seen_block_header_denunciation.contains(&key) {
            warn!(
                "Denunciation factory process a block header that have already been denounced: {}",
                secured_header
            );
            return;
        }

        // TODO: need 2 separate constants here?
        if self.seen_block_header_denunciation.len()
            > DENUNCIATION_FACTORY_BLOCK_HEADER_CACHE_MAX_LEN
            || self.block_header_by_slot.len() > DENUNCIATION_FACTORY_BLOCK_HEADER_CACHE_MAX_LEN
        {
            warn!(
                "Denunciation factory cannot process - cache full: {}, {}",
                self.seen_block_header_denunciation.len(),
                self.block_header_by_slot.len()
            );
            return;
        }

        let denunciation_: Option<Denunciation> = match self.block_header_by_slot.entry(key) {
            Entry::Occupied(mut eo) => {
                let secured_headers = eo.get_mut();
                // Store at max 2 SecuredHeader's
                if secured_headers.len() == 1 {
                    secured_headers.push(secured_header);

                    // TODO / OPTIM?: use [] ?
                    // safe to unwrap as we just checked the vec len
                    let header_1 = secured_headers.get(0).unwrap();
                    let header_2 = secured_headers.get(1).unwrap();
                    let de_ = Denunciation::try_from((header_1, header_2));
                    if let Err(e) = de_ {
                        debug!("Denunciation factory cannot create denunciation from block headers: {}", e);
                        return;
                    }
                    Some(de_.unwrap()) // Already checked for error
                } else {
                    // Already 2 entries - so a Denunciation has already been created
                    None
                }
            }
            Entry::Vacant(ev) => {
                ev.insert(vec![secured_header]);
                None
            }
        };

        if let Some(denunciation) = denunciation_ {
            info!(
                "Created a new block header denunciation : {:?}",
                denunciation
            );

            let mut de_storage = self.channels.storage.clone_without_refs();
            de_storage.store_denunciation(denunciation);
            self.channels.pool.add_denunciations(de_storage);
        }
    }

    /// Process new secure share endorsement (~ endorsement)
    fn process_new_secure_share_endorsement(
        &mut self,
        secure_share_endorsement: SecureShareEndorsement,
    ) {
        let key = (
            secure_share_endorsement.content.slot,
            secure_share_endorsement.content.index,
        );
        if self.seen_endorsement_denunciation.contains(&key) {
            warn!(
                "Denunciation factory process an endorsement that have already been denounced: {}",
                secure_share_endorsement
            );
            return;
        }

        if self.seen_endorsement_denunciation.len() > DENUNCIATION_FACTORY_ENDORSEMENT_CACHE_MAX_LEN
            || self.endorsements_by_slot_index.len()
                > DENUNCIATION_FACTORY_ENDORSEMENT_CACHE_MAX_LEN
        {
            warn!(
                "Denunciation factory cannot process - cache full: {}, {}",
                self.seen_endorsement_denunciation.len(),
                self.endorsements_by_slot_index.len()
            );
            return;
        }

        let denunciation_: Option<Denunciation> = match self.endorsements_by_slot_index.entry(key) {
            Entry::Occupied(mut eo) => {
                let secure_share_endorsements = eo.get_mut();
                // Store at max 2 endo
                if secure_share_endorsements.len() == 1 {
                    secure_share_endorsements.push(secure_share_endorsement);

                    // TODO / OPTIM?: use [] ?
                    // safe to unwrap as we just checked the vec len
                    let header_1 = secure_share_endorsements.get(0).unwrap();
                    let header_2 = secure_share_endorsements.get(1).unwrap();
                    let de_ = Denunciation::try_from((header_1, header_2));
                    if let Err(e) = de_ {
                        debug!("Denunciation factory cannot create denunciation from block headers: {}", e);
                        return;
                    }
                    Some(de_.unwrap()) // Already checked for error
                } else {
                    // Already 2 entries - so a Denunciation has already been created
                    None
                }
            }
            Entry::Vacant(ev) => {
                ev.insert(vec![secure_share_endorsement]);
                None
            }
        };

        if let Some(denunciation) = denunciation_ {
            info!(
                "Created a new endorsement denunciation : {:?}",
                denunciation
            );
        }
    }

    /// main run loop of the denunciation creator thread
    fn run(&mut self) {
        loop {
            select! {
                recv(self.consensus_receiver) -> secured_header_ => {
                    match secured_header_ {
                        Ok(secured_header) => {
                            info!("Denunciation factory receives a new block header: {}", secured_header);
                            self.process_new_secured_header(secured_header);
                        },
                        Err(e) => {
                            warn!("Denunciation factory cannot receive from consensus receiver: {}", e);
                            break;
                        }
                    }
                },
                recv(self.endorsement_pool_receiver) -> secure_share_endorsement_ => {
                    match secure_share_endorsement_ {
                        Ok(secure_share_endorsement) => {
                            info!("Denunciation factory receives a new endorsement: {}", secure_share_endorsement);
                            self.process_new_secure_share_endorsement(secure_share_endorsement)
                        }
                        Err(e) => {
                            warn!("Denunciation factory cannot receive from endorsement pool receiver: {}", e);
                            break;
                        }
                    }
                }
                recv(self.factory_receiver) -> msg_ => {
                    if let Err(e) = msg_ {
                        warn!("Denunciation factory receive an error from factory receiver: {}", e);
                    }
                    // factory_receiver send () and is supposed to be a STOP message
                    break;
                }
            }
        }
    }
}
