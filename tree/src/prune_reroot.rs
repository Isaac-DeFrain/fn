use crate::common::show;

use id_tree::{
    InsertBehavior::{AsRoot, UnderNode},
    MoveBehavior::ToRoot,
    Node,
    RemoveBehavior::*,
    Tree,
};

#[allow(dead_code)]
pub fn main() {
    let mut tree: Tree<i32> = Tree::new();
    let root_id = tree.insert(Node::new(0), AsRoot).unwrap();
    let node_id1 = tree.insert(Node::new(5), UnderNode(&root_id)).unwrap();
    let node_id2 = tree.insert(Node::new(1), UnderNode(&node_id1)).unwrap();
    let _node_id3 = tree.insert(Node::new(2), UnderNode(&root_id)).unwrap();
    let _node_id4 = tree.insert(Node::new(3), UnderNode(&node_id2)).unwrap();
    let _node_id5 = tree.insert(Node::new(-1), UnderNode(&node_id1)).unwrap();
    let _node_id6 = tree.insert(Node::new(-2), UnderNode(&node_id2)).unwrap();

    println!("=== Initial tree ===");
    show(&tree);

    // println!("=== Preorder ===");
    // for (n, node) in tree.traverse_pre_order(tree.root_node_id().unwrap()).unwrap().enumerate() {
    //     println!("{n}: {}", node.data())
    // }

    // println!("=== Postorder ===");
    // for (n, node) in tree.traverse_post_order(tree.root_node_id().unwrap()).unwrap().enumerate() {
    //     println!("{n}: {}", node.data())
    // }

    let prune_node_id = node_id2;
    let parent_node_id = node_id1;

    // remove all prune node siblings
    for node_id in tree.get(&parent_node_id).unwrap().children().clone() {
        if node_id != prune_node_id {
            tree.remove_node(node_id, DropChildren).unwrap();
        }
    }

    // remove parent node + orphan children
    tree.remove_node(parent_node_id, OrphanChildren).unwrap();

    // remove original root + drop children
    tree.remove_node(root_id, DropChildren).unwrap();

    // move prune node to root
    tree.move_node(&prune_node_id, ToRoot).unwrap();

    println!("=== Pruned tree ===");
    show(&tree);

    assert_eq!(&prune_node_id, tree.root_node_id().unwrap());
}
