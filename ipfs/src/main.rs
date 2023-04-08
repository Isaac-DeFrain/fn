use ipfs_api::{IpfsApi, IpfsClient};
use futures::TryStreamExt;
use std::io::{Cursor, self, Write};

#[tokio::main]
async fn main() {
    // Write file to IPFS
    let client = IpfsClient::default();
    let data = Cursor::new("Hello World!");

    match client.add(data).await {
        Ok(res) => println!("{}", res.hash),
        Err(e) => eprintln!("error adding file: {}", e)
    }

    // Read file from IPFS
    match client
        .get("/test/file.json")
        .map_ok(|chunk| chunk.to_vec())
        .try_concat()
        .await
    {
        Ok(res) => {
            let out = io::stdout();
            let mut out = out.lock();

            out.write_all(&res).unwrap();
        }
        Err(e) => eprintln!("error getting file: {}", e)
    }
}
