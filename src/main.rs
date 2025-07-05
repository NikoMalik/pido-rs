use anyhow::Result;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use pido_rs::parallel_check;
use rand::Rng;
use thiserror::Error;
use tokio::fs::File as AsyncFile;
use tokio::io::AsyncWriteExt;
use tracing::{error, info};

#[derive(Error, Debug)]
#[error("Pido-rs error")]
enum PidoError {
    #[error("Failed to write output file: {0}")]
    FileWrite(#[from] std::io::Error),
    #[error("Failed to serialize results: {0}")]
    Serialization(#[from] bincode::Error),
    #[error("Failed to serialize JSON: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Parser, Debug)]
#[clap(
    name = "pido-rs",
    about = "A BLAZINGLY FAST and MEMORY SAFE even/odd checker!",
    version = "0.1.0"
)]
struct Args {
    #[clap(short, long, default_value_t = 1000000)]
    count: usize,
    #[clap(short, long, default_value = "results.bin")]
    output: String,
    #[clap(long, help = "Output results as JSON instead of binary")]
    json: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Parse CLI arguments
    let args = Args::parse();
    info!("🚀 Starting pido-rs with {} numbers", args.count);

    // Generate random numbers
    let pb = ProgressBar::new(args.count as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .expect("Failed to set progress bar template")
            .progress_chars("#>-"),
    );
    let numbers: Vec<i32> = (0..args.count)
        .map(|_| {
            pb.inc(1);
            rand::thread_rng().gen_range(-1000000..=1000000)
        })
        .collect();
    pb.finish_with_message("Generated numbers! 🎉");

    // Perform parallel check
    let start = std::time::Instant::now();
    let results = parallel_check(&numbers);
    let duration = start.elapsed();
    info!("Parallel check completed in {:?}", duration);

    // Print first few results
    for result in results.iter().take(5) {
        println!(
            "Number {}: is_even: {}, is_odd: {}",
            result.number, result.is_even, result.is_odd
        );
    }

    // Save results (async)
    if args.json {
        let json = serde_json::to_vec(&results)?;
        let mut file = AsyncFile::create(&args.output).await?;
        file.write_all(&json).await?;
        info!("Results saved as JSON to {}", args.output);
    } else {
        let serialized = bincode::serialize(&results)?;
        let mut file = AsyncFile::create(&args.output).await?;
        file.write_all(&serialized).await?;
        info!("Results saved as binary to {}", args.output);
    }

    Ok(())
}
