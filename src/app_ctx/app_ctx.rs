use std::sync::Arc;

use my_service_bus_tcp_client::MyServiceBusClient;
use rust_extensions::AppStates;
use crate::{
    SettingsModel, SendgridBridgeGrpcClient, ClientCredentialsGrpc,
};

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");

pub struct AppContext {
    pub app_states: Arc<AppStates>,
    pub settings: SettingsModel,
    pub my_sb_connection: MyServiceBusClient,
    pub sendgrid_bridge: SendgridBridgeGrpcClient,
    pub client_credentials: ClientCredentialsGrpc,
}

impl AppContext {
    pub async fn new(settings_reader: &Arc<crate::settings::SettingsReader>) -> Self {
        let settings = settings_reader.get_settings().await;
        let app_states = Arc::new(AppStates::create_initialized());
        let sendgrid_bridge = SendgridBridgeGrpcClient::new(settings.sendgrid_bridge_url.clone()).await;
        let client_credentials = ClientCredentialsGrpc::new(settings.client_credentials_url.clone()).await;
        let my_sb_connection = MyServiceBusClient::new(
            APP_NAME,
            APP_VERSION,
            settings_reader.clone(),
            my_logger::LOGGER.clone(),

        );

        Self {
            app_states,
            settings,
            sendgrid_bridge,
            my_sb_connection,
            client_credentials
        }
    }
}


// pub async fn bind_sb_subscribers(app: Arc<AppContext>) {
//     app.my_sb_connection.subscribe(
//         TEST_EVENT_SB_TOPIC_NAME.to_string(),
//         APP_NAME.to_string(),
//         TopicQueueType::DeleteOnDisconnect,
//         Arc::new(TestEventListener::new(app.clone())),
//     )
//     .await;
// }
