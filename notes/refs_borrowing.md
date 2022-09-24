<!--
@name: References
@title: Refereneces and Borrowing in Rust
@description:
  Explains the concepts of references and borrowing in Rust
@tags:
 - rust
 - borrow
 - reference
 - pointer
 - ownership
 - concurrency
-->

# References and Borrowing

Instead of transferring ownership to a function, computing a value, and then returning ownership of the new value back to the original owner, Rust provides *references*. References allow you to pass values between functions without having to explicitly manage ownership.

A reference is like a pointer, but *not* a pointer. In the sense that a reference is a memory address which can be followed to access the data stored at this address, it is like a pointer. However, a reference is always *guaranteed* to point to a valid value of a particular type. This is a consequence of [lifetimes](./lifetimes.md).

*Borrowing* is the concept of using a value without taking ownership of it. It goes hand in hand with references.

## Borrowing and Mutability

- references are immutable by default
- if a mutable reference to a value exists, there cannot be any other references to that value

TODO immutable reference
TODO mutable reference
TODO multiple references

Why it's important

data races are prevented at compile time

*data race*
- two or more pointers access the same data at the same time
- at least one pointer is being used to write to the data
- there's no mechanism being used to synchronize access to the data, e.g. lock, semaphore, etc

Data races cause undefined behavior and it's notoriously difficult to debug code with data races. Rust simply does not compile programs with data races. Done.

TODO example which "discharges" references like

```rust
fn main() {
    fn print(s: String) {
      println!("{}", s);
    }

    let mut s = String::from("hello");

    let r1 = &s; // s borrowed as immutable
    let r2 = &s; // s borrowed as immutable again, this is fine
    print(*r1);
    print(*r2);
    // ownership of variables r1, r2 are transferred to println!
    // and println! never returns ownership because it drops both values
    // therefore, r1 and r2 cannot be used after this point, effectively ceasing to exist

    let r3 = &mut s; // no active borrows of s so s can be borrowed as mutable now
    r3.push_str(" world");
    println!("{}", r3);
}
```

## Dangling references

In languages with pointers, one can potentially create *dangling pointers* - a pointer that refereneces a location in memory that has been freed. This is an easy mistake to make, and luckily, to avoid. In fact, this is not even possible in Rust.
