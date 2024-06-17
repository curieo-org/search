use std::error::Error;
use std::{fs, io};

fn main() -> Result<(), Box<dyn Error>> {
    // trigger recompilation when a new migration is added
    println!("cargo:rerun-if-changed=migrations");

    // build the proto files
    let proto_dir = ["proto"];
    let proto_files = ["proto/agency.proto"];

    copy_proto_files(&proto_files)?;
    println!("cargo:rerun-if-changed=proto");

    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .type_attribute(
            "SearchInput",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .type_attribute(
            "PubmedSource",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .type_attribute(
            "PubmedResponse",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .type_attribute(
            "EmbeddingsOutput",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .type_attribute(
            "Double2D",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .type_attribute("Int2D", "#[derive(serde::Deserialize, serde::Serialize)]")
        .type_attribute(
            "Embeddings",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .compile(&proto_files, &proto_dir)?;

    Ok(())
}

/// Copies protobuf files into project
fn copy_proto_files(proto_files: &[&str]) -> io::Result<()> {
    if fs::metadata("proto").is_err() {
        // proto dir doesn't exist. Just copy all the files.
        fs::create_dir("proto")?;
        proto_files.iter().for_each(|path| {
            if fs::metadata(path).is_err() {
                fs::copy(format!("../{path}"), path).unwrap();
            }
        });
        return Ok(());
    }

    // proto dir exists, so we have to do a bit more work.
    for path in proto_files {
        let source = format!("../{path}");
        match (fs::read(&source), fs::read(path)) {
            // Source exists, sink does not. Copy.
            (Ok(_), Err(_)) => fs::copy(source, path).map(|_| ())?, // Force () arm.
            // Source does not exist. Fail.
            (Err(e), Err(_)) => Err(e)?,
            // Source does not exist, local does. We trust in our local version.
            (Err(_), Ok(_)) => (),
            // Both exist. Compare contents and update if needed.
            // If we update when not needed we may trigger cargo:rerun-if-changed.
            (Ok(source_data), Ok(data)) => {
                if source_data != data {
                    fs::copy(source, path).map(|_| ())?
                }
            }
        };
    }

    Ok(())
}
