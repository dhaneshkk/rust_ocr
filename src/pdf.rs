use crate::error::OcrError;
use log::{debug, info};
use pdfium_render::prelude::*;
use std::path::{Path, PathBuf};
use tempfile::{tempdir, TempDir};
/// Renders all pages of a PDF into high-resolution TIFF images in a temporary directory.
///
/// Returns a tuple containing the `TempDir` handle (which must be kept in scope to prevent
/// premature cleanup) and a sorted vector of paths to the rendered images.
pub fn render_pdf_pages(
    pdf_path: &Path,
    _dpi: u16,
) -> Result<(TempDir, Vec<PathBuf>), OcrError> {
    info!("Initializing PDFium library...");
    let pdfium = Pdfium::new(
        Pdfium::bind_to_system_library().map_err(|_| OcrError::PdfiumInit)?,
    );

    info!("Loading PDF document from '{}'...", pdf_path.display());
    let document = pdfium.load_pdf_from_file(pdf_path, None)?;

    let temp_dir = tempdir()?;
    info!(
        "Created temporary directory for images at '{}'",
        temp_dir.path().display()
    );

    let page_count = document.pages().len();
    info!(
        "PDF has {} pages. Starting sequential rendering...",
        page_count
    );

    let mut image_paths = Vec::with_capacity(page_count as usize);

    for (i, page) in document.pages().iter().enumerate() {
        let page_num = i as u16;
        debug!("Rendering page {}...", page_num + 1);

        let tiff_path = temp_dir.path().join(format!("page_{:04}.tiff", page_num));

        // Calculate the target width and height in pixels
        // Get page dimensions
        let page_width_points = page.width();
        let page_height_points = page.height();

        // Calculate pixel dimensions
        let desired_dpi: f32 = 300.0;
        let standard_dpi: f32 = 72.0;
        let scale_factor = desired_dpi / standard_dpi;

        let target_width_f32 = (page_width_points * scale_factor).value as f32;
        let target_height_f32 = (page_height_points * scale_factor).value as f32;

        // Use a safe float-to-u32 conversion function


        // Create the config object
        let render_config = PdfRenderConfig::new().set_target_width(target_width_f32 as Pixels).set_target_height(target_height_f32 as Pixels);
       // render_config.set_target_height(target_height_f32 as Pixels);

        let image = page
            .render_with_config(&render_config)
            .map_err(|_| OcrError::PageRender {
                page_num: page_num + 1,
                path: pdf_path.to_path_buf(),
            })?
            .as_image();

        image
            .save_with_format(&tiff_path, image::ImageFormat::Tiff)
            .map_err(|_| OcrError::ImageSave(tiff_path.clone()))?;

        image_paths.push(tiff_path);
    }

    Ok((temp_dir, image_paths))
}