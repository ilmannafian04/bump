use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(short, long, default_value_t = String::from("/etc/bump/bump.yml"))]
    pub config: String,
}
