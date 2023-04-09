<!--
@name: Ownership
@title: Rust Ownership
@description:
  Explain the ownership model in Rust
@tags:
 - rust
 - ownership
 - concurrency
 - stack
 - heap
-->

# Ownership in Rust

According to [the book](https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html)

> *Ownership* is a set of rules that governs how a Rust program manages memory.

All programs, in any language, need a way to manage their use of the computer's memory. Some languages (e.g. Java, Scala, OCaml) use a garbage collector which is just a program running in tandem that regularly scans for memory which is no longer in use by your program. In other languages, memory most be explicitly managed (e.g. C). Rust takes a third approach, managing the memory through a set of rules which the compiler checks. This is nice because it exists in the space between garbage collection and manual memory management, giving us the performance we expect from manual memory management without having to `alloc` or `free` anything!

How can this be? All pros? Not quite! As with everything though, it has its cons. It is common for beginners to struggle to understand Rust's concept of *ownership*. Here, we attempt to demystify *ownership*!

## Stack and Heap

We won't say too much about these structures here (check out [Geeks for Geeks](https://www.geeksforgeeks.org/stack-vs-heap-memory-allocation/) for a more thorough explanation). However, one should at least be aware of the fact that we typically have two types of memory to use in a program, the *stack* and the *heap*. For our discussion, the relevant bits:

- stack
  - access to elements on the stack is fast and cheap, and
  - we are only allowed to store specific types of values on the stack (those with known size)

- heap
  - access to elements on the heap is slow and expensive, and
  - we are allowed to store basically any data we want on the heap

## Ownership Rules

Without further ado, the ownership rules:

- Each value in Rust has an *owner*
- There can be at most *one* owner at a time
- When the owner goes out of scope, the value is dropped

Simple enough, right!? While the rules may be simple, their consequences are quite profound! Essentially, ownership is used to guarantee the absence of data races. If only the owner of some data can access it and there is at most one owner for any data, then we have effectively made race conditions impossible!

At this point, you would be justified in wondering about variable scopes.

## Variable Scopes

An example

```rust
{
                     // s is not in scope yet because it hasn't been declared yet
    let s = "hello"; // s is valid, i.e. in scope, from this point on within this code block

    // s is in scope here so we can use it in computations

}                    // s is no longer in scope
```

Here, `s` is a *string literal*, denoted by `str`. String literals are a simple, immutable, stack-allocated type.

## The `String` type

`String` is a complex type because it has a variable size. As a consequence, `String`s cannot be stored on the stack and are thus, stored on the heap.

```rust
let mut s = String::from("Hello");
    s.push_str(", ");
    s.push_str("world!");
    println!("{}", s)
```

## Memory and Allocation

String literals are hardcoded into the final executable because they are immutable and have a known size at compile time. On the other hand, `String` values can't be hardcoded into the executable since they are mutable and their size is not known at compile time. As a result

1. the memory must be requested from the *memory allocator* at runtime, and
2. we need a way to return this memory to the allocator once we're done with it

The programmer does 1. with `String::from` and the compiler handles 2. via the ownership rules.

Different languages use different solutions, e.g.
- Java
  - [garbage collection](https://www.geeksforgeeks.org/garbage-collection-java/)
- C++
  - [smart pointers](https://www.geeksforgeeks.org/smart-pointers-cpp/)
  - [Resource Acquisition Is Initialization (RAII) pattern](https://en.cppreference.com/w/cpp/language/raii)

### Move and Borrow

Rust calls the transfer of ownership from one variable to another a *move*. Whether or not the original variable is still in scope after a move depends on type of the variable. If the values are stored on the stack, the original variable will still be in scope. If the values are stored on the heap, the original variable will be dropped.

We do *not* need to explicitly borrow values of any primitive type

```rust
let a: &str = "hello";
let b = a;
println!("{}, {}", a, b); // a is still in scope!

let a: i8 = 42;
let b = a;
println!("{}, {}", a, b); // a is still in scope!

let a : u32 = 42;
let b = a;
println!("{}, {}", a, b); // a is still in scope!
```

Heap-allocated values must be explicitly borrowed or cloned to prevent the original variable from being dropped.

```rust
let a = String::from("hello");
let b = a; // ownership of the value is transferred to b; a is dropped
// println!("{}, {}", a, b); // we can't use a because it was dropped
println!("{}", b);

let a = String::from("hello");
let b = &a; // ownership of the value is transferred to b; a is dropped
println!("{}, {}", a, b); // a was NOT dropped
```

Since (deeply) copying the data owned by `a` would be expensive in general, the declaration `let b = a` instead, simply takes `a` out of scope and gives `b` the stack data from `a`.

> Move = shallow copy + drop original

Rust will never automatically create deep copies of data. Thus, any automatic copying can be assumed to be inexpensive.

## Traits

Rust controls the behavior of values of types through the use of *traits* which are similar to Haskell type classes. Relevant traits to our current are `Clone` and `Copy`.

### `Clone`

`Clone` is a derivable trait in Rust. If it suits your needs better, you can also write a custom implementation by overloading the `clone` and `clone_from` functions (see the [docs](https://doc.rust-lang.org/std/clone/trait.Clone.html)). A *clone* makes a copy of the corresponding segment of heap memory and this must be done explicitly.

> Clone = explicit deep copy

```rust
let a = String::from("hello");
let b = a.clone();
println!("{}, {}", a, b); // a is still in scope!
```

### `Copy`

A *copy* makes a copy of the bits of the value. The implmentation cannot be overloaded; it is always a simple bit-wise copy. Copies happen implicitly during assignment `x = y`.

`Copy` can only be implemented on a type for which no part has implemented the `Drop` trait. `Copy` is also a derivable type.

For data stored entirely on the stack, there's no difference between a shallow copy and a deep copy, and hence, no difference between a copy and a clone.

Some types which implement `Copy`:

- primitive types
- tuples of types which implement `Copy`, e.g. `(i8, bool)`

> Copy = implicit shallow copy
