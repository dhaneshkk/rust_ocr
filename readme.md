
# Rust OCR Tool using Tesseract and PDFium

A robust, command-line tool for extracting text from image-based PDF files using the Tesseract OCR engine, written in Rust.

This project serves as a complete, end-to-end example of how to:
-   Interface with native C/C++ libraries (Tesseract, PDFium) from Rust.
-   Render PDF pages to high-quality images suitable for OCR.
-   Perform basic image preprocessing to improve OCR accuracy.
-   Build a clean and reliable command-line application with proper error handling.

## Features

-   Processes multi-page PDF files sequentially.
-   Renders each PDF page to a 300 DPI TIFF image to maximize OCR quality.
-   Extracts UTF-8 text from each page.
-   Handles temporary file creation and cleanup automatically.
-   Built with robust error handling to provide clear feedback.

## Prerequisites & Installation

Setting up this project involves three main steps: installing the Rust toolchain, installing the native system dependencies required for **compilation**, and installing the Tesseract language data required for **execution**.

### Step 1: Install the Rust Toolchain

Ensure you have the Rust compiler and Cargo installed. The easiest way is via [rustup.rs](https://rustup.rs/).

### Step 2: Install System Build Dependencies

You must install development packages for Cargo to compile the project and its native dependencies.

#### On Debian / Ubuntu / Linux Mint

These packages provide the C/C++ toolchain and libraries for Tesseract. **Note that we do NOT need to install PDFium manually.**

```bash
sudo apt-get update
sudo apt-get install -y \
    build-essential \
    pkg-config \
    libtesseract-dev \
    libleptonica-dev \
    libclang-dev \
    clang
```
-   `build-essential`: Provides the fundamental C/C++ compiler (`gcc`, `make`).
-   `pkg-config`: Helps Rust's build scripts find system libraries.
-   `libtesseract-dev` & `libleptonica-dev`: The development files for the Tesseract OCR engine.
-   `libclang-dev` & `clang`: Required by the `bindgen` crate to generate Rust bindings from C/C++ headers.

**PDFium Library Note:** This project uses the `pdfium-render` crate with its `bundle` feature enabled in `Cargo.toml`. This tells the build script to automatically download and compile the PDFium library, so no manual installation is required.

### Step 3: Install Tesseract Language Data (Runtime Dependency)

The compiled program needs Tesseract's language data files (`.traineddata`) to run.

#### On Debian / Ubuntu / Linux Mint
Install the English language pack:
```bash
sudo apt-get install tesseract-ocr-eng
```
(For all languages, you can use `tesseract-ocr-all`).

## Building and Running the Tool

After completing all prerequisites, you can build and run the application.
0. ** Download libpdfium.so under lib folder
get libpdfium.so from https://github.com/lailogue/rust-pdf-viewer
```mkdir -p lib

# Download PDFium library for x64 systems
curl -L -o pdfium-linux-x64.tgz \
  "https://github.com/bblanchon/pdfium-binaries/releases/latest/download/pdfium-linux-x64.tgz" && \
  tar -xzf pdfium-linux-x64.tgz -C lib --strip-components=1 && \
  rm pdfium-linux-x64.tgz
  ```
1.  **Build the Project**
    Cargo will download and compile all the Rust crates. The first build may take several minutes as it also compiles the C++ PDFium library.
    ```bash
    RUSTFLAGS="-L /home/ubuntu/rust_ocr/lib" LD_LIBRARY_PATH="/home/ubuntu/rust_ocr/lib" cargo build
    ```
    The final executable will be located at `target/release/rust_ocr_tool`.

2.  **Run the OCR Tool**
    Run the tool from the command line, passing the path to the PDF you want to process as an argument.
    ```bash
    RUSTFLAGS="-L /home/ubuntu/rust_ocr/lib" LD_LIBRARY_PATH="/home/ubuntu/rust_ocr/lib" cargo run --release -- path/to/your/document.pdf
    ```
    

## Troubleshooting

Working with native C/C++ libraries can sometimes lead to build-time or run-time errors. We have compiled a comprehensive guide based on common issues encountered during the development of this tool.

**➡️ See the detailed [TROUBLESHOOTING.md](./TROUBLESHOOTING.md) guide for solutions to common problems.**

This guide covers:
-   Build errors like `linker 'cc' not found` or missing system libraries (`leptonica`, `tesseract`).
-   Runtime errors like missing `.traineddata` files.
-   Runtime warnings about invalid image resolution (DPI).
-   Issues related to breaking changes in dependency APIs.



---

