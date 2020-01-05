use futures::{future, join};

extern crate libfrps_rs;

use hyper::{Body, Method, Request, Uri, Version};
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
    use futures::{Future, Stream};
    use tokio::runtime::Runtime;

    #[test]
    fn it_works() {
        let _ = pretty_env_logger::try_init();

        let mut rt = Runtime::new().expect("new rt");
        let client = hyper::Client::new();

        let mut serializer = Serializer::new();
        let mut buffer: Vec<u8> = vec![];
        buffer.reserve(256);

        // call
        let mut _written = 0;
        let cnt = serializer.write_call(&mut buffer[..], "server.stat");
        assert_eq!(cnt.is_ok(), true);
        _written += cnt.unwrap();

        // Int
        serializer.reset();
        let val = Value::Int(1224);
        let cnt = serializer.write_value(&mut buffer[_written..], &val);
        assert_eq!(cnt.is_ok(), true);
        _written += cnt.unwrap();

        buffer.resize(_written, 0);

        let req = Request::builder()
            .method("POST")
            .version(Version::HTTP_11)
            .uri("http://localhost:30001/RPC2")
            .header("User-Agent", "frps-client-rs")
            .header("Transfer-Encoding", "chunked")
            .header("Accept", "application/x-frps, application/x-frpc")
            .header("Content-Type", "application/x-frpc")
            .body(Body::from(buffer))
            .expect("request builder");

        let res1 = client.request(req);
        let res = rt.block_on(res1);
        assert!(res.is_ok());
    }
}
