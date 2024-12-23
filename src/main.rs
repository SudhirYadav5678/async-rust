use error_chain::error_chain;


error_chain! {
    foreign_links{
        IO(std::io::Error);
        HttpRequest(reqwest::Error)
    }
}

#[tokio::main]
async fn main()->Result<()> {
    println!("Hello, world!");
    Let res = reqwest::get("http://httpbin.org/get");
    println!("Status:{}",res.Status());
    
    Let body =res.text().await?;
    println!("Body",body);
    Ok({})
}
