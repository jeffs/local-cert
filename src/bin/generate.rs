use std::env;
use std::process;
use std::process::Command;

fn openssl(args: &[&str]) -> Result<(), String> {
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

#[rustfmt::skip]
fn generate_ca(name: &str) -> Result<(), String> {
    let key = format!("{}.key", name);
    let pem = format!("{}.pem", name);
    openssl(&[
        "genpkey",           // Generate a private key.
        "-out", &key,        // output file name
        "-outform", "PEM",   // output format
        "-algorithm", "RSA", // algorithm to generate the key
        "-aes-256-cbc",      // algorithm to encrypt the key
    ])?;
    openssl(&[
        "req", "-new", "-x509", // Request a new X.509 certificate.
        "-key", &key,           // public key to be signed (and private key to sign with)
        "-sha256",              // message digest to sign the request
        "-out", &pem,           // output file name
        "-days", "7300",        // how long before the cert expires
    ])
}

#[allow(unused_variables)]
fn generate_server(name: &str, ca_name: &str) -> Result<(), String> {
    todo!()
}

fn main() {
    let argv: Vec<_> = env::args().skip(1).collect();
    if let Err(err) = match argv.len() {
        1 => generate_ca(&argv[0]),
        2 => generate_server(&argv[1], &argv[0]),
        _ => {
            eprintln!("usage: generate <ca> [server]");
            process::exit(2);
        }
    } {
        eprintln!("error: {}", err);
        process::exit(1);
    }
}
