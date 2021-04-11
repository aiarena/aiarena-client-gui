## Setting up dev environment
Except for openssl, it is pretty straightforward.<br> 
- Install [Rust](https://www.rust-lang.org/tools/install)
- `git clone https://github.com/danielvschoor/aiarena-client-gui.git`
- `cargo run`

If you receive a certificate error, download the certificate "cacert.pem" from [here](https://curl.se/docs/caextract.html), place it in a directory, 
and set the following environment variable to that directory:
``set SSL_CERT_FILE=C:\OpenSSL-Win64\certs\cacert.pem``


## Available Endpoints
### /index (get)
Frontend.

Renders index.html

### /settings (get)
Frontend.

Renders settings.html

### /api (get)
Swagger API documentation.

### /get_maps (get)
API (documented by Swagger)

Returns available maps on host PC. Requires sc2_directory_location to be set

### /get_bots (get)
API (documented by Swagger)

Returns available maps on host PC. Requires bots_directory_location to be set

### /get_settings (get)
API (documented by Swagger)

Returns settings saved in `settings.json`

### /get_arena_bots (get)
API (documented by Swagger)

Returns all publicly downloaded bots on `aiarena.net`. Requires API Token to be set.

### /run_games (post)
API (documented by Swagger)

Runs games with selected bots and maps.

### /handle_data (post)
API (documented by Swagger)

Updates `settings.json` with new values.

### /get_results (get)
API (documented by Swagger)

Returns results from `results.json`.

### /clear_results (post)
API (documented by Swagger)

Clears `results.json` file.



