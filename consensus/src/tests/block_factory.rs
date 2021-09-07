use crypto::{hash::Hash, signature::PrivateKey};
use models::{Block, BlockHeader, BlockHeaderContent, BlockId, Endorsement, Operation, Slot};

use super::{
    mock_protocol_controller::MockProtocolController,
    tools::{validate_notpropagate_block, validate_propagate_block},
};

pub struct BlockFactory {
    best_parents: Vec<BlockId>,
    creator_priv_key: PrivateKey,
    slot: Slot,
    endorsements: Vec<Endorsement>,
    operations: Vec<Operation>,
    protocol_controller: MockProtocolController,
}

impl BlockFactory {
    pub fn start_block_factory(
        genesis: Vec<BlockId>,
        protocol_controller: MockProtocolController,
    ) -> BlockFactory {
        BlockFactory {
            best_parents: genesis,
            creator_priv_key: crypto::generate_random_private_key(),
            slot: Slot::new(1, 0),
            endorsements: Vec::new(),
            operations: Vec::new(),
            protocol_controller,
        }
    }

    pub async fn create_and_receive_block(&mut self, valid: bool) -> (BlockId, Block) {
        let public_key = crypto::derive_public_key(&self.creator_priv_key);
        let (hash, header) = BlockHeader::new_signed(
            &self.creator_priv_key,
            BlockHeaderContent {
                creator: public_key,
                slot: self.slot,
                parents: self.best_parents.clone(),
                operation_merkle_root: Hash::hash(
                    &self
                        .operations
                        .iter()
                        .map(|op| op.get_operation_id().unwrap().to_bytes().clone())
                        .flatten()
                        .collect::<Vec<_>>()[..],
                ),
                endorsements: self.endorsements.clone(),
            },
        )
        .unwrap();

        let block = Block {
            header,
            operations: self.operations.clone(),
        };

        self.protocol_controller.receive_block(block.clone()).await;
        if valid {
            // Assert that the block is propagated.
            validate_propagate_block(&mut self.protocol_controller, hash, 2000).await;
        } else {
            // Assert that the the block is not propagated.
            validate_notpropagate_block(&mut self.protocol_controller, hash, 500).await;
        }
        (hash, block)
    }

    pub fn sign_header(&self, header: BlockHeaderContent) -> Block {
        let public_key = crypto::derive_public_key(&self.creator_priv_key);
        let (hash, header) = BlockHeader::new_signed(&self.creator_priv_key, header).unwrap();

        Block {
            header,
            operations: self.operations.clone(),
        }
    }

    pub async fn receieve_block(&mut self, valid: bool, block: Block) {
        let hash = block.header.compute_block_id().unwrap();
        self.protocol_controller.receive_block(block.clone()).await;
        if valid {
            // Assert that the block is propagated.
            validate_propagate_block(&mut self.protocol_controller, hash, 2000).await;
        } else {
            // Assert that the the block is not propagated.
            validate_notpropagate_block(&mut self.protocol_controller, hash, 500).await;
        }
    }

    pub fn set_slot(&mut self, slot: Slot) {
        self.slot = slot
    }

    pub fn set_parents(&mut self, parents: Vec<BlockId>) {
        self.best_parents = parents
    }

    pub fn set_creator(&mut self, creator: PrivateKey) {
        self.creator_priv_key = creator
    }
    pub fn set_endorsements(&mut self, endorsements: Vec<Endorsement>) {
        self.endorsements = endorsements
    }
    pub fn set_operations(&mut self, operations: Vec<Operation>) {
        self.operations = operations
    }

    pub fn give_protocol_controller(self) -> MockProtocolController {
        self.protocol_controller
    }
}
