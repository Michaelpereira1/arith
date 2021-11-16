use rpeg::codec::{compress, decompress};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let argnum = args.len();
    assert!(argnum == 2 || argnum == 3);
    let filename = args.iter().nth(2).unwrap();
    match args[1].as_str() {
        "-c" => compress(filename),
        "-d" => decompress(filename),
        _ => {
            eprintln!("Usage: rpeg -d [filename]\nrpeg -c [filename]")
        }
    }
}
