// `From` and `Into` are inherently linked
// 
// `From`
// The `From` trait allows for a type to define how to create itself from another type,
// hence providing a very simple mechanism for converting between several types.
// 
// `Into`
// The `Into` trait is the reciprocal of `From`.

use std::convert::From;

#[derive(Debug)]
pub struct Number {
    pub value: i32,
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self { value }
    }
}

pub fn main() {
    println!("- From and Into");
    // convert a `str` into a `String`
    let my_str = "hello";
    let my_string = String::from(my_str);
    println!("Convert str {} to String {}", my_str, my_string);
    println!("");

    // using `Number::from`
    let num = Number::from(42);
    println!("My number is {:?}", num);
    println!("it holds the value {}", num.value);
    println!("");

    // using the `Into` trait for free :)
    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);
    println!("");
}
