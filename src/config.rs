use clap::Parser;
use std::path::PathBuf;

/// A robust tool to extract text from image-based PDF files using Tesseract OCR.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Path to the input PDF file.
    #[arg(required = true)]
    pub pdf_path: PathBuf,

    /// Path to the output text file. If not provided, output will be printed to the console.
    #[arg(short, long)]
    pub output_path: Option<PathBuf>,

    /// Tesseract language for OCR (e.g., "eng" for English).
    #[arg(short, long, default_value = "eng")]
    pub lang: String,

    /// DPI (dots per inch) to use for rendering PDF pages. Higher values improve accuracy but use more memory.
    #[arg(short, long, default_value_t = 300)]
    pub dpi: u16,

    // --- ADD THESE FLAGS ---
    /// Enable advanced image preprocessing (adaptive thresholding).
    /// Recommended for scanned documents with uneven lighting.
    #[arg(long)]
    pub preprocess: bool,

    /// Path to the Tesseract `tessdata` directory.
    /// Use this to specify the location of high-quality models (e.g., tessdata_best).
    #[arg(long, value_name = "PATH")]
    pub tessdata_path: Option<PathBuf>,


    // --- ADD THESE FLAGS ---
    /// Tesseract Page Segmentation Mode (PSM).
    /// Controls how Tesseract segments the page layout.
    /// Use `1` for automatic layout detection (default). Use `7` for a single line of text.
    #[arg(long, default_value_t = 1, value_name = "0-13")]
    pub psm: u8,

    /// Tesseract OCR Engine Mode (OEM).
    /// Use `3` for the default engine (best). Use `1` for the LSTM engine only.
    #[arg(long, default_value_t = 3, value_name = "0-3")]
    pub oem: u8,


}