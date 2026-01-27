use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter};

use anyhow::{Result, Context};
use log::{info, error};
use simplelog::{
    Config,
    TermLogger,
    TerminalMode,
    LevelFilter,
    ColorChoice,
};

use meika256_lib::{
    encrypt_file,
    decrypt_file,
    encrypt_stream,
    decrypt_stream,
};

const STREAM_THRESHOLD: u64 = 10 * 1024 * 1024; // 10 MB
const STREAM_CHUNK: usize = 64 * 1024;          // 64 KB

fn main() -> Result<()> {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;

    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!(
            "Usage: {} [encrypt|decrypt] <input> <output> <password>",
            args[0]
        );
        std::process::exit(1);
    }

    let mode = &args[1];
    let input = &args[2];
    let output = &args[3];
    let password = &args[4];

    let file_size = std::fs::metadata(input)
        .with_context(|| format!("Cannot access file {}", input))?
        .len();

    info!("Processing {} ({} bytes)", input, file_size);

    match mode.as_str() {
        "encrypt" => {
            if file_size >= STREAM_THRESHOLD {
                info!("Using streaming encryption");

                let reader = BufReader::new(File::open(input)?);
                let writer = BufWriter::new(File::create(output)?);

                encrypt_stream(reader, writer, password, STREAM_CHUNK)?;
            } else {
                info!("Using buffer encryption");

                let data = std::fs::read(input)?;
                let encrypted = encrypt_file(&data, password)?;
                std::fs::write(output, encrypted)?;
            }
        }

        "decrypt" => {
            if file_size >= STREAM_THRESHOLD {
                info!("Using streaming decryption");

                let reader = BufReader::new(File::open(input)?);
                let writer = BufWriter::new(File::create(output)?);

                decrypt_stream(reader, writer, password)?;
            } else {
                info!("Using buffer decryption");

                let data = std::fs::read(input)?;
                let decrypted = decrypt_file(&data, password)?;
                std::fs::write(output, decrypted)?;
            }
        }

        _ => {
            error!("Invalid mode: {}", mode);
            std::process::exit(2);
        }
    }

    info!("Done → {}", output);
    Ok(())
}
