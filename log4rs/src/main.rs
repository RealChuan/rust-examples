use log::{debug, error, info, trace, warn};
use log4rs;

fn main() {
    log4rs::init_file("log/src/log.yml", Default::default()).unwrap();

    let text = "Hello, world!";
    debug!("{}", text);
    info!("{}", text);
    warn!("{}", text);
    error!("{}", text);
    trace!("{}", text);
}
