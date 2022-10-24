/*
 * Examples
 */

mod crypto;
mod time;

fn main() {
    fn take_ownership(s: String) {
        // since the function takes a heap-allocated argument
        // it assumes ownership of the value
        // it is also said that s is moved into take_ownership
        println!("the ownership of String({}) has been transferred to me :D", s);
    }
    
    // String => heap-allocated
    let s = String::from("hello");
    take_ownership(s);
    // now s is out of scope
    // println!("{}", s);

    // Okay!
    let s = "hello";
    take_ownership(String::from(s));
    take_ownership(String::from(s));

    // Okay!
    let mut s = String::from("greetings");
    s.push_str(" from");
    s.push_str(" main");
    let r1 = &s;
    let r2 = &s; // we  can create as many immutable references as we like
    println!("{}, {}", r1, r2);
    
    let r3 = &s;
    println!("{}", r3);

    // Not okay!
    // s cannot be simultaneously borrowed as both mutable and immutable
    // let r3 = &mut s;
    // println!("{}", r3);

    // string literals - &str
    // no borrow or change of ownership required since these values are allocated on the stack
    let a: &str = "hello";
    let b = a;
    println!("{}, {}", a, b); // a is still in scope!

    let a: i8 = 42;
    let b = a;
    println!("{}, {}", a, b); // a is still in scope!

    let a : u32 = 42;
    let b = a;
    println!("{}, {}", a, b); // a is still in scope!

    {
        #[derive(Clone, Copy, Debug)]
        struct T(u8, &'static str);

        let a = T(42, "hello");
        let b = a;
        println!("{:?}, {:?}", a, b); // a is still in scope!
    }

    // clone = deep copy
    let a = String::from("hello");
    let b = a.clone();
    println!("{}, {}", a, b); // a is still in scope

    // borrow
    let a = String::from("hello");
    let b = &a;
    println!("a : {}, b : {}", a, b); // a is still in scope

    // borrowing + mutability
    fn modify(s : &mut String) {
        s.push_str(", world");
    }
    // the function requires a mutable borrow
    // when provided with one, it is the evidence that both
    // no immutable borrows and no other mutable borrows exist

    let mut s = String::from("hello");
    let m1 = &mut s;
    modify(m1);
    println!("{}", s);

    let m2 = &mut s;
    // modify(m1); // no no
    modify(m2);
    println!("{}", s);

    // we're fine as long as we don't use m1 again

    // clonable function pointer
    use std::fmt::{Debug, Formatter, Result};
    use rand::prelude::*;
    #[derive(Copy, Clone)]
    struct Gen<T>(fn() -> T);

    impl<T: Debug> Debug for Gen<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            self.0().fmt(f)
        }
    }

    use rand::distributions::Standard;
    fn gen<T>() -> T where Standard: Distribution<T> {
        rand::random::<T>()
    }

    fn gen_u32() -> u32 {
        gen::<u32>() % 100_000
    }

    fn gen_float() -> f64 {
        gen::<f64>() * 1_000_000_000f64
    }

    let a = Gen(gen_u32);
    let b = a;
    let c = Gen::<bool>(gen);
    // the type parameter needs to be fixed
    // let c = Gen(gen);
    let d = Gen(gen_float);
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let mut nums: Vec<i32> = (0..100).collect();
    nums.shuffle(&mut rand::thread_rng());
    println!("start with {}\npop: {:?}, {:?}, {:?}, {:?}\nremaining: {}",
        nums.len(),
        nums.pop().unwrap(),
        nums.pop().unwrap(),
        nums.pop().unwrap(),
        nums.pop().unwrap(),
        nums.len()
    );

    // macros
    macro_rules! eval {
        () => {
            println!("please provide a function to evaluate");
        };
        ($f: ident) => {
            println!("{}() = {}", stringify!($f), $f());
        };
    }

    macro_rules! expr_value {
        ($e: expr) => {
            println!("{:?} = {:?}", stringify!($e), $e);
        };
    }

    eval!(gen_u32);

    expr_value!({
        let x = 2i32;
        fn f(z: i32) -> i32 { z + 1 }
        f(x) * 7
    });

    // 
    crypto::hmac::hello();

    // 
    time::examples::main();

}
