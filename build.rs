fn main() {
    tonic_build::compile_protos("proto/EmailAlerts.proto").unwrap();
    tonic_build::compile_protos("proto/SendGridBridge.proto").unwrap();
    tonic_build::compile_protos("proto/ClientCredentialsFlows.proto").unwrap();
}
