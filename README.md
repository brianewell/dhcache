# DHCache ![crates.io version](https://img.shields.io/crates/v/dhcache) ![crates.io license](https://img.shields.io/crates/l/dhcache) ![circleci build](https://img.shields.io/circleci/build/github/brianewell/dhcache) ![crates.io downloads](https://img.shields.io/crates/d/dhcache)

A micro Diffie-Hellman parameter generation web-service.

_Notice: this readme is currently a CLI spec, and not indicitive of this project's current capabilities._

## Synopsis

```
dhcache [-?hv] [-c|--config file] [-e|--error file] [-l|--listen address] [-p|--port number] [-w|--workers number] [--version] [bitsize:count]...
```

## Description

Dhcache is a compact self-contained web-service that maintains a cache of independently generated Diffie-Hellman parameters at various bit lengths.

The options are as follows:

- `-?`, `-h` Display usage information.
- `-c file`, `--config file` Use an alternative configuration file.
- `-e file`, `--error file` Use an alternative error log file. The (default) special value of *stderr* indicates that the standard error output should be used.
- `-l address`, `--listen address` Listen on the specified address.
- `-p number`, `--port number` Listen on the specified port.
- `-w number`, `--workers number` Use the specified number of worker threads to generate Diffie-Hellman parameters. `0` automatically uses the maximum number of available cores.
- `-v`, `--version` Display the dhcache version.

Additionally, one or more Diffie-Hellman parameter sizes and counts can be specified on the command-line to indicate what size of parameters and how many should be generated and cached. Supported bitsizes include: 1024, 2048, 3072, 4096, 7680, 8192, 15360 and 16384.

The following URLs are exposed through the web-service:

- `/` provides a JSON encoded response listing available Diffie-Hellman parameters, including their bit sizes, available URLs, number currently cached, and intended number to cache.
- `/:bitsize:` returns a set of Diffie-Hellman parameters of the requested size. Possible response codes include:
    - An HTTP 200 (Ok) response code with the Diffie-Hellman parameters in the response body if there is a parameter of the requested size available.
    - An HTTP 503 (Service Unavailable) response code with a JSON encoded response if the cache for that configured Diffie-Hellman parameter size is currently unavailable.
    - An HTTP 404 (Not Found) response code with a JSON encoded response if dhcache isn't configured to provide the requested Diffie-Hellman parameter size.
