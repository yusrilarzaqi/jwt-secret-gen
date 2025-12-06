use clap::{Parser, ValueEnum};
use std::{
    fs::OpenOptions,
    io::{self, Write},
};

use jwt_secret_gen::{encode_secret, generate_secret, SecretFormat};

#[derive(ValueEnum, Clone, Copy, Debug)]
enum CliFormat {
    Base64,
    Urlsafe,
    Hex,
}

impl From<CliFormat> for SecretFormat {
    fn from(v: CliFormat) -> Self {
        match v {
            CliFormat::Base64 => SecretFormat::Base64,
            CliFormat::Urlsafe => SecretFormat::UrlSafe,
            CliFormat::Hex => SecretFormat::Hex,
        }
    }
}

#[derive(Parser, Debug)]
#[command(
    name = "jwt-secret-gen",
    version = "1.0.0",
    author = "Yusril Arzaqy",
    about = "A blazing-fast cryptographically secure JWT secret generator.",
    long_about = None
)]
struct Args {
    /// Number of random bytes to generate
    #[arg(short, long, default_value_t = 32)]
    bytes: usize,

    /// Output format: base64 | urlsafe | hex
    #[arg(short, long, value_enum, default_value_t = CliFormat::Urlsafe)]
    format: CliFormat,

    /// Generate multiple secrets
    #[arg(short, long, default_value_t = 1)]
    count: usize,

    /// Save to file (append)
    #[arg(short, long)]
    out: Option<String>,

    /// Add newline after each secret
    #[arg(long, default_value_t = false)]
    newline: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let format: SecretFormat = args.format.into();
    let mut results = Vec::with_capacity(args.count);

    if args.bytes == 0 {
        eprintln!("bytes must be > 0");
        std::process::exit(1);
    }

    for _ in 0..args.count {
        let raw = generate_secret(args.bytes);
        let encoded = encode_secret(&raw, format);
        results.push(encoded);
    }

    match args.out {
        Some(path) => {
            let mut file = OpenOptions::new().create(true).append(true).open(path)?;

            for (i, s) in results.iter().enumerate() {
                file.write_all(s.as_bytes())?;
                if args.newline || i + 1 < results.len() {
                    file.write_all(b"\n")?;
                }
            }

            println!("Wrote {} secrets to file.", results.len());
        }
        None => {
            for (i, s) in results.iter().enumerate() {
                if args.newline || i + 1 < results.len() {
                    println!("{}\n", s);
                } else {
                    print!("{}", s);
                }
            }
        }
    }

    Ok(())
}
