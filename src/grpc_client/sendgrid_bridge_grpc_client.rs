use std::time::Duration;

use my_grpc_extensions::GrpcClientInterceptor;
use my_telemetry::MyTelemetryContext;
use tonic::{codegen::InterceptedService, transport::Channel};

use crate::sendgridbridge_grpc::{
    send_grid_bridge_grpc_service_client::SendGridBridgeGrpcServiceClient,
    SendCreateMerchantNewDepositAddressRequest, SendMerchantDepositReceivedEmailRequest,
    SendMerchantExchangeEmailRequest, SendMerchantOrderExecuteRequest,
    SendMerchantSettingsUpdateRequest, SendMerchantWithdrawalCreateRequest,
    SendMerchantWithdrawalStatusUpdateRequest,
};

pub struct SendgridBridgeGrpcClient {
    timeout: Duration,
    channel: Channel,
}

impl SendgridBridgeGrpcClient {
    pub async fn new(grpc_address: String) -> Self {
        let channel = Channel::from_shared(grpc_address)
            .unwrap()
            .connect()
            .await
            .unwrap();
        Self {
            timeout: Duration::from_secs(5),
            channel,
        }
    }

    async fn create_grpc_service(
        &self,
        my_telemetry_context: &MyTelemetryContext,
    ) -> SendGridBridgeGrpcServiceClient<InterceptedService<Channel, GrpcClientInterceptor>> {
        return SendGridBridgeGrpcServiceClient::with_interceptor(
            self.channel.clone(),
            GrpcClientInterceptor::new(my_telemetry_context.clone()),
        );
    }

    pub async fn send_merchant_exchange_email(
        &self,
        email: String,
        amount: f64,
        currency_from: String,
        currency_to: String,
        my_telemetry_context: &MyTelemetryContext,
    ) {
        let mut client = self.create_grpc_service(my_telemetry_context).await;
        let request = SendMerchantExchangeEmailRequest {
            email,
            lang_id: "en".to_string(),
            amount,
            currency_from,
            currency_to,
        };

        tokio::time::timeout(self.timeout, client.send_merchant_exchange_email(request))
            .await
            .unwrap()
            .unwrap();
    }

    pub async fn send_deposit_received_email(
        &self,
        email: String,
        amount: f64,
        currency: String,
        my_telemetry_context: &MyTelemetryContext,
    ) {
        let mut client = self.create_grpc_service(my_telemetry_context).await;
        let request = SendMerchantDepositReceivedEmailRequest {
            email,
            lang_id: "en".to_string(),
            amount,
            currency,
        };

        tokio::time::timeout(
            self.timeout,
            client.send_merchant_deposit_received_email(request),
        )
        .await
        .unwrap()
        .unwrap();
    }

    pub async fn send_order_execute_email(
        &self,
        email: String,
        amount: f64,
        currency: String,
        order_id: String,
        my_telemetry_context: &MyTelemetryContext,
    ) {
        let mut client = self.create_grpc_service(my_telemetry_context).await;
        let request = SendMerchantOrderExecuteRequest {
            email,
            lang_id: "en".to_string(),
            amount,
            currency,
            order_id,
        };

        tokio::time::timeout(
            self.timeout,
            client.send_merchant_order_execute_email(request),
        )
        .await
        .unwrap()
        .unwrap();
    }

    pub async fn send_withdrawal_create_email(
        &self,
        email: String,
        amount: f64,
        currency: String,
        withdrawal_id: String,
        my_telemetry_context: &MyTelemetryContext,
    ) {
        let mut client = self.create_grpc_service(my_telemetry_context).await;
        let request = SendMerchantWithdrawalCreateRequest {
            email,
            lang_id: "en".to_string(),
            amount,
            currency,
            withdrawal_id,
        };

        tokio::time::timeout(
            self.timeout,
            client.send_merchant_withdrawal_create_email(request),
        )
        .await
        .unwrap()
        .unwrap();
    }

    pub async fn send_withdrawal_status_update_email(
        &self,
        email: String,
        withdrawal_id: String,
        status: String,
        my_telemetry_context: &MyTelemetryContext,
    ) {
        let mut client = self.create_grpc_service(my_telemetry_context).await;
        let request = SendMerchantWithdrawalStatusUpdateRequest {
            email,
            lang_id: "en".to_string(),
            status,
            withdrawal_id,
        };

        tokio::time::timeout(
            self.timeout,
            client.send_merchant_withdrawal_status_update(request),
        )
        .await
        .unwrap()
        .unwrap();
    }

    pub async fn send_merchant_settings_update_email(
        &self,
        email: String,
        my_telemetry_context: &MyTelemetryContext,
    ) {
        let mut client = self.create_grpc_service(my_telemetry_context).await;
        let request = SendMerchantSettingsUpdateRequest {
            email,
            lang_id: "en".to_string(),
        };

        tokio::time::timeout(self.timeout, client.send_merchant_settings_update(request))
            .await
            .unwrap()
            .unwrap();
    }

    pub async fn send_merchant_create_deposit_address(
        &self,
        email: String,
        id: String,
        currency: String,
        network: String,
        tag: String,
        my_telemetry_context: &MyTelemetryContext,
    ) {
        let mut client = self.create_grpc_service(my_telemetry_context).await;
        let request = SendCreateMerchantNewDepositAddressRequest {
            email,
            lang_id: "en".to_string(),
            id,
            currency,
            network,
            tag,
        };

        tokio::time::timeout(
            self.timeout,
            client.send_merchant_new_deposit_address_create(request),
        )
        .await
        .unwrap()
        .unwrap();
    }
}
