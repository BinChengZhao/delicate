fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("proto/generated_codes")
        //  protos: &[Specific proto file path], includes: &[The directory to which the proto path is included]
        .compile(
            &["proto/actuator.proto", "proto/actuator.health.proto"],
            &["proto"],
        )?;
    Ok(())
}
