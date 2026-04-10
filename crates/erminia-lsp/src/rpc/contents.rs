use std::io::{BufRead, Read};

pub trait Extract {
    fn extract(opts: &mut StateOpts) -> Result<(), ()>;
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

    pub fn strip_next_line_suffix(opts: &mut StateOpts) {
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
    fn extract(opts: &mut StateOpts) -> Result<(), ()> {
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
pub(crate) struct Body {
    pub body_string: String,
}

impl Body {
    pub fn new(body_string: String) -> Self {
        Body { body_string }
    }
}

impl Extract for Body {
    fn extract(opts: &mut StateOpts) -> Result<(), ()> {
        let _ = opts.logger.log("In extract Body");
        if let Some(header) = &opts.header {
            let mut buf = vec![0u8; header.bytes];
            if let Ok(_n) = opts.buffer.read_exact(&mut buf) {
                let _ = opts.logger.log(&format!("Managed to read body: {:?}", buf));

                let mut body_string = String::new();

                for byte in buf.bytes() {
                    body_string.push(byte.unwrap() as char);
                }

                let _ = opts.logger.log(&format!(
                    "Now this should be the actual content: {}",
                    body_string
                ));

                opts.body = Some(Body::new(body_string));
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

pub(crate) struct StateOpts<'a> {
    buffer: std::io::BufReader<std::io::StdinLock<'static>>,
    pub logger: crate::logger::logger::Logger<'a>,
    header: Option<Header>,
    body: Option<Body>,
    initialized: bool,
}

impl<'a> StateOpts<'a> {
    pub fn new(file: &'a std::fs::File) -> Self {
        let buffer = std::io::BufReader::new(std::io::stdin().lock());
        let logger = crate::logger::logger::Logger::new(file);
        StateOpts {
            buffer,
            logger,
            header: None,
            body: None,
            initialized: false,
        }
    }

    pub fn get_body(&mut self) -> &Option<Body> {
        &self.body
    }

    pub fn set_initialized(&mut self, flag: bool) {
        self.initialized = flag;
    }
}
