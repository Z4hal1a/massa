//! Copyright (c) 2023 MASSA LABS <info@massa.net>

use std::sync::Arc;

use parking_lot::RwLock;

use crate::active_history::{ActiveHistory, HistorySearchResult};
use massa_executed_ops::ProcessedDenunciationsChanges;
use massa_final_state::FinalState;
use massa_models::denunciation::DenunciationIndex;

pub(crate) struct SpeculativeProcessedDenunciations {
    /// Thread-safe shared access to the final state. For reading only.
    final_state: Arc<RwLock<FinalState>>,

    /// History of the outputs of recently executed slots.
    /// Slots should be consecutive, newest at the back.
    active_history: Arc<RwLock<ActiveHistory>>,

    /// executed operations: maps the operation ID to its validity slot end - included
    processed_denunciations: ProcessedDenunciationsChanges,
}

impl SpeculativeProcessedDenunciations {
    /// Creates a new `SpeculativeProcessedDenunciations`
    ///
    /// # Arguments
    /// * `final_state`: thread-safe shared access the the final state
    /// * `active_history`: thread-safe shared access the speculative execution history
    pub fn new(
        final_state: Arc<RwLock<FinalState>>,
        active_history: Arc<RwLock<ActiveHistory>>,
    ) -> Self {
        Self {
            final_state,
            active_history,
            processed_denunciations: Default::default(),
        }
    }

    /// Returns the set of operation IDs caused to the `SpeculativeProcessedDenunciations` since
    /// its creation, and resets their local value to nothing
    pub fn take(&mut self) -> ProcessedDenunciationsChanges {
        std::mem::take(&mut self.processed_denunciations)
    }

    /// Takes a snapshot (clone) of the changes since its creation
    pub fn get_snapshot(&self) -> ProcessedDenunciationsChanges {
        self.processed_denunciations.clone()
    }

    /// Resets the `SpeculativeRollState` to a snapshot (see `get_snapshot` method)
    pub fn reset_to_snapshot(&mut self, snapshot: ProcessedDenunciationsChanges) {
        self.processed_denunciations = snapshot;
    }

    /// Checks if a denunciation was processed previously
    pub fn is_de_processed(&self, de_idx: &DenunciationIndex) -> bool {
        // check in the current changes
        if self.processed_denunciations.contains(de_idx) {
            return true;
        }

        // check in the active history, backwards
        match self.active_history.read().fetch_processed_de(de_idx) {
            HistorySearchResult::Present(_) => {
                return true;
            }
            HistorySearchResult::Absent => {
                return false;
            }
            HistorySearchResult::NoInfo => {}
        }

        // check in the final state
        self.final_state
            .read()
            .processed_denunciations
            .contains(de_idx)
    }

    /// Insert a processed denunciation.
    /// Does not check for reuse, please use `SpeculativeExecutedOps::is_de_processed` before.
    pub fn insert_processed_de(&mut self, de_idx: DenunciationIndex) {
        self.processed_denunciations.insert(de_idx);
    }
}
