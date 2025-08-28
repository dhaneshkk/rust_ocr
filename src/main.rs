use anyhow::{Context, Result};
use pdfium_render::prelude::*;
use std::env;
use std::path::Path;
use tempfile::tempdir;
use tesseract::Tesseract;

fn main() -> Result<()> {
    // ... (main function is unchanged)
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        anyhow::bail!("Usage: {} <path_to_pdf>", args[0]);
    }
    let pdf_path = Path::new(&args[1]);
    if !pdf_path.exists() {
        anyhow::bail!("PDF path does not exist: {}", pdf_path.display());
    }

    println!("Processing PDF: {}", pdf_path.display());

    let extracted_pages = process_pdf_sequentially(pdf_path)
        .context("Failed to process PDF sequentially")?;

    println!("\n--- OCR Results ---");
    for (page_num, text) in extracted_pages {
        println!("\n--- Page {} ---\n", page_num + 1);
        println!("{}", text);
    }
    println!("\n--- End of Document ---");

    Ok(())
}

fn process_pdf_sequentially(pdf_path: &Path) -> Result<Vec<(usize, String)>> {
    // ... (this function is unchanged)
    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library())?,
    );

    let document = pdfium.load_pdf_from_file(pdf_path, None)
        .with_context(|| format!("Failed to load PDF file: {}", pdf_path.display()))?;

    let temp_dir = tempdir().context("Failed to create temporary directory")?;

    let page_count = document.pages().len();
    println!("PDF has {} pages. Starting sequential processing...", page_count);

    let mut results = Vec::with_capacity(page_count as usize);

    for page_index in 0..page_count {
        let current_page_num = page_index + 1;
        println!("Processing page {} of {}...", current_page_num, page_count);

        let tiff_path = temp_dir.path().join(format!("page_{}.tiff", page_index));

        render_page_to_tiff(&document, page_index as usize, &tiff_path)
            .with_context(|| format!("Failed to render page {}", current_page_num))?;

        let text = extract_text_from_image(&tiff_path)
            .with_context(|| format!("Failed to extract text from page {}", current_page_num))?;

        results.push((page_index as usize, text));
    }

    Ok(results)
}

fn render_page_to_tiff(document: &PdfDocument, page_index: usize, output_path: &Path) -> Result<()> {
    // ... (this function is unchanged)
    let render_config = PdfRenderConfig::new()
        .set_maximum_width(300);
       // .for_printing(true);

    let page = document.pages().get(page_index as u16)
        .context("Failed to get page from PDF document")?;

    let image = page.render_with_config(&render_config)?
        .as_image();

    image.save_with_format(output_path, image::ImageFormat::Tiff)
        .with_context(|| format!("Failed to save image to {}", output_path.display()))?;

    Ok(())
}

/// Initializes Tesseract and extracts text from a given image file.
fn extract_text_from_image(image_path: &Path) -> Result<String> {
    // This function contains the fix.
    let text = Tesseract::new(None, Some("eng"))
        .context("Failed to initialize Tesseract")?
        .set_image(image_path.to_str().unwrap())
        .context("Failed to set image for Tesseract")?
        // Explicitly tell Tesseract the image resolution.
        .set_variable("user_defined_dpi", "300")
        .context("Failed to set DPI variable")?
        .get_text()
        .context("Failed to get UTF-8 text from Tesseract")?;

    Ok(text)
}