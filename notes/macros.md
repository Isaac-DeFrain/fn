# Rust's Macro System

Macros are created using the `macro_rules!` macro.

## Designators

[Rust by Example](https://doc.rust-lang.org/rust-by-example/macros/designators.html)
[Rust Reference](https://doc.rust-lang.org/reference/macros-by-example.html)

- `block`
- `expr`
- `ident`
- `item`
- `lifetime`
- `literal`
- `meta`
- `pat`
- `pat_param`
- `path`
- `stmt`
- `tt`
- `ty`
- `vis`

```rust
macro_rules! expr_value {
    ($e: expr) => {
        println!("{:?} = {:?}", stringify!($e), $e);
    };
}
```

```rust
fn abc() -> u32 {
    use rand::random;
    random::<u32>()
}

macro_rules! eval {
    ($f: ident) => { $f() };
}

eval!(abc);
```
