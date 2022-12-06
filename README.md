# template-cli-rs

[![Clippy](https://github.com/FL03/template-cli-rs/actions/workflows/clippy.yml/badge.svg)](https://github.com/FL03/template-cli-rs/actions/workflows/clippy.yml)
[![Docker](https://github.com/FL03/template-cli-rs/actions/workflows/docker.yml/badge.svg)](https://github.com/FL03/template-cli-rs/actions/workflows/docker.yml)
[![Rust](https://github.com/FL03/template-cli-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/FL03/template-cli-rs/actions/workflows/rust.yml)

***

template-cli-rs

## Installation

Make sure you have docker installed on the target system

### *Pull the image*

```bash
docker pull jo3mccain/template-cli-rs:latest
```

### *Build the image locally (optional)*

```bash
docker buildx build --tag jo3mccain/template-cli-rs:latest .
```

### *Run the image*

```bash
docker run -P jo3mccain/template-cli-rs:latest
```

## Usage

```bash
    cli -h 
```

## Contributors

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

* [Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)
* [MIT](https://choosealicense.com/licenses/mit/)
