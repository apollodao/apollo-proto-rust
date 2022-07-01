use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(
        &["src/osmosis/gamm/v1beta1/tx.proto"],
        &["src/"], // NOTE: must have the slash in the end, i.e. `src/` not `src`
    )?;
    Ok(())
}
