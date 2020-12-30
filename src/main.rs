use structopt::StructOpt;
use themelio_core::{run_main, Config};

fn main() {
    env_logger::Builder::from_env("THEMELIO_LOG")
        .parse_filters("themelio_core")
        .init();
    let opts = Config::from_args();
    smol::block_on(run_main(opts))
}
