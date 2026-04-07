mod logger;
mod rpc;

use rpc::Extract;

fn main() -> std::io::Result<()> {
    let path = std::env::var("HOME").unwrap() + "/Coding/Personal/erminia/debug_log.txt";
    let file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&path)
        .unwrap();

    let mut opts = rpc::ExtractOpts::new(&file);

    if let Err(()) = rpc::Header::extract(&mut opts) {
        let _ = opts.logger.error("HAHAHHAHAH");
    }

    if let Some(_header) = opts.get_header() {
        let _ = opts.logger.log("Header does in fact exist here");
    };

    if let Err(()) = rpc::Body::extract(&mut opts) {
        let _ = opts.logger.error("Hahahahh");
    }

    Ok(())
}
