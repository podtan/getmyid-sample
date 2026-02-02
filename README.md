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

# Send instance ID for dynamic configuration routing
./target/release/getmyid-sample -i 42

# Include current timestamp in runner context
./target/release/getmyid-sample --with-timestamp

# Combine instance ID and timestamp
./target/release/getmyid-sample -i 42 --with-timestamp
```

## Options

```
Options:
  -s, --socket <SOCKET>      Path to the whoami Unix socket [default: /var/run/whoami.sock]
  -t, --timeout <TIMEOUT>    Connection timeout in seconds [default: 5]
      --async                Use async client instead of sync
  -f, --format <FORMAT>      Output format (text or json) [default: text]
  -i, --instance-id <ID>     Instance ID to send in runner context (for dynamic config routing)
      --with-timestamp       Include current timestamp in runner context
  -h, --help                 Print help
```

## Example Output

### Text Format

```
Identity retrieved successfully!

  Identity:   BILLING_PROD
  IDM URL:    https://auth.example.com/oauth2/billing
  Config URL: https://config.example.com/api/billing
  Token:      tok_billing_xxx

  Runner:
    Hostname:    worker-node-03
    Process:     getmyid-sample
    PID:         12345
    UID:         1001
    GID:         1001
    Instance ID: 42
    Timestamp:   1738512000
```

**Top-level Identity Fields:**
| Field | Description |
|-------|-------------|
| `Identity` | Application-level identity name from whoami rules |
| `IDM URL` | Identity Management (Kanidm) OAuth2/OIDC endpoint |
| `Config URL` | Application configuration endpoint URL |
| `Token` | Pre-shared authentication token for API access |

**Runner Object Fields (for passing to config server):**
| Field | Source | Description |
|-------|--------|-------------|
| `Hostname` | server | Machine hostname |
| `Process` | server | Process name |
| `PID` | server | Process ID (kernel-verified) |
| `UID` | server | User ID (kernel-verified) |
| `GID` | server | Group ID (kernel-verified) |
| `Instance ID` | client | Client-provided instance identifier (optional) |
| `Timestamp` | client | Client-provided timestamp (optional) |

### JSON Format

```json
{
  "identity": "BILLING_PROD",
  "idm_url": "https://auth.example.com/oauth2/billing",
  "config_url": "https://config.example.com/api/billing",
  "token": "tok_billing_xxx",
  "runner": {
    "identity": "BILLING_PROD",
    "hostname": "worker-node-03",
    "process": "getmyid-sample",
    "pid": 12345,
    "uid": 1001,
    "gid": 1001,
    "instance_id": 42,
    "timestamp": 1738512000
  }
}
```

## Testing with whoami daemon

```bash
# Start whoami daemon (in whoami directory)
./bin/whoamid -f -d -s /tmp/test.sock -r config/rules.conf.example

# Run sample (in another terminal)
cargo run -- -s /tmp/test.sock

# Run with instance ID for dynamic config routing
cargo run -- -s /tmp/test.sock -i 42 --with-timestamp
```
