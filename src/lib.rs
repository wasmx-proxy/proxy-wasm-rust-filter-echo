#[macro_use]
extern crate serde_derive;

use chrono::{DateTime, Utc};
use http::StatusCode;
use log::info;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use serde::Serialize;

use std::time::Duration;

#[derive(Serialize)]
struct Headers {
    #[serde(with = "tuple_vec_map")]
    headers: Vec<(String, String)>,
}

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Info);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(HttpHeadersRoot) });
}

struct HttpHeadersRoot;
impl Context for HttpHeadersRoot {}
impl RootContext for HttpHeadersRoot {
    fn on_vm_start(&mut self, _: usize) -> bool {
        self.set_tick_period(Duration::from_secs(10));
        true
    }

    fn on_tick(&mut self) {
        let now: DateTime<Utc> = self.get_current_time().into();
        info!("ticking at: {}", now);
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    fn create_http_context(&self, context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(HttpHeaders {
            _context_id: context_id,
        }))
    }
}

struct HttpHeaders {
    _context_id: u32,
}

impl HttpHeaders {
    fn send_error_response(&mut self) {
        self.send_http_response(
            StatusCode::INTERNAL_SERVER_ERROR.as_u16() as u32,
            vec![],
            None,
        )
    }

    fn send_json_response<T>(&mut self, status: StatusCode, body: Option<T>)
    where
        T: Serialize,
    {
        if let Some(b) = body {
            match serde_json::to_string(&b) {
                Ok(s) => self.send_http_response(
                    status.as_u16() as u32,
                    vec![("Content-Type", "application/json")],
                    Some(s.as_bytes()),
                ),
                Err(_) => self.send_error_response(),
            }
        } else {
            self.send_http_response(status.as_u16() as u32, vec![], None)
        }
    }
}

impl Context for HttpHeaders {}
impl HttpContext for HttpHeaders {
    fn on_http_request_headers(&mut self, _: usize) -> Action {
        match self.get_http_request_header(":path") {
            Some(p) => {
                /*
                 * METHOD /<endpoint>/<path>
                 */
                let (endpoint, path);
                {
                    let mut segments: Vec<&str> = p.split('/').collect::<Vec<&str>>().split_off(1);
                    endpoint = segments.remove(0);
                    path = segments.join("/");
                }

                match &endpoint as &str {
                    "headers" => {
                        let headers = self.get_http_request_headers();
                        self.send_json_response(StatusCode::OK, Some(Headers { headers }));
                    }
                    "user-agent" => {
                        #[derive(Serialize)]
                        struct UA {
                            #[serde(rename = "user-agent")]
                            inner: Option<String>,
                        }

                        let ua = self.get_http_request_header("user-agent");
                        self.send_json_response(StatusCode::OK, Some(UA { inner: ua }));
                    }
                    "status" => {
                        match StatusCode::from_bytes(path.as_bytes())
                            .map_err(|_| StatusCode::BAD_REQUEST)
                        {
                            Ok(status) => self.send_json_response::<i32>(status, None),
                            Err(status) => self.send_json_response::<i32>(status, None),
                        }
                    }
                    _ => self.send_json_response::<String>(StatusCode::NOT_FOUND, None),
                }

                Action::Continue
            }
            _ => Action::Continue,
        }
    }
}
