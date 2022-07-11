use std::io::Result;

fn main() -> Result<()> {
    // prost_build::Config::new()
    //     .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
    //     .compile_protos(
    //         &[
    //             "src/osmosis/gamm/v1beta1/tx.proto",
    //             "src/osmosis/tokenfactory/v1beta1/tx.proto",
    //             "src/osmosis/lockup/tx.proto",
    //             "src/osmosis/superfluid/tx.proto",
    //         ],
    //         &["src/"], // NOTE: must have the slash in the end, i.e. `src/` not `src`
    //     )?;
    Ok(())
}
