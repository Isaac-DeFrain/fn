use id_tree::{InsertBehavior::*, Node, NodeId, Tree};

#[derive(Clone, PartialEq, Eq)]
pub struct Block {
    pub data: String,
    pub weight: u32,
}

#[derive(Debug, Clone)]
pub struct WeightedTree {
    pub tree: id_tree::Tree<Block>,
    pub weight: u32,
}

impl Block {
    pub fn new(data: &str, weight: u32) -> Self {
        Self {
            data: data.to_string(),
            weight,
        }
    }
}

impl std::fmt::Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", (self.data.clone(), self.weight))
    }
}

impl WeightedTree {
    pub fn new() -> Self {
        Self {
            tree: Tree::new(),
            weight: 0,
        }
    }

    pub fn new_root(&mut self, block: Block) -> NodeId {
        self.weight += block.weight;
        self
            .tree
            .insert(Node::new(block.clone()), AsRoot)
            .unwrap()
    }

    pub fn new_leaf(&mut self, block: Block, parent: &NodeId) -> NodeId {
        self.weight += block.weight;
        self
            .tree
            .insert(Node::new(block.clone()), UnderNode(parent))
            .unwrap()
    }

    pub fn support(&self, id: &NodeId) -> u32 {
        self.tree
            .traverse_level_order_ids(id).unwrap()
            .map(|nid| self.tree.get(&nid).unwrap().data().weight)
            .reduce(|acc, w| acc + w).unwrap()
    }
}

pub fn add_weighted_block_example() {
    println!("~~~~~ Add weighted block example ~~~~~");
    println!();
    // Base tree
    //    (a, 1)
    //    /    \
    // (b, 3) (c, 2)
    //          |
    //        (d, 1)
    let tree = &mut WeightedTree::new();
    let node0_id = tree.new_root(Block::new("a", 1));
    let node1_id = tree.new_leaf(Block::new("b", 3), &node0_id);
    let node2_id = tree.new_leaf(Block::new("c", 2), &node0_id);
    let node3_id = tree.new_leaf(Block::new("d", 1), &node1_id);
    
    println!("=== Before adding block ===");
    println!();

    println!("Weight: {}", tree.weight);
    println!();

    println!("=== Ancestors ===");
    println!("{:?}: {:?}", tree.tree.get(&node0_id).unwrap().data(), tree.tree.ancestors(&node0_id).unwrap().map(|n| n.data()).cloned().collect::<Vec<Block>>());
    println!("{:?}: {:?}", tree.tree.get(&node1_id).unwrap().data(), tree.tree.ancestors(&node1_id).unwrap().map(|n| n.data()).cloned().collect::<Vec<Block>>());
    println!("{:?}: {:?}", tree.tree.get(&node2_id).unwrap().data(), tree.tree.ancestors(&node2_id).unwrap().map(|n| n.data()).cloned().collect::<Vec<Block>>());
    println!("{:?}: {:?}", tree.tree.get(&node3_id).unwrap().data(), tree.tree.ancestors(&node3_id).unwrap().map(|n| n.data()).cloned().collect::<Vec<Block>>());
    println!();

    println!("=== Support ===");
    println!("{:?}: {:?}", tree.tree.get(&node0_id).unwrap().data(), tree.support(&node0_id));
    println!("{:?}: {:?}", tree.tree.get(&node1_id).unwrap().data(), tree.support(&node1_id));
    println!("{:?}: {:?}", tree.tree.get(&node2_id).unwrap().data(), tree.support(&node2_id));
    println!("{:?}: {:?}", tree.tree.get(&node3_id).unwrap().data(), tree.support(&node3_id));
    println!();

    println!("=== Tree ===");
    let mut w = String::new();
    tree.tree.write_formatted(&mut w).unwrap();
    println!("{}", w);
    
    println!("=== After block ===");
    println!();

    let node4_id = tree.new_leaf(Block::new("f", 2), &node1_id);
    // Final tree
    //    (a, 1)
    //    /    \
    // (c, 3) (b, 2)
    //        /    \
    //     (f, 2) (d, 1)

    println!("Weight: {}", tree.weight);
    println!();

    println!("=== Support ===");
    println!("{:?}: {:?}", tree.tree.get(&node0_id).unwrap().data(), tree.support(&node0_id));
    println!("{:?}: {:?}", tree.tree.get(&node1_id).unwrap().data(), tree.support(&node1_id));
    println!("{:?}: {:?}", tree.tree.get(&node2_id).unwrap().data(), tree.support(&node2_id));
    println!("{:?}: {:?}", tree.tree.get(&node3_id).unwrap().data(), tree.support(&node3_id));
    println!("{:?}: {:?}", tree.tree.get(&node4_id).unwrap().data(), tree.support(&node4_id));
    println!();

    println!("=== Ancestors ===");
    println!("{:?}: {:?}", tree.tree.get(&node0_id).unwrap().data(), tree.tree.ancestors(&node0_id).unwrap().map(|n| n.data()).cloned().collect::<Vec<Block>>());
    println!("{:?}: {:?}", tree.tree.get(&node1_id).unwrap().data(), tree.tree.ancestors(&node1_id).unwrap().map(|n| n.data()).cloned().collect::<Vec<Block>>());
    println!("{:?}: {:?}", tree.tree.get(&node2_id).unwrap().data(), tree.tree.ancestors(&node2_id).unwrap().map(|n| n.data()).cloned().collect::<Vec<Block>>());
    println!("{:?}: {:?}", tree.tree.get(&node3_id).unwrap().data(), tree.tree.ancestors(&node3_id).unwrap().map(|n| n.data()).cloned().collect::<Vec<Block>>());
    println!("{:?}: {:?}", tree.tree.get(&node4_id).unwrap().data(), tree.tree.ancestors(&node4_id).unwrap().map(|n| n.data()).cloned().collect::<Vec<Block>>());
    println!();

    println!("=== Tree ===");
    let mut w = String::new();
    tree.tree.write_formatted(&mut w).unwrap();
    println!("{}", w);
}
