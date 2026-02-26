# Taphouse 🌟

Welcome to Taphouse, a terminal UI designed to simplify the process of browsing, searching, and managing Homebrew packages! Taphouse offers a user-friendly interface with four main tabs:
- **Installed Formulae** 🍺
- **Installed Casks** 🥂
- **Browse Formulae** 🔍
- **Browse Casks** 📦

## Overview
Taphouse allows users to easily install, uninstall, and upgrade packages directly from their terminal. You can navigate between tabs to view and manage your Homebrew packages effortlessly.

## Features
- **Search Functionality**: Quickly find packages!
- **Keyboard Shortcuts**:
  - `q`: Quit
  - `Tab`/`BackTab`: Switch tabs
  - `j`/`k` or `up`/`down`: Navigate
  - `/`: Search
  - `i`: Install from browse tabs
  - `u`: Uninstall
  - `U`: Upgrade
  - `r`: Refresh

## Project Structure
```
/src/
  ├── main.rs
  ├── app.rs
  ├── brew
  └── ui
```

## Getting Started
To build and run Taphouse, follow these steps:
1. **Build the project**: Run `cargo build`
2. **Run the project**: Run `cargo run`

## Keyboard Shortcuts Documentation
Refer to the features section for a list of keyboard shortcuts that help you navigate and manage your Homebrew packages efficiently!

Happy brewing! 🍻