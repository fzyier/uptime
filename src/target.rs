use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Instant;

pub trait Probe {
    fn probe(&self) -> Result<f64, String>;
}

pub enum Transport {
    TCP,
    UDP,
}

pub struct Target {
    host: String,
    port: i16,
    transport: Transport,
    request: String,
    successful_response: String,
}

impl Target {
    pub fn new(
        host: &str,
        port: i16,
        transport: Transport,
        request: &str,
        successful_response: &str,
    ) -> Self {
        Self {
            host: host.to_string(),
            port,
            transport,
            request: request.to_string(),
            successful_response: successful_response.to_string(),
        }
    }

    fn probe_tcp(&self) -> Result<f64, String> {
        let endpoint = format!("{}:{}", self.host, self.port.to_string());
        let now = Instant::now();
        let mut stream = TcpStream::connect(endpoint)
            .map_err(|err| format!("Connection to {} failed with the error {}", self.host, err))?;

        stream.write_all(self.request.as_bytes()).map_err(|err| {
            format!(
                "Failed to send the request '{}' to {} with the error {}",
                self.request, self.host, err
            )
        })?;

        let mut buf: String = String::new();
        stream.read_to_string(&mut buf).map_err(|err| {
            format!(
                "Failed to read the {} response with the error {}",
                self.host, err
            )
        })?;

        let elapsed_time = now.elapsed();

        Ok(elapsed_time.as_secs_f64())
    }
}

impl Probe for Target {
    fn probe(&self) -> Result<f64, String> {
        match self.transport {
            Transport::TCP => self.probe_tcp(),
            Transport::UDP => Err("UDP is not implemented yet".to_string()),
        }
    }
}
