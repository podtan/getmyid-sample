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
  IDM URL:    https://auth.example.com/oauth2/billing
  Config URL: https://config.example.com/api/billing
  Token:      tok_billing_xxx
  Process:    getmyid-sample
  PID:        12345
  UID:        1001
  GID:        1001
```

**Output Fields:**
| Field | Description |
|-------|-------------|
| `Identity` | Application-level identity name from whoami rules |
| `IDM URL` | Identity Management (Kanidm) OAuth2/OIDC endpoint |
| `Config URL` | Application configuration endpoint URL |
| `Token` | Pre-shared authentication token for API access |
| `Process` | Process name of the client application |
| `PID` | Process ID |
| `UID` | User ID |
| `GID` | Group ID |

### JSON Format

```json
{
  "identity": "BILLING_PROD",
  "idm_url": "https://auth.example.com/oauth2/billing",
  "config_url": "https://config.example.com/api/billing",
  "token": "tok_billing_xxx",
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
