use std::env;
use std::path::Path;
use std::process::{self, Command};

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
    let key = format!("{}.key", name);  // CA's keys
    let pem = format!("{}.pem", name);  // self-signed certificate
    openssl(&[
        "genpkey",           // Generate a private key.
        "-out", &key,        // output file name
        "-outform", "PEM",   // output format
        "-algorithm", "RSA", // algorithm to generate the key
        "-aes-256-cbc",      // algorithm to encrypt the key
    ])?;
    openssl(&[
        "req", "-new",   // Request a new certificate.
        "-x509",         // self-signed X.509 cert, not merely a CSR
        "-key", &key,    // public key to be signed and private key to sign with
        "-sha256",       // message digest to sign the request
        "-out", &pem,    // output file name
        "-days", "7300", // requested time before the cert expires
    ])
}

#[rustfmt::skip]
fn generate_server(name: &str, ca_name: &str) -> Result<(), String> {
    let ca_key = format!("{}.key", ca_name); // CA key to sign the cert with
    let ca_pem = format!("{}.pem", ca_name); // CA cert
    let key = format!("{}.key", name);       // server private/public key
    let cnf = "data/server.cnf";             // CSR field values
    let ext = "data/v3.ext";                 // certificate extensions
    let csr = format!("{}.csr", name);       // certificate signing request
    let crt = format!("{}.crt", name);       // certificate
    if !Path::new(&ca_pem).exists() {
        generate_ca(ca_name)?
    }
    openssl(&[
        "genpkey",           // Generate a private key.
        "-out", &key,        // output file name
        "-algorithm", "RSA", // algorithm to generate the key
        "-aes-256-cbc",      // algorithm to encrypt the key
    ])?;
    openssl(&[
        "req", "-new",   // Request a new certificate; i.e., generate a CSR.
        "-out", &csr,    // output file name
        "-key", &key,    // key to be signed
        "-config", &cnf, // field values, like Common Name
    ])?;
    openssl(&[
        "x509", "-req",    // Request an X.509 certificate.
        "-in", &csr,       // input (CSR) file name
        "-out", &crt,      // output (cert) file name
        "-CA", &ca_pem,    // CA that should sign the cert
        "-CAkey", &ca_key, // key to sign the cert with
        "-CAcreateserial", // bump the serial number in a .srl file
        "-days", "7300",   // actual time before the cert expires
        "-extfile", &ext,  // optional extension data
    ])
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
