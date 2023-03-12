use std::io::{stdin, Write};
use std::string::String;
use std::thread;
use clap::Parser;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::fs::File;

#[derive(Parser, Debug)]
struct Args {

    #[arg(short, long)]
    dir: PathBuf,

    #[arg(long, default_value_t=20)]
    num_files: usize,

    #[arg(long, help="size in MB of each file", default_value_t=2.0)]
    size_files: f32,
}

fn create_log_file(args: &Args, i: usize) -> File {
    return File::create(args.dir.join(format!("log{i}", i=i))).unwrap();
}

fn main() {
    let args = Args::parse();

    let (sndr, recv) = channel();

    thread::spawn(move || {
        let mut buf = String::new();
        while let Ok(_) = stdin().read_line(&mut buf) {
            sndr.send(buf).unwrap();
            buf = String::new();
        }
    });

    let mut bytes_written: usize = 0;
    let mut file_num: usize = 0;
    let mut log_file = create_log_file(&args, file_num);

    while let Ok(line) = recv.recv() {
         match log_file.write_all(line.as_bytes()) {
            Err(e) => eprintln!("Error writing line {}: {:?}", line, e),
            Ok(()) => bytes_written += line.len(),
        }
        if bytes_written > (args.size_files * 1024.0 * 1024.0) as usize {
            bytes_written = 0;
            file_num = (file_num + 1) % (args.num_files - 1);
            log_file = create_log_file(&args, file_num);
        }
    };
}