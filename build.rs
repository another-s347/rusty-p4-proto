use std::env;

fn main() {
    tonic_build::configure()
        .build_server(false)
        .compile(
        &[
            "./p4runtime/proto/p4/v1/p4runtime.proto",
            "./p4runtime/proto/p4/v1/p4data.proto",
            "./p4runtime/proto/p4/config/v1/p4info.proto",
            "./p4runtime/proto/p4/config/v1/p4types.proto",
            "./googleapis/google/rpc/status.proto",
            "./googleapis/google/rpc/code.proto"],
        &["./p4runtime/proto/","./googleapis/"]
    ).unwrap();
}