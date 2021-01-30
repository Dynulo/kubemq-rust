fn main() {
    tonic_build::compile_protos("proto/kubemq/kubemq.proto").unwrap();
}
