//! Grammar boundaries for failed option fields during temperature discovery.
//!
//! No values are evaluated or supplied here. The retained field error prevents
//! publication; the fresh temperature pass must run the ordinary grammar again.
use super::*;

pub(super) fn consume_failed_field(
    stream: &mut TokenStream,
    start: usize,
    package: Option<&str>,
    key: &str,
    params: &ParamContext,
) {
    stream.restore_checkpoint(start);
    if !consume_scalar(stream) {
        return;
    }
    match (package, key) {
        (Some("TIMEINT"), "BREAKPOINTS") | (Some("OUTPUT"), "OUTPUTTIMEPOINTS") => {
            while stream.consume(&TokenKind::Comma) && consume_scalar(stream) {}
        }
        (Some("OUTPUT"), "INITIAL_INTERVAL" | "INITIALINTERVAL") => loop {
            skip_commas(stream);
            if !restart_interval_schedule_starts(stream, params) || !consume_scalar(stream) {
                break;
            }
        },
        _ => {}
    }
}

fn consume_scalar(stream: &mut TokenStream) -> bool {
    skip_commas(stream);
    // A missing value must not consume the following assignment's name.
    if matches!(stream.peek().kind, TokenKind::Ident(_))
        && matches!(stream.peek_n(1).kind, TokenKind::Equals)
    {
        return false;
    }
    if matches!(stream.peek().kind, TokenKind::Plus | TokenKind::Minus) {
        stream.advance();
    }
    if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
        return false;
    }
    stream.advance();
    true
}
