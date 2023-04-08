// Peano arithmetic

// type for 0
struct _0;

// successor type
struct Succ<T>(T);

type _1 = Succ<_0>;
type _2 = Succ<_1>;
type _3 = Succ<_2>;
type _4 = Succ<_3>;
type _5 = Succ<_4>;

trait Nat {}

impl Nat for _0 {}
impl<T: Nat> Nat for Succ<T> {}

#[allow(dead_code)]
fn test_is_nat<T: Nat>() {}

// Less than trait
trait Lt<A: Nat, B: Nat> {
    fn check() {}
}

// NonZero trait
trait NonZero: Nat {}

// any successor is nonzero
impl<N: Nat> NonZero for Succ<N> {}

struct ProofLt<A: Nat, B: Nat>(A, B);

// Base case: 0 is less than any nonzero Nat
impl<N: NonZero> Lt<_0, N> for ProofLt<_0, N> {}

// inductive hypothesis
impl<A: Nat, B: Nat> Lt<Succ<A>, Succ<B>> for ProofLt<Succ<A>, Succ<B>> where ProofLt<A, B>: Lt<A, B>
{}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn main() {
        test_is_nat::<_0>();
        test_is_nat::<_1>();
        test_is_nat::<_2>();
        test_is_nat::<_3>();
        test_is_nat::<_4>();
        test_is_nat::<_5>();
        ProofLt::<_1, _3>::check();
        // ProofLt::<_4, _3>::check();
    }
}
