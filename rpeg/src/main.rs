use rpeg::codec::{compress, decompress};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let argnum = args.len();
    println!("number of arguments = {}", argnum);
    assert!(argnum == 4 || argnum == 5);
    let filename = args.iter().nth(3).unwrap();
    let output = args.iter().nth(4).unwrap();
    match args[2].as_str() {
        "-c" => compress(filename, output),
        "-d" => decompress(filename),
        _ => {
            eprintln!("Usage: rpeg -d [filename]\nrpeg -c [filename]")
        }
    }
}
