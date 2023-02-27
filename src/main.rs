use std::env::args;
mod compress;

fn main() {
    use compress::Args;
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`");
        return;
    }
   
    let arguments = Args { 
        input_arg: args().nth(1).unwrap().to_string(),
        output_arg: args().nth(2).unwrap().to_string()
    };
    arguments.copy_contents().expect("Failed to copy");
    arguments.finished()
}
