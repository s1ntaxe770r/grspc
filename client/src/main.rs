use hello::{hello_client::HelloClient,GreetRequest};
use tonic::Request;

pub mod hello {
    tonic::include_proto!("hello");
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client =  HelloClient::connect("http://[::1]:8080").await?;
    let request = Request::new(GreetRequest{
        name: "something like that".to_string()
    });
    let resp = client.greet(request).await.unwrap();
    println!("{:?}",resp.into_inner());
    Ok(())

}
