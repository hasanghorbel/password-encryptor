#[macro_use]
extern crate magic_crypt;

use magic_crypt::MagicCryptTrait;
use structopt::StructOpt;

fn main() {
    let options = Options::from_args();

    if !options.encrypt.is_empty() {
        let mcrypt = new_magic_crypt!("magickey", 256);

        let encrypted_string = mcrypt.encrypt_str_to_base64(options.encrypt);

        println!("Encrypted String: {}", encrypted_string);
    } else if !options.decrypt.is_empty() {
        let mcrypt = new_magic_crypt!("magickey", 256);

        let decrypted_string = mcrypt.decrypt_base64_to_string(options.decrypt).unwrap();

        println!("Decrypted String: {}", decrypted_string);
    }
}

#[derive(StructOpt)]
struct Options {
    #[structopt(short = "e", long = "encrypt", default_value = "")]
    encrypt: String,
    #[structopt(short = "d", long = "decrypt", default_value = "")]
    decrypt: String,
}
