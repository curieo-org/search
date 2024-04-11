use std::error::Error;

// generated by `sqlx migrate build-script`
fn main() -> Result<(), Box<dyn Error>> {
    // trigger recompilation when a new migration is added
    println!("cargo:rerun-if-changed=migrations");

    // build the proto files
    let proto_files = ["protos/rag.proto"];

    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .type_attribute(
            "SearchRequest",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .type_attribute(
            "SearchResponse",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .type_attribute(
            "RouteCategory",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .type_attribute("Source", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute("Pair", "#[derive(serde::Deserialize, serde::Serialize)]")
        .compile(&proto_files, &["."])?;

    Ok(())
}
