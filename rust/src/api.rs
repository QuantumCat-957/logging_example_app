use crate::logger;

use anyhow::Result;
use flutter_rust_bridge::StreamSink;

use log::{trace, debug, warn, info, error};

pub struct LogEntry {
    pub time_millis: i64,
    pub level: i32,
    pub tag: String,
    pub user_id: String,
    pub user: String,
    pub msg: String,
}

// Dummy function to fix Rust compiler complaints...
// See https://github.com/fzyzcjy/flutter_rust_bridge/issues/398
// Workaround:
// 1. Save Rust Code
// 2. Execute flutter_rust_bridge_codegen command
// 3. Make any change to Rust code (e.g. add blank) and save again
//    -> next compile is ok
#[allow(dead_code, unused_variables)]
pub fn dummy(a: LogEntry) {}

pub fn rust_set_up() -> String {
    logger::init_logger();
    "Logger was initialized".into()
}

pub fn create_log_stream(s: StreamSink<LogEntry>) -> Result<()> {
    logger::SendToDartLogger::set_stream_sink(s);
    Ok(())
}

pub fn publish_message(message: String) {

    trace!("TRACE   --------------- {}", message );
    debug!("DEBUG   --------------- {}", message );
    warn!("WARNING --------------- {}", message );
    info!("INFO    --------------- {}", message );
    error!("ERROR   --------------- {}", message );

    debug!("my_domain@@@my_tag@@@uuid@@@name@@@This logger message comes from RUST:\n{}", message );

}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
