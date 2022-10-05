fn main() {
    tonic_build::compile_protos("proto/TestService.proto").unwrap();
}