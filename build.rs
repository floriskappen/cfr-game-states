fn main() {
    prost_build::Config::new()
        .out_dir("src/proto/build")
        .compile_protos(&[
            "src/proto/clustered_data_labels.proto",
        ], &["src/"])
        .unwrap();
}
