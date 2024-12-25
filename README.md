# CLI2REST

A simple REST API server that converts CLI commands to HTTP endpoints.

## Features

- Execute CLI commands via HTTP POST requests
- JSON request and response format
- Configurable port number
- Docker support

## Usage

By default, the application listens on `[::]` (both IPv4 and IPv6 on all interfaces).

### Running locally

```bash
# Run with default settings ([::]::8000 - listens on both IPv4 and IPv6)
cargo run

# IPv4 examples
cargo run -- --bind 127.0.0.1     # IPv4 localhost only
cargo run -- --bind 0.0.0.0       # IPv4 all interfaces

# IPv6 examples
cargo run -- --bind [::1]         # IPv6 localhost only
cargo run -- --bind [::]          # IPv6 all interfaces (includes IPv4)

# Custom port examples
cargo run -- --port 3000
cargo run -- --bind [::1] --port 3000
```

### Using Docker

```bash
# Run with default settings (listens on both IPv4 and IPv6)
docker run -p 8000:8000 xlmnxp/cli2rest

# IPv4 specific
docker run -p 127.0.0.1:8000:8000 xlmnxp/cli2rest --bind 0.0.0.0

# IPv6 specific
docker run -p [::1]:8000:8000 xlmnxp/cli2rest --bind [::]

# Custom settings
docker run -p 3000:3000 xlmnxp/cli2rest --bind 0.0.0.0 --port 3000
```

### API Example

```bash
# IPv4
curl -X POST http://127.0.0.1:8000/execute \
  -H "Content-Type: application/json" \
  -d '{"command":"ls","args":["-l"]}'

# IPv6
curl -X POST http://[::1]:8000/execute \
  -H "Content-Type: application/json" \
  -d '{"command":"ls","args":["-l"]}'
```

## Response Format

```json
{
  "stdout": "command output",
  "stderr": "error output if any",
  "status": 0
}
```

## Security Notice

This tool executes system commands directly. Use with caution in production environments.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
