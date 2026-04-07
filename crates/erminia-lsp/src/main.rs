mod logger;
mod rpc;

use rpc::Extract;

struct Request {}

struct InitializeResponse {}
enum Response {
    Initialize(InitializeResponse),
}

impl Response {
    pub fn send(&self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Request {
    pub fn handle(_opts: &mut rpc::ExtractOpts) -> Result<Response, ()> {
        Ok(Response::Initialize(InitializeResponse {}))
    }
}

fn main() -> std::io::Result<()> {
    let path = std::env::var("HOME").unwrap() + "/Coding/Personal/erminia/debug_log.txt";
    let file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&path)
        .unwrap();

    let mut opts = rpc::ExtractOpts::new(&file);

    loop {
        if let Err(()) = rpc::Header::extract(&mut opts) {
            break;
        };
        if let Err(()) = rpc::Body::extract(&mut opts) {
            break;
        };

        if let Ok(response) = Request::handle(&mut opts) {
            response.send()?;
        }
    }

    Ok(())
}
