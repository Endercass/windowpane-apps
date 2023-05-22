fn main() {
    protobuf_codegen::Codegen::new()
        .input("manifest.proto")
        .include(".")
        .cargo_out_dir("manifest")
        .run_from_script();
}