#![feature(try_blocks)]
#[macro_use]
extern crate serde_derive;
mod endpoints;
mod util;

use endpoints::*;
use http::StatusCode;
use log::*;
use matchit::*;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use std::collections::HashMap;
use url::Url;

type HttpEchoRouter = Node<HttpEchoHandler>;

struct HttpEchoRoot {}

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Debug);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(HttpEchoRoot {}) });
}

impl Context for HttpEchoRoot {}
impl RootContext for HttpEchoRoot {
    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    fn create_http_context(&self, context_id: u32) -> Option<Box<dyn HttpContext>> {
        let res: Result<HttpEchoRouter, InsertError> = try {
            let mut root = HttpEchoRouter::new();

            // explicitly type the 1st element
            let endpoint: fn(&mut HttpEcho) = endpoints::send_request_anything;

            // anything
            root.insert("/anything", endpoint)?;

            // status codes
            root.insert("/status/:code", endpoints::echo_status)?;

            // request inspection
            root.insert("/headers", endpoints::send_request_headers)?;
            root.insert("/ip", endpoints::send_request_ip)?;
            root.insert("/user-agent", endpoints::send_request_user_agent)?;

            // response inspection
            root.insert("/response-headers", endpoints::send_response_headers)?;

            // dynamic data
            root.insert("/base64/:value", endpoints::send_decoded_base64)?;

            root
        };

        if res.is_err() {
            return None;
        }

        Some(Box::new(HttpEcho {
            context_id,
            router: res.unwrap(),
            url: None,
            match_params: HashMap::new(),
        }))
    }
}

struct HttpEcho {
    context_id: u32,
    router: HttpEchoRouter,
    url: Option<Url>,
    match_params: HashMap<String, String>,
}

impl Context for HttpEcho {}
impl HttpContext for HttpEcho {
    fn on_http_request_headers(&mut self, _: usize) -> Action {
        let url = format!(
            "{}://{}{}",
            self.get_http_request_header(":scheme").unwrap(),
            self.get_http_request_header(":authority").unwrap(),
            self.get_http_request_header(":path").unwrap()
        );

        debug!("#{} request url: {}", self.context_id, url);

        self.url = Some(Url::parse(url.as_str()).expect("failed parsing request url"));

        if let Ok(matched) = self.router.at(self.url.as_ref().unwrap().path()) {
            for (key, value) in matched.params.iter() {
                // copy out of context
                self.match_params.insert(key.to_owned(), value.to_owned());
            }

            let handler = *matched.value;
            handler(self)
        } else {
            self.send_json_response::<String>(StatusCode::NOT_FOUND, None);
        }

        Action::Continue
    }
}
