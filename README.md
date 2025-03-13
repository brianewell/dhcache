# DHCache

## Overview

**DHCache** is a micro Diffie-Hellman parameter generation web service. It provides a lightweight and efficient way to concurrently pre-generate and maintain an ephemeral cache of unique DH parameters in various sizes for secure key exchange operations.

## Features

- **Concurrent Pre-Generation** of Diffie-Hellman parameters
- **Ephemeral Caching** to securely minimize delays
- **RESTful API** to retrieve and/or evict cached parameters
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

All configuration is performed through command-line parameters, available by calling the program with the `--help` parameter:

```sh
dhcache --help
A micro Diffie-Hellman parameter generation web-service

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

Listening address, port and worker numbers have overridable defaults, the cache behavior is defined by a collection of `count` and `bit` pairs, with `count` representing the number of DH parameters of size `size` to cache.

The following command-line parameters would start DHCache on its default listening address and port 80, and use two worker threads to maintain a cache of eight 1024-bit DH parameters, four 2048-bit DH parameters, and two 4096-bit DH parameters:

```sh
dhcache -w 2 8:1024 4:2048 2:4096
```

## Usage

All normal interactions occurs via the configured web service.

#### List Available DH Parameters

```
GET /
GET /status
```

The service can be queried to determine the overall status of the cache.

- **Parameters:** __none__
- **Response:**
  - **HTTP 200**: Returns a JSON-encoded response listing DH parameters, including their bit sizes, relative paths, available parameters and maximum numbers of parameters that will be cached.

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

#### Retrieve DH Parameters

```
GET /:bits
```

The service can be queried to return a set of DH parameters.

- **Parameters:**
  - `:bits` (integer): Specifies the key size of DH parameters to return from the cache
- **Response:**
  - `HTTP 200` (OK): Returns a _ encoded response containing the requested DH parameters.
  - `HTTP 404` (Not Found): Returns an HTTP Not Found error if the cache isn't configured to serve the requested bit size.
  - `HTTP 503` (Service Unavailable): Returns an HTTP Service Unavailable error if the service is configured to serve this DH parameter size, but the cache has no parameters available for that size.

#### Evict Cached Parameters

```
DELETE /:bits
```

The service can be queried to completely clear and regenerate all DH parameters for a given parameter size.

- **Parameters:**
  - `:bits` (integer): Specifies the key size of DH parameters to clear from the cache.
- **Response:**
  - `HTTP 204`: The service sucessfully processed the request.
  - `HTTP 404`: The service could not clear the cache since its not configured to cache that size of DH parameters.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request to improve DHCache.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Contact

For questions or issues, feel free to open an issue on [GitHub](https://github.com/brianewell/dhcache).
