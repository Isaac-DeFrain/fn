fn main() {
    use std::cmp::Reverse;
    use std::collections::{BinaryHeap, HashMap};

    let mut ordered_queue = BinaryHeap::new();
    let common = HashMap::from([("a", 3), ("c", 2), ("b", 1)]);
    for entry in common {
        ordered_queue.push(Reverse((entry.1, entry.0)));
        println!("insert entry: {:?}", &entry);
    }
    println!("ordered_queue: {:?}", &ordered_queue);
    let mut commit = vec![];
    while let Some(Reverse((_, req))) = ordered_queue.pop() {
        commit.push(req);
    }
    assert_eq!(vec!["b", "c", "a"], commit);
    println!("highest scoring entry: {}", commit.pop().unwrap());
}
