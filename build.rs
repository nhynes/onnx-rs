extern crate protoc_rust;

macro_rules! src_dir {
    () => { "third_party/onnx/onnx" };
    ($src_path:expr) => { concat!(src_dir!(), "/", $src_path) };
}

fn main() {
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/",
        #[cfg(feature = "proto3")]
        input: &[src_dir!("onnx.proto3")],
        #[cfg(not(feature = "proto3"))]
        input: &[src_dir!("onnx.proto")],
        includes: &[src_dir!()],
    }).expect("Failed to run protoc");
}
