use async_h1::client;
use async_std::net::TcpStream;
use http_types::{Error, Method, Request, StatusCode, Url};

use libfrps_rs::{Serializer, Tokenizer, Value, ValueTreeBuilder};

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

    pub fn call(method: &str) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_std::io::Cursor;
    use http_types::Body;

    #[async_std::test]
    async fn it_works() {
        let _ = pretty_env_logger::try_init();

        let mut serializer = Serializer::new();
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
    }
}
