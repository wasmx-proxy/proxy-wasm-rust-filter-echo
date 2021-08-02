use crate::*;
use http::StatusCode;
use serde::Serialize;

impl HttpEcho {
    pub(crate) fn send_response(
        &mut self,
        status: StatusCode,
        headers: Option<Vec<(&str, &str)>>,
        body: Option<&str>,
    ) {
        self.send_http_response(
            status.as_u16() as u32,
            headers.unwrap_or_default(),
            body.map(|b| b.as_bytes()),
        );
    }

    pub(crate) fn send_error_response(&mut self, status: StatusCode) {
        self.send_response(status, None, None)
    }

    pub(crate) fn send_json_response<T>(&mut self, status: StatusCode, body: Option<T>)
    where
        T: Serialize,
    {
        if let Some(b) = body {
            match serde_json::to_string(&b) {
                Ok(s) => self.send_response(
                    status,
                    Some(vec![("Content-Type", "application/json")]),
                    Some(&s),
                ),
                Err(_) => self.send_error_response(StatusCode::INTERNAL_SERVER_ERROR),
            }
        } else {
            self.send_response(status, None, None)
        }
    }
}
