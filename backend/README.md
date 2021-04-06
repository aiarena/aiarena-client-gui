## Setting up dev environment
Except for openssl, it is pretty straightforward.<br> 
- Install [Rust](https://www.rust-lang.org/tools/install)
- `git clone https://github.com/danielvschoor/aiarena-client-gui.git`
- `cargo run`

# Installing openssl
 - Install vcpkg by following the quickstart guide at https://github.com/Microsoft/vcpkg
- Install the 64-bit version of openssl: vcpkg.exe install openssl:x64-windows
- `set VCPKGRS_DYNAMIC=1`

If you receive a certificate error, download the certificate "cacert.pem" from [here](https://curl.se/docs/caextract.html), place it in a directory, 
and set the following environment variable to that directory:
``set SSL_CERT_FILE=C:\OpenSSL-Win64\certs\cacert.pem``
