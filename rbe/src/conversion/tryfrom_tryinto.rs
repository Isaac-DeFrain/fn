use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(Self(value))
        } else {
            Err(())
        }
    }
}

pub fn main() {
    println!("- TryFrom and TryInto");

    // TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto
    let res: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(res, Ok(EvenNumber(8)));
    let res: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(res, Err(()));

    println!("All asserts passed!");
    println!("");
}
