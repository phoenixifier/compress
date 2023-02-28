use std::io::Write;
use flate2::Compression;
use flate2::write::GzEncoder;
use crate::args;
use crate::args::Args;

pub struct Compressor {
    args: Args
}

impl Compressor {
   pub fn new(args: Args) -> Compressor {
        Compressor {
            args
        }
    }

    pub fn compress_output(&self) -> GzEncoder<Box<dyn Write>> {
        GzEncoder::new(self.args.output_reader(), Compression::default())
    }

    pub fn finished(&self){
        self.compress_output().finish().unwrap();
    }
}