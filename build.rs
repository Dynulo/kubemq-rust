fn main() {
    tonic_build::configure()
        .build_server(false)
        .compile(&["proto/kubemq/kubemq.proto"], &["proto/kubemq/"])
        .unwrap();
}
