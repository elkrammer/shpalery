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
    match &s[..] {
        "hot" => Ok(s.to_string()),
        "hour" => Ok(s.to_string()),
        "week" => Ok(s.to_string()),
        "month" => Ok(s.to_string()),
        "year" => Ok(s.to_string()),
        "all" => Ok(s.to_string()),
        _ => Err("".to_string()),
    }
}
