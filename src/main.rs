use futures;
use futures::future;
use futures::stream::{self, StreamExt};
#[tokio::main]
async fn main(){

    let paths = vec![
        "https://www.baidu.com".to_string(),
        "https://www.baidu.com".to_string(),
        "https://www.baidu.com".to_string(),
        "https://www.baidu.com".to_string(),
        "https://www.baidu.com".to_string(),
        "https://www.baidu.com".to_string(),
        "https://www.littley.top".to_string(),
        "https://www.littley.top".to_string(),
        "https://www.littley.top".to_string(),
        "https://www.littley.top".to_string(),
        "https://www.littley.top".to_string(),
        "https://www.littley.top".to_string(),
        "https://www.littley.top".to_string(),
        "https://blog.csdn.net/".to_string(),
        "https://blog.csdn.net/".to_string(),
        "https://blog.csdn.net/".to_string(),
        "https://blog.csdn.net/".to_string(),
        "https://blog.csdn.net/".to_string(),
        "https://blog.csdn.net/".to_string(),
    ];
    let fetches = futures::stream::iter(
        paths.into_iter().map(|path| {
            async move {
                match reqwest::get(&path).await {
                    Ok(resp) => {
                        match resp.text().await {
                            Ok(text) => {
                                println!("RESPONSE: {} bytes from {}", text.len(), path);
                            }
                            Err(_) => println!("ERROR reading {}", path),
                        }
                    }
                    Err(_) => println!("ERROR downloading {}", path),
                }
            }
        })
    ).buffer_unordered(8).collect::<Vec<()>>();
    println!("Waiting...");
    fetches.await;
}