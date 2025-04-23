use clap::{Parser, ValueEnum};

#[derive(Parser)]
pub struct CliArgs {
    pub command: String,
    pub input: String,
    pub output: String,

    #[clap(long, value_enum)]
    pub algo: Option<Algo>,
}

#[derive(ValueEnum, Clone)]
pub enum Algo {
    Rle,
    Lz77,
}
impl CliArgs {
    pub fn parse_args() -> Result<Self, String> {
        let args = Self::parse();

        // Validate command is either "compress" or "decompress"
        if args.command != "compress" && args.command != "decompress" {
            return Err("Command must be either 'compress' or 'decompress'".to_string());
        }

        // If no algorithm specified, detect based on file extension
        let args = if args.algo.is_none() {
            let detected_algo = match crate::detect::detect_best_algorithm(&args.input) {
                "rle" => Algo::Rle,
                "lz" => Algo::Lz77,
                _ => Algo::Lz77 // Default to LZ77
            };
            Self {
                algo: Some(detected_algo),
                ..args
            }
        } else {
            args
        };

        Ok(args)
    }

    pub fn is_compress(&self) -> bool {
        self.command == "compress"
    }

    pub fn is_decompress(&self) -> bool {
        self.command == "decompress" 
    }

    pub fn is_rle(&self) -> bool {
        matches!(self.algo, Some(Algo::Rle))
    }

    pub fn is_lz77(&self) -> bool {
        matches!(self.algo, Some(Algo::Lz77))
    }
}

