use crate::lib::validate_fetch_type;
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct ShpaleryArgs {
    #[clap(short = 'a', value_parser = clap::value_parser!(i32).range(1..=200))]
    /// Amount of Wallpapers to Download
    pub amount: Option<i32>,
    #[clap(short = 'f', long = "fetch_type", value_parser = fetch_type_parser)]
    /// Fetch Type - hot, hour, day, week, month, year, all
    pub fetch_type: Option<String>,
    #[clap(short = 'v', long = "verbose")]
    /// Run in Verbose mode
    pub verbose: bool,
}

fn fetch_type_parser(s: &str) -> Result<String, String> {
    if validate_fetch_type(s.to_string()) {
        return Ok(s.to_string());
    } else {
        return Err(
        "Invalid fetch type\nFetch Type has to be one of: hot, hour, day, week, month, year, all"
            .to_string(),
    );
    }
}
