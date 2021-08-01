use crate::*;
use http::{Method, StatusCode};
use std::str;

pub(crate) type HttpEchoHandler = fn(&mut HttpEcho) -> ();

pub(crate) fn send_request_anything(ctx: &mut HttpEcho) {
    #[derive(Serialize)]
    struct Anything {
        #[serde(with = "tuple_vec_map")]
        headers: Vec<(String, String)>,
        method: String,
    }

    let headers = ctx.get_http_request_headers();
    ctx.send_json_response(
        StatusCode::OK,
        Some(Anything {
            headers,
            method: ctx.get_http_request_header(":method").unwrap(),
        }),
    );
}

pub(crate) fn echo_status(ctx: &mut HttpEcho) {
    let status = ctx.match_params.get("code");
    if status.is_none() {
        ctx.send_error_response(StatusCode::BAD_REQUEST);
        return;
    }

    match StatusCode::from_bytes(status.unwrap().as_bytes()).map_err(|_| StatusCode::BAD_REQUEST) {
        Ok(status) => ctx.send_json_response::<i32>(status, None),
        Err(status) => ctx.send_json_response::<i32>(status, None),
    }
}

pub(crate) fn send_request_headers(ctx: &mut HttpEcho) {
    #[derive(Serialize)]
    struct Headers {
        #[serde(with = "tuple_vec_map")]
        headers: Vec<(String, String)>,
    }

    let headers = ctx.get_http_request_headers();
    ctx.send_json_response(StatusCode::OK, Some(Headers { headers }));
}

pub(crate) fn send_request_ip(ctx: &mut HttpEcho) {
    #[derive(Serialize)]
    struct RequestIp {
        origin: String,
    }

    let origin = {
        let mut address = String::from_utf8(ctx.get_property(vec!["source", "address"]).unwrap())
            .expect("Invalid UTF-8 sequence: {}");
        let port_offset = address.find(':').unwrap_or_else(|| address.len());
        address.replace_range(port_offset.., "");
        address
    };

    ctx.send_json_response(StatusCode::OK, Some(RequestIp { origin }));
}

pub(crate) fn send_request_user_agent(ctx: &mut HttpEcho) {
    #[derive(Serialize)]
    struct UA {
        #[serde(rename = "user-agent")]
        inner: Option<String>,
    }

    let ua = ctx.get_http_request_header("user-agent");
    ctx.send_json_response(StatusCode::OK, Some(UA { inner: ua }));
}

pub(crate) fn send_response_headers(ctx: &mut HttpEcho) {
    #[derive(Serialize)]
    struct UA {
        #[serde(with = "tuple_vec_map")]
        headers: Vec<(String, String)>,
    }

    match Method::from_bytes(ctx.get_http_request_header(":method").unwrap().as_bytes()) {
        Ok(method) => match method {
            Method::GET => {
                for (_, (key, value)) in ctx.url.as_ref().unwrap().query_pairs().enumerate() {
                    debug!("inserting response header: \"{}: {}\"", key, value);
                    ctx.set_http_response_header(&key, Some(&value));
                }
            }
            _ => {
                ctx.send_error_response(StatusCode::METHOD_NOT_ALLOWED);
                return;
            }
        },
        Err(_) => {
            ctx.send_error_response(StatusCode::BAD_REQUEST);
            return;
        }
    }

    let headers = ctx.get_http_response_headers();
    ctx.send_json_response(StatusCode::OK, Some(UA { headers }));
}

pub(crate) fn send_decoded_base64(ctx: &mut HttpEcho) {
    let value = ctx.match_params.get("value");
    if value.is_none() {
        ctx.send_error_response(StatusCode::BAD_REQUEST);
        return;
    }

    let decoded = base64::decode(value.unwrap()).expect("failed decoding value: {}");
    ctx.send_response(
        StatusCode::OK,
        Some(vec![("Content-Type", "text/plain")]),
        Some(str::from_utf8(&decoded).expect("Invalid UTF-8 sequence: {}")),
    );
}
