use awc::Client;
use std::io;


#[actix_rt::main]
async fn main() -> io::Result<()> {
    //
    let res = Client::default()
        .get("https://httpbin.org/status/200")
        .send()
        .await;

    assert_eq!(200, res.unwrap().status());

    // 
    let res = Client::default().get("https://yandex.ru").send().await;
    assert_eq!(200, res.unwrap().status());

    Ok(())
}
