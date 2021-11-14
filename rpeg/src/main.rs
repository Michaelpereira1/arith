use rpeg::codec::{compress, decompress};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let argnum = args.len();
    assert!(argnum == 4 || argnum == 5);
    let filename = args.iter().nth(3).unwrap();
    match args[2].as_str() {
        "-c" => compress(filename),
        "-d" => decompress(filename),
        _ => {
            eprintln!("Usage: rpeg -d [filename]\nrpeg -c [filename]")
        }
    }
}
