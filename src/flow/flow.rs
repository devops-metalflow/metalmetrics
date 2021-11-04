use crate::config::config::Config;
use flow::flow_proto_server::{FlowProto, FlowProtoServer};
use flow::{FlowReply, FlowRequest};
use std::error::Error;
use tonic::{transport::Server, Request, Response, Status};

pub mod flow {
    tonic::include_proto!("flow");
}

#[derive(Debug, Default)]
pub struct Flow {
    pub config: Config,
}

impl Flow {
    // TODO
    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        let url = self.config.listen_url.parse().unwrap();
        let server = FlowServer::default();

        Server::builder()
            .add_service(FlowProtoServer::new(server))
            .serve(url)
            .await?;

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct FlowServer {}

#[tonic::async_trait]
impl FlowProto for FlowServer {
    // TODO
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
