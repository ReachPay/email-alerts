use std::sync::Arc;

use my_telemetry::MyTelemetryContext;

use super::server::GrpcService;

use crate::{
    alerts_service_grpc::{
        alerts_service_server::AlertsService, SendCreateMerchantNewDepositAddressAlertRequest,
        SendMerchantDepositReceivedAlertRequest, SendMerchantExchangeAlertRequest,
        SendMerchantOrderExecuteAlertRequest, SendMerchantSettingsUpdateAlertRequest,
        SendMerchantWithdrawalCreateAlertRequest, SendMerchantWithdrawalStatusUpdateAlertRequest,
    },
    AppContext,
};

#[tonic::async_trait]
impl AlertsService for GrpcService {
    async fn send_merchant_exchange_alert(
        &self,
        request: tonic::Request<SendMerchantExchangeAlertRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let telemetry = my_grpc_extensions::get_telemetry(
            &request.metadata(),
            request.remote_addr(),
            "send_merchant_exchange_alert",
        );
        let SendMerchantExchangeAlertRequest {
            merchant_id,
            amount,
            currency_from,
            currency_to,
        } = request.into_inner();

        let email = get_email(&self.app, &merchant_id, telemetry.get_ctx())
            .await
            .unwrap();

        self.app
            .sendgrid_bridge
            .send_merchant_exchange_email(
                email,
                amount,
                currency_from,
                currency_to,
                telemetry.get_ctx(),
            )
            .await;

        return Ok(tonic::Response::new(()));
    }

    async fn send_merchant_deposit_received_alert(
        &self,
        request: tonic::Request<SendMerchantDepositReceivedAlertRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let telemetry = my_grpc_extensions::get_telemetry(
            &request.metadata(),
            request.remote_addr(),
            "send_merchant_deposit_received_alert",
        );
        let SendMerchantDepositReceivedAlertRequest {
            merchant_id,
            amount,
            currency,
        } = request.into_inner();

        let email = get_email(&self.app, &merchant_id, telemetry.get_ctx())
            .await
            .unwrap();

        self.app
            .sendgrid_bridge
            .send_deposit_received_email(email, amount, currency, telemetry.get_ctx())
            .await;

        return Ok(tonic::Response::new(()));
    }

    async fn send_merchant_order_execute_alert(
        &self,
        request: tonic::Request<SendMerchantOrderExecuteAlertRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let telemetry = my_grpc_extensions::get_telemetry(
            &request.metadata(),
            request.remote_addr(),
            "send_merchant_order_execute_alert",
        );
        let SendMerchantOrderExecuteAlertRequest {
            merchant_id,
            amount,
            currency,
            order_id,
        } = request.into_inner();

        let email = get_email(&self.app, &merchant_id, telemetry.get_ctx())
            .await
            .unwrap();

        self.app
            .sendgrid_bridge
            .send_order_execute_email(email, amount, currency, order_id, telemetry.get_ctx())
            .await;

        return Ok(tonic::Response::new(()));
    }

    async fn send_merchant_withdrawal_create_alert(
        &self,
        request: tonic::Request<SendMerchantWithdrawalCreateAlertRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let telemetry = my_grpc_extensions::get_telemetry(
            &request.metadata(),
            request.remote_addr(),
            "send_merchant_withdrawal_create_alert",
        );
        let SendMerchantWithdrawalCreateAlertRequest {
            merchant_id,
            amount,
            currency,
            withdrawal_id,
        } = request.into_inner();

        let email = get_email(&self.app, &merchant_id, telemetry.get_ctx())
            .await
            .unwrap();

        self.app
            .sendgrid_bridge
            .send_withdrawal_create_email(
                email,
                amount,
                currency,
                withdrawal_id,
                telemetry.get_ctx(),
            )
            .await;

        return Ok(tonic::Response::new(()));
    }

    async fn send_merchant_withdrawal_status_update_alert(
        &self,
        request: tonic::Request<SendMerchantWithdrawalStatusUpdateAlertRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let telemetry = my_grpc_extensions::get_telemetry(
            &request.metadata(),
            request.remote_addr(),
            "send_merchant_withdrawal_status_update_alert",
        );
        let SendMerchantWithdrawalStatusUpdateAlertRequest {
            merchant_id,
            withdrawal_id,
            status,
        } = request.into_inner();

        let email = get_email(&self.app, &merchant_id, telemetry.get_ctx())
            .await
            .unwrap();

        self.app
            .sendgrid_bridge
            .send_withdrawal_status_update_email(email, withdrawal_id, status, telemetry.get_ctx())
            .await;

        return Ok(tonic::Response::new(()));
    }

    async fn send_merchant_new_deposit_address_create_alert(
        &self,
        request: tonic::Request<SendCreateMerchantNewDepositAddressAlertRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let telemetry = my_grpc_extensions::get_telemetry(
            &request.metadata(),
            request.remote_addr(),
            "send_merchant_new_deposit_address_create_alert",
        );
        let SendCreateMerchantNewDepositAddressAlertRequest {
            id,
            currency,
            network,
            tag,
            merchant_id,
        } = request.into_inner();

        let email = get_email(&self.app, &merchant_id, telemetry.get_ctx())
            .await
            .unwrap();

        self.app
            .sendgrid_bridge
            .send_merchant_create_deposit_address(
                email,
                id,
                currency,
                network,
                tag,
                telemetry.get_ctx(),
            )
            .await;

        return Ok(tonic::Response::new(()));
    }

    async fn send_merchant_settings_update_alert(
        &self,
        request: tonic::Request<SendMerchantSettingsUpdateAlertRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let telemetry = my_grpc_extensions::get_telemetry(
            &request.metadata(),
            request.remote_addr(),
            "send_merchant_settings_update_alert",
        );
        let SendMerchantSettingsUpdateAlertRequest { merchant_id } = request.into_inner();

        let email = get_email(&self.app, &merchant_id, telemetry.get_ctx())
            .await
            .unwrap();

        self.app
            .sendgrid_bridge
            .send_merchant_settings_update_email(email, telemetry.get_ctx())
            .await;

        return Ok(tonic::Response::new(()));
    }
}

async fn get_email(
    app: &Arc<AppContext>,
    client_id: &String,
    my_telemetry_context: &MyTelemetryContext,
) -> Option<String> {
    return app
        .client_credentials
        .get_email_by_id(client_id.clone(), my_telemetry_context)
        .await;
}
