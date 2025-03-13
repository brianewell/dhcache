pub use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Cli {
    // Bind address
    #[arg(short, long, default_value="::1", value_name="address")]
    pub bind: String,

    // Bind port
    #[arg(short, long, default_value_t=4000, value_name="port")]
    pub port: u16,

    // Number of worker threads to use for concurrent parameter generation
    #[arg(short, long, default_value_t=1, value_name="workers")]
    pub workers: u8,

    // Follow-on arguments in the form of "<bits>:<count>"
    #[arg(value_name="count>:<bits", required=true, value_parser=parse_count_bits)]
    pub cache_parameters: Vec<(u8,u16)>
}

fn parse_count_bits(s: &str) -> Result<(u8,u16), String> {
    let parts: Vec<&str> = s.split(':').collect();

    if parts.len() != 2 {
        return Err(format!("Invalid format: '{}'. Expected <bits>:<count>.", s));
    }

    let count = parts[0]
        .parse::<u8>()
        .map_err(|_| format!("Invalid count value: '{}'. Must be a positive integer.", parts[0]))?;

    if count == 0 {
        return Err(format!("Invalid count: '{}'. Must be greater than zero.", count));
    }

    let bits = parts[1]
        .parse::<u16>()
        .map_err(|_| format!("Invalid bits value: '{}'. Must be a valid integer.", parts[1]))?;

    Ok((count,bits))
}
