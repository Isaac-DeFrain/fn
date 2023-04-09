use std::io::stdin;
use voting::{VotingRequest, voting_client::VotingClient};

pub mod voting {
    tonic::include_proto!("voting");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = VotingClient::connect("https://127.0.0.1:8080").await?;
    loop {
        println!("~~~Vote for a word~~~");
        let mut u = String::new();
        let mut vote = String::new();
        println!("Provide a word: ");
        stdin().read_line(&mut u).unwrap();
        let u = u.trim();
        println!("Vote up (u) or down (d): ");
        stdin().read_line(&mut vote).unwrap();
        let v = match vote.trim().to_lowercase().chars().next().unwrap() {
            'u' => 0,
            'd' => 1,
            _ => break,
        };
        // service invocation
        let req = tonic::Request::new(VotingRequest {
            url: String::from(u),
            vote: v,
        });
        let res = client.vote(req).await?;
        println!("Got: '{}' from service", res.into_inner().confirmation);
    }
    Ok(())
}
