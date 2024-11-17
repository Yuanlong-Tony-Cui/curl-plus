use structopt::StructOpt;

// Use `StructOpt` to collect target CLI arguments by their flags:
#[derive(StructOpt, Debug)]
pub struct Cli {
    pub url: String,
    #[structopt(short = "X", long)]
    pub method: Option<String>,
    #[structopt(short = "d", long)]
    pub data: Option<String>,
    #[structopt(long = "json", conflicts_with("data"), conflicts_with("method"))]
    pub json_data: Option<String>,
}

impl Cli {
    pub fn parse_args() -> Self {
        StructOpt::from_args()
    }
}