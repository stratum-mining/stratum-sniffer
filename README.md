# Stratum Sniffer

A sniffer tool for monitoring and analyzing Stratum V1 and Stratum V2 mining protocol traffic. This tool acts as a proxy between mining clients and pools, allowing you to inspect and analyze the communication flow.

### Configuration

- `server_addr`: The address of the upstream Stratum server to forward traffic to
- `listen_addr`: The local address and port to bind the sniffer proxy to
- `sv2`: Boolean flag to enable Stratum V2 (`true`) or Stratum V1 (`false`) protocol handling

## Usage

Run the sniffer with a configuration file:

```bash
cargo run -- --config config.toml
```

Make sure that when connecting some Sv2 application to `listen_addr`, you should use pubkey `9auqWEzQDVyd2oe1JVGFLMLHZtCo2FFqZwtKA5gd9xbuEu7PH72`.

## License

This software is licensed under Apache 2.0 or MIT, at your option.

## Related Projects

- [Stratum V2 Reference Implementation](https://github.com/stratum-mining/stratum)
- [Stratum V2 Specification](https://stratumprotocol.org/)
