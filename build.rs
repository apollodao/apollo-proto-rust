use std::io::Result;

fn main() -> Result<()> {
    // prost_build::Config::new()
    //     .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
    //     .compile_well_known_types()
    //     .compile_protos(
    //         &[
    //             // osmosis
    //             "src/osmosis/gamm/v1beta1/tx.proto",
    //             "src/osmosis/gamm/v1beta1/query.proto",
    //             "src/osmosis/gamm/poolmodels/balancer/balancer_pool.proto",
    //             "src/osmosis/gamm/poolmodels/stableswap/v1beta1/stableswap_pool.proto",
    //             "src/osmosis/tokenfactory/v1beta1/tx.proto",
    //             "src/osmosis/lockup/tx.proto",
    //             "src/osmosis/lockup/query.proto",
    //             "src/osmosis/superfluid/tx.proto",
    //             // cosmos
    //             "src/cosmos/auth/v1beta1/auth.proto",
    //             "src/cosmos/base/v1beta1/coin.proto",
    //             "src/cosmos/base/query/v1beta1/pagination.proto",
    //             // google
    //             "src/google/protobuf/any.proto",
    //         ],
    //         &["src/"], // NOTE: must have the slash in the end, i.e. `src/` not `src`
    //     )?;
    Ok(())
}
