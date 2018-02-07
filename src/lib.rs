extern crate protobuf;

mod onnx;

pub const ONNX_SRC_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/third_party/onnx/onnx");

pub mod proto {
    #[allow(unused)]
    use onnx::*;
}
