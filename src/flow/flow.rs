use flow::flow_proto_server::{FlowProto, FlowProtoServer};
use flow::{FlowReply, FlowRequest};
use tonic::{transport::Server, Request, Response, Status};

pub mod flow {
    tonic::include_proto!("flow");
}

#[derive(Debug, Default)]
pub struct Flow {}

impl Flow {
    async fn run() -> Result<(), Box<dyn std::error::Error>> {
        let addr = "[::1]:50051".parse().unwrap();
        let server = FlowServer::default();

        Server::builder()
            .add_service(FlowProtoServer::new(server))
            .serve(addr)
            .await?;

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct FlowServer {}

#[tonic::async_trait]
impl FlowProto for FlowServer {
    async fn send_flow(
        &self,
        request: Request<FlowRequest>,
    ) -> Result<Response<FlowReply>, Status> {
        let reply = flow::FlowReply {
            message: format!("{}", request.into_inner().message),
        };

        Ok(Response::new(reply))
    }
}
