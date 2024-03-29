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
    Coinbase(String, u64),
    Delegation(String, String),
    Transfer(String, String, u64),
}

#[derive(Clone, PartialEq, Eq)]
pub struct LedgerDiff(HashMap<(String, String), Diff>);

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    InvalidCreation(String),
    InvalidAbsentFromLedger(String),
    InvalidTransfer,
    DelegateAbsentFromLedger(String),
    InsufficientFunds(String, u64, u64),
    TransferAbsentFromLedger(String),
    InvalidUpdate,
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
        // iterate over diffs to make account updates
        for ((from, to), diff) in delta.0 {
            let mut updates = Vec::new();
            match self.map.get(&from) {
                // if not in the ledger, must be creation
                None => {
                    if let Diff::Creation(pk) = diff {
                        updates.push(AccountUpdate::new(None, &pk, false, None, None));
                    } else {
                        return Err(Error::InvalidAbsentFromLedger(from));
                    }
                }
                // non-creation
                Some(Account {
                    pk: _,
                    balance,
                    delegate: _,
                    delegations: _,
                }) => match diff {
                    Diff::Creation(pk) => {
                        return Err(Error::InvalidCreation(pk));
                    }
                    Diff::Coinbase(pk, amount) => {
                        if from == to {
                            updates.push(AccountUpdate::new(None, &pk, true, Some(amount), None));
                        }
                    }
                    // delegation
                    Diff::Delegation(from, delegate) => match self.map.get(&delegate) {
                        None => {
                            return Err(Error::DelegateAbsentFromLedger(to));
                        }
                        Some(_) => {
                            updates.push(AccountUpdate::new(
                                Some(&from),
                                &delegate,
                                false,
                                None,
                                Some(delegate.clone()),
                            ));
                        }
                    },
                    Diff::Transfer(from, to, amount) => {
                        if *balance < amount {
                            return Err(Error::InsufficientFunds(from, *balance, amount));
                        } else {
                            match self.map.get(&to) {
                                None => {
                                    return Err(Error::TransferAbsentFromLedger(to.clone()));
                                }
                                Some(_) => {
                                    updates.push(AccountUpdate::new(
                                        Some(&from),
                                        &to,
                                        false,
                                        Some(amount),
                                        None,
                                    ));
                                }
                            }
                        }
                    }
                },
            }
            // apply all updates for the diff
            for update in updates {
                match update {
                    Err(err) => {
                        println!("{:?}", err);
                        return Err(Error::InvalidUpdate);
                    }
                    Ok(AccountUpdate { diff }) => {
                        match diff {
                            Diff::Coinbase(pk, amount) => {
                                if let Some(account) = self.map.get_mut(&pk) {
                                    account.balance += amount;
                                    self.total += amount;
                                } else {
                                    return Err(Error::InvalidAbsentFromLedger(pk.clone()));
                                }
                            }
                            Diff::Creation(pk) => {
                                self.map.insert(pk.clone(), Account::new(to.clone(), 0, 0));
                            }
                            Diff::Delegation(from, to) => {
                                self.map.get_mut(&from).unwrap().delegate = Some(to.clone());
                                self.map.get_mut(&to).unwrap().delegations +=
                                    self.map.get(&from).unwrap().balance;
                            }
                            Diff::Transfer(from, to, amount) => {
                                let from_account = self.map.get_mut(&from).unwrap();
                                println!("{}", from_account.balance);
                                if from_account.balance >= amount {
                                    from_account.balance -= amount;
                                    self.map.get_mut(&to).unwrap().balance += amount;
                                } else {
                                    // println!("{:?}, {:?}, {:?}, {:?}", from, to, from_account.balance, amount);
                                    return Err(Error::InvalidTransfer);
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

#[derive(Debug, Clone)]
pub struct UpdateError {
    pub from: Option<String>,
    pub to: String,
    pub coinbase: bool,
    pub amount: Option<u64>,
    pub delegate: Option<String>,
}

impl AccountUpdate {
    fn new(
        from: Option<&str>,
        to: &str,
        coinbase: bool,
        amount: Option<u64>,
        delegate: Option<String>,
    ) -> Result<Self, UpdateError> {
        match (coinbase, amount, delegate.clone(), from) {
            // coinbase (coinbase = true)
            (true, _, _, None) => {
                // validate positive amount
                if delegate.is_none() {
                    Ok(Self {
                        diff: Diff::Coinbase(to.to_string(), amount.unwrap()),
                    })
                } else {
                    Err(UpdateError {
                        from: from.map(|x| x.to_string()),
                        to: to.to_string(),
                        coinbase,
                        amount,
                        delegate,
                    })
                }
            }
            // creation (coinbase = false && everything None)
            (_, None, None, _) => Ok(Self {
                diff: Diff::Creation(to.to_string()),
            }),
            // delegation (coinbase = false && amount = None)
            (_, None, Some(delegate), Some(from)) => Ok(Self {
                diff: Diff::Delegation(from.to_string(), delegate),
            }),
            // transfer (amount.is_some())
            (_, Some(amount), None, Some(from)) => Ok(Self {
                diff: Diff::Transfer(from.to_string(), to.to_string(), amount),
            }),
            _ => {
                let from = from.map(|x| x.to_string());
                Err(UpdateError {
                    from,
                    to: to.to_string(),
                    coinbase,
                    amount,
                    delegate,
                })
            }
        }
    }
}

#[allow(dead_code)]
impl LedgerDiff {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn from(diffs: &[Diff]) -> Self {
        let mut delta = LedgerDiff::new();
        for diff in diffs.iter().cloned() {
            match diff.clone() {
                Diff::Coinbase(pk, _) => {
                    delta.0.insert((pk.clone(), pk), diff);
                }
                Diff::Creation(pk) => {
                    delta.0.insert((pk.clone(), pk), diff);
                }
                Diff::Delegation(from, to) => {
                    delta.0.insert((from, to), diff);
                }
                Diff::Transfer(from, to, _) => {
                    delta.0.insert((from, to), diff);
                }
            }
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
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("~~~~~ ledger_example ~~~~~");
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~\n");

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
    let diff0 = LedgerDiff::from(&[Diff::Transfer(a.clone(), b.clone(), trans_amt)]);
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
    let diff1 = LedgerDiff::from(&[Diff::Coinbase(a.clone(), cb_amt)]);
    let old_ledger = ledger.clone();
    ledger.apply(diff1.clone()).expect("coinbase apply is ok");

    // total increase
    match diff1.0.get(&(a.clone(), a.clone())).unwrap() {
        Diff::Coinbase(_, amount) => {
            assert_eq!(ledger.total, old_ledger.total + amount);
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
    let diff2 = LedgerDiff::from(&[Diff::Delegation(a.clone(), b.clone())]);
    let old_ledger = ledger.clone();

    ledger.apply(diff2).expect("delegation apply is ok");
    for (pk, old_account) in &old_ledger.map {
        let new_account = ledger.map.get(pk).unwrap();
        println!(
            "pk: {:?}, before: {}, after: {}",
            pk, old_account.balance, new_account.balance
        );
        assert_eq!(new_account.balance, old_account.balance);
        if pk == &a {
            // delegation
            assert_eq!(old_account.delegate, None);
            assert_eq!(new_account.delegate, Some(b.clone()));
            assert_eq!(new_account.delegations, old_account.delegations);
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
    let diff3 = LedgerDiff::from(&[Diff::Creation(d.clone())]);
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
    let diff4 = LedgerDiff::from(&[Diff::Coinbase(e.clone(), 10)]);
    let old_ledger = ledger.clone();
    assert_eq!(
        ledger.apply(diff4),
        Err(Error::InvalidAbsentFromLedger(e.clone()))
    );
    assert_eq!(old_ledger, ledger);

    // delegation error
    let diff5 = LedgerDiff::from(&[Diff::Delegation(e.clone(), e.clone())]);
    let old_ledger = ledger.clone();
    assert_eq!(
        ledger.apply(diff5),
        Err(Error::InvalidAbsentFromLedger(e.clone()))
    );
    assert_eq!(old_ledger, ledger);

    assert!(!ledger.map.contains_key(&e));
    let diff6 = LedgerDiff::from(&[Diff::Transfer(e.clone(), d.clone(), 5)]);
    let old_ledger = ledger.clone();
    assert_eq!(
        ledger.apply(diff6),
        Err(Error::InvalidAbsentFromLedger(e.clone()))
    );
    assert_eq!(old_ledger, ledger);

    // receiver doesn't exist
    let diff7 = LedgerDiff::from(&[Diff::Transfer(c, e.clone(), 5)]);
    let old_ledger = ledger.clone();
    assert_eq!(ledger.apply(diff7), Err(Error::TransferAbsentFromLedger(e)));
    assert_eq!(old_ledger, ledger);

    // insufficient funds error
    let diff8 = LedgerDiff::from(&[Diff::Transfer(a.clone(), b, 500)]);
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
