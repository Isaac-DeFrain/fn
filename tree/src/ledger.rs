use std::{collections::HashMap, fmt::Debug};

#[derive(Debug, Clone)]
pub struct Account {
    pk: String,
    delegate: Option<String>,
    balance: u64,
    delegations: u64,
}

#[derive(Debug, Clone)]
pub struct AccountUpdate {
    pk: String,
    amount: Option<i64>,
    delegate: Option<String>,
    delegation: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Diff {
    Coinbase(u64),
    Delegation,
    Transfer(u64),
}

#[derive(Clone)]
pub struct Ledger(HashMap<String, Account>);

#[derive(Clone, PartialEq, Eq)]
pub struct LedgerDiff(HashMap<(String, String), Diff>);

impl AccountUpdate {
    pub fn new(pk: String, amount: Option<i64>, delegate: Option<String>, delegation: Option<u64>) -> Self {
        Self { pk, amount, delegate, delegation }
    }
}

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

impl Ledger {
    pub fn apply(&mut self, delta: LedgerDiff) -> Result<(), String> {
        for ((from, to), diff) in delta.0 {
            let mut updates = Vec::new();
            match self.0.get(&from) {
                None => {
                    return Err(format!("Error: pk = {:?} is not in the ledger", from));
                }
                Some(Account { pk: _pk, balance, delegate: _delegate, delegations: _delegations }) => {
                    match diff {
                        Diff::Coinbase(amount) => {
                            if from == to {
                                updates.push(AccountUpdate::new(from.clone(), Some(amount as i64), None, None));
                            }
                        }
                        Diff::Delegation => {
                            match self.0.get(&to) {
                                None => {
                                    return Err(format!("[Error] delegation: pk = {:?} is not in the ledger", to));
                                }
                                Some(_) => {
                                    updates.push(AccountUpdate::new(from.clone(), None, Some(to.clone()), None));
                                    updates.push(AccountUpdate::new(to.clone(), None, Some(from.clone()), Some(balance.clone())))
                                }
                            }
                        }
                        Diff::Transfer(amount) => {
                            if balance.clone() < amount {
                                return Err(format!("[Error] transfer: pk = {:?} has insufficient funds (balance = {}, amount = {})", from, balance.clone(), amount));
                            } else {
                                match self.0.get(&to) {
                                    None => {
                                        return Err(format!("[Error] transfer: pk = {:?} is not in the ledger", to));
                                    }
                                    Some(to_acct) => {
                                        updates.push(AccountUpdate::new(from.clone(), Some(-(amount as i64)), None, None));
                                        updates.push(AccountUpdate::new(to.clone(), Some(amount as i64), None, None));
                                    }
                                }
                            }
                        }
                    }
                }
            }
            for AccountUpdate { pk, amount, delegate, delegation } in updates {
                if let Some(account) = self.0.get_mut(&pk) {
                    if let Some(amount) = amount {
                        // coinbase or transfer
                        account.balance = (account.balance as i64 + amount) as u64;
                    } else if let Some(_) = delegate {
                        match delegation {
                            None => {
                                // delegator
                                continue;
                            }
                            Some(amount) => {
                                // delegate
                                account.delegations += amount;
                            }
                        }
                    }
                }
            }
        }
        Ok(())
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

    pub fn from(updates: &[(&str, &str, Diff)]) -> Self {
        let mut delta = LedgerDiff::new();
        for (from, to, diff) in updates.to_vec() {
            delta.0.insert((from.to_string(), to.to_string()), diff);
        }
        delta
    }

    pub fn insert(&mut self, from: &str, to: &str, diff: Diff) {
        self.0.insert((from.to_string(), to.to_string()), diff);
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
        .insert(bp0.clone(), Account::new(bp0.clone(), 100, 25));
    ledger
        .0
        .insert(bp1.clone(), Account::new(bp1.clone(), 150, 5));
    ledger
        .0
        .insert(bp2.clone(), Account::new(bp2.clone(), 87, 10));

    println!("=== Initial ===");
    println!("{:?}", ledger);
    
    let diff0 = LedgerDiff::from(&[("a", "b", Diff::Transfer(20))]);
    ledger.apply(diff0).expect("diff application is ok");
    
    println!("=== After transfer: a -20-> b ===");
    println!("{:?}", ledger);
    
    let diff1 = LedgerDiff::from(&[("a", "a", Diff::Coinbase(5))]);
    ledger.apply(diff1).expect("diff application is ok");
    
    println!("=== After coinbase: a -5-> a ===");
    println!("{:?}", ledger);
    
    let diff2 = LedgerDiff::from(&[("a", "b", Diff::Delegation)]);
    ledger.apply(diff2).expect("diff application is ok");
    
    println!("=== After delegation: a -> b ===");
    println!("{:?}", ledger);
    assert!(false);
}
