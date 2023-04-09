// Conversion

mod from_into;
mod to_from_string;
mod tryfrom_tryinto;

pub fn main() {
    // `From` and `Into`
    from_into::main();

    // `TryFrom` and `TryInto`
    tryfrom_tryinto::main();

    // `ToString`
    to_from_string::main();
}
