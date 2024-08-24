use clap::Parser;
use qrcode_generator::QrCodeEcc;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// input string, such as a url
    #[arg(short, long, required = true)]
    input: String,

    // #[arg(short, long, required = true)]
    // correction_level: String,
    /// size in pixels for output qr png
    #[arg(short, long, default_value = "1024")]
    size: usize,

    /// output png file path
    #[arg(short, long, required = true)]
    output: String,
}

fn main() {
    let cli = Cli::parse();
    if let Some(parent) = std::path::Path::new(&cli.output).parent() {
        std::fs::create_dir_all(parent).expect(&format!(
            "cannot create folder: {}",
            parent.to_string_lossy()
        ));
    }
    qrcode_generator::to_png_to_file(&cli.input, QrCodeEcc::Low, cli.size, &cli.output)
        .expect("fail to create qr png");
}
