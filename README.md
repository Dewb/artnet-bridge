# artnet-bridge

Translate Art-Net packets from popular lighting control software into KiNET packets destined for multiple Philips Color Kinetics power/data supplies.

Building from source requires the Rust toolchain. https://www.rust-lang.org/tools/install

## Build and run with cargo

```bash
$ cargo run -- -a 192.168.1.1 -k 10.0.0.1 -m 1:10.32.152.122 -m 1:0:10.32.152.122:1
 2020-06-07T18:53:33.773Z INFO  artnet_bridge > Listening for Art-Net packets on 192.168.1.1
 2020-06-07T18:53:33.773Z INFO  artnet_bridge > Transmitting KiNET on 10.0.0.1
 2020-06-07T18:53:33.774Z INFO  artnet_bridge > Mapping Art-Net to the following KiNET destinations:
 2020-06-07T18:53:33.774Z INFO  artnet_bridge > KinetDestination { artnet_network: 0, artnet_subnet: 0, artnet_universe: 1, kinet_address: "10.32.152.122", kinet_socket_addr: V4(10.32.152.122:6038), kinet_universe: 0 }
 2020-06-07T18:53:33.774Z INFO  artnet_bridge > KinetDestination { artnet_network: 0, artnet_subnet: 1, artnet_universe: 0, kinet_address: "10.32.152.122", kinet_socket_addr: V4(10.32.152.122:6038), kinet_universe: 1 }
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
    -a <artnet-receive-ip>           The IPv4 network address where Art-Net packets will be received
    -k <kinet-send-ip>               The IPv4 network address that KiNET packets should be sent from
    -m, --mapping <map-string>...    Map a single Art-Net universe data to a KiNET destination. Map-string should
                                     consist of an Art-Net source universe and a KiNET destination IPv4 address, with
                                     optional output universe, all separated by colons. Art-Net source universes can be
                                     specified with just a universe, or a network, subnet, and universe.
                                     1:0:15:10.0.0.1:3 would send listen for Art-Net output for network 1, subnet 0,
                                     universe 15, and send it to the KiNET PDS at 10.0.0.1 for output on
                                     universe/channel 3. If any network/subnet/universe values are not provided, they
                                     will be assumed to be 0, so the following are all valid: -m 10.0.0.4 -m
                                     3:192.168.10.100 -m 1:4:13:10.0.1.4 -m 192.168.0.15:10 -m 1:1:10.0.0.2:2
    -f, --file <config-file>         Path to a file containing configuration options. All command-line options can be
                                     specified in the config file; command-line options will override options in file
                                     where there's a conflict
```

## Configuration files

Options can be specified in a configuration file in addition to the command line. If an argument is provided both in 
the configuration file and on the command line, the command line value will be used, except for PDS addresses, which
will be combined from both sources.

*examples/config.json*

```json
{
    "artnet_receive_ip": "192.168.1.1",
    "kinet_send_ip": "10.0.0.1",
    "mappings": [
        "0:10.32.152.122:0",
        "1:10.32.152.123:0"
    ]
}
```

```bash
$ cargo run -- -f examples/config.json
 2020-06-07T19:03:22.651Z INFO  artnet_bridge > Listening for Art-Net packets on 192.168.1.1
 2020-06-07T19:03:22.656Z INFO  artnet_bridge > Transmitting KiNET on 10.0.0.1
 2020-06-07T19:03:22.673Z INFO  artnet_bridge > Mapping Art-Net to the following KiNET destinations:
 2020-06-07T19:03:22.673Z INFO  artnet_bridge > KinetDestination { artnet_network: 0, artnet_subnet: 0, artnet_universe: 0, kinet_address: "10.32.152.123", kinet_socket_addr: V4(10.32.152.123:6038), kinet_universe: 0 }
 2020-06-07T19:03:22.691Z INFO  artnet_bridge > KinetDestination { artnet_network: 0, artnet_subnet: 0, artnet_universe: 1, kinet_address: "10.32.152.122", kinet_socket_addr: V4(10.32.152.122:6038), kinet_universe: 0 }
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
