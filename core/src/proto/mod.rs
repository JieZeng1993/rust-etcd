

#[test]
fn code_gen(){
    protobuf_codegen_pure::Codegen::new()
        .out_dir("test-lib/src/proto")
        .include("test-lib")
        .inputs(&[
            // "src/protos/src/prototype.proto",
            "test-lib/src/proto/instruction.proto",
        ])
        .run()
        .expect("Running protoc failed.");

}

