# template-rs-bin

[![crates.io](https://img.shields.io/crates/v/template-rs-bin.svg)](https://crates.io/cratestemplate-rs-bin)
[![docs.rs](https://docs.rs/pzzld-server/badge.svg)](https://docs.rs/template-rs-bin)

***

_**Warning: use with caution as the application is currently in the early stages of development and is unfit for production environments.**_

Welcome to `template-rs-bin`!

## Features

- Authentication
  - OAuth2
  - OpenID Connect
- Hosting
  - Static files
  - WebAssembly
- Storage
  - Database
    - NoSQL
    - Object Storage
    - SQL
  - Virtual Filesystem

## Getting Started

### Usage

#### Installation

```bash
cargo install template-rs-bin
```

or if you have `cargo-binstall` installed

```bash
cargo binstall template-rs-bin
```

#### Running the server

```bash
rs-server serve --port 8080
```

```none
Usage: pzzld [OPTIONS] [COMMAND]

Commands:
  build  
  serve  
  help   Print this message or the help of the given subcommand(s)

Options:
  -C, --config <CONFIG>  [default: Puzzled.toml]
  -r, --release          
  -u, --update           
  -v, --verbose          
  -h, --help             Print help
  -V, --version          Print version
```

### Building from the source

Start by cloning the repository

```bash
git clone https://github.com/FL03/template-rs-bin.git
cd template-rs-bin
```

#### _Build the application_

```bash
cargo build --all-features -r -v --workspace
```

#### _Run the application_

```bash
cargo run --all-features -r -v --workspace
```

#### _Test out the application_

```bash
cargo test --all-features -r -v --workspace
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.
