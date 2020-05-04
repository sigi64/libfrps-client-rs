use async_h1::client;
use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::task;
use http_types::headers::{HeaderName, HeaderValue};
use http_types::{Body, Error, Method, Request, Response, StatusCode, Url};
use libfrps_rs::{Serializer, Tokenizer, Value, ValueTreeBuilder};
use std::str::FromStr;

struct Client<'a, R, W>
where
    R: Fn(&[u8]),
    W: Fn() -> &'a [u8],
{
    uri: &'a str,
    method_name: &'a str,
    params: Option<Vec<&'a Value>>,
    read_cb: Option<R>,
    write_cb: Option<W>,
}

impl<'a, R, W> Client<'a, R, W>
where
    R: Fn(&[u8]),
    W: Fn() -> &'a [u8],
{
    pub fn new(uri: &'a str) -> Self {
        Self {
            uri: uri,
            method_name: "",
            params: None,
            read_cb: None,
            write_cb: None,
        }
    }

    pub fn method(&mut self, method: &'a str) {
        self.method_name = method;
    }

    pub fn param(&mut self, method: &'a str) {
        self.method_name = method;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_std::io::Cursor;
    use http_types::Body;

    fn client() {
        let mut client = Client::new("127.0.0.1:30001");
        client.call("server.stat")
        client.param(method)
    }

    #[async_std::test]
    async fn it_works() {
        let _ = pretty_env_logger::try_init();

        let mut serializer = Serializer::new();

        // uninitialized buffer (unsound solution!!!)
        // let mut buffer = Vec::with_capacity(1024);
        // unsafe { buffer.set_len(1024); }

        let mut buffer = Vec::new();
        buffer.resize(1024, 0u8);

        // call
        let mut written = 0;
        let cnt = serializer.write_call(&mut buffer[..], "server.stat");
        assert_eq!(cnt.is_ok(), true);
        written += cnt.unwrap();

        println!("written {} bytes", written);

        // Int
        serializer.reset();
        let val = Value::Int(1224);
        let cnt = serializer.write_value(&mut buffer[written..], &val);
        assert_eq!(cnt.is_ok(), true);
        written += cnt.unwrap();

        println!("written {} bytes", written);
        println!("written {} bytes", buffer.len());

        let stream = TcpStream::connect("127.0.0.1:30001").await.unwrap();
        let peer_addr = stream.peer_addr().unwrap();
        println!("connecting to {}", peer_addr);

        let url = Url::parse(&format!("http://{}/RPC2", peer_addr)).unwrap();
        let mut req = Request::new(Method::Post, url);
        req.insert_header("User-Agent", "frps-client-rs");
        //req.insert_header("Transfer-Encoding", "chunked");
        req.insert_header("Accept", "application/x-frps, application/x-frpc");
        req.insert_header("Content-Type", "application/x-frpc");

        let buff = Cursor::new(buffer);
        req.set_body(Body::from_reader(buff, Some(written)));

        let res = client::connect(stream.clone(), req).await.unwrap();
        assert_eq!(res.status(), StatusCode::Ok);

        let content_length = HeaderName::from_str("content-length").unwrap();
        let content_type = HeaderName::from_str("content-type").unwrap();
        let user_agent = HeaderName::from_str("user-agent").unwrap();

        if let Some(val) = res.header(&user_agent) {
            println!("User-agent: {}", val[0])
        }

        // Get type of protocol
        let mut is_frps = false;
        if let Some(val) = res.header(&content_type) {
            let val = &val[0];
            println!("Content-type: {}", val);

            match val.as_str() {
                "application/x-frpc" => is_frps = false,
                "application/x-frps" => is_frps = true,
                _ => println!("Unsupported content/type?: {}", val.as_str()),
            }
        }

        if let Some(val) = res.header(&content_length) {
            println!("Content-lenght: {}", val[0])
        }

        println!("response body: {}", res.body_string().await.unwrap());
    }
}
