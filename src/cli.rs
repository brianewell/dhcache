pub use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Cli {
    // Bind address
    #[arg(short, long, default_value="0.0.0.0", value_name="address")]
    bind: String,

    // Bind port
    #[arg(short, long, default_value_t=4000, value_name="port")]
    port: u16,

    // Number of worker threads to use for concurrent parameter generation
    #[arg(short, long, default_value_t=1, value_name="workers")]
    workers: u8,

    // Follow-on arguments in the form of "<bits>:<count>"
    #[arg(value_name="count>:<bits", required=true, value_parser=parse_count_bits)]
    cache_parameters: Vec<(u8,u16)>
}

fn parse_count_bits(_s: &str) -> Result<(u8,u16), String> {
    Ok((0,0))
}
