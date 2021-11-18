use crate::config::config::Config;
use flow::flow_proto_server::{FlowProto, FlowProtoServer};
use flow::{FlowReply, FlowRequest};
use std::error::Error;
use tonic::{transport::Server, Request, Response, Status};

pub mod flow {
    tonic::include_proto!("flow");
}

pub struct Flow {
    pub config: Config,
    pub routine: fn(Config, &str) -> Result<String, Box<dyn Error>>,
}

impl Flow {
    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        let url = self.config.listen_url.parse().unwrap();
        let server = FlowServer {
            config: self.config.clone(),
            routine: self.routine,
        };

        Server::builder()
            .add_service(FlowProtoServer::new(server))
            .serve(url)
            .await?;

        Ok(())
    }
}

pub struct FlowServer {
    pub config: Config,
    pub routine: fn(Config, &str) -> Result<String, Box<dyn Error>>,
}

#[tonic::async_trait]
impl FlowProto for FlowServer {
    async fn send_flow(
        &self,
        request: Request<FlowRequest>,
    ) -> Result<Response<FlowReply>, Status> {
        let msg: String;
        match (self.routine)(self.config.clone(), &request.into_inner().message) {
            Ok(buf) => msg = buf,
            Err(_) => msg = "".to_string(),
        }

        let reply = flow::FlowReply { message: msg };
        Ok(Response::new(reply))
    }
}
