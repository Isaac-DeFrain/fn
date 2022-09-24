/*
 * Ownership in Rust
 */ 

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

    use std::fmt;
    use std::fmt::{Debug, Formatter};
    #[derive(Copy, Clone)]
    struct Gen<T>(fn() -> T);

    impl<T: Debug> Debug for Gen<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            self.0().fmt(f)
        }
    }

    // impl<T> Copy for Gen<T> {}
    
    // impl<T> Clone for Gen<T> {
    //     fn clone(&self) -> Self {
    //         *self
    //     }
    // }
    fn f() -> u32 { 42 }
    let a = Gen::<u32>(f);
    let b = a;
    println!("{:?}, {:?}", a, b)

}
