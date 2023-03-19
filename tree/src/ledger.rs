use std::{collections::HashMap, fmt::Debug};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Account {
    pk: String,
    balance: u64,
    delegate: Option<String>,
    delegations: u64,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Ledger {
    total: u64,
    map: HashMap<String, Account>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AccountUpdate {
    pk: String,
    coinbase: bool,
    amount: Option<i64>,
    delegate: Option<String>,
    delegation: Option<u64>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Diff {
    Coinbase(u64),
    Delegation,
    Transfer(u64),
}

#[derive(Clone, PartialEq, Eq)]
pub struct LedgerDiff(HashMap<(String, String), Diff>);

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

#[allow(dead_code)]
impl Ledger {
    pub fn new() -> Self {
        Self {
            total: 0,
            map: HashMap::new(),
        }
    }

    pub fn apply(&mut self, delta: LedgerDiff) -> Result<(), String> {
        for ((from, to), diff) in delta.0 {
            let mut updates = Vec::new();
            match self.map.get(&from) {
                None => {
                    return Err(format!("Error: pk = {:?} is not in the ledger", from));
                }
                Some(Account {
                    pk: _,
                    balance,
                    delegate: _,
                    delegations: _,
                }) => match diff {
                    Diff::Coinbase(amount) => {
                        if from == to {
                            updates.push(AccountUpdate::new(
                                from.clone(),
                                true,
                                Some(amount as i64),
                                None,
                                None,
                            ));
                        }
                    }
                    Diff::Delegation => match self.map.get(&to) {
                        None => {
                            return Err(format!(
                                "[Error] delegation: pk = {:?} is not in the ledger",
                                to
                            ));
                        }
                        Some(_) => {
                            // delegator
                            updates.push(AccountUpdate::new(
                                from.clone(),
                                false,
                                None,
                                Some(to.clone()),
                                None,
                            ));
                            // delegate
                            updates.push(AccountUpdate::new(
                                to.clone(),
                                false,
                                None,
                                None,
                                Some(*balance),
                            ))
                        }
                    },
                    Diff::Transfer(amount) => {
                        if *balance < amount {
                            return Err(format!("[Error] transfer: pk = {:?} has insufficient funds (balance = {}, amount = {})", from, balance.clone(), amount));
                        } else {
                            match self.map.get(&to) {
                                None => {
                                    return Err(format!(
                                        "[Error] transfer: pk = {:?} is not in the ledger",
                                        to
                                    ));
                                }
                                Some(_) => {
                                    // from
                                    updates.push(AccountUpdate::new(
                                        from.clone(),
                                        false,
                                        Some(-(amount as i64)),
                                        None,
                                        None,
                                    ));
                                    // to
                                    updates.push(AccountUpdate::new(
                                        to.clone(),
                                        false,
                                        Some(amount as i64),
                                        None,
                                        None,
                                    ));
                                }
                            }
                        }
                    }
                },
            }
            // apply all updates for the diff
            for AccountUpdate {
                pk,
                coinbase,
                amount,
                delegate,
                delegation,
            } in updates
            {
                if let Some(account) = self.map.get_mut(&pk) {
                    // coinbase or transfer
                    if let Some(amount) = amount {
                        account.balance = (account.balance as i64 + amount) as u64;
                        if coinbase {
                            assert!(amount >= 0);
                            self.total += amount as u64;
                        }
                    } else {
                        // delegation
                        match delegation {
                            None => {
                                // delegator
                                if delegate.is_some() {
                                    account.delegate = delegate
                                }
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

impl AccountUpdate {
    pub fn new(
        pk: String,
        coinbase: bool,
        amount: Option<i64>,
        delegate: Option<String>,
        delegation: Option<u64>,
    ) -> Self {
        Self {
            pk,
            amount,
            coinbase,
            delegate,
            delegation,
        }
    }
}

#[allow(dead_code)]
impl LedgerDiff {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn from(updates: &[(&str, &str, Diff)]) -> Self {
        let mut delta = LedgerDiff::new();
        for (from, to, diff) in updates.iter().cloned() {
            delta.0.insert((from.to_string(), to.to_string()), diff);
        }
        delta
    }

    pub fn insert(&mut self, from: &str, to: &str, diff: Diff) {
        self.0.insert((from.to_string(), to.to_string()), diff);
    }
}

#[test]
pub fn ledger_example() {
    let mut ledger = Ledger::new();

    let a = "a".to_string();
    let b = "b".to_string();
    let c = "c".to_string();
    let a_balance = 100;
    let b_balance = 150;
    let c_balance = 87;

    // insert accounts
    ledger
        .map
        .insert(a.clone(), Account::new(a.clone(), a_balance, 25));
    ledger
        .map
        .insert(b.clone(), Account::new(b.clone(), b_balance, 5));
    ledger
        .map
        .insert(c.clone(), Account::new(c.clone(), c_balance, 10));
    ledger.total = a_balance + b_balance + c_balance;

    println!("=== Initial ===");
    println!("{:?}", ledger);

    let diff0 = LedgerDiff::from(&[("a", "b", Diff::Transfer(20))]);
    ledger.apply(diff0).expect("diff application is ok");

    println!("=== After transfer: a -> b (20) ===");
    println!("{:?}", ledger);

    let diff1 = LedgerDiff::from(&[("a", "a", Diff::Coinbase(5))]);
    let old_ledger = ledger.clone();
    ledger.apply(diff1.clone()).expect("diff application is ok");
    // TODO total increase
    match diff1.0.get(&(a.clone(), a.clone())).unwrap() {
        Diff::Coinbase(n) => {
            assert_eq!(ledger.total, old_ledger.total + n);
        }
        _ => {
            assert!(false);
        }
    }

    // TODO balance increase

    println!("=== After coinbase: a (5) ===");
    println!("{:?}", ledger);

    let diff2 = LedgerDiff::from(&[("a", "b", Diff::Delegation)]);
    let old_ledger = ledger.clone();
    ledger.apply(diff2).expect("diff application is ok");
    for (pk, old_account) in &old_ledger.map {
        let new_account = ledger.map.get(pk).unwrap();
        assert_eq!(new_account.balance, old_account.balance);
        if pk == &a {
            // delegation
            assert_eq!(old_account.delegate, None);
            assert_eq!(new_account.delegate, Some(b.clone()));
            // other
            assert_eq!(old_account.balance, old_account.balance);
            assert_eq!(old_account.delegations, old_account.delegations);
        } else if pk == &b {
            let a_balance = ledger.map.get(&a).unwrap().balance;
            assert_eq!(new_account.delegations, old_account.delegations + a_balance)
        }
    }

    println!("=== After delegation: a -> b ===");
    assert_eq!(
        ledger.map.iter().fold(0, |acc, (_, a)| acc + a.balance),
        ledger.total
    );
    println!("{:?}", ledger);
    // assert!(false);
}

impl Debug for Ledger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = writeln!(f, "=== Ledger ===");
        for (pk, acct) in &self.map {
            writeln!(f, "pk:   {:?},\nacct: {:?}\n", pk, acct).unwrap();
        }
        res
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
