use clap::{Arg, Command};
use encoding_rs::{Encoding, UTF_16BE, UTF_16LE, UTF_8};
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let matches = Command::new("File Encoding Checker")
        .version("1.0")
        .author("MJH (codeberg.org/mjh)")
        .about("Detects the encoding of a plaintext file")
        .arg(
            Arg::new("FILE")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    // Path to the plaintext file
    let file_path = matches
        .try_get_one::<String>("FILE")
        .or(Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "No file path provided",
        )))?;

    let file_path = file_path.unwrap().as_str();

    let path = PathBuf::from(file_path);

    // Read the file into a byte vector
    let mut file = File::open(&path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Detect the encoding of the file
    let (encoding_used, _) = for_bom(&buffer);

    // encodings are statics so we need a little bit of hackery here
    let encoding_name = match encoding_used {
        enc if enc == Some(UTF_8) => "UTF-8",
        enc if enc == Some(UTF_16LE) => "UTF-16LE",
        enc if enc == Some(UTF_16BE) => "UTF-16BE",
        _ => "Unknown",
    };

    println!("Detected encoding: {}", encoding_name);

    Ok(())
}

fn for_bom(buffer: &[u8]) -> (Option<&'static Encoding>, Option<usize>) {
    if buffer.starts_with(b"\xEF\xBB\xBF") {
        (Some(&UTF_8), Some(3))
    } else if buffer.starts_with(b"\xFF\xFE") {
        (Some(&UTF_16LE), Some(2))
    } else if buffer.starts_with(b"\xFE\xFF") {
        (Some(&UTF_16BE), Some(2))
    } else {
        (None, None)
    }
}
