macro_rules! unwrap_logger {
    ($log:expr) => ($log.unwrap_or($crate::slog::Logger::root($crate::slog::DrainExt::fuse($crate::slog_stdlog::StdLog), o!())))
}

macro_rules! serialize_via_display {
    ($t: ty) => {
        impl ::slog::ser::Serialize for $t {
            fn serialize(&self, _record: &::slog::Record, key: &'static str, serializer: &mut ::slog::ser::Serializer) -> ::std::result::Result<(), ::slog::ser::Error> {
                serializer.emit_str(key, &format!("{}", self))
            }
        }
    }
}

macro_rules! serialize_via_debug {
    ($t: ty) => {
        impl ::slog::ser::Serialize for $t {
            fn serialize(&self, _record: &::slog::Record, key: &'static str, serializer: &mut ::slog::ser::Serializer) -> ::std::result::Result<(), ::slog::ser::Error> {
                serializer.emit_str(key, &format!("{:?}", self))
            }
        }
    }
}

macro_rules! addr_str {
    ($a:expr) => (format!("{:?}", $a.map(|x| format!("${:04X}", x))))
}

macro_rules! throw_log {
    ($l: expr, $err:expr) => {
        err_log!($l, $err, "error thrown");
        return Err(::std::convert::From::from($err));
    }
}

macro_rules! log_err {
    ($l:expr, $err:expr, $($k:expr => $v:expr),+; $($args:tt)+) => (
        slog_error!($l, "error" => $err, $( $k => $v ),+; $args);
    );
    ($l:expr, $err:expr, $msg:expr) => (
        slog_error!($l, $msg; "error" => $err);
    );
    ($l:expr, $err:expr, $msg:expr; $($k:expr => $v:expr),+) => (
        slog_error!($l, $msg; "error" => $err, $( $k => $v ),+);
    );
    ($l:expr, $err:expr, $msg:expr; $($k:expr => $v:expr),+,) => (
        slog_error!($l, $msg; "error" => $err, $( $k => $v ),+,);
    )
}

macro_rules! try_log {
    ($expr: expr, $l:expr) => (
        match $expr {
            Ok(val) => val,
            Err(err) => {
                log_err!($l, err, "error rethrown");
                return Err(::std::convert::From::from(err));
            }
        }
    );
    ($expr: expr, $l:expr; $($k:expr => $v:expr),+) => (
        match $expr {
            Ok(val) => val,
            Err(err) => {
                log_err!($l, err, "error rethrown"; $($k => $v),+);
                return Err(::std::convert::From::from(err));
            }
        }
    )
}