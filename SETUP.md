# Setup Requirements

## System Dependencies

This project requires the following system dependencies for compilation:

### OpenBLAS (for ndarray-linalg)

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install -y libopenblas-dev pkg-config
```

**macOS:**
```bash
brew install openblas pkg-config
```

**Fedora/RHEL:**
```bash
sudo dnf install openblas-devel pkg-config
```

### Fontconfig (for plotters - optional, only needed for plotting feature)

**Ubuntu/Debian:**
```bash
sudo apt-get install -y libfontconfig1-dev
```

**macOS:**
```bash
brew install fontconfig
```

**Fedora/RHEL:**
```bash
sudo dnf install fontconfig-devel
```

## Building

Once system dependencies are installed:

```bash
# Build the library
cargo build

# Run tests
cargo test

# Build with plotting support
cargo build --features plotting
```

## Alternative: Use system LAPACK

If you prefer to use system LAPACK instead of OpenBLAS, update `Cargo.toml`:

```toml
ndarray-linalg = { version = "0.16", features = ["lapack"] }
```

Then install LAPACK development libraries:
- Ubuntu/Debian: `sudo apt-get install liblapack-dev`
- macOS: `brew install lapack`
- Fedora/RHEL: `sudo dnf install lapack-devel`

