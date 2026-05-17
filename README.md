# rs_imsg

Library-first iMessage toolkit for macOS — read `chat.db`, stream new messages,
and send via Messages.app (AppleScript). Built for agent runtimes; ships an
optional CLI behind the `cli` feature.

**License:** [Mozilla Public License 2.0](LICENSE)

This crate is **original work**. It is not a fork of the projects listed under
[Acknowledgements](#acknowledgements); those repositories informed design and
API shape only.

## Mac hosts the bridge

Apple only delivers iMessage on a **Mac signed into Messages**. The pattern we target:

```text
┌──────────────────────────── Mac (always on) ────────────────────────────┐
│  Messages.app  →  chat.db  →  rs_imsg bridge (HTTP on :8721)              │
└────────────────────────────────────┬────────────────────────────────────┘
                                     │  LAN / Tailscale / SSH tunnel
                                     ▼
┌──────────────────────────── Your agent host ──────────────────────────────┐
│  unthinkclaw / mono gateway  →  RS_IMSG_URL + token  →  send + SSE events │
└───────────────────────────────────────────────────────────────────────────┘
```

On the Mac:

```bash
export RS_IMSG_TOKEN="$(openssl rand -hex 24)"
cargo run --features cli -- serve --bind 0.0.0.0:8721
```

Remote callers use `Authorization: Bearer $RS_IMSG_TOKEN` (or `?token=`).

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/health` | GET | Liveness (no auth) |
| `/api/v1/ping` | GET | Auth check |
| `/api/v1/chats` | GET | List chats |
| `/api/v1/messages/history` | POST | History for one chat |
| `/api/v1/messages/send` | POST | Send text / file |
| `/api/v1/events` | GET | SSE stream of new messages |

**Same machine:** link `rs_imsg` as a library (`Client`) or run `unthinkclaw --channel imsg` — no HTTP hop.

**Hosted live:** use Linq in mono; the Mac bridge is for self-host / home lab.

Library API: enable feature `serve` and call `rs_imsg::run_bridge(ServeConfig { ... }).await`.

## Requirements

- macOS 14+
- Messages.app signed in
- **Full Disk Access** for the process using this library
- **Automation** permission for Messages when calling `send`

## Library

```toml
rs_imsg = { git = "https://github.com/undivisible/rs_imsg", default-features = false }
```

```rust
use rs_imsg::{Client, ClientConfig};
use rs_imsg::watch::WatchOptions;

let client = Client::open(ClientConfig::default())?;
let chats = client.list_chats(20)?;
let mut stream = client.watch(WatchOptions::default())?;
```

On non-macOS targets the crate compiles; macOS-only operations return
`RsImsgError::UnsupportedPlatform`.

### Modules

| Module | Role |
|--------|------|
| `client` | High-level `Client` / `ClientConfig` |
| `db` | Read-only `chat.db` access |
| `watch` | Filesystem notify + poll fallback |
| `send` | AppleScript send path |
| `rpc` | JSON-RPC 2.0 (`run_stdio`) |
| `types` | Stable JSON records |
| `private_api` (`private-api` feature) | IMCore dylib bridge (openclaw/imsg protocol) |

### Tier 1 — Private API (`private-api` feature)

Uses the MIT [openclaw/imsg](https://github.com/openclaw/imsg) `imsg-bridge-helper.dylib`
(not GPL BlueBubbles server). Requires **SIP disabled** and Messages injection.

```bash
./scripts/build-bridge-from-imsg.sh   # builds lib/imsg-bridge-helper.dylib from openclaw/imsg (MIT)
# optional: INSTALL_LIB=/opt/homebrew/lib ./scripts/build-bridge-from-imsg.sh
cargo build --features private-api
```

```rust
use rs_imsg::{Client, ClientConfig};

let client = Client::open(ClientConfig::default())?;
let bridge = client.bridge()?;
bridge.ping()?;
bridge.send_message("iMessage;-;+15551234567", "hello", None)?;
```

Set `RS_IMSG_BRIDGE_DYLIB` to override dylib search path.

**FaceTime Audio** is intentionally out of scope here — use the separate
[`rs_facetime`](https://github.com/undivisible/rs_facetime) crate.

## Optional CLI

```bash
cargo build --features cli
./target/debug/rs_imsg chats --limit 10 --json
```

Environment: `RS_IMSG_DB` overrides `~/Library/Messages/chat.db`.

## Acknowledgements

We are grateful to the authors of these projects for publishing their work and
shaping the ecosystem. **rs_imsg does not include their source code**; the table
describes conceptual debt only.

| Project | Authors / org | License (as published) | What we learned |
|---------|---------------|------------------------|-----------------|
| [openclaw/imsg](https://github.com/openclaw/imsg) | OpenClaw contributors | MIT | Agent-oriented JSON lines, JSON-RPC over stdio, `watch` with fs events + poll fallback, stderr for human logs |
| [jesec/imessage-rs](https://github.com/jesec/imessage-rs) | Jesse Chan | MIT | Modular crate layout, BlueBubbles-shaped HTTP ideas, group participants, attachment metadata |
| [photon-hq/imessage-kit](https://github.com/photon-hq/imessage-kit) | Photon | MIT | Typed chat/message models, send vs observe semantics, staged attachments |
| [BlueBubblesApp/bluebubbles-server](https://github.com/BlueBubblesApp/bluebubbles-server) | BlueBubbles | GPL-3.0 (server) | REST envelope patterns and route naming for future compatibility |
| [OpenBubbles/openbubbles-app](https://github.com/OpenBubbles/openbubbles-app) | OpenBubbles | Apache-2.0 | Product-level feature set reference (groups, FaceTime, Find My) |

Apple, iMessage, Messages, and FaceTime are trademarks of Apple Inc. This
project is not affiliated with Apple.

## Related repos

- **[rs_facetime](https://github.com/undivisible/rs_facetime)** — FaceTime Audio private API (separate dylib / feature matrix).
