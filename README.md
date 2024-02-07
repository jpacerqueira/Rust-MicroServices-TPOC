# Rust-MicroServices-TPOC
## This is a small TPOC of Rust MicroServices  Frameworks


  - 1. - Run Script 

  - -  $  bash -x install-rust.sh

 
  - 2. - Post command enable language environment


  - -  $  source "$HOME/.cargo/env"

 
  - 3. - Create a rust package


  - -  $  cargo new server_package

 
  - 4. - Copy package and main


  - -  $  cp -rf the_webserver/Cargo.toml   server_package/Cargo.toml
  - -  $  cp -rf the_webserver/src/main.rs  server_package/src/main.rs

  - 5. - Launch server_package

  - -  $  cd server_package
  - -  $  cargo run


  - 6. - Open your favourite Mozilla tab with

  - -    http://localhost:8080

#
