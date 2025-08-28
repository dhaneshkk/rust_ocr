use crate::error::OcrError;
use log::debug;
use std::path::Path;
use tesseract::Tesseract;

/// Performs OCR on a single image file using Tesseract.
pub fn extract_text_from_image(
    image_path: &Path,
    lang: &str,
    dpi: u16,
    tessdata_path: Option<&Path>,
    psm: u8, // <-- ADD THIS
    oem: u8, // <-- ADD THIS
) -> Result<String, OcrError> {
    debug!(
        "Performing OCR on image '{}' with lang '{}' and dpi {}",
        image_path.display(),
        lang,
        dpi
    );

    let image_path_str = image_path.to_str().ok_or_else(|| OcrError::InvalidImagePath(image_path.to_path_buf()))?;
    let dpi_str = dpi.to_string();
    let psm_str = psm.to_string(); // Convert to string
    let oem_str = oem.to_string(); // Convert to string
    // --- UPDATE THIS BLOCK ---
    // Convert the optional Path to an optional &str for the Tesseract API.
    let tessdata_path_str = tessdata_path.and_then(|p| p.to_str());
    if let Some(path) = tessdata_path_str {
        debug!("Using custom tessdata path: {}", path);
    }

    let text = Tesseract::new(tessdata_path_str, Some(lang))


        .map_err(|e| OcrError::TesseractInit(e.to_string()))?
        .set_image(image_path_str)
        .map_err(|_| OcrError::TesseractOcr(image_path.to_path_buf()))?
        // --- UPDATE THIS BLOCK ---
        .set_variable("tessedit_pageseg_mode", &psm_str)
        .map_err(|_| OcrError::TesseractVariable { key: "tessedit_pageseg_mode".into(), value: psm_str })?
        .set_variable("tessedit_ocr_engine_mode", &oem_str)
        .map_err(|_| OcrError::TesseractVariable { key: "tessedit_ocr_engine_mode".into(), value: oem_str })?
        .set_variable("user_defined_dpi", &dpi_str)
        .map_err(|_| OcrError::TesseractVariable { key: "user_defined_dpi".into(), value: dpi_str })?
        .get_text()
        .map_err(|_| OcrError::TesseractOcr(image_path.to_path_buf()))?;

    Ok(text)
}

