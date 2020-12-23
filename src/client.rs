use helloworld::greeter_client::GreeterClient;
use helloworld::{HelloRequest,HelloReply};
// use tonic::metadata::*;
// use tonic::client::Grpc;
// use tonic::IntoRequest;
// use tonic::codec::ProstCodec;

pub mod helloworld {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://127.0.0.1:8585").await?;

    let mut req = tonic::Request::new(HelloRequest {
        name: "dollarkiller".into(),
    });

    req.metadata_mut().insert("auth","fu".parse().unwrap());
    println!("REQUEST: {:#?}",req);

    let response = client.say_hello(req).await?;
    println!("RESPONSE: {:#?}",response);

    let data:HelloReply = response.into_inner();
    println!("into_inner  = {:#?}", data);  // 查看内容
    Ok(())
}