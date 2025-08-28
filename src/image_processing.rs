// src/image_processing.rs
use crate::error::OcrError;
use image::{io::Reader as ImageReader, ImageFormat, Luma};
use imageproc::contrast::adaptive_threshold;
use log::debug;
use std::path::Path;

/// Applies advanced preprocessing to an image to improve OCR accuracy.
///
/// This involves converting the image to grayscale and applying adaptive thresholding
/// to create a clean, high-contrast black and white image.
pub fn preprocess_image(image_path: &Path) -> Result<(), OcrError> {
    debug!("Applying advanced preprocessing to '{}'", image_path.display());

    // Load the image from the temporary file
    let img = ImageReader::open(image_path)
        .map_err(|_| OcrError::ImageProcessingFailed(image_path.to_path_buf()))?
        .decode()
        .map_err(|_| OcrError::ImageProcessingFailed(image_path.to_path_buf()))?;

    // Convert to grayscale
    let grayscale_img = img.to_luma8();

    // Apply adaptive thresholding. The block radius determines the size of the local
    // region to analyze for thresholding. This value may need tuning for different
    // document types, but 21 is a reasonable default.
    let thresholded_img = adaptive_threshold(&grayscale_img, 21);

    // Convert back to a type that can be saved easily.
    // The thresholded image is already a Luma<u8> image.
    let final_img = image::DynamicImage::ImageLuma8(thresholded_img);

    // Overwrite the original temporary image with the preprocessed version
    final_img
        .save_with_format(image_path, ImageFormat::Tiff)
        .map_err(|_| OcrError::ImageProcessingFailed(image_path.to_path_buf()))?;

    Ok(())
}