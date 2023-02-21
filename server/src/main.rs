use tonic::{transport::Server,Request,Response,Status};
use hello::{GreetRequest,Reply,hello_server::{Hello,HelloServer}};

pub mod hello {
    tonic::include_proto!("hello");
}





#[tokio::main]  
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let  addr = "[::1]:8080".parse().unwrap();
    let hello_serivce = HelloService::default();
    println!("serving rpc on 8080");
    Server::builder().add_service(HelloServer::new(hello_serivce))
        .serve(addr)
        .await?;
    Ok(())
}

#[derive(Debug,Default)]
pub struct HelloService {}

#[tonic::async_trait]
impl Hello for HelloService {
    async fn greet(&self,request: Request<GreetRequest>) -> Result<Response<Reply>,Status> {
        let r = request.into_inner();
        println!("{}",r.name);
        let rep =  Reply{
                response: "wow it actuallt worked".to_string(),
        };
        Ok(Response::new(rep))
    }
}




