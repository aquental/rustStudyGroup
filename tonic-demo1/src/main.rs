use proto::calculator_server::{Calculator, CalculatorServer};
use proto::{AddRequest, AddResponse, SubRequest, SubResponse, DivRequest, DivResponse};
use proto::{GetRequestCountRequest, GetRequestCountResponse};
use proto::admin_server::{Admin, AdminServer};
use tonic::transport::Server;

mod proto {
    tonic::include_proto!("calculator");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] = 
        include_bytes!(concat!(env!("OUT_DIR"), "/calculator_descriptor.bin"));
}

type State = std::sync::Arc<tokio::sync::RwLock<u64>>;

#[derive(Debug, Default)]
struct AdminService{
    state: State,
}

#[tonic::async_trait]
impl Admin for AdminService {
    async fn get_request_count(&self, _request: tonic::Request<GetRequestCountRequest>) -> Result<tonic::Response<GetRequestCountResponse>, tonic::Status> {
        let count = self.state.read().await;
        let response = GetRequestCountResponse { count: *count as i64 };
        Ok(tonic::Response::new(response))
    }
}

#[derive(Debug, Default)]
struct CalculatorService{
    state: State,
}
impl CalculatorService {
    async fn increment_counter(&self) {
        let mut count = self.state.write().await;
        *count += 1;
        println!("Counter: {}", *count);
    }
}
#[tonic::async_trait]
impl Calculator for CalculatorService {
    async fn add(&self, request: tonic::Request<AddRequest>) -> Result<tonic::Response<AddResponse>, tonic::Status> {
        self.increment_counter().await;
        let req = request.into_inner();
        let result = req.a + req.b;
        let response = AddResponse { result };
        Ok(tonic::Response::new(response))
    }

    async fn sub(&self, request: tonic::Request<SubRequest>) -> Result<tonic::Response<SubResponse>, tonic::Status> {
        self.increment_counter().await;
        let req = request.into_inner();
        let result = req.a - req.b;
        let response = SubResponse { result };
        Ok(tonic::Response::new(response))
    }

    async fn div(&self, request: tonic::Request<DivRequest>) -> Result<tonic::Response<DivResponse>, tonic::Status> {
        self.increment_counter().await;
        let req = request.into_inner();
        if req.b == 0 {
            return Err(tonic::Status::invalid_argument("Divide by zero"));
        }
        let result = req.a / req.b;
        let response = DivResponse { result };
        Ok(tonic::Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let state = State::default();

    let calc = CalculatorService {
        state: state.clone()
    };

    let admin = AdminService {
        state: state.clone()
    };

    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    Server::builder()
        .add_service(service)
        .add_service(CalculatorServer::new(calc))
        .add_service(AdminServer::new(admin))
        .serve(addr)
        .await?;

    Ok(())
}
