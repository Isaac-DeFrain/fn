// Instead of implementing `ToString` directly,
// one should implement the `fmt::Display` trait

pub fn main() {
    println!("- To and from String");
    // As long as `FromStr` is implemented, we can use `parse`
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    println!("Sum: {:?}\n", parsed + turbo_parsed);
}
