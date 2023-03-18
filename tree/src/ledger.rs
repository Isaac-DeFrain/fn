use std::{collections::HashMap, fmt::Debug};

#[derive(Debug, Clone)]
struct Account {
    pk: String,
    delegate: String,
    balance: u64,
    stake: u64,
    delegations: u64,
}

#[derive(Clone)]
pub struct Ledger(HashMap<String, Account>);

#[derive(Clone, PartialEq, Eq)]
pub struct LedgerDiff(HashMap<(String, String), u64>);

impl Account {
    pub fn new(pk: String, balance: u64, delegations: u64) -> Self {
        Self {
            delegate: pk.clone(),
            pk,
            balance,
            stake: balance,
            delegations,
        }
    }
}

impl Ledger {
    pub fn apply(&mut self, diff: LedgerDiff) {
        for ((from, to), amount) in diff.0 {
            match self.0.get_mut(&from) {
                None => {
                    continue;
                }
                Some(from_acct) => {
                    if from_acct.balance < amount {
                        continue;
                    } else {
                        from_acct.stake -= amount;
                        match self.0.get_mut(&to) {
                            None => {
                                self.0.insert(to.clone(), Account::new(to, amount, 0));
                            }
                            Some(to_acct) => {
                                to_acct.balance += amount;
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Debug for Ledger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = writeln!(f, "=== Ledger ===");
        for (pk, acct) in &self.0 {
            writeln!(f, "pk:   {:?},\nacct: {:?}\n", pk, acct).unwrap();
        }
        res
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

impl Debug for LedgerDiff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ ").unwrap();
        for ((from, to), amount) in self.0.clone() {
            write!(f, "{:?} ", (from, to, amount)).unwrap()
        }
        write!(f, "]")
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

    let diff = LedgerDiff::from(&[("a", "b", 20), ("b", "c", 12)]);
    ledger.apply(diff);

    println!("{:?}", ledger);
    // assert!(false);
}
