
# Rust OCR Tool using Tesseract

A simple, robust command-line tool for extracting text from images using the Tesseract OCR engine, written in Rust.

This project serves as a complete, end-to-end example of how to:
-   Interface with a native C/C++ library (Tesseract) from Rust.
-   Perform basic image preprocessing to improve OCR accuracy.
-   Build a clean and reliable command-line application with proper error handling.

## Features

-   Extracts UTF-8 text from various image formats (PNG, JPEG, etc.).
-   Includes an image preprocessing step (grayscale conversion) to enhance accuracy.
-   Simple and straightforward command-line interface.
-   Built with robust error handling.

## Prerequisites & Installation

Setting up this project involves three main steps: installing the Rust toolchain, installing the native system dependencies required for **compilation**, and installing the Tesseract language data required for **execution**.

### Step 1: Install the Rust Toolchain

Ensure you have the Rust compiler and Cargo installed. The easiest way is via [rustup.rs](https://rustup.rs/).

### Step 2: Install System Build Dependencies

You must install the development packages for **Tesseract**, **Leptonica**, and **Clang** so that Cargo can compile the project.

#### On Debian / Ubuntu / Linux Mint

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
-   `libtesseract-dev`: The Tesseract OCR engine development files.
-   `libleptonica-dev`: The image processing library Tesseract depends on.
-   `libclang-dev` & `clang`: Required by the `bindgen` crate to generate Rust bindings from C/C++ headers.

#### On Fedora / CentOS / RHEL

```bash
sudo dnf groupinstall "Development Tools"
sudo dnf install -y \
    pkg-config \
    tesseract-devel \
    leptonica-devel \
    clang-devel
```

#### On macOS

Using [Homebrew](https://brew.sh/):
```bash
# This will install tesseract and its dependencies, including leptonica
brew install tesseract

# Install the Xcode Command Line Tools if you haven't already
xcode-select --install
```

#### On Windows

1.  **Install Build Tools for Visual Studio:**
    -   Download the installer from the [Visual Studio Downloads page](https://visualstudio.microsoft.com/downloads/) (under "Tools for Visual Studio").
    -   Run the installer and select the **"Desktop development with C++"** workload.
2.  **Install Tesseract:**
    -   Download an installer from the official [Tesseract at UB Mannheim](https://github.com/UB-Mannheim/tesseract/wiki) repository.
    -   **Important:** During installation, ensure you select to install the **development files (headers and libraries)**.
    -   Add the Tesseract installation directory (e.g., `C:\Program Files\Tesseract-OCR`) to your system's `PATH` environment variable.

### Step 3: Install Tesseract Language Data (Runtime Dependency)

The compiled program needs Tesseract's language data files (`.traineddata`) to run. Without them, it will fail with an error like `Failed loading language 'eng'`.

#### On Debian / Ubuntu / Linux Mint
Install the English language pack:
```bash
sudo apt-get install tesseract-ocr-eng
```
(For all languages, you can use `tesseract-ocr-all`).

#### On Fedora / CentOS / RHEL
Install the English language pack:
```bash
sudo dnf install tesseract-langpack-eng
```

#### On macOS
The English language pack is typically installed by default with `brew install tesseract`.

#### On Windows
The language packs are chosen during the Tesseract installation process. Re-run the installer and make sure you have selected "English" (or any other languages you need).

## Building and Running the Tool

After completing all prerequisites, you can build and run the application.

1.  **Build the Project**
    Cargo will download and compile all the Rust crates.
    ```bash
    cargo build --release
    ```
    The final executable will be located at `target/release/rust_ocr_tool`.

2.  **Download a Test Image**
    A standard image for testing Tesseract is `eurotext.png`. You can download it into your project directory:
    ```bash
    curl -o test.png https://raw.githubusercontent.com/tesseract-ocr/tessdoc/main/images/eurotext.png
    ```

3.  **Run the OCR Tool**
    Using Cargo is the easiest way:
    ```bash
    cargo run -- test.png
    ```
    Or by running the compiled binary directly:
    ```bash
    ./target/release/rust_ocr_tool test.png
    ```

### Expected Output

```
Preprocessing image...
Extracting text from image...

--- Extracted Text ---
The (quick) [brown] {fox} jumps!
Over the $43,456.78 <lazy> #90 dog
& duck/goose, as 12.5% of E-mail
from aspammer@website.com is spam.
...
--- End of Text ---
```