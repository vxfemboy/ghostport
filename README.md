[![Rust](https://github.com/vxfemboy/ghostport/actions/workflows/rust.yml/badge.svg)](https://github.com/vxfemboy/ghostport/actions/workflows/rust.yml)
# Ghostport

Ghostport is a sophisticated port spoofing tool designed to confuse and mislead port scanners. It's a Rust implementation inspired by the concept of portspoof, offering enhanced performance and flexibility.

![Ghostport Demo](/contrib/ghostport_demo.gif)

## Features

- **Dynamic Port Emulation**: Responds to port scans with a variety of convincing service signatures.
- **Customizable Signatures**: Easily add or modify service signatures through a simple text file.
- **High Performance**: Built with Rust and Tokio for efficient, asynchronous handling of connections.
- **Flexible Logging**: Offers debug, verbose, and quiet logging modes for different use cases.
- **Easy to Use**: Simple command-line interface with sensible defaults.

## Installation

```bash
git clone https://github.com/vxfemboy/ghostport.git
cd ghostport
cargo build --release
```

## Usage

Basic usage:

```bash
./target/release/ghostport -s signatures.txt
```

or you can run with cargo 
```bash
git clone https://github.com/vxfemboy/ghostport.git
cd ghostport 
cargo run -- -s signatures.txt
```

This will start Ghostport on the default address (127.0.0.1:8888) using the signatures from `signatures.txt`.

### Command-line Options

- `-s, --signatures <FILE>`: Path to the signatures file (default: "signatures")
- `-l, --listen <ADDRESS>`: Address to listen on (default: "127.0.0.1:8888")
- `-d, --debug`: Enable debug logging
- `-v, --verbose`: Enable verbose logging
- `-q, --quiet`: Enable quiet logging
- `-V, --version`: Print version information

### Examples

Run with custom address and verbose logging:

```bash
./target/release/ghostport -s signatures.txt -l 0.0.0.0:8888 -v
```

Run with debug logging:

```bash
./target/release/ghostport -s signatures.txt -l 0.0.0.0:8888 -d
```

## Signature File Format

The signature file should contain one signature per line. Signatures can be raw text or regex patterns. For example:

```
HTTP/1.1 200 OK\r\nServer: Apache/2.4.41 (Unix)\r\n
SSH-2.0-OpenSSH_8.2p1 Ubuntu-4ubuntu0.1
220 (vsFTPd 3.0.3)
```
for more examples, see the [signatures](signatures.txt) file.

## Routing Traffic to Ghostport

To redirect all incoming TCP traffic to Ghostport, you can use iptables. This will allow Ghostport to respond to connections on any port, effectively spoofing all services:

```bash

INTERFACE="eth0" # change to your network interface

iptables -t nat -A PREROUTING -i $INTERFACE -p tcp -m tcp -m multiport --dports 1:65535 -j REDIRECT --to-ports 8888

```

This command will redirect all TCP traffic on ports 1-65535 to port 8888, where Ghostport is listening. Make sure to replace "eth0" with your actual network interface.

> [!NOTE]
> This requires root privileges and will affect all incoming TCP connections on the specified interface. Use with caution, especially on production systems.

To remove this rule:

```bash

iptables -t nat -D PREROUTING -i $INTERFACE -p tcp -m tcp -m multiport --dports 1:65535 -j REDIRECT --to-ports 8888

```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the GNU License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by the original [portspoof project](https://github.com/drk1wi/portspoof)
- Built with Rust and Tokio
