## Setting up dev environment
Except for openssl, it is pretty straightforward.<br> 
- Install [Rust](https://www.rust-lang.org/tools/install)
- `git clone https://github.com/danielvschoor/aiarena-client-gui.git`
- `cargo run`

# Installing openssl Windows
 - Install vcpkg by following the quickstart guide at https://github.com/Microsoft/vcpkg
- Install the 64-bit version of openssl: vcpkg.exe install openssl:x64-windows
  <br> Dynamic linking
- `set VCPKGRS_DYNAMIC=1`<br>
Static linking 
- `set OPENSSL_STATIC=1` and `set RUSTFLAGS=-Ctarget-feature=+crt-static`

# Installing openssl Linux
Untested. Follow instructions [here](https://github.com/sfackler/rust-openssl/blob/b8fb29db5c246175a096260eacca38180cd77dd0/README.md)
Static linking is still done by `set OPENSSL_STATIC=1`, although I'm unsure if `set RUSTFLAGS=-Ctarget-feature=+crt-static` still needs to be done

If your openssl library is not in path, set `OPENSSL_DIR` to the openssl installation directory. See [here](https://docs.rs/openssl/0.10.33/openssl/#manual) for more information. 

If you receive a certificate error, download the certificate "cacert.pem" from [here](https://curl.se/docs/caextract.html), place it in a directory, 
and set the following environment variable to that directory:
``set SSL_CERT_FILE=C:\OpenSSL-Win64\certs\cacert.pem``
