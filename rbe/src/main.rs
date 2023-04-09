mod conversion;
mod custom_types;
mod hello_world;

fn main() {
    println!("-----------");
    println!("hello_world");
    println!("-----------");
    hello_world::main();

    println!("------------");
    println!("custom_types");
    println!("------------");
    custom_types::main();

    println!("----------");
    println!("conversion");
    println!("----------");
    conversion::main();
}
