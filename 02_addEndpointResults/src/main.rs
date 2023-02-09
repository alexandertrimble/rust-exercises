use futures::{stream, StreamExt};
use reqwest::Client;
use tokio;

// const PARALLEL_REQUESTS: usize = 2;
const REQUEST_URL: &str =
    "https://www.random.org/integers/?num=1&min=1&max=99999&col=1&base=10&format=plain&rnd=new";
const NUM_REQUESTS: usize = 20;
const MAX_CONCURRENT_REQUESTS: usize = 10;

#[tokio::main]
async fn main() {
    fetch_and_sum_numbers().await;
}

async fn fetch_and_sum_numbers() {
    let mut sum = 0;

    let mut stream = stream::iter(0..NUM_REQUESTS)
        .map(|_| fetch_number())
        .buffer_unordered(MAX_CONCURRENT_REQUESTS);

    while let Some(number) = stream.next().await {
        sum += number;
    }

    println!("Sum: {}", sum);
}

async fn fetch_number() -> i32 {
    let client = Client::new();
    let response = client.get(REQUEST_URL).send().await.unwrap();
    let text = response.text().await.unwrap();
    let result = text.trim().parse::<i32>().unwrap();
    println!("Fetch result: {}", result);

    return result;
}
