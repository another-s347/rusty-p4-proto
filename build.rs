fn main() {
    let mut config = prost_build::Config::new();
    config.bytes(&["."]);
    tonic_build::configure()
        .build_server(false)
        .compile_with_config(
            config,
            &[
                "./p4runtime/proto/p4/v1/p4runtime.proto",
                "./p4runtime/proto/p4/v1/p4data.proto",
                "./p4runtime/proto/p4/config/v1/p4info.proto",
                "./p4runtime/proto/p4/config/v1/p4types.proto",
                "./googleapis/google/rpc/status.proto",
                "./googleapis/google/rpc/code.proto"],
            &["./p4runtime/proto/","./googleapis/"]
        ).unwrap();
    let mut config = prost_build::Config::new();
    config.bytes(&["."]);
    tonic_build::configure()
        .build_server(false)
        .compile_with_config(
            config,
            &[
                "./gnmi/gnmi.proto",
                "./gnmi/gnmi_ext.proto"],
            &["./gnmi","./googleapis/"]
        ).unwrap();
}