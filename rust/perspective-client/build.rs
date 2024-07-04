// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ ██████ ██████ ██████       █      █      █      █      █ █▄  ▀███ █       ┃
// ┃ ▄▄▄▄▄█ █▄▄▄▄▄ ▄▄▄▄▄█  ▀▀▀▀▀█▀▀▀▀▀ █ ▀▀▀▀▀█ ████████▌▐███ ███▄  ▀█ █ ▀▀▀▀▀ ┃
// ┃ █▀▀▀▀▀ █▀▀▀▀▀ █▀██▀▀ ▄▄▄▄▄ █ ▄▄▄▄▄█ ▄▄▄▄▄█ ████████▌▐███ █████▄   █ ▄▄▄▄▄ ┃
// ┃ █      ██████ █  ▀█▄       █ ██████      █      ███▌▐███ ███████▄ █       ┃
// ┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫
// ┃ Copyright (c) 2017, the Perspective Authors.                              ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃ This file is part of the Perspective library, distributed under the terms ┃
// ┃ of the [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0). ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use std::io::Result;
use std::path::Path;

fn prost_build() -> Result<()> {
    // This source file is included at `publish` time, but not `sbuild` time
    // because it is initially generated from the `perspective.proto` definition
    // in the C++ source.
    if std::env::var("CARGO_FEATURE_EXTERNAL_PROTO").is_ok() {
        println!("cargo:warning=MESSAGE Building in development mode");
        let root_dir_env = std::env::var("PSP_ROOT_DIR").expect("Must set PSP_ROOT_DIR");
        let root_dir = Path::new(root_dir_env.as_str());
        let proto_file = Path::join(root_dir, "cpp/protos/perspective.proto");
        let include_path = proto_file
            .parent()
            .expect("Couldn't determine parent directory of proto_file")
            .to_path_buf();

        println!("cargo:rerun-if-changed={}", proto_file.to_str().unwrap());

        #[cfg(feature = "external-proto")]
        std::env::set_var("PROTOC", protobuf_src::protoc());

        prost_build::Config::new()
            // .bytes(["ViewToArrowResp.arrow", "from_arrow"])
            .type_attribute("ViewOnUpdateResp", "#[derive(ts_rs::TS)]")
            .field_attribute("ViewOnUpdateResp.delta", "#[serde(with = \"serde_bytes\")]")
            .field_attribute("ViewToArrowResp.arrow", "#[serde(skip)]")
            .field_attribute("from_arrow", "#[serde(skip)]")
            .type_attribute(".", "#[derive(serde::Serialize)]")
            .type_attribute("ViewDimensionsResp", "#[derive(serde::Deserialize)]")
            .type_attribute("TableValidateExprResp", "#[derive(serde::Deserialize)]")
            .type_attribute(
                "ColumnType",
                "#[derive(serde::Deserialize)]  #[serde(rename_all = \"snake_case\")]",
            )
            .type_attribute("ExprValidationError", "#[derive(serde::Deserialize)]")
            .compile_protos(&[proto_file], &[include_path])
            .unwrap();

        std::fs::rename(
            std::env::var("OUT_DIR").unwrap() + "/perspective.proto.rs",
            "src/rust/proto.rs",
        )?;
    }

    Ok(())
}

fn main() -> Result<()> {
    prost_build()?;
    Ok(())
}
