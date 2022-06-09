use std::fmt;

// here's a comment
/// here's a doc comment
pub fn main() {
    println!("{:?}", DebugPrintable(42));
    println!("{:?}", Person { name: "Rusty", age: 11 });
    println!("{:#?}", Person { name: "Rusty", age: 11 });
    println!("{}", Structure(42));
    println!("{}", List(vec![0, 1, 2, 3, 4]));
    
    let x = 123;
    println!("{}", x);
    println!("0x{:X}", x);
    println!("0o{:o}", x);
}

#[derive(Debug)]
pub struct DebugPrintable(i32);

#[allow(dead_code)]
#[derive(Debug)]
pub struct Person<'a> {
    name: &'a str,
    age: u8
}

struct Structure(i32);

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed.
        write!(f, "*{}*", self.0)
    }
}

// Define a structure named `List` containing a `Vec`.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]")
    }
}
