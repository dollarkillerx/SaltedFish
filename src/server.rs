use tonic::{transport::Server, Request, Response, Status};
use tonic;
use helloworld::greeter_server::{Greeter, GreeterServer};
use helloworld::{HelloRequest, HelloReply};

pub mod helloworld {
    tonic::include_proto!("helloworld");
}

#[derive(Debug, Default)]
pub struct MyServer {}

#[tonic::async_trait]
impl Greeter for MyServer {
    async fn say_hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloReply>, Status> {
        println!("request: {:?}", request);
        println!("remote_adrr: {:?}", request.remote_addr());
        println!("metadata: {:?} ", request.metadata());

        let req: HelloRequest = request.into_inner();
        println!("req: {:#?}",req);
        let response = helloworld::HelloReply {message: "hello world".to_string()};
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8585".parse().unwrap();
    let my_server = MyServer::default();
    println!("run on {}",addr);
    Server::builder()
        .add_service(GreeterServer::new(my_server))
        .serve(addr)
        .await?;
    Ok(())
}