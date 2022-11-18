// Conversion

mod from_into;
mod tryfrom_tryinto;
mod to_from_string;

pub fn main() {
    // `From` and `Into`
    from_into::main();

    // `TryFrom` and `TryInto`
    tryfrom_tryinto::main();

    // `ToString`
    to_from_string::main();
}
