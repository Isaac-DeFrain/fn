#[test]
fn remove_node() {
    use id_tree::InsertBehavior::*;
    use id_tree::RemoveBehavior::*;
    use id_tree::*;

    let mut tree: Tree<i32> = Tree::new();
    let root_id = tree.insert(Node::new(0), AsRoot).unwrap();

    let child_id = tree.insert(Node::new(1), UnderNode(&root_id)).unwrap();
    let _grandchild0_id = tree.insert(Node::new(2), UnderNode(&child_id)).unwrap();
    let _grandchild1_id = tree.insert(Node::new(3), UnderNode(&child_id)).unwrap();

    let mut w = String::new();
    tree.write_formatted(&mut w).unwrap();
    println!("Before:\n{}", w);

    let _root = tree.remove_node(root_id, LiftChildren).unwrap();

    let mut w = String::new();
    tree.write_formatted(&mut w).unwrap();
    println!("After:\n{}", w);
}
