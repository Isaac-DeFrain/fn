fn main() {
    let write_path = env!("WRITE_PATH");
    let mut contents = String::new();

    // write a file download the first 99_999 Mina block heights
    for height in 2..100_001 {
        if height == 2 {
            contents.push_str("gs://mina_network_block_data/mainnet-2-*.json");
        } else {
            contents.push_str(&format!("\ngs://mina_network_block_data/mainnet-{height}-*.json"));
        }
    }

    contents.push('\n');    
    std::fs::write(write_path, contents).expect("File write failed");
}
