use std::time::Duration;

use crate::credentials_grpc::client_credentials_flows_grpc_service_client::ClientCredentialsFlowsGrpcServiceClient;
use crate::credentials_grpc::*;
use my_grpc_extensions::GrpcClientInterceptor;
use my_telemetry::MyTelemetryContext;
use tonic::codegen::InterceptedService;
use tonic::transport::Channel;

pub enum RegistrationResult {
    Registered(String),
    UserAlreadyExists,
}

pub struct ClientCredentialsGrpc {
    channel: Channel,
    timeout: Duration,
}

impl ClientCredentialsGrpc {
    pub async fn new(grpc_address: String) -> Self {
        let channel = Channel::from_shared(grpc_address)
            .unwrap()
            .connect()
            .await
            .unwrap();
        Self {
            channel,
            timeout: Duration::from_secs(3),
        }
    }

    fn create_grpc_service(
        &self,
        my_telemetry_context: &MyTelemetryContext,
    ) -> ClientCredentialsFlowsGrpcServiceClient<InterceptedService<Channel, GrpcClientInterceptor>>
    {
        let client: ClientCredentialsFlowsGrpcServiceClient<
            InterceptedService<Channel, GrpcClientInterceptor>,
        > = ClientCredentialsFlowsGrpcServiceClient::with_interceptor(
            self.channel.clone(),
            GrpcClientInterceptor::new(my_telemetry_context.clone()),
        );

        client
    }

    pub async fn get_email_by_id(
        &self,
        client_id: String,
        my_telemetry_context: &MyTelemetryContext,
    ) -> Option<String> {
        let grpc_request = GetEmailByIdRequest { client_id };

        let mut client = self.create_grpc_service(my_telemetry_context);

        let request = client.get_email_by_id(grpc_request);

        let response = tokio::time::timeout(self.timeout, request)
            .await
            .unwrap()
            .unwrap();

        let response = response.into_inner();

        if response.email.is_empty() {
            None
        } else {
            Some(response.email)
        }
    }
}
