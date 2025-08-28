
# Troubleshooting Guide for Rust OCR Tool

This guide provides solutions to common build-time and run-time errors you may encounter while setting up and using this project.

## Table of Contents
1.  [Build-Time (Compilation) Errors](#build-time-compilation-errors)
2.  [Run-Time Errors](#run-time-errors)

---

## Build-Time (Compilation) Errors

These errors occur when you run `cargo build` or `cargo run`. They almost always indicate a missing system dependency.

### Error: `linker 'cc' not found`

-   **Cause:** Cargo cannot find a C/C++ compiler on your system. Rust crates with C/C++ dependencies need this to link everything together.
-   **Solution:** Install the standard build toolchain for your OS.
    -   **Debian/Ubuntu:** `sudo apt-get install build-essential`
    -   **macOS:** `xcode-select --install`
    -   **Windows:** Install "Build Tools for Visual Studio" with the "Desktop development with C++" workload.

### Error: `The system library 'lept' required by crate 'leptonica-sys' was not found`

-   **Cause:** The Tesseract OCR engine depends on the Leptonica image processing library. The development headers for this library are missing.
-   **Solution:** Install the Leptonica development package.
    -   **Debian/Ubuntu:** `sudo apt-get install libleptonica-dev`

### Error: `fatal error: 'stddef.h' file not found`

-   **Cause:** This error occurs while building `leptonica-sys` or `tesseract-sys`. The `bindgen` tool, which generates Rust code from C headers, requires the Clang compiler's libraries and headers to work correctly.
-   **Solution:** Install the Clang development package.
    -   **Debian/Ubuntu:** `sudo apt-get install libclang-dev clang`

### Error: `The system library 'tesseract' required by crate 'tesseract-sys' was not found`

-   **Cause:** The development headers for the main Tesseract library are missing.
-   **Solution:** Install the Tesseract development package.
    -   **Debian/Ubuntu:** `sudo apt-get install libtesseract-dev`

### Error: `E: Unable to locate package libpdfium-dev`

-   **Cause:** The PDFium library used for rendering PDFs is not available in standard system repositories like `apt`.
-   **Solution:** Do not try to install it manually. Instead, use the `bundle` feature in `Cargo.toml` to have the build script download and compile PDFium for you automatically.
    ```toml
    [dependencies]
    pdfium-render = { version = "0.8.35", features = ["pdfium-bindings/bundle"] }
    ```

### Error: `mismatched types`, `method not found`, or `module is private`

-   **Cause:** A dependency crate (like `pdfium-render`) has been updated to a new version with breaking API changes. Your `main.rs` code is written for an older version.
-   **Solution:** This is a normal part of software development. You must update your code to match the new API.
    1.  Read the compiler errors carefully—they are your best guide.
    2.  Check the crate's documentation on [crates.io](https://crates.io/) or its repository for a changelog or updated examples.
    3.  The `Cargo.lock` file is designed to prevent this by locking dependencies to specific versions. If you run `cargo update`, you may pull in breaking changes.

---

## Run-Time Errors

These errors occur after the project has successfully compiled and you are running the executable (e.g., `cargo run --release -- my_doc.pdf`).

### Error: `Error opening data file ... eng.traineddata`

-   **Cause:** The program is running, but the Tesseract engine cannot find the language data files it needs to perform OCR.
-   **Solution:** Install the Tesseract language packs for your system.
    -   **Debian/Ubuntu:** `sudo apt-get install tesseract-ocr-eng` (for English) or `tesseract-ocr-all` (for all languages).

### Warning: `Invalid resolution 1 dpi. Using 70 instead.`

-   **Cause:** The intermediate TIFF image saved to disk does not contain DPI metadata. Tesseract sees this, warns you, and falls back to a very low default resolution (70 DPI), which harms OCR accuracy.
-   **Solution:** Do not worry about the image metadata. Instead, explicitly tell Tesseract what resolution to expect using the `user_defined_dpi` variable. In `extract_text_from_image`, add `.set_variable("user_defined_dpi", "300")` to your Tesseract command chain.

### Error: `Failed to load PDF file` or the program is `Killed`

-   **Cause:** The input PDF file itself is the problem. This can be due to:
    1.  **Corruption:** The file is damaged. Try opening it in a standard PDF viewer.
    2.  **Encryption:** The file is password-protected. This tool does not support encrypted PDFs.
    3.  **Out of Memory:** The PDF page is extremely large or complex, causing the rendering engine to exhaust system memory and crash.
-   **Solution:** Verify the integrity and security settings of your input PDF. If it's a memory issue, try running the tool on a machine with more RAM.
