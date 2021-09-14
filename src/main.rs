use hyper::{Client, Uri};


fn main() {
    loop{
    rq();
}
}

#[tokio::main]
async fn rq() {
    let client = Client::new();
    let url: Uri = ("http://192.168.86.48")
    .parse()
    .unwrap();

    match client.get(url).await {
        Ok(res) => println!("Response: {}", res.status()),
        Err(err) => println!("Error:{}", err),
    }
}
