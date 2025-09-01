# Sanup

![License](https://img.shields.io/badge/License-GNU-blue)
![Platform](https://img.shields.io/badge/Platform-Linux-lightgrey)
![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange?logo=rust)

Sanup is a TUI application for creating and managing backups on Linux-based operating systems. Sanup provides an intuitive interface for selecting files and directories, performing backups, and restoring data.

## Features

- **Browse and Select**: Navigate and select files or directories for backup using a vim keymap.
- **Backup Creation**: Create backups with options for compression and customizable destinations.
- **Restore Backups**: Restore files from previous backups with ease.
- **Backup History**: View and manage past backups.
- **Linux-Native**: Optimized for Linux, handling file permissions and paths correctly.

## Installation

### Prerequisites

- Rust (stable, version 1.80 or higher)
- Linux-based OS (e.g., Arch, Manjaro)
- Optional: `zip` or `tar` for compression support

### Steps

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/yourusername/sanup.git
   cd sanup
   ```

2. **Build the Project**:

   ```bash
   cargo build --release
   ```

3. **Run the Application**:

   ```bash
   cargo run --release
   ```

### Arch Linux (AUR)

Sanup is available on the [Arch User Repository (AUR)](https://aur.archlinux.org/). To install:

```bash
[paru](https://github.com/Morganamilo/paru) sanup
```

Or use your preferred AUR helper.

## Usage

1. Launch Sanup using `sanup` (if installed via AUR) or `cargo run --release`.
2. Use vim keymaps to navigate the TUI:
   - Select files/directories in the browse mode.
   - Configure backup options (e.g., destination, compression).
3. Press `q` or `Ctrl+C` to exit.

## Project Structure

## Dependencies

- `crossterm`: Terminal handling for input and raw mode.
- `tui`: TUI widgets and layouts (using crossterm backend).

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/YourFeature`).
3. Commit your changes (`git commit -m "Add YourFeature"`).
4. Push to the branch (`git push origin feature/YourFeature`).
5. Open a Pull Request.

## License

This project is licensed under the GNU General Public License v3.0. See the `LICENSE` file for details.

## Contact

For issues or suggestions, please open an issue on the GitHub repository.
