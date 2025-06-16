use std::error::Error;
use proto::calculator_client::CalculatorClient;

pub mod proto {
    tonic::include_proto!("calculator");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://[::1]:50051";
    let mut client = CalculatorClient::connect(url).await?;
    //ADD
    let request = tonic::Request::new(proto::AddRequest { a: 5, b: 4 });
    let response = client.add(request).await?;
    println!("Response (add 5,4): {}", response.get_ref().result);
    //SUB
    let request = tonic::Request::new(proto::SubRequest { a: 5, b: 4 });
    let response = client.sub(request).await?;
    println!("Response (sub 5,4): {}", response.get_ref().result);
    //DIV
    let request = tonic::Request::new(proto::DivRequest { a:6 , b: 0 });
    let response = client.div(request).await?;
    println!("Response (div 6,2): {}", response.get_ref().result);
    Ok(())
}
