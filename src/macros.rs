macro_rules! unwrap_logger {
    ($log:expr) => ($log.unwrap_or($crate::slog::Logger::root($crate::slog::DrainExt::fuse($crate::slog_stdlog::StdLog), o!())))
}