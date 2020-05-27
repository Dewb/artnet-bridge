# artnet-bridge

## Build and run with cargo

```bash
$ cargo run -- -l 192.168.1.1 -k 10.0.0.1 -p 10.32.152.122 -p 10.32.152.123
 2020-05-27T06:24:00.318Z INFO  artnet_bridge > Listening for Art-Net packets on 192.168.1.173
 2020-05-27T06:24:00.320Z INFO  artnet_bridge > Transmitting KiNET on 10.0.0.1
 2020-05-27T06:24:00.345Z INFO  artnet_bridge > Mapping universes to the following addresses:
 2020-05-27T06:24:00.346Z INFO  artnet_bridge > ["10.32.152.122", "10.32.152.123"]
```

## Full options

```bash
artnet-bridge 0.1.0
Map Art-Net universes to KiNET PDS endpoints

USAGE:
    artnet-bridge.exe [FLAGS] --listen <artnet-address> --kinet <kinet-address> --pds <pds-addresses>...

FLAGS:
    -h, --help
            Prints help information

    -q, --quiet
            Pass many times for less log output

    -V, --version
            Prints version information

    -v, --verbose
            Pass many times for more log output

            By default, it'll only report errors. Passing `-v` one time also prints warnings, `-vv` enables info
            logging, `-vvv` debug, and `-vvvv` trace.

OPTIONS:
    -l, --listen <artnet-address>
            The network address to listen on

    -k, --kinet <kinet-address>
            The network address to send KiNET from

    -p, --pds <pds-addresses>...
            The KiNET PDS addresses to send to
```

## Project Initial Goals

* Provide a way to integrate CK lighting hardware with popular software control environments
* Create a virtual ArtNet destination that bridges to KiNET protocol devices
* Run on PC/Mac/Linux desktops and Raspberry Pis
* Maximize performance & reliability

## Potential Future Goals

* Support OpenPixelControl as an output protocol alongside KiNET
* Support sACN as an input protocol alongside ArtNet
* Support KiNET discovery and readdressing protocols, capture all known KiNET implementation details
* Eventually support building for embedded platforms without heap allocation (e.g. compile with `#![no_std]`).