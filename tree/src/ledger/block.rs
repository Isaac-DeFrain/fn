use crate::ledger::LedgerDiff;

#[derive(Clone, PartialEq, Eq)]
pub struct Block {
    pub pk: String,
    pub diff: LedgerDiff,
    pub weight: u32,
}

#[allow(dead_code)]
impl Block {
    pub fn new(pk: String, weight: u32, diff: LedgerDiff) -> Self {
        Self { pk, diff, weight }
    }
}
