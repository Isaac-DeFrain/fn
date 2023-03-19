use std::collections::HashMap;

use id_tree::{InsertBehavior::*, Node, NodeId, Tree};

use crate::ledger::block::Block;

#[derive(Clone)]
pub struct WeightedTree {
    pub tree: id_tree::Tree<Block>,
    pub weight: u32,
    pub weights: HashMap<NodeId, u32>,
}

impl WeightedTree {
    pub fn new() -> Self {
        Self {
            tree: Tree::new(),
            weight: 0,
            weights: HashMap::new(),
        }
    }

    fn new_root(&mut self, block: Block) -> NodeId {
        self.weight += block.weight;
        let id = self.tree.insert(Node::new(block.clone()), AsRoot).unwrap();
        self.weights.insert(id.clone(), block.weight);
        id
    }

    fn new_leaf(&mut self, block: Block, parent: &NodeId) -> NodeId {
        self.weight += block.weight;
        let id = self
            .tree
            .insert(Node::new(block), UnderNode(parent))
            .unwrap();
        id
    }

    pub fn insert(&mut self, block: Block, parent: Option<&NodeId>) -> NodeId {
        let id = match parent {
            None => self.new_root(block.clone()),
            Some(parent) => self.new_leaf(block.clone(), parent),
        };
        self.weights.insert(id.clone(), block.weight);
        id
    }

    pub fn support(&self, node_id: &NodeId) -> u32 {
        self.tree
            .traverse_level_order_ids(node_id)
            .unwrap()
            .map(|id| self.tree.get(&id).unwrap().data().weight)
            .reduce(|acc, w| acc + w)
            .unwrap()
    }
}

impl PartialEq for WeightedTree {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl Eq for WeightedTree {}

impl PartialOrd for WeightedTree {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.weight.cmp(&other.weight))
    }
}

impl Ord for WeightedTree {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl std::fmt::Debug for WeightedTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut w = String::new();
        self.tree.write_formatted(&mut w).unwrap();
        write!(f, "{}", w)
    }
}

#[test]
pub fn insert_weighted_block() {
    use crate::ledger::*;

    println!("~~~~~ insert_weighted_block ~~~~~\n");
    // Base tree
    //    (a, 1)
    //    /    \
    // (b, 3) (c, 2)
    //          |
    //        (d, 1)

    let a = "A".to_string();
    let b = "B".to_string();
    let c = "C".to_string();
    let d = "D".to_string();

    let tree = &mut WeightedTree::new();
    let root_id = tree.insert(
        Block::new(&a, 1, LedgerDiff::from(&[(&b, &a, Diff::Transfer(2))])),
        None,
    );
    let node1_id = tree.insert(
        Block::new(&b, 3, LedgerDiff::from(&[(&c, &b, Diff::Transfer(1))])),
        Some(&root_id),
    );
    let _node2_id = tree.insert(
        Block::new(&a, 2, LedgerDiff::from(&[(&c, &b, Diff::Transfer(2))])),
        Some(&root_id),
    );
    let _node3_id = tree.insert(
        Block::new(&c, 1, LedgerDiff::from(&[(&a, &c, Diff::Transfer(1))])),
        Some(&node1_id),
    );

    println!("=== Before adding block ===");
    println!("\n** Weight: {}", tree.weight);
    println!("\n** Support");
    for node_id in tree
        .tree
        .traverse_level_order_ids(tree.tree.root_node_id().unwrap())
        .unwrap()
    {
        println!(
            "{:?}: {:?}",
            tree.tree.get(&node_id).unwrap().data(),
            tree.support(&node_id)
        );
    }

    println!("\n** Ancestors");
    for node_id in tree
        .tree
        .traverse_level_order_ids(tree.tree.root_node_id().unwrap())
        .unwrap()
    {
        println!(
            "{:?} => {:?}",
            tree.tree.get(&node_id).unwrap().data(),
            tree.tree
                .ancestors(&node_id)
                .unwrap()
                .map(|n| n.data())
                .cloned()
                .collect::<Vec<Block>>()
        );
    }

    println!("\n** Tree");
    println!("{:?}", tree);

    // add block

    // final tree
    //    (a, 1)
    //    /    \
    // (c, 3) (b, 2)
    //        /    \
    //     (f, 2) (d, 1)

    let _node4_id = tree.insert(
        Block::new(
            &d,
            2,
            LedgerDiff::from(&[(&b, &c, Diff::Transfer(1)), (&a, &d, Diff::Transfer(2))]),
        ),
        Some(&node1_id),
    );

    println!("=== After adding block ===");
    println!("\n** Weight: {}", tree.weight);
    println!("\n** Support");
    for node_id in tree
        .tree
        .traverse_level_order_ids(tree.tree.root_node_id().unwrap())
        .unwrap()
    {
        println!(
            "{:?}: {:?}",
            tree.tree.get(&node_id).unwrap().data(),
            tree.support(&node_id)
        );
    }

    println!("\n** Ancestors");
    for node_id in tree
        .tree
        .traverse_level_order_ids(tree.tree.root_node_id().unwrap())
        .unwrap()
    {
        println!(
            "{:?} => {:?}",
            tree.tree.get(&node_id).unwrap().data(),
            tree.tree
                .ancestors(&node_id)
                .unwrap()
                .map(|n| n.data())
                .cloned()
                .collect::<Vec<Block>>()
        );
    }

    println!("\n** Tree");
    println!("{:?}", tree);

    // assert!(false); // uncomment to see stdout
}

impl std::fmt::Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ pk: {:?}, weight: {} }}",
            self.pk.clone(),
            self.weight
        )
    }
}
