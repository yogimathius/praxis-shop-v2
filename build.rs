fn main() {
    cynic_codegen::register_schema("forge")
        .from_sdl_file("schema.graphql")
        .unwrap()
        .as_default()
        .unwrap();
    println!("cargo:rerun-if-changed=schema.graphql");
}
