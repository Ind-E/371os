use std::{
    fs::File,
    io::{BufReader, Read},
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, args_override_self = true)]
struct Args {
    #[arg(short = 'c', long)]
    bytes: bool,
    #[arg(short = 'm', long)]
    chars: bool,
    #[arg(short, long)]
    lines: bool,
    #[arg(short = 'L', long)]
    max_line_length: bool,
    #[arg(short, long)]
    words: bool,

    filename: Option<String>,
}

#[derive(Default)]
struct Counts {
    bytes: usize,
    chars: usize,
    lines: usize,
    max_line_length: usize,
    words: usize,
}

fn main() {
    let mut args = Args::parse();
    if !args.bytes && !args.chars && !args.lines {
        args.bytes = true;
        args.chars = true;
        args.lines = true;
    }
    let mut counts = Counts::default();

    if let Some(f) = args.filename {
        let file = File::open(&f).unwrap();
        let mut reader = BufReader::new(file);

        let mut buf = [0; 8192];
        let mut in_word = false;
        let mut current_line_len = 0;

        while let Ok(bytes_read) = reader.read(&mut buf) {
            if bytes_read == 0 {
                break;
            }

            let chunk = &buf[..bytes_read];

            if args.bytes {
                counts.bytes += bytes_read;
            }

            for &b in chunk {
                if args.chars && (b & 0xC0 != 0x80) {
                    counts.chars += 1;
                }

                if b == b'\n' {
                    if args.lines {
                        counts.lines += 1;
                    }

                    if args.max_line_length {
                        if current_line_len > counts.max_line_length {
                            counts.max_line_length = current_line_len;
                        }
                        current_line_len = 0;
                    }
                } else {
                    current_line_len += 1;
                }

                if args.words {
                    if (b as char).is_whitespace() {
                        in_word = false;
                    } else if !in_word {
                        in_word = true;
                        counts.words += 1;
                    }
                }
            }
        }

        if current_line_len > counts.max_line_length {
            counts.max_line_length = current_line_len;
        }

        if args.lines {
            print!(" {}", counts.lines);
        }
        if args.words {
            print!(" {}", counts.words);
        }
        if args.chars {
            print!(" {}", counts.chars);
        }
        if args.bytes {
            print!(" {}", counts.bytes);
        }
        if args.max_line_length {
            print!(" {}", counts.max_line_length);
        }
        println!(" {}", f);
    } else {
        println!("stdio mode");
    }
}
