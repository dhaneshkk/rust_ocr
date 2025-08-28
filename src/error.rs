use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OcrError {
    #[error("Failed to initialize PDFium library")]
    PdfiumInit,

   #[error("Failed to load PDF file: {0}")]
    PdfLoad(#[from] pdfium_render::prelude::PdfiumError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to render page {page_num} of '{path}'")]
    PageRender { page_num: u16, path: PathBuf },

    #[error("Failed to save temporary image to '{0}'")]
    ImageSave(PathBuf),

    #[error("Tesseract engine failed to initialize: {0}")]
    TesseractInit(String),

    #[error("Tesseract failed to process image '{0}'")]
    TesseractOcr(PathBuf),

    #[error("Image path is not valid UTF-8: '{0}'")]
    InvalidImagePath(PathBuf),

    #[error("Could not set Tesseract variable '{key}' to '{value}'")]
    TesseractVariable { key: String, value: String },

 // --- ADD THIS VARIANT ---
 #[error("Advanced image preprocessing failed for '{0}'")]
 ImageProcessingFailed(PathBuf),
}