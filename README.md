# getmyid-sample

Sample application demonstrating usage of the [getmyid](../getmyid) Rust client library.

## Usage

```bash
# Build
cargo build --release

# Get identity using default socket path
./target/release/getmyid-sample

# Specify custom socket path
./target/release/getmyid-sample -s /tmp/whoami.sock

# Use async client
./target/release/getmyid-sample --async

# Output as JSON
./target/release/getmyid-sample -f json

# Custom timeout (10 seconds)
./target/release/getmyid-sample -t 10
```

## Options

```
Options:
  -s, --socket <SOCKET>    Path to the whoami Unix socket [default: /var/run/whoami.sock]
  -t, --timeout <TIMEOUT>  Connection timeout in seconds [default: 5]
      --async              Use async client instead of sync
  -f, --format <FORMAT>    Output format (text or json) [default: text]
  -h, --help               Print help
```

## Example Output

### Text Format

```
Identity retrieved successfully!

  Identity:   BILLING_PROD
  Kanidm URL: https://auth.example.com/oauth2/billing
  Process:    getmyid-sample
  PID:        12345
  UID:        1001
  GID:        1001
```

### JSON Format

```json
{
  "identity": "BILLING_PROD",
  "kanidm_url": "https://auth.example.com/oauth2/billing",
  "pid": 12345,
  "uid": 1001,
  "gid": 1001,
  "process": "getmyid-sample"
}
```

## Testing with whoami daemon

```bash
# Start whoami daemon (in whoami directory)
./bin/whoamid -f -d -s /tmp/test.sock -r config/rules.conf.example

# Run sample (in another terminal)
cargo run -- -s /tmp/test.sock
```
