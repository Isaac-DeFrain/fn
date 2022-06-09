/// Two kinds of constants:
/// 1. `const`: an unchangeable value
/// 2. `static`: a possibly `mut`able variable with `'static` lifetime. Accessing or modifying is `unsafe`.

static LANG: &str = "Rust";
const THRESHOLD: i32 = 10;

pub fn main() {
    println!("{} is static", LANG);
    println!("{} is const", THRESHOLD);
}
