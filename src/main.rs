mod config;
mod error;
mod ocr;
mod pdf;
mod image_processing;

use crate::config::Config;
use crate::error::OcrError;
use clap::Parser;
use log::{error, info, warn};
use std::fs::File;
use std::io::Write;
use std::time::Instant;

fn main() {
    // Initialize the logger. The user can control the verbosity
    // by setting the `RUST_LOG` environment variable.
    // Example: `RUST_LOG=debug cargo run -- ...`
    env_logger::init();

    // Parse command-line arguments.
    let config = Config::parse();

    // Record the start time.
    let start_time = Instant::now();
    info!("Starting OCR process for '{}'", config.pdf_path.display());

    // Run the main application logic.
    if let Err(e) = run(&config) {
        error!("Application failed: {}", e);
        // Additional context for specific errors can be logged here if needed.
        std::process::exit(1);
    }

    info!(
        "Successfully finished OCR process in {:?}",
        start_time.elapsed()
    );
}

/// The core application logic.
fn run(config: &Config) -> Result<(), OcrError> {
    // Step 1: Render PDF pages to a temporary directory of images.
    // The `_temp_dir` handle is kept in scope to ensure the directory is cleaned up
    // automatically when this function returns (due to RAII).
    let (_temp_dir, image_paths) = pdf::render_pdf_pages(&config.pdf_path, config.dpi)?;

    let mut all_text = String::new();
    let total_pages = image_paths.len();

    // Step 2: Sequentially perform OCR on each rendered image.
    for (i, image_path) in image_paths.iter().enumerate() {
        let page_num = i + 1;
        info!("Processing page {} of {}...", page_num, total_pages);
        // Step 2a (Optional): Apply advanced image preprocessing if the flag is set.
        if config.preprocess {
            image_processing::preprocess_image(image_path)?;
        }

        // Step 2b: Perform OCR, passing the optional tessdata path.
        let tessdata_path_as_ref = config.tessdata_path.as_deref();
        match ocr::extract_text_from_image(image_path, &config.lang, config.dpi, tessdata_path_as_ref,  config.psm,  config.oem, ) {
            Ok(text) => {
                all_text.push_str(&format!("\n\n--- OCR Results for Page {} ---\n\n", page_num));
                all_text.push_str(&text);
            }
            Err(e) => {
                // Log a warning for a single failing page but continue processing others.
                warn!("Could not process page {}: {}. Skipping.", page_num, e);
                all_text.push_str(&format!(
                    "\n\n--- OCR FAILED for Page {} ---\n\n",
                    page_num
                ));
            }
        }
    }

    // Step 3: Write the collected text to the specified output.
    if let Some(output_path) = &config.output_path {
        info!("Writing results to file '{}'", output_path.display());
        let mut file = File::create(output_path)?;
        file.write_all(all_text.as_bytes())?;
    } else {
        info!("Printing results to console.");
        println!("{}", all_text);
    }

    Ok(())
}