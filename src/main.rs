use std::env::args;
use args::Args;
use compress::Compressor;
mod args;
mod compress;

fn main() {
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`");
        return;
    }
   
    let arguments = Args { 
        input_arg: args().nth(1).unwrap().to_string(),
        output_arg: args().nth(2).unwrap().to_string()
    };

    args::copy_contents(&arguments).expect("Failed to copy");
    let c = Compressor::new(arguments);
    c.finished();
}
