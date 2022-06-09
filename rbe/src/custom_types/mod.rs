mod constants;
use List::*;

enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

impl List {
    // Create an empty list
    fn new() -> List {
        Nil
    }

    fn cons(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        // after Rust 2018 you can use self here and tail (with no ref) below as well,
        // rust will infer &s and ref tail. 
        // See https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
        match *self {
            // Can't take ownership of the tail, because `self` is borrowed;
            // instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn string_of_t(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.string_of_t())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

pub fn main() {
    println!("- linked list testcase");
    let mut list = List::new();

    list = list.cons(1);
    list = list.cons(2);
    list = list.cons(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.string_of_t());
    println!("");

    println!("- constants");
    constants::main();
}
