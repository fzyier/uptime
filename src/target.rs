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
}
