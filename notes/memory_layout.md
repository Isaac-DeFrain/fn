# Memory Layout of Rust's Data Types

[Visualizing memory layout of Rust's data types](https://www.youtube.com/watch?app=desktop&v=rDoqT-a6UFg)

Linux: ELF-64

Virtual address space of the program
- kernel and RAM map these to physical address at runtime

*Process* - a running program

Multiple sections in the program's address space
- text/code
  - read-only
- data
- bss (*block started by symbol*)
  - uninitialized global variables
- heap
- stack
  - main thread 8MB
  - spawned threads' stack size can be specified
- env vars, arguments, argument  count

## Stack

Stack frame + stack pointer

Fast because no system calls need to be made

Can only store variables with a fixed size known at compile time

A function cannot return a reference to one of its local variables

## Data types

### Stack allocated

- `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
- `char`
- `&T`
- arrays (fixed size)

### Heap allocated

- `String`
- vectors `Vec<T>`
- slices `[T]`

### In the binary

- static `&str`


TODO


### Copying and moving

26:35

- copy = bit-by-bit copy of value
- clone

## `Rc` reference counted pointers

TODO

### `Send` and `Sync` traits

`Send` = a value of a type implementing this trait can be moved from one thread to another

`Sync` = a value of a type implementing this trait can be shared among multiple threads using shared references

`Rc` is neither `Send` nor `Sync`...

## `Arc` atomic reference counted pointers

`Arc` values are both `Send` and `Sync`

Cannot be mutable, must use `Arc<Mutex<T>>`

## Trait objects

Reference to a trait type

In memory, a trait object is a fat pointer consisting of two pointers, one to the value `data` and one to the table representing the value's type `vtable`

### conversion

### function pointers

Traits

- `Fn`
- `FnMut`
- `FnOnce`

Represented as a struct
