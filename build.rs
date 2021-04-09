fn main() {
    tonic_build::compile_protos("proto/helloworld.proto")
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}
//
// fn main() {
//     tonic_build::configure()
//         .build_client(false)
//         .out_dir("data/")
//         .compile(&["proto/echo_def.proto"], &["."])
//         .expect("failed to compile protos");
// }
