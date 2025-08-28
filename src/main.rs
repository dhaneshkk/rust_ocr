use anyhow::{Context, Result};
use std::env;
use std::path::Path;
use tesseract::Tesseract;

fn main() -> Result<()> {
    // 1. Get image path from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        anyhow::bail!("Usage: {} <path_to_image>", args[0]);
    }
    let image_path = Path::new(&args[1]);
    if !image_path.exists() {
        anyhow::bail!("Image path does not exist: {}", image_path.display());
    }

    // 2. Preprocess the image for better OCR results
    println!("Preprocessing image...");
    let preprocessed_image_path = "temp_processed.png";
    prepare_image_for_ocr(image_path, preprocessed_image_path)
        .context("Failed to preprocess image")?;

    // 3. Extract text using Tesseract
    println!("Extracting text from image...");
    let extracted_text = extract_text(&preprocessed_image_path)
        .context("Failed to extract text with Tesseract")?;

    // 4. Clean up the temporary file
    std::fs::remove_file(preprocessed_image_path)?;

    // 5. Print the final result
    println!("\n--- Extracted Text ---");
    println!("{}", extracted_text);
    println!("--- End of Text ---");

    Ok(())
}

/// Preprocesses an image to improve OCR accuracy.
/// This function converts the image to grayscale and saves it to a temporary path.
fn prepare_image_for_ocr(input_path: &Path, output_path: &str) -> Result<()> {
    // Load the image from the specified path
    let img = image::open(input_path)
        .with_context(|| format!("Failed to open image: {}", input_path.display()))?;

    // Convert the image to grayscale to enhance text visibility
    let grayscale_img = img.grayscale();

    // Save the preprocessed image to the output path
    grayscale_img.save(output_path)
        .with_context(|| format!("Failed to save preprocessed image to: {}", output_path))?;

    Ok(())
}

/// Initializes Tesseract, sets configuration, and extracts text from an image.
fn extract_text(image_path: &str) -> Result<String> {
    // Initialize Tesseract for the English language.
    // `None` for the tessdata path lets Tesseract find it automatically.
    let mut ocr = Tesseract::new(None, Some("eng"))
        .context("Failed to initialize Tesseract")?;

    // Configure Tesseract with advanced options for better accuracy.
    // 'tessedit_char_whitelist' can be used to restrict recognition to certain characters.
    // For example, to only recognize numbers: .set_variable("tessedit_char_whitelist", "0123456789")?
    // 'tessedit_pageseg_mode' adjusts how Tesseract segments the image. '3' is the default.
    ocr = ocr
        .set_image(image_path)
        .context("Failed to set image for Tesseract")?
        .set_variable("tessedit_pageseg_mode", "3")
        .context("Failed to set page segmentation mode")?;

    // Perform the OCR and get the recognized text.
    let text = ocr.get_text()
        .context("Failed to get UTF-8 text from Tesseract")?;

    Ok(text)
}