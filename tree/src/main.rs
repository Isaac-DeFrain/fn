use std::collections::HashMap;

use id_tree::{Tree, Node, NodeId, InsertBehavior::*};

fn merge(base_tree: &mut Tree<i32>, incoming_tree: &mut Tree<i32>, junction_node_id: &NodeId) {
    println!("=== Base tree before merge ===");
    let mut w = String::new();
    base_tree.write_formatted(&mut w).unwrap();
    println!("{}", w);

    println!("=== Incoming tree before merge ===");
    let mut w = String::new();
    incoming_tree.write_formatted(&mut w).unwrap();
    println!("{}", w);

    let mut merge_id_map = HashMap::new();
    // associate the incoming tree's root node id with it's new id in the base tree
    let incoming_root_id = incoming_tree.root_node_id().unwrap();
    let node_data = incoming_tree.get(incoming_root_id).unwrap().data();
    let new_node_id = base_tree.insert(Node::new(*node_data), UnderNode(junction_node_id)).unwrap();
    merge_id_map.insert(incoming_root_id, new_node_id);
    let traversal_ids = incoming_tree.traverse_level_order_ids(incoming_root_id).unwrap();
    for old_node_id in traversal_ids {
        let mim = merge_id_map.clone();
        let under_node_id = mim.get(&old_node_id).unwrap();
        let children_ids = incoming_tree.children_ids(&old_node_id).unwrap();
        for child_id in children_ids {
            let child_node_data = incoming_tree.get(&child_id).unwrap().data();
            let new_child_id = base_tree.insert(Node::new(*child_node_data), UnderNode(under_node_id)).unwrap();
            merge_id_map.insert(child_id, new_child_id);
        }
    }

    println!("=== Tree after merge ===");
    let mut w = String::new();
    base_tree.write_formatted(&mut w).unwrap();
    println!("{}", w);
}

fn main() {
    // Base tree
    let mut base_tree: Tree<i32> = Tree::new();
    let root_id = base_tree.insert(Node::new(0), AsRoot).unwrap();
    let child_1_id = base_tree.insert(Node::new(1), UnderNode(&root_id)).unwrap();
    let _child_2_id = base_tree.insert(Node::new(2), UnderNode(&root_id)).unwrap();
    let _child_3_id = base_tree.insert(Node::new(3), UnderNode(&child_1_id)).unwrap();

    // Incoming tree
    let mut incoming_tree: Tree<i32> = Tree::new();
    let incoming_root_id = incoming_tree.insert(Node::new(4), AsRoot).unwrap();
    let incoming_child_1_id = incoming_tree.insert(Node::new(5), UnderNode(&incoming_root_id)).unwrap();
    let incoming_child_2_id = incoming_tree.insert(Node::new(6), UnderNode(&incoming_root_id)).unwrap();
    let _incoming_child_3_id = incoming_tree.insert(Node::new(7), UnderNode(&incoming_child_1_id)).unwrap();
    let _incoming_child_4_id = incoming_tree.insert(Node::new(8), UnderNode(&incoming_child_2_id)).unwrap();
    let _incoming_child_5_id = incoming_tree.insert(Node::new(9), UnderNode(&incoming_child_2_id)).unwrap();

    merge(&mut base_tree, &mut incoming_tree, &child_1_id);
}
