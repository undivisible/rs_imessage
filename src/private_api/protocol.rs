//! Wire protocol aligned with [openclaw/imsg](https://github.com/openclaw/imsg) v2 bridge (MIT).

pub const PROTOCOL_VERSION: i32 = 2;
pub const RPC_DIR: &str = ".imsg-rpc";
pub const INBOX: &str = "in";
pub const OUTBOX: &str = "out";
pub const READY_LOCK: &str = ".imsg-bridge-ready";
pub const EVENTS_LOG: &str = ".imsg-events.jsonl";
pub const DEFAULT_DYLIB_NAME: &str = "imsg-bridge-helper.dylib";
pub const DEFAULT_TIMEOUT_MS: u64 = 10_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeAction {
    Ping,
    Status,
    ListChats,
    StartTyping,
    StopTyping,
    SendMessage,
    SendReaction,
    EditMessage,
    UnsendMessage,
    MarkChatRead,
    CreateChat,
}

impl BridgeAction {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ping => "ping",
            Self::Status => "status",
            Self::ListChats => "list_chats",
            Self::StartTyping => "start-typing",
            Self::StopTyping => "stop-typing",
            Self::SendMessage => "send-message",
            Self::SendReaction => "send-reaction",
            Self::EditMessage => "edit-message",
            Self::UnsendMessage => "unsend-message",
            Self::MarkChatRead => "mark-chat-read",
            Self::CreateChat => "create-chat",
        }
    }
}
