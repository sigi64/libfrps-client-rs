extern crate libfrps_rs;

use hyper::{Body, Method, Request, Uri};


struct Client {
    uri: String
}

impl Client {
    pub fn new(uri: &str) -> Self {
        Client {}
    }

    pub fn call(method: &str) {
        
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

