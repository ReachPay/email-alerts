use std::sync::Arc;

use email_alerts::{start_grpc_server, AppContext, SettingsReader, APP_NAME, APP_VERSION};

use my_seq_logger::SeqLogger;

#[tokio::main]
async fn main() {
    let settings_reader = SettingsReader::new(".email-alerts").await;
    let settings_reader = Arc::new(settings_reader);

    SeqLogger::enable_from_connection_string(
        APP_NAME.to_string(),
        APP_VERSION.to_string(),
        settings_reader.clone(),
        None,
    );

    let app = AppContext::new(&settings_reader).await;
    let app = Arc::new(app);

    let telemetry_writer =
        my_telemetry_writer::MyTelemetryWriter::new(APP_NAME.to_string(), settings_reader.clone());

    telemetry_writer.start(app.app_states.clone(), my_logger::LOGGER.clone());

    http_is_alive_shared::start_up::start_server(
        APP_NAME.to_string(),
        APP_VERSION.to_string(),
        app.app_states.clone(),
    );

    start_grpc_server(app.clone(), 8888);
    app.app_states.wait_until_shutdown().await;
}
