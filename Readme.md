
# qrgen

`qrgen` is a simple command-line tool for generating QR code PNG images. 

## Features
- Generate QR codes from any input string, such as URLs, text, or other data.
- Customize the size of the QR code in pixels.
- Output the QR code as a PNG file.

## Installation
To build the project, you'll need to have [Rust](https://www.rust-lang.org/) installed. Clone the repository and build the project using `cargo`:

```sh
git clone https://github.com/bguo068/qrgen.git
cd qrgen
cargo build --release
```

## Usage

```sh
Usage: qrgen [OPTIONS] --input <INPUT> --output <OUTPUT>

Options:
  -i, --input <INPUT>    Input string, such as a URL or text.
  -s, --size <SIZE>      Size in pixels for output QR PNG [default: 1024].
  -o, --output <OUTPUT>  Output PNG file path.
  -h, --help             Print help information.
  -V, --version          Print version information.
```

## Example

Generate a QR code from a URL and save it as a PNG file:

```sh
./target/release/qrgen \
  --input 'https://github.com/bguo068/qrgen' --output url_qr.png
```

## Result

Here is an example of a generated QR code:

![Generated QR Code](./url_qr.png)

## Contributing

We welcome contributions! Please feel free to submit a pull request or open an issue.

## License

`qrgen` is licensed under the MIT License. See the [LICENSE](./LICENSE) file for more details.

