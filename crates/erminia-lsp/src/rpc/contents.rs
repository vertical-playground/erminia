use std::io::{BufRead, Read};

pub trait Extract {
    fn extract(opts: &mut ExtractOpts) -> Result<(), ()>;
}

#[derive(Debug)]
pub(crate) struct Header {
    #[allow(unused)]
    bytes: usize,
}

impl Header {
    pub fn new(bytes: usize) -> Self {
        Header { bytes }
    }

    pub fn strip_next_line_suffix(opts: &mut ExtractOpts) {
        let mut sep = String::new();
        let _ = opts.buffer.read_line(&mut sep);
    }

    pub fn strip_header_bytes(buf: &mut str) -> &str {
        buf.strip_prefix("Content-Length: ")
            .unwrap()
            .strip_suffix("\r\n")
            .unwrap()
    }
}

impl Extract for Header {
    fn extract(opts: &mut ExtractOpts) -> Result<(), ()> {
        let mut buf = String::new();
        if opts.buffer.read_line(&mut buf).is_ok() {
            let _ = opts
                .logger
                .log(&format!("Managed to read buffer contents: {:?}", &buf));
            let header_bytes = Header::strip_header_bytes(&mut buf);
            if let Ok(bytes) = str::parse::<usize>(header_bytes) {
                let _ = opts
                    .logger
                    .log(&format!("Managed to parse buffer contents: {:?}", bytes));
                let _ = opts.logger.log("Saving header struct.");
                Header::strip_next_line_suffix(opts);
                opts.header = Some(Header::new(bytes));
                Ok(())
            } else {
                let _ = opts.logger.error("Failed to parse buffer contents");
                Err(())
            }
        } else {
            let _ = opts.logger.error("Failed to read buffer contents");
            Err(())
        }
    }
}

#[derive(Debug)]
pub(crate) struct Body {}

impl Body {
    pub fn new() -> Self {
        Body {}
    }
}

impl Extract for Body {
    fn extract(opts: &mut ExtractOpts) -> Result<(), ()> {
        let _ = opts.logger.log("In extract Body");
        if let Some(header) = &opts.header {
            let mut buf = vec![0u8; header.bytes];
            if let Ok(_n) = opts.buffer.read_exact(&mut buf) {
                let _ = opts.logger.log(&format!("Managed to read body: {:?}", buf));

                opts.body = Some(Body::new());
                Ok(())
            } else {
                let _ = opts.logger.warn("Failed to read body");
                Err(())
            }
        } else {
            let _ = opts.logger.error("Failed to produce a header.");
            Err(())
        }
    }
}

pub(crate) struct ExtractOpts<'a> {
    buffer: std::io::BufReader<std::io::StdinLock<'static>>,
    pub logger: crate::logger::logger::Logger<'a>,
    header: Option<Header>,
    body: Option<Body>,
}

impl<'a> ExtractOpts<'a> {
    pub fn new(file: &'a std::fs::File) -> Self {
        let buffer = std::io::BufReader::new(std::io::stdin().lock());
        let logger = crate::logger::logger::Logger::new(file);
        ExtractOpts {
            buffer,
            logger,
            header: None,
            body: None,
        }
    }

    pub fn get_header(&mut self) -> &Option<Header> {
        &mut self.header
    }
}
