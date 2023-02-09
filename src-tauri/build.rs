fn main() {
    tauri_build::build();
    // prost_build::Config::new()
    //     .out_dir("src/pb")
    //     .compile_protos(&["./abi/abi.proto"], &["."])
    //     .unwrap()
}
