use spelchek::prelude::*;

use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let config = spelchek::Config::build(&args[1..])?; // it is assumed that the first argument is
                                                       // the program name, therefore we pass only the actual arguments
    let res = spelchek::run(&config);
    if let Result::Err(error) = res {
        eprintln!("{error:?}");
        std::process::exit(1);
    }
    Ok(())
}
