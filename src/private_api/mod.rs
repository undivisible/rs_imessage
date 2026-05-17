//! IMCore bridge via [openclaw/imsg](https://github.com/openclaw/imsg) dylib injection (MIT).
//!
//! Enable with `feature = "private-api"`. Requires SIP disabled and a built
//! `imsg-bridge-helper.dylib` (see `scripts/build-bridge-from-imsg.sh`).

mod bridge;
mod ipc;
mod launcher;
mod paths;
mod protocol;
mod sip;

pub use bridge::BridgeClient;
pub use ipc::BridgeResponse;
pub use launcher::Launcher;
pub use protocol::BridgeAction;
pub use sip::{current_sip_status, require_sip_disabled, SipStatus};
