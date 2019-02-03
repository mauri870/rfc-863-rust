# RFC 863 implemented in Rust

[RFC 863](https://tools.ietf.org/html/rfc863) defines a discard protocol, that's it, a TCP/UDP service that listens on port 9 and throws away any data it receives.

## Compilation

```bash
cargo build --release
```

## Usage

### TCP

```bash
./target/release/tcp-discard
```

In another terminal:

```bash
echo "Hello" | nc -q1 127.0.0.1 9
```

### UDP

```bash
./target/release/udp-discard
```

In another terminal:

```bash
echo "Hello" | nc -u 127.0.0.1 9
```
