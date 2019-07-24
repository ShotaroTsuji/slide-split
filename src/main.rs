use slide_split::*;
use structopt::StructOpt;
use std::path::PathBuf;
use std::fs::File;
use std::io::{BufRead, BufReader, Write, BufWriter};

fn execute_sliding_windows<R: BufRead>(reader: &mut R, opt: Opt) {
    let iter = reader.lines()
        .map(|result| result.unwrap());
    let iter: Box<Iterator<Item=Vec<String>>> =
        if opt.exact {
            Box::new(exact_sliding_windows_from_iter(iter, opt.width, opt.stride))
        } else {
            Box::new(sliding_windows_from_iter(iter, opt.width, opt.stride))
        };
    for (number, window) in iter.enumerate() {
        if let Some(max_windows) = opt.max_windows {
            if number >= max_windows {
                break;
            }
        }
        let file_name = format!("{}{:0digits$}", opt.prefix, number, digits=opt.digits);
        let file = File::create(file_name).unwrap();
        let mut writer = BufWriter::new(file);
        for x in window.iter() {
            writeln!(writer, "{}", x).unwrap();
        }
    }
}

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short="f", long="prefix", default_value="w")]
    prefix: String,
    #[structopt(short="n", long="digits", default_value="2")]
    digits: usize,
    #[structopt(short="w", long="width")]
    width: usize,
    #[structopt(short="s", long="stride")]
    stride: usize,
    #[structopt(short="m", long="max-windows")]
    max_windows: Option<usize>,
    #[structopt(short="e", long="exact")]
    exact: bool,
    #[structopt(name="FILE", parse(from_os_str))]
    file: PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    if opt.file.to_str().unwrap() == "-" {
        let handle = std::io::stdin();
        let mut handle = handle.lock();
        execute_sliding_windows(&mut handle, opt);
    } else {
        let file = File::open(&opt.file).unwrap();
        let mut reader = BufReader::new(file);
        execute_sliding_windows(&mut reader, opt);
    }
}
