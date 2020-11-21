use core::fmt::Debug;

use std::env;
use std::ffi::OsStr;
use std::path::Path;
use std::process;
use std::process::Command;

fn openssl<S>(args: &[S]) -> Result<(), String>
where
    S: AsRef<OsStr> + Debug,
{
    match Command::new("openssl").args(args).status() {
        Ok(status) => {
            if status.success() {
                Ok(())
            } else {
                Err(format!("openssl failed; args: {:?}", args))
            }
        }
        Err(err) => Err(format!("can't run openssl: {}", err)),
    }
}

fn main() {
    let argv: Vec<_> = env::args_os().skip(1).collect();
    if !(1..2).contains(&argv.len()) {
        eprintln!("usage: generate <ca> [server]");
        process::exit(2);
    }

    let ca_name = Path::new(&argv[0]);
    let ca_key = ca_name.with_extension("key");

    // There's no good reason file paths should ever be represented as &str
    // rather than &OsStr or &Path, except that only &str supports literal
    // syntax, and I'll be damned if I'm going to junk up every argument to
    // every subprocess with OsStr::new().
    let ca_key = ca_key
        .to_str()
        .expect("name must be representable in UTF-8");

    #[rustfmt::skip]
    let command = [
        "genpkey",           // Generate a private key.
        "-out", &ca_key,     // output file name
        "-outform", "PEM",   // output format
        "-algorithm", "RSA", // algorithm to generate the key
        "-aes-256-cbc",      // algorithm to encrypt the key
    ];

    if let Err(err) = openssl(&command) {
        eprintln!("error: {}", err);
        process::exit(1);
    }
}
