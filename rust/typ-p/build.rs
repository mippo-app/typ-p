use std::fs;
use std::path;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    {
        let dir = fs::read_dir(format!("../../proto/typ_p"))?;
        let mut files: Vec<path::PathBuf> = Vec::new();
        for fff in dir.into_iter() {
            // println!("Name: {}", item?.path().display());
            let p = fff.unwrap().path();

            if p.extension().unwrap() == "proto" {
                files.push(p);
            }
        }
        // tonic_build::configure().compile(&files, &[PathBuf::from("../../proto/")])?;
        tonic_build::configure()
            .build_client(true)
            .build_server(true)
            // .extern_path(".basic", "framework::simple_type::pb::basic")
            // .extern_path(".simpleType.basic", "framework::simple_type::pb::basic")
            .out_dir("./src/pb/")
            .compile(
                &files,
                &[PathBuf::from("../../proto/")], /*&[
                                                      PathBuf::from(format!("{}/framework/apps/proto/", config.git_home.clone())),
                                                      PathBuf::from(format!("{}/googleapis/", config.git_home.clone())),
                                                      PathBuf::from(format!("{}/grpc-gateway/", config.git_home.clone())),
                                                      PathBuf::from(format!("{}/service/hapii/proto/", config.git_home.clone())),
                                                  ],*/
            )?;
    }

    Ok(())
}

/*
$ export PROTOBUF_LOCATION=$(nix-env -q protobuf --out-path --no-name)
$ export PROTOC=$PROTOBUF_LOCATION/bin/protoc
$ export PROTOC_INCLUDE=$PROTOBUF_LOCATION/include
$ cargo build
*/
/*
tonic_build::configure()
        .include_file("mod.rs")
        .build_client(false)
        .build_server(true)
        .out_dir("src/generated")
        .compile(&["../proto/service.proto"], &["../proto"])?;
    Ok(())
    */
