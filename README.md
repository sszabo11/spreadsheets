![Spreadsheet](https://github.com/sszabo11/spreadsheets/blob/main/screenshots/spreadsheet.png?raw=true)

# 📝 Terminal Spreadsheet App 🦀

Welcome to the Terminal Spreadsheet App, a Rust-powered, low-level, terminal-based spreadsheet application! 🚀

## 🎉 Features

- **Keyboard Navigation**: Use arrow keys to move between cells effortlessly.
- **Custom Cell Colors**: Highlight cells with custom colors for better visibility.
- **Text Editing**: Enter and edit text directly in any cell.
- **Minimal Flicker**: Optimized rendering to minimize flicker during screen updates.
- **Simple formulas**: Currently Sum and Product formulas work to a range of cells.
- **Commands (In progress)**: Enter command mode to execute tasks. Currently, only `:w` (save) works.
- **Save and load cells from database**: Automatically loads cell data from the database and saves it using the `:w` command.

## 🏗️ Still working progress
- I still want to include all the future features listed below
- Still very buggy

## 🛠️ Installation

To get started, clone this repository and build the project using Cargo:

```bash
git clone https://github.com/sszabo11/spreadsheet-app-rust.git
cd spreadsheet-app-rust
cargo build --release
```

## 🧑‍💻 Usage
- Before running make sure you have a redis server on `127.0.0.1:6379`
- This is where the cell data will be stored
  
[](url)
Run the application with:

```bash
cargo run --release
```

## 🔮 Future Features
- **Home screen to view sheets:** List of sheets and ability to create or edit.
- **Complex formulas:** Support for advanced formulas.
- **Styling:** Apply different styles to cells (bold, italic, etc.).
- **Import Excel Sheets:** Import existing Excel sheets into the terminal app.
- **Export as Excel:** Export your spreadsheet as an Excel file.

## 🎮 Controls
- **Arrow Keys:** Navigate between cells.
- **Enter:** Start editing the selected cell.
- **Backspace:** Delete characters in the selected cell.
- **Esc:** Exit the application.
- **Tab:** To escape edit mode

## 📦 Dependencies
This project uses the following crates:
- **crossterm**: For terminal input/output handling.
- **redis**: For communicating to the database.

## 🥰 Feel free to use this
