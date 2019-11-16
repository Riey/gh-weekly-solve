use byte_slice_cast::AsMutByteSlice;
use crypto::digest::Digest;
use crypto::sha2::Sha512;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::io::{BufRead, BufReader};
use structopt::StructOpt;

use std::path::PathBuf;

#[derive(StructOpt)]
#[structopt(name = "gs")]
enum Opt {
    #[structopt(name = "gen", about = "Generate key and hash")]
    Gen {
        key_len: usize,
    },

    #[structopt(name = "pick", about = "Pick key from file")]
    Pick {
        #[structopt(parse(from_os_str))]
        file: PathBuf,
    },
}

fn main() {
    let opt = Opt::from_args();

    match opt {
        Opt::Gen { key_len } => {
            let key: String = thread_rng().sample_iter(Alphanumeric).take(key_len).collect();
            println!("----key----\n{}", key);

            let mut hash = Sha512::new();
            hash.input_str(&key);
            let hash = hash.result_str();

            println!("----hash----\n{}", hash);
        }
        Opt::Pick { file } => {
            let file = BufReader::with_capacity(1024 * 16, std::fs::File::open(file).unwrap());
            let mut keys: Vec<String> = file.lines().collect::<Result<_, _>>().unwrap();
            keys.sort();

            let mut hash = Sha512::new();

            for key in keys.iter() {
                hash.input_str(key);
            }

            let mut out = [0usize; 512 / std::mem::size_of::<usize>()];
            hash.result(out.as_mut_byte_slice());

            let result = out.iter().fold(0, |acc, x| acc ^ x);

            let index = result % keys.len();

            println!("----picked key----");
            println!("{}", keys[index]);
        }
    }
}
