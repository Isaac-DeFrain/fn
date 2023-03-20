use crate::ledger::Diff;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Account {
    pub pk: String,
    pub balance: u64,
    pub delegate: Option<String>,
    pub delegations: u64,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AccountUpdate {
    pub diff: Diff,
}

#[allow(dead_code)]
impl Account {
    pub fn new(pk: String, balance: u64, delegations: u64) -> Self {
        Self {
            delegate: None,
            pk,
            balance,
            delegations,
        }
    }
}
