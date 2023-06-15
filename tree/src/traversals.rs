use crate::common::show;

use id_tree::{
    InsertBehavior::{AsRoot, UnderNode},
    Node,
    Tree,
};

pub fn main() {
    let mut tree: Tree<i32> = Tree::new();
    let root_id = tree.insert(Node::new(0), AsRoot).unwrap();
    let node_id1 = tree.insert(Node::new(5), UnderNode(&root_id)).unwrap();
    let node_id2 = tree.insert(Node::new(1), UnderNode(&node_id1)).unwrap();
    let _node_id3 = tree.insert(Node::new(2), UnderNode(&root_id)).unwrap();
    let node_id4 = tree.insert(Node::new(3), UnderNode(&node_id2)).unwrap();
    let _node_id5 = tree.insert(Node::new(4), UnderNode(&node_id4)).unwrap();
    let _node_id6 = tree.insert(Node::new(-1), UnderNode(&node_id1)).unwrap();
    let _node_id7 = tree.insert(Node::new(-2), UnderNode(&node_id2)).unwrap();

    println!("=== Tree ===");
    show(&tree);

    println!("=== Level ===");
    println!("(Breadth-first starting from root)");
    for (n, node) in tree.traverse_level_order(tree.root_node_id().unwrap()).unwrap().enumerate() {
        println!("Node {n}: {}", node.data())
    }

    println!("\n=== Preorder ===");
    println!("(Depth-first starting from root)");
    for (n, node) in tree.traverse_pre_order(tree.root_node_id().unwrap()).unwrap().enumerate() {
        println!("Node {n}: {}", node.data())
    }

    println!("\n=== Postorder ===");
    println!("(Breadth-first starting from highest leaf)");
    for (n, node) in tree.traverse_post_order(tree.root_node_id().unwrap()).unwrap().enumerate() {
        println!("Node {n}: {}", node.data())
    }
}
