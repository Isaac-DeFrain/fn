use std::time::Duration;
use tokio::time::sleep;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("Start fetching:");

    let mut handles = vec![];

    for i in 0..2 {
        let handle = tokio::spawn(async move {
            fetch(i).await;
        });
        handles.push(handle);
    };

    for handle in handles {
        handle.await.unwrap();
    }
}

async fn fetch(id: i32) {
    println!("task {id} fetching the stuff");

    let s1 = read_from_db().await;
    println!("[{id}] first result: {s1}");
    let s2 = read_from_db().await;
    println!("[{id}] second result: {s2}");
}

async fn read_from_db() -> String {
    sleep(Duration::from_millis(50)).await;
    "db_result".to_owned()
}
