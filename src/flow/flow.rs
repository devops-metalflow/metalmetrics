use flow::flowproto_server::{FlowProto, FlowProtoServer};
use flow::{FlowReply, FlowRequest};
use tonic::{transport::Server, Request, Response, Status};

pub mod flow {
    tonic::include_proto!("flow");
}

#[derive(Debug, Default)]
pub struct Flow {}

#[tonic::async_trait]
impl Flow {
    async fn run() -> Result<(), Box<dyn std::error::Error>> {
        let addr = "[::1]:50051".parse().unwrap();
        let fp = FlowProto::default();

        Server::builder()
            .add_service(FlowProtoServer::new(fp))
            .serve(addr)
            .await?;

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct FlowProto {}

#[tonic::async_trait]
impl FlowProto {
    async fn send(
        &self,
        request: Request<FlowRequest>,
    ) -> Result<Response<FlowReply>, Status> {
        let reply = flow::FlowReply {
            message: format!("{}", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}
