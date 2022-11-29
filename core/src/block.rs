use crate::{pb::antelope as pb};

impl pb::Block {

    /// returns all transaction traces from the block.
    pub fn all_transaction_traces(&self) -> impl Iterator<Item = &pb::TransactionTrace> {
        return if self.filtering_applied == true {
            self.filtered_transaction_traces.iter()
        } else {
            self.unfiltered_transaction_traces.iter()
        }
    }

    /// returns the number of transaction traces included in this block
    pub fn transaction_traces_count(&self) -> u32 {
        return if self.filtering_applied == true {
            self.filtered_transaction_count
        } else {
            self.unfiltered_transaction_count
        }
    }

}
