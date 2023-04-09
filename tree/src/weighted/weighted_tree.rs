use std::collections::{HashMap, HashSet};

use id_tree::{InsertBehavior::*, Node, NodeId, NodeIdError, Tree};

use crate::ledger::block::Block;

#[derive(Clone)]
pub struct WeightedTree {
    pub tree: id_tree::Tree<Block>,
    /// total tree weight (sum of max BP weights)
    pub weight: u32,
    /// each block's weight
    pub weights: HashMap<NodeId, u32>,
    /// each BP's max weight
    pub pk_weights: HashMap<String, u32>,
    /// disjoint sets of each BP's blocks
    pub pk_blocks: HashMap<String, HashSet<NodeId>>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum WeightError {
    WeightCalculationError,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum TreeError {
    NodeIdError(NodeIdError),
    WeightError(WeightError),
}

impl WeightedTree {
    pub fn new() -> Self {
        Self {
            tree: Tree::new(),
            weight: 0,
            weights: HashMap::new(),
            pk_weights: HashMap::new(),
            pk_blocks: HashMap::new(),
        }
    }

    fn add_bp_block_id(&mut self, pk: &str, node_id: &NodeId) {
        match self.pk_blocks.get_mut(pk) {
            None => {
                self.pk_blocks
                    .insert(pk.to_string(), HashSet::from([node_id.clone()]));
            }
            Some(id_set) => {
                id_set.insert(node_id.clone());
            }
        }
    }

    // TODO fn add_block_weight
    // TODO fn add_tree_weight

    fn new_root(&mut self, block: Block) -> NodeId {
        let weight = if !self.pk_weights.contains_key(&block.pk) {
            self.weight += block.weight;
            block.weight
        } else {
            let w = self.weight.max(block.weight);
            self.weight += w - self.pk_weights.get(&block.pk).unwrap();
            w
        };
        let id = self.tree.insert(Node::new(block.clone()), AsRoot).unwrap();
        self.pk_weights.insert(block.pk.clone(), weight);
        self.add_bp_block_id(&block.pk, &id);
        self.weights.insert(id.clone(), block.weight);
        id
    }

    fn new_leaf(&mut self, block: Block, parent: &NodeId) -> NodeId {
        if !self.pk_weights.contains_key(&block.pk) {
            self.weight += block.weight;
            block.weight
        } else {
            let w = self.weight.max(block.weight);
            self.weight += w - self.weight.min(block.weight);
            w
        };
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

    // block support => weight of subtree under the block
    pub fn support(&self, node_id: &NodeId) -> Result<u32, TreeError> {
        match self.tree.get(node_id) {
            Ok(_) => {
                let mut supporters = HashMap::new();
                let res = Ok(self
                    .tree
                    .traverse_level_order_ids(node_id)
                    .unwrap()
                    .map(|id| {
                        let pk = self.tree.get(&id).unwrap().data().pk.clone();
                        let weight = self.tree.get(&id).unwrap().data().weight;
                        match supporters.get(&pk) {
                            None => {
                                supporters.insert(pk, weight);
                                weight
                            }
                            Some(&old_weight) => {
                                let w = old_weight.max(weight);
                                supporters.insert(pk, w);
                                w - old_weight.min(weight)
                            }
                        }
                    })
                    .reduce(|acc, w| acc + w)
                    .unwrap());

                res
            }
            Err(err) => Err(TreeError::NodeIdError(err)),
        }
    }

    /// sums weights of ancestors of the node and records the value in the node
    #[allow(dead_code)]
    pub fn branch_support(&self, node_id: &NodeId) -> Result<u32, TreeError> {
        let mut sum = Ok(0);
        for id in self.tree.ancestor_ids(node_id).unwrap() {
            match self.tree.get(id) {
                Ok(node) => {
                    if let Ok(mut s) = sum {
                        s += node.data().weight;
                        sum = Ok(s)
                    } else {
                        sum = Err(TreeError::WeightError(WeightError::WeightCalculationError))
                    }
                }
                Err(err) => sum = Err(TreeError::NodeIdError(err)),
            }
        }
        sum
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

impl std::fmt::Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ pk: {:?}, weight: {} }}", &self.pk, self.weight)
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    pub fn insert_weighted_block() {
        use crate::ledger::*;

        println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
        println!("~~~~~ insert_weighted_block ~~~~~");
        println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

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
            Block::new(
                &a,
                1,
                LedgerDiff::from(&[Diff::Transfer(b.clone(), a.clone(), 2)]),
            ),
            None,
        );
        let node1_id = tree.insert(
            Block::new(
                &c,
                2,
                LedgerDiff::from(&[Diff::Transfer(c.clone(), b.clone(), 1)]),
            ),
            Some(&root_id),
        );
        let _node2_id = tree.insert(
            Block::new(
                &b,
                3,
                LedgerDiff::from(&[Diff::Transfer(c.clone(), b.clone(), 2)]),
            ),
            Some(&root_id),
        );
        let _node3_id = tree.insert(
            Block::new(
                &d,
                1,
                LedgerDiff::from(&[Diff::Transfer(a.clone(), c.clone(), 1)]),
            ),
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
        //     (a, 2) (d, 1)

        let _node4_id = tree.insert(
            Block::new(
                &a,
                2,
                LedgerDiff::from(&[
                    Diff::Transfer(b, c, 1),
                    Diff::Transfer(a.clone(), d.clone(), 2),
                ]),
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

        // get a leaf and compute branch support
        let mut leaves = HashMap::new();
        for id in tree
            .tree
            .traverse_level_order_ids(&tree.tree.root_node_id().unwrap())
            .unwrap()
        {
            if tree.tree.children(&id).unwrap().next().is_none() {
                leaves.insert(id.clone(), tree.tree.get(&id).unwrap().data().weight);
            }
        }
        println!(
            "Leaf branch support:\n{:?}",
            leaves
                .iter()
                .map(|x| (tree.branch_support(x.0).unwrap(), x.1))
                .collect::<Vec<_>>()
        );
        // panic!(); // uncomment to see stdout
    }
}
