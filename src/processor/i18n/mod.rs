#[cfg(feature = "br")]
mod messages_br;
#[cfg(feature = "br")]
use messages_br as msgs;

#[cfg(feature = "en")]
mod messages;
#[cfg(feature = "en")]
use messages as msgs;

#[inline(always)]
pub fn command_msgs(code: usize) -> &'static str {
    msgs::COMMAND_MSGS[code]
}

#[inline(always)]
pub fn error_msgs() -> &'static [&'static str] {
    msgs::ERROR_MSGS
}

#[inline(always)]
pub fn msg_help() -> &'static str {
    msgs::MSG_HELP
}
