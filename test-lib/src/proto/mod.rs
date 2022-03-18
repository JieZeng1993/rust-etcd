use protobuf::Message;

use crate::instruction::Instruction;

mod instruction;

fn main() {
    // protobuf_codegen_pure::Codegen::new()
    //     .out_dir("test-lib/src/proto")
    //     .include("test-lib")
    //     .inputs(&[
    //         // "src/protos/src/prototype.proto",
    //         "test-lib/src/proto/instruction.proto",
    //     ])
    //     .run()
    //     .expect("Running protoc failed.");
    //
    let mut instruction = Instruction::new();
    println!("instruction: {:?}", instruction.get_a());
    instruction.set_a(1);
    instruction.set_b(2);
    // instruction.set_b("我司曾杰123123123123123123".to_string());
    println!("instruction: {:?}", instruction);

    println!("direct: 曾杰");

    println!("direct: {:?}", "我司曾杰".to_string());


    let bytes = instruction.write_to_bytes();
    let instruction = Instruction::parse_from_bytes(&*bytes.unwrap());
    println!("instruction: {:?}", instruction);
}