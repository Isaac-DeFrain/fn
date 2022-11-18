use std::sync::Arc;
use anyhow::Result;
use router::Router;

mod router;

#[tokio::main]
async fn main() -> Result<()> {
    let router = Router::new()
        .add_handler("add", add)
        .add_handler("sub", sub)
        .add_handler("mult", |a, b| async move {
            Ok(format!("{a} * {b} = {}", a * b))
        });
    
    let router_arc = Arc::new(router);

    let router = router_arc.clone();
    tokio::task::spawn(async move {
        if let Ok(handler) = router.get("add") {
            println!("->>! {:?}", handler.call((19, 23)).await)
        }
    });

    let router = router_arc.clone();
    tokio::task::spawn(async move {
        if let Ok(handler) = router.get("sub") {
            println!("->>! {:?}", handler.call((19, 23)).await)
        }
    });

    let router = router_arc.clone();
    println!("->> {}", router.get("add")?.call((19, 23)).await?);
    println!("->> {}", router.get("sub")?.call((19, 23)).await?);
    println!("->> {}", router.get("mult")?.call((19, 23)).await?);
    // println!("->> {}", router.get("blah")?.call((19, 23)).await?); // error

    Ok(())
}

async fn add(a: i32, b: i32) -> Result<String> {
    Ok(format!("{a} + {b} = {}", a + b))
}

async fn sub(a: i32, b: i32) -> Result<String> {
    Ok(format!("{a} - {b} = {}", a - b))
}
