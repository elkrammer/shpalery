use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct ShpaleryArgs {
    /// Amount of Wallpapers to Download
    // #[clap(short, long, value_parser, default_value_t = 10)]
    #[clap(value_parser)]
    pub amount: Option<i32>,
}
