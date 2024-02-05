
#[macro_export]
macro_rules! msg_reply {
    ($msg:ident, $ctx:ident, $($s:expr),+) => {
        if let Err(e) = $msg.reply_ping($ctx.http.clone(), format!($($s),+)).await {
            log::error!("Failed to send message: {e}");
        }
    };
}

#[macro_export]
macro_rules! run_cmd {
    ($cfg:expr, $ctx:expr, $msg:expr, $args:expr, $p:tt) => {
        match $p::Args::try_parse_from($args) {
            Ok(args) => $p::exec($cfg, $ctx, $msg, args).await,
            Err(e) => {
                let _ = $msg.reply_ping($ctx.http.clone(), format!("```sh\n{}\n```", e)).await;
                Ok(())
            }
        }
    };
}

#[macro_export]
macro_rules! add_cmd {
    ($list:expr, $p:tt) => {
        $list.push($p::INFO);
    };
}

#[macro_export]
macro_rules! check_res {
    ($e:expr) => {
        match $e {
            Ok(r) => r,
            Err(e) => {
                log::debug!("{e}");
                return;
            }
        }
    };
}