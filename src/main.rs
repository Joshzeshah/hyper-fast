
use reqwest::Client;
use tokio::time::{self, Duration};
use futures::future::join_all;

async fn curl_racer_multirace(url: &str, total_requests: usize, concurrency: usize) {
    let client = Client::new();
    let mut tasks = vec![];

    for _ in 0..total_requests {
        let client = client.clone();
        let url = url.to_string();

        let task = tokio::spawn(async move {
            let start_time = time::Instant::now();
            let response = client.get(&url).send().await;
            let duration = start_time.elapsed();

            match response {
                Ok(resp) => println!("Success: {:?} - Time: {:?}", resp.status(), duration),
                Err(e) => println!("Error: {:?}", e),
            }
        });

        tasks.push(task);

        if tasks.len() >= concurrency {
            join_all(tasks).await;
            tasks = vec![];
        }
    }

    // Wait for the remaining tasks, if any
    join_all(tasks).await;
}

#[tokio::main]
async fn main() {
    let url = "http://example.com"; // URL to be tested
    let total_requests = 100; // Total number of requests to send
    let concurrency = 10; // Number of concurrent requests

    curl_racer_multirace(&url, total_requests, concurrency).await;
}
