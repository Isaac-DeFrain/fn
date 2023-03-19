pub(crate) mod account;
pub(crate) mod block;

use crate::ledger::account::{Account, AccountUpdate};
use std::collections::HashMap;

// types

#[allow(dead_code)]
#[derive(Clone, PartialEq)]
pub struct Ledger {
    pub total: u64,
    map: HashMap<String, Account>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Diff {
    Creation(String),
    Coinbase(u64),
    Delegation,
    Transfer(u64),
}

#[derive(Clone, PartialEq, Eq)]
pub struct LedgerDiff(HashMap<(String, String), Diff>);

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    InvalidAbsentFromLedger(String),
    DelegationAbsentFromLedger(String),
    InsufficientFunds(String, u64, u64),
    TransferAbsentFromLedger(String),
    InvalidTransactionData,
}

// impls

#[allow(dead_code)]
impl Ledger {
    pub fn new() -> Self {
        Self {
            total: 0,
            map: HashMap::new(),
        }
    }

    pub fn apply(&mut self, delta: LedgerDiff) -> Result<(), Error> {
        for ((from, to), diff) in delta.0 {
            let mut updates = Vec::new();
            match self.map.get(&from) {
                None => {
                    if let Diff::Creation(pk) = diff {
                        updates.push(AccountUpdate::new(pk, false, None, None, None));
                    } else {
                        return Err(Error::InvalidAbsentFromLedger(from));
                    }
                }
                Some(Account {
                    pk: _,
                    balance,
                    delegate: _,
                    delegations: _,
                }) => match diff {
                    Diff::Creation(pk) => {
                        updates.push(AccountUpdate::new(pk, false, None, None, None));
                    }
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
                            return Err(Error::DelegationAbsentFromLedger(to));
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
                                Some(from.to_string()),
                                Some(*balance),
                            ))
                        }
                    },
                    Diff::Transfer(amount) => {
                        if *balance < amount {
                            return Err(Error::InsufficientFunds(from, *balance, amount));
                        } else {
                            match self.map.get(&to) {
                                None => {
                                    return Err(Error::TransferAbsentFromLedger(to));
                                }
                                Some(_) => {
                                    // - from
                                    updates.push(AccountUpdate::new(
                                        from.clone(),
                                        false,
                                        Some(-(amount as i64)),
                                        None,
                                        None,
                                    ));
                                    // + to
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
                match self.map.get_mut(&pk) {
                    None => {
                        // creation
                        if !coinbase
                            && amount.is_none()
                            && delegate.is_none()
                            && delegation.is_none()
                        {
                            // insert pk account into ledger
                            self.map.insert(pk.clone(), Account::new(pk, 0, 0));
                        } else {
                            //
                            return Err(Error::InvalidAbsentFromLedger(to));
                        }
                    }
                    Some(account) => {
                        match amount {
                            // coinbase or transfer
                            Some(amount) => {
                                account.balance = (account.balance as i64 + amount) as u64;
                                if coinbase {
                                    assert!(amount >= 0);
                                    self.total += amount as u64;
                                }
                            }
                            None => {
                                // delegation
                                match (delegation, delegate) {
                                    // delegator
                                    (None, delegate) => account.delegate = delegate,
                                    // delegate
                                    (Some(amount), Some(_)) => {
                                        account.delegations += amount;
                                    }
                                    _ => {
                                        return Err(Error::InvalidTransactionData);
                                    }
                                }
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

// debug

impl std::fmt::Debug for Ledger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = writeln!(f, "=== Ledger ===");
        for (pk, acct) in &self.map {
            writeln!(f, "pk:   {:?},\nacct: {:?}\n", pk, acct).unwrap();
        }
        res
    }
}

impl std::fmt::Debug for LedgerDiff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ ").unwrap();
        for ((from, to), amount) in self.0.clone() {
            write!(f, "{:?} ", (from, to, amount)).unwrap()
        }
        write!(f, "]")
    }
}

// unit tests

#[test]
pub fn ledger_example() {
    let mut ledger = Ledger::new();

    // initial pks + balances
    let (a, a_balance) = ("A".to_string(), 100);
    let (b, b_balance) = ("B".to_string(), 150);
    let (c, c_balance) = ("C".to_string(), 87);

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

    // Transfer
    // A sends 20 to B
    let trans_amt = 20;
    let diff0 = LedgerDiff::from(&[(&a, &b, Diff::Transfer(trans_amt))]);
    let old_ledger = ledger.clone();
    ledger.apply(diff0).expect("transfer apply is ok");

    let from_account = old_ledger.map.get(&a).unwrap();
    if from_account.balance < trans_amt {
        assert_eq!(ledger, old_ledger);
    }

    println!(
        "=== After transfer: {:?} -> {:?} ({}) ===",
        a.clone(),
        b.clone(),
        trans_amt
    );
    println!("{:?}", ledger);

    // Coinbase
    // A gets a 5 coinbase
    let cb_amt = 5;
    let diff1 = LedgerDiff::from(&[(&a, &a, Diff::Coinbase(cb_amt))]);
    let old_ledger = ledger.clone();
    ledger.apply(diff1.clone()).expect("coinbase apply is ok");

    // total increase
    match diff1.0.get(&(a.clone(), a.clone())).unwrap() {
        Diff::Coinbase(n) => {
            assert_eq!(ledger.total, old_ledger.total + n);
        }
        _ => {
            assert!(false);
        }
    }

    // balance increase
    assert_eq!(
        ledger.map.get(&a).unwrap().balance,
        old_ledger.map.get(&a).unwrap().balance + cb_amt
    );

    println!("=== After coinbase: * -> A ({}) ===", cb_amt);
    println!("{:?}", ledger);

    // Delegation
    // A delegates to B
    let diff2 = LedgerDiff::from(&[(&a, &b, Diff::Delegation)]);
    let old_ledger = ledger.clone();
    ledger.apply(diff2).expect("delegation apply is ok");
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

    println!(
        "=== After delegation: A({}), B({}) -> B({}) ===",
        old_ledger.map.get(&a).unwrap().balance,
        old_ledger.map.get(&b).unwrap().delegations,
        ledger.map.get(&b).unwrap().delegations,
    );

    // sum of all account balances = total
    assert_eq!(
        ledger.map.iter().fold(0, |acc, (_, a)| acc + a.balance),
        ledger.total
    );

    // Creation
    // Create account D
    let d = "D".to_string();
    let diff3 = LedgerDiff::from(&[(&d, &d, Diff::Creation(d.clone()))]);
    let old_ledger = ledger.clone();

    // D is not in the ledger
    assert!(!old_ledger.map.contains_key(&d));

    // after diff apply, D is in the ledger
    ledger.apply(diff3).expect("creation apply is ok");
    assert!(ledger.map.contains_key(&d));

    // Errors
    // errors don't change the ledger

    // coinbase error
    let e = "E".to_string();
    let diff4 = LedgerDiff::from(&[(&e, &e, Diff::Coinbase(10))]);
    let old_ledger = ledger.clone();
    assert_eq!(
        ledger.apply(diff4),
        Err(Error::InvalidAbsentFromLedger(e.clone()))
    );
    assert_eq!(old_ledger, ledger);

    // delegation error
    let diff5 = LedgerDiff::from(&[(&e, &e, Diff::Delegation)]);
    let old_ledger = ledger.clone();
    assert_eq!(
        ledger.apply(diff5),
        Err(Error::InvalidAbsentFromLedger(e.clone()))
    );
    assert_eq!(old_ledger, ledger);

    assert!(!ledger.map.contains_key(&e));
    let diff6 = LedgerDiff::from(&[(&e, &d, Diff::Transfer(5))]);
    let old_ledger = ledger.clone();
    assert_eq!(
        ledger.apply(diff6),
        Err(Error::InvalidAbsentFromLedger(e.clone()))
    );
    assert_eq!(old_ledger, ledger);

    // receiver doesn't exist
    let diff7 = LedgerDiff::from(&[(&c, &e, Diff::Transfer(5))]);
    let old_ledger = ledger.clone();
    assert_eq!(ledger.apply(diff7), Err(Error::TransferAbsentFromLedger(e)));
    assert_eq!(old_ledger, ledger);

    // insufficient funds error
    let diff8 = LedgerDiff::from(&[(&a, &b, Diff::Transfer(500))]);
    let old_ledger = ledger.clone();
    assert_eq!(
        ledger.apply(diff8),
        Err(Error::InsufficientFunds(a, 85, 500))
    );
    assert_eq!(old_ledger, ledger);

    // print final ledger
    println!("{:?}", ledger);

    // assert!(false); // uncomment to see stdout
}
