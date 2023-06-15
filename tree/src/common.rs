use id_tree::Tree;

pub fn show<T: std::fmt::Debug>(tree: &Tree<T>) {
    let mut w = String::new();
    tree.write_formatted(&mut w).unwrap();
    println!("{w}");
}
