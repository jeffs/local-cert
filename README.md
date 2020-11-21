# Local Cert Generator

This repo defines a [program](src/bin/generate.rs) that wraps OpenSSL commands
to generate certificates for use by https:// servers on localhost.  If you give
the program one argument (your preferred CA file base name), it generates a
self-signed root certificate that you can install to become a locally trusted
Certificate Authority.  If you give it a second argument, it generates a
private key (in a file having that base name), and signs it using your local
root.  The two-argument version implicitly creates the root cert automatically.

This approach has been [best practice][].  This project was inspired by Daksh Shah's
[local-cert-generator][], and by [this article][].  Caveat emptor:  I am not a
security expert.

Sample creation of a root CA and a signed cert for one server:

```sh
(3978) ~/git/local-cert ───────────────────────────────────────────────────────────────────────────────
$ ls
Cargo.lock  Cargo.toml  data  README.md  src  target

(3979) ~/git/local-cert ───────────────────────────────────────────────────────────────────────────────
$ mkdir out

(3980) ~/git/local-cert ───────────────────────────────────────────────────────────────────────────────
$ cargo run -- out/{myca,myserver}
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/generate out/myca out/myserver`
..............................+++++
....................................................................................................................+++++
Enter PEM pass phrase:
Verifying - Enter PEM pass phrase:
Enter pass phrase for out/myca.key:
You are about to be asked to enter information that will be incorporated
into your certificate request.
What you are about to enter is what is called a Distinguished Name or a DN.
There are quite a few fields but you can leave some blank
For some fields there will be a default value,
If you enter '.', the field will be left blank.
-----
Country Name (2 letter code) [AU]:US
State or Province Name (full name) [Some-State]:NY
Locality Name (eg, city) []:NYC
Organization Name (eg, company) [Internet Widgits Pty Ltd]:My Certificate Authority
Organizational Unit Name (eg, section) []:
Common Name (e.g. server FQDN or YOUR name) []:My Certificate Authority
Email Address []:myemail@example.com
.+++++
.....................................+++++
Enter PEM pass phrase:
Verifying - Enter PEM pass phrase:
Enter pass phrase for out/myserver.key:
Signature ok
subject=C = US, ST = NY, L = NYC, O = Dummy Organization, OU = Local Server, emailAddress = hello@example.com, CN = localhost
Getting CA Private Key
Enter pass phrase for out/myca.key:

(3981) ~/git/local-cert ───────────────────────────────────────────────────────────────────────────────
$ ls out
myca.key  myca.pem  myca.srl  myserver.crt  myserver.csr  myserver.key
```

Creation of a signed cert for a second server:

```sh
$ cargo run -- out/{myca,server2}
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/generate out/myca out/server2`
..............+++++
......................................................+++++
Enter PEM pass phrase:
Verifying - Enter PEM pass phrase:
Enter pass phrase for out/server2.key:
Signature ok
subject=C = US, ST = NY, L = NYC, O = Dummy Organization, OU = Local Server, emailAddress = hello@example.com, CN = localhost
Getting CA Private Key
Enter pass phrase for out/myca.key:

(4460) ~/git/local-cert ───────────────────────────────────────────────────────────────────────────────
$ ls out/server2*
out/server2.crt  out/server2.csr  out/server2.key
```

[best practice]: https://deliciousbrains.com/ssl-certificate-authority-for-local-https-development/
[local-cert-generator]: https://github.com/dakshshah96/local-cert-generator
[this article]: https://www.freecodecamp.org/news/how-to-get-https-working-on-your-local-development-environment-in-5-minutes-7af615770eec/
