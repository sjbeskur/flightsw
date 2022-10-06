use std::env;
use std::io::Write;
use log::LevelFilter;
use env_logger::Builder;


pub fn init() {
    let mut builder = Builder::new();
    builder.format(|buf, record| writeln!(buf, "{}", record.args())).filter(None, LevelFilter::Trace);
    builder.format(|buf, record| writeln!(buf, "{}", record.args())).filter(None, LevelFilter::Debug);
    builder.format(|buf, record| writeln!(buf, "{}", record.args())).filter(None, LevelFilter::Info);
    builder.format(|buf, record| writeln!(buf, "{}", record.args())).filter(None, LevelFilter::Warn);
    builder.format(|buf, record| writeln!(buf, "{}", record.args())).filter(None, LevelFilter::Error);

    if let Ok(rust_log) = env::var("RUST_LOG") {
      // builder.parse(&rust_log);
    }

    builder.init();

    let app_name = env!("CARGO_PKG_NAME");
    let app_ver = env!("CARGO_PKG_VERSION");
    error!("Starting... {}-{}", app_name, app_ver);

}
