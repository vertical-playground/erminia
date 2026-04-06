use std::io::BufRead;

mod logger;

#[derive(Debug)]
struct Header {
    #[allow(unused)]
    bytes: u32,
}

struct ExtractOpts<'a> {
    buffer: std::io::BufReader<std::io::StdinLock<'static>>,
    logger: logger::logger::Logger<'a>,
}

impl<'a> ExtractOpts<'a> {
    pub fn new(file: &'a std::fs::File) -> Self {
        let buffer = std::io::BufReader::new(std::io::stdin().lock());
        let logger = logger::logger::Logger::new(file);
        ExtractOpts { buffer, logger }
    }
}

fn extract_header(opts: &mut ExtractOpts) -> Result<Header, ()> {
    let mut buf = String::new();

    if opts.buffer.read_line(&mut buf).is_ok() {
        let _ = opts.logger.warn("Using unwrap might be an issue");
        let header = buf
            .strip_prefix("Content-Length: ")
            .unwrap()
            .strip_suffix("\r\n")
            .unwrap();

        if let Ok(bytes) = str::parse::<u32>(header) {
            let _ = opts
                .logger
                .log(&format!("Extracting header with bytes: {}", bytes));
            Ok(Header { bytes })
        } else {
            let _ = opts
                .logger
                .error("Failed to parse bytes from header extractor");
            Err(())
        }
    } else {
        Err(())
    }
}

fn main() -> std::io::Result<()> {
    let path = std::env::var("HOME").unwrap() + "/Coding/Personal/erminia/debug_log.txt";
    let file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&path)
        .unwrap();
    let mut opts = ExtractOpts::new(&file);

    if let Ok(_header) = extract_header(&mut opts) {}

    Ok(())
}
