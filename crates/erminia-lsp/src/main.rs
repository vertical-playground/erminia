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

    let mut opts = rpc::StateOpts::new(&file);

    loop {
        if let Err(()) = rpc::Header::extract(&mut opts) {
            break;
        };
        if let Err(()) = rpc::Body::extract(&mut opts) {
            break;
        };

        if let Ok(response) = rpc::Request::handle::<rpc::InitializeParams>(&mut opts) {
            response.send()?;
        }
    }

    Ok(())
}
