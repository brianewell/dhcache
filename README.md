# DHCache

## Overview

**DHCache** is a micro Diffie-Hellman parameter generation web service. It provides a lightweight and efficient way to concurrently pre-generate and maintain an ephemeral cache of unique DH parameters in various sizes for secure key exchange operations.

## Features

- **Concurrent Pre-Generation** of Diffie-Hellman parameters
- **Ephemeral Caching** to securely minimize delays
- **RESTful API** to retrieve and evict cached parameters
- **Configurable Key Sizes and Parameter Counts** at runtime
- **Lightweight and Fast** implementation in Rust

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)
- [Cargo](https://doc.rust-lang.org/cargo/) (Rust package manager)

### Build and Run

```sh
# Clone the repository
git clone https://github.com/brianewell/dhcache.git
cd dhcache

# Optionally run tests
cargo test

# Build the project
cargo build --release

# Run the service
./target/release/dhcache
```

## Configuration

All configuration is performed through command-line parameters, available by calling the program with the `--help` flag:

```sh
dhcache --help
A micro Diffie-Hellman parameter generation web service

Usage: dhcache [OPTIONS] <count>:<bits>...

Arguments:
  <count>:<bits>...

Options:
  -b, --bind <address>     [default: 0.0.0.0]
  -p, --port <port>        [default: 4000]
  -w, --workers <workers>  [default: 1]
  -h, --help               Print help
  -V, --version            Print version
```

The listening address, port, and worker count have configurable defaults. The cache behavior is defined by a collection of `<count>:<bits>` pairs, where `<count>` represents the number of DH parameters of size `<bits>` to cache.

The following command starts DHCache on its default listening address, port 80, and uses two worker threads to maintain a cache of eight 1024-bit DH parameters, four 2048-bit DH parameters, and two 4096-bit DH parameters:

```sh
dhcache -w 2 8:1024 4:2048 2:4096
```

## Usage

All interactions with DHCache occur via the configured web service.

### List Available DH Parameters

```http
GET /
GET /status
```

Returns the overall status of the cache.

- **Parameters:** _None_
- **Response:**
  - **HTTP 200**: Returns a JSON-encoded response listing DH parameters, including their bit sizes, relative paths, available parameters, and the maximum number of parameters cached.

**Example Response:**

```json
{
  "1024": {
    "path": "/1024",
    "available": 6,
    "max": 8
  },
  "2048": {
    "path": "/2048",
    "available": 4,
    "max": 4
  },
  "4096": {
    "path": "/4096",
    "available": 2,
    "max": 2
  }
}
```

### Retrieve DH Parameters

```http
GET /<bits>
```

Returns a set of DH parameters.

- **Parameters:**
  - `<bits>` (integer): Specifies the key size of DH parameters to retrieve from the cache.
- **Response:**
  - **HTTP 200** (OK): Returns a JSON-encoded response containing the requested DH parameters.
  - **HTTP 404** (Not Found): The requested bit size is not configured in the cache.
  - **HTTP 503** (Service Unavailable): The requested bit size is supported, but no parameters are currently available.

### Evict Cached Parameters

```http
DELETE /<bits>
```

Clears and regenerates all DH parameters for a given size.

- **Parameters:**
  - `<bits>` (integer): Specifies the key size of DH parameters to clear from the cache.
- **Response:**
  - **HTTP 204**: The request was successfully processed.
  - **HTTP 404**: The specified bit size is not configured for caching.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request to improve DHCache.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Contact

For questions or issues, feel free to open an issue on [GitHub](https://github.com/brianewell/dhcache).
