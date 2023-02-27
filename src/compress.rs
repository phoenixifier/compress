use flate2::write::GzEncoder;
use std::fs::OpenOptions;
use flate2::Compression;
use std::fs::File;
use std::io::{copy, Read, Write};
use std::io::BufReader;


pub struct Args {
   pub input_arg: String,
   pub output_arg: String
}

impl Args {
    fn new(input_arg: String, output_arg: String) -> Args {
        Args{
            input_arg,
            output_arg
        }
    }

    pub fn input_reader(&self) -> Box<dyn Read>{
        Box::new(BufReader::new(
            File::open(&self.input_arg)
                .expect("Failed to read the file")))
    }

    pub fn output_reader(&self) -> Box<dyn Write>{
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.output_arg)
            .expect("Failed to create the file");
        Box::new(file)
    }

    pub fn compress_output(&self) -> GzEncoder<Box<dyn Write>> {
        GzEncoder::new(self.output_reader(), Compression::default())
    }

    pub fn copy_contents(&self) -> std::io::Result<u64> {
        copy(&mut self.input_reader(), &mut self.output_reader())
    }

    pub fn finished(&self){
        self.compress_output().finish().unwrap();
    }

}