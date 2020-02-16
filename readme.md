## Dev-Setup

`export DATABASE_URL="postgres://postgres:password@localhost:5432/test"` - Set the DATABASE_URL 
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` - Installs the dependency for cargo and all the compilers for rust
`cargo run` - Run the web server

## Directories

`src\` - Source for the rust server
`static\` - Location for the static assets, like index.html