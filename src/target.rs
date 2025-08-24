pub struct Target {
    host: String,
    port: i16,
    transport: String,
    request: String,
    successful_response: String,
}

impl Target {
    pub fn new(
        host: &str,
        port: i16,
        transport: String,
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
