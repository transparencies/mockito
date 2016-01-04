#[cfg(feature = "use_hyper")]
extern crate hyper;
#[cfg(feature = "use_hyper")]
extern crate url as servo_url;

pub mod server;
pub mod url;
#[cfg(feature = "use_hyper")]
pub mod mockable_hyper;
pub mod mockable_tcp_stream;

pub mod mock_builder;

pub type Url<'a> = url::Url<'a>;
pub type MockServer = server::MockServer;
pub type MockBuilder = mock_builder::MockBuilder;

pub fn start() {
    MockServer::new(vec!());
}

pub fn mock(request_line: &str) -> MockBuilder {
    MockBuilder::new(request_line)
}

#[cfg(test)]
#[cfg(feature = "mock_hyper")]
mod mock_hyper_tests {
    use hyper::Client;
    use hyper::header::Connection;
    use server;
    use url::Url;
    use std::io::Read;

    #[test]
    fn test_proxying() {
        super::start();

        let client = Client::new();
        let mut res = client.get(Url("http://www.example.com"))
            .header(Connection::close())
            .send()
            .unwrap();

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();

        assert_eq!(body, "Hello world");
    }
}

#[cfg(test)]
#[cfg(feature = "mock_tcp_stream")]
mod mock_tcp_stream_tests {
    use MockBuilder;
    use super::mock;

    #[test]
    fn test_mock() {
        let mock = mock("GET /");

        assert_eq!("GET /", mock.request_line);
    }
}
