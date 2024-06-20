# Kernpad

This is a basic Notepad-style text editor written in Rust using the [Druid](https://github.com/linebender/druid) library. This project is intended as a learning project for exploring GUI development in Rust. 

The goal is to create a cross-platform, lightweight, and portable plain text editor that is simple and streamlined, similar to Windows Notepad, but with the potential for more features to be added over time.

## Features

- [x] Cross-platform (Windows, macOS, Linux)
- [x] Basic text editing
- [x] Load and save files
- [ ] New File
- [ ] Undo/Redo
- [ ] Find/Replace
- [ ] Preferences for customizing the editor
- [ ] Status bar toggle 
- [ ] Localization?
- [ ] Printing? 
- [ ] Tabbed interface for multiple files? 

## Building from Source

Clone the repository:

```bash
git clone https://github.com/k3rs3d/kernpad.git
cd kernpad
```

Build the project:

```bash
cargo build
```

Run the project:

```bash
cargo run
```

## Usage

Simply type directly into the text box. 

Use the menu options to open, save, or create new files. 

Current keyboard shortcuts:

- **Load File**: `Ctrl + O`
- **Save File**: `Ctrl + S`
- **Save File As**: `ctrl + Shift + S`
- **Quit**: `Ctrl + Q`