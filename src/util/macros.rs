
#[macro_export]
macro_rules! msg_reply {
    ($msg:ident, $ctx:ident, $($s:expr),+) => {
        $msg.reply_ping($ctx.http.clone(), format!($($s),+)).await?
    };
}