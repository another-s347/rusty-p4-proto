use grpcio_compiler;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").expect("The compiler did not provide $OUT_DIR");
    grpcio_compiler::prost_codegen::compile_protos(
        &[
            "./p4runtime/proto/p4/v1/p4runtime.proto",
            "./p4runtime/proto/p4/v1/p4data.proto",
            "./p4runtime/proto/p4/config/v1/p4info.proto",
            "./p4runtime/proto/p4/config/v1/p4types.proto",
            "./googleapis/google/rpc/status.proto",
            "./googleapis/google/rpc/code.proto"],
        &["./p4runtime/proto/","./googleapis/"],
        &out_dir
    ).unwrap();
}