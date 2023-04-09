use std::collections::HashMap;

use id_tree::NodeId;

use crate::{ledger::block::Block, weighted::weighted_tree::WeightedTree};

#[allow(dead_code)]
struct WeightedForest {
    indices: HashMap<usize, usize>,
    trees: Vec<WeightedTree>,
    weight: u32,
}

#[allow(dead_code)]
impl WeightedForest {
    pub fn new() -> Self {
        Self {
            indices: HashMap::new(),
            trees: Vec::new(),
            weight: 0,
        }
    }

    pub fn insert(&mut self, block: Block, parent: Option<&NodeId>, idx: usize) -> NodeId {
        self.weight += block.weight;
        let id = match self.indices.get(&idx) {
            None => {
                let mut tree = WeightedTree::new();
                let id = tree.insert(block, None);
                self.indices.insert(idx, self.trees.len());
                self.trees.push(tree);
                id
            }
            Some(i) => match self.trees.get_mut(*i) {
                None => {
                    let mut tree = WeightedTree::new();
                    let id = tree.insert(block, None);
                    self.trees.push(tree);
                    id
                }
                Some(tree) => tree.insert(block, parent),
            },
        };
        id
    }
}

#[test]
#[allow(unused_variables)]
fn forest_example() {
    use crate::ledger::*;

    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("~~~~~ forest_example ~~~~~\n");
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~");

    let mut forest = WeightedForest::new();
    let a = "A".to_string();
    let b = "B".to_string();
    let c = "C".to_string();

    // add to tree 0
    let id0 = forest.insert(
        Block::new(&a, 10, LedgerDiff::from(&[Diff::Coinbase(a.clone(), 2)])),
        None,
        0,
    );
    let id1 = forest.insert(
        Block::new(
            &b,
            15,
            LedgerDiff::from(&[Diff::Transfer(a.clone(), b.clone(), 3)]),
        ),
        Some(&id0),
        0,
    );

    // add to tree 2
    let id6 = forest.insert(
        Block::new(
            &a,
            3,
            LedgerDiff::from(&[
                Diff::Transfer(b.clone(), c.clone(), 2),
                Diff::Coinbase(c.clone(), 3),
            ]),
        ),
        None,
        2,
    );

    // add to tree 0
    let id2 = forest.insert(
        Block::new(
            &a,
            2,
            LedgerDiff::from(&[Diff::Transfer(a.clone(), b.clone(), 3)]),
        ),
        Some(&id0),
        0,
    );

    // add to tree 1
    let id4 = forest.insert(
        Block::new(
            &b,
            4,
            LedgerDiff::from(&[Diff::Transfer(b.clone(), a.clone(), 2)]),
        ),
        None,
        1,
    );
    let id5 = forest.insert(
        Block::new(&c, 1, LedgerDiff::from(&[Diff::Transfer(a.clone(), b, 2)])),
        None,
        1,
    );

    // add to tree 0
    let id3 = forest.insert(
        Block::new(&c, 5, LedgerDiff::from(&[Diff::Transfer(c.clone(), a, 2)])),
        None,
        0,
    );

    // print final forest
    println!("{:?}", forest);

    // assert!(false); // uncomment to see stdout
}

impl std::fmt::Debug for WeightedForest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = Ok(());
        writeln!(f, "=== Weighted Forest ===\n").unwrap();
        writeln!(f, "Forest weight: {}\n", self.weight).unwrap();
        let mut trees = self.trees.clone();
        trees.sort();
        for (n, tree) in trees.iter().enumerate() {
            writeln!(f, "**********").unwrap();
            writeln!(f, "* Tree {} *", n).unwrap();
            writeln!(f, "**********").unwrap();
            writeln!(f, "* tree weight: {}", tree.weight).unwrap();
            writeln!(f, "* support").unwrap();
            for node_id in tree
                .tree
                .traverse_level_order_ids(tree.tree.root_node_id().unwrap())
                .unwrap()
            {
                writeln!(
                    f,
                    "  - [ {:?}: {:?} ]",
                    tree.support(&node_id),
                    tree.tree.get(&node_id).unwrap().data()
                )
                .unwrap();
            }
            res = write!(f, "* tree\n{:?}\n", tree);
        }
        res
    }
}
