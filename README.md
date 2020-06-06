# artnet-bridge

Translate Art-Net packets from popular lighting control software into KiNET packets destined for multiple Philips Color Kinetics power/data supplies.

Building from source requires the Rust toolchain. https://www.rust-lang.org/tools/install

## Build and run with cargo

```bash
$ cargo run -- -l 192.168.1.1 -k 10.0.0.1 -p 10.32.152.122 -p 10.32.152.123
 2020-05-27T06:24:00.318Z INFO  artnet_bridge > Listening for Art-Net packets on 192.168.1.1
 2020-05-27T06:24:00.320Z INFO  artnet_bridge > Transmitting KiNET on 10.0.0.1
 2020-05-27T06:24:00.345Z INFO  artnet_bridge > Mapping universes to the following addresses:
 2020-05-27T06:24:00.346Z INFO  artnet_bridge > ["10.32.152.122", "10.32.152.123"]
```

## Full options

```bash
artnet-bridge 0.1.0
Map Art-Net universes to KiNET PDS endpoints

USAGE:
    artnet-bridge.exe [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -q, --quiet      Make output less verbose. Add -q to only show warnings and errors, -qq to only show errors, and
                     -qqq to silence output completely
    -V, --version    Prints version information
    -v, --verbose    Make output more verbose. Add -v for debugging info, add -vv for even more detailed message tracing

OPTIONS:
    -l, --listen <artnet-address>    The network address to listen on
    -f, --file <config-file>         Path to a file containing configuration options. All command-line options can be
                                     specified in the config file; command-line options will override options in file
                                     where there's a conflict
    -k, --kinet <kinet-address>      The network address to send KiNET from
    -p, --pds <pds-addresses>...     The KiNET PDS addresses to send to
```

## Configuration files

Options can be specified in a configuration file in addition to the command line. If an argument is provided both in 
the configuration file and on the command line, the command line value will be used, except for PDS addresses, which
will be combined from both sources.

```bash
$ cargo run -- -f examples/config.json
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target\debug\artnet-bridge.exe -f examples/config.json`
 2020-06-06T23:05:58.119Z INFO  artnet_bridge > Listening for Art-Net packets on 192.168.1.1
 2020-06-06T23:05:58.126Z INFO  artnet_bridge > Transmitting KiNET on 10.0.0.1
 2020-06-06T23:05:58.126Z INFO  artnet_bridge > Mapping universes to the following addresses:
 2020-06-06T23:05:58.127Z INFO  artnet_bridge > ["10.32.152.122", "10.32.152.123"]
 ```

## Project Initial Goals

* Provide a way to integrate CK lighting hardware with popular software control environments
* Create a virtual Art-Net destination that bridges to KiNET protocol devices
* Run on PC/Mac/Linux desktops and Raspberry Pis
* Maximize performance & reliability

## Potential Future Goals

* Support OpenPixelControl as an output protocol alongside KiNET
* Support sACN as an input protocol alongside Art-Net
* Support KiNET discovery and readdressing protocols, capture all known KiNET implementation details
* Eventually support building for embedded platforms without heap allocation (e.g. compile with `#![no_std]`).
* Implement a web-based live configuration panel
