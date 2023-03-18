use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Account {
    pk: String,
    stake: u64,
    delegations: u64,
}

#[derive(Debug)]
pub struct Ledger(HashMap<String, Account>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerDiff(HashMap<(String, String), u64>);

impl Account {
    pub fn new(pk: String, stake: u64, delegations: u64) -> Self {
        Self {
            pk,
            stake,
            delegations,
        }
    }
}

impl LedgerDiff {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn from(updates: &[(&str, &str, u64)]) -> Self {
        let mut diff = LedgerDiff::new();
        for (from, to, amount) in updates.to_vec() {
            diff.0.insert((from.to_string(), to.to_string()), amount);
        }
        diff
    }

    pub fn insert(&mut self, from: &str, to: &str, amount: u64) {
        self.0.insert((from.to_string(), to.to_string()), amount);
    }
}

#[test]
pub fn ledger_example() {
    let mut ledger = Ledger(HashMap::new());

    let bp0 = "a".to_string();
    let bp1 = "b".to_string();
    let bp2 = "c".to_string();

    // insert accounts
    ledger
        .0
        .insert(bp0.clone(), Account::new(bp0.clone(), 100, 0));
    ledger
        .0
        .insert(bp1.clone(), Account::new(bp1.clone(), 150, 5));
    ledger
        .0
        .insert(bp2.clone(), Account::new(bp2.clone(), 87, 10));

    println!("{:?}", ledger);
    // assert!(false);
}
