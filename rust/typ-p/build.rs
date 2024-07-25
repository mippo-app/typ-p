use std::path::PathBuf;

use m_helper::fs::get_files;

// -> Result<(), Box<dyn std::error::Error>>
fn main() {
    {
        let protos = get_files("../../proto/typ_p/", &"proto".into());

        tonic_build::configure()
            .build_client(true)
            .build_server(true)
            .out_dir("./src/pb/")
            .compile(&protos, &[PathBuf::from("../../proto/")])
            .unwrap();
    }

    // Ok(())
}
