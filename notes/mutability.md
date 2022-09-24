<!--
@name: Mutability
@title: Mutability in Rust
@description:
  Explains the how mutability is used in Rust
@tags:
 - rust
 - mutable
 - mutability
 - lifetime
 - borrow
 - reference
 - pointer
 - ownership
 - concurrency
-->

# Mutability in Rust

Values in Rust are immutable by default

```rust
let s = String::from("rust");
```

`s` is immutable so we cannot mutate its value, i.e.

```rust
s.push_str(" is immutable by default");
```

does not compile.
