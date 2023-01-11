mod app_ctx;
mod grpc_client;
mod grpc_server;
mod settings;

pub mod alerts_service_grpc {
    tonic::include_proto!("alerts_service");
}

pub mod sendgridbridge_grpc {
    tonic::include_proto!("sendgridbridge");
}

pub mod credentials_grpc {
    tonic::include_proto!("credentials");
}

pub use app_ctx::*;
pub use grpc_client::*;
pub use grpc_server::*;
pub use settings::*;
