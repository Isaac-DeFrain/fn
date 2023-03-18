use std::collections::HashMap;

use id_tree::NodeId;

use crate::weighted_tree::{Block, WeightedTree};

struct WeightedForest {
    indices: HashMap<usize, usize>,
    trees: Vec<WeightedTree>,
    weight: u32,
}

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
                let id = tree.insert(block.clone(), None);
                self.indices.insert(idx, self.trees.len());
                self.trees.push(tree);
                id
            }
            Some(i) => match self.trees.get_mut(*i) {
                None => {
                    let mut tree = WeightedTree::new();
                    let id = tree.insert(block.clone(), None);
                    self.trees.push(tree);
                    id
                }
                Some(tree) => {
                    let id = tree.insert(block.clone(), parent);
                    id
                }
            },
        };
        // self.trees.sort();
        id
    }
}

impl std::fmt::Debug for WeightedForest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = Ok(());
        let mut n = 0;
        writeln!(f, "=== Weighted Forest ===\n").unwrap();
        writeln!(f, "Weight: {}\n", self.weight).unwrap();
        let mut trees = self.trees.clone();
        trees.sort();
        for tree in &trees {
            write!(f, "************\n").unwrap();
            write!(f, "** Tree {} **\n", n).unwrap();
            write!(f, "************\n").unwrap();
            write!(f, "* weight: {}\n", tree.weight).unwrap();
            write!(f, "* support\n").unwrap();
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
            n += 1;
            res = write!(f, "* tree\n{:?}\n", tree);
        }
        res
    }
}

#[test]
fn forest_example() {
    use crate::ledger::LedgerDiff;

    let mut forest = WeightedForest::new();
    
    // tree 0
    let id0 = forest.insert(
        Block::new("abc", 10, LedgerDiff::from(&[("def", "abc", 2)])),
        None,
        0,
    );
    let id1 = forest.insert(
        Block::new("def", 15, LedgerDiff::from(&[("abc", "def", 3)])),
        Some(&id0),
        0,
    );
    
    // tree 2
    let id6 = forest.insert(
        Block::new(
            "abc",
            3,
            LedgerDiff::from(&[("def", "abc", 2), ("ghi", "abc", 1)]),
        ),
        None,
        2,
    );

    // tree 0
    let id2 = forest.insert(
        Block::new("abc", 2, LedgerDiff::from(&[("abc", "def", 3)])),
        Some(&id0),
        0,
    );

    // tree 1
    let id4 = forest.insert(
        Block::new("def", 4, LedgerDiff::from(&[("b", "a", 2)])),
        None,
        1,
    );
    let id5 = forest.insert(
        Block::new("ghi", 1, LedgerDiff::from(&[("b", "c", 2)])),
        None,
        1,
    );

    // tree 0
    let id3 = forest.insert(
        Block::new("ghi", 5, LedgerDiff::from(&[("b", "a", 2)])),
        None,
        0,
    );

    println!("{:?}", forest);
    // assert!(false);
}
