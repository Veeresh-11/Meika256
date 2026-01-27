# Meika-256

Meika-256 is a high-performance encryption system featuring:

- AES-256-GCM authenticated encryption
- Custom Meika diffusion transform
- Streaming-safe file encryption
- CLI, DLL (C FFI), and SDK support
- Windows installer and VS Code extension

---

## Components

| Component | Description |
|--------|-------------|
| Engine | Core crypto implementation |
| CLI | File encryption command-line tool |
| DLL | C-compatible shared library |
| SDK | Headers, libs, examples |
| Installer | Windows setup package |
| VS Code Extension | Editor integration |

---

## CLI Usage

```sh
meika256-cli encrypt input.txt output.meika password
meika256-cli decrypt output.meika output.txt password
