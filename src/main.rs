pub use anyhow::Result;
mod icon;

use std::path::PathBuf;
use structopt::StructOpt;
use log::{info};

#[derive(Debug, StructOpt)]
#[structopt(name = "icons-cli", about = "从png生成跨平台图标集")]
struct Opt {
    #[structopt(short, long = "--input", parse(from_os_str))]
    input: PathBuf,
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf
}

fn main() {
    std::env::set_var("RUST_LOG", "info");
    let args = Opt::from_args();

    env_logger::init();
    info!("input: {:?}", args.input);
    info!("output: {:?}", args.output);

    let options = icon::Options {
        input: args.input,
        output: Option::from(args.output),
        png: None,
        ios_color: "#fff".to_string(),
    };
    
    icon::command(options).unwrap();
}
