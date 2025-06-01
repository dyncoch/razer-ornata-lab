# Razer RGB Control for macOS

**Stage: Alpha**

This Rust-based tool lets you control the RGB lighting of supported Razer keyboards on macOS.

## ✅ Supported Device

- Razer Ornata V3

## ⚙️ Features

- Set static RGB color (red, green, blue)
- Breathing, Spectrum, and Wave effects
- Low-level USB control using `rusb`
- No kernel extensions or drivers required

## 🚀 Getting Started

### 1. Install Dependencies

- Rust: https://rustup.rs
- `libusb` (via Homebrew)

```bash
brew install libusb
git clone https://github.com/dyncoch/razer-rgb-mac.git
cd razer-rgb-mac
cargo build --release

# may need sudo, don't know yet
sudo ./target/release/razer-rgb-mac
```

# 🔍 Based On
OpenRazer

librazermacos

# 📎 Notes
This is experimental, for development and testing only.

Only tested on macOS Ventura with Razer Ornata V3.

Contributions and testing feedback welcome!

📜 License
MIT
