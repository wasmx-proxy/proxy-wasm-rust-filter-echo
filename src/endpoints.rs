use crate::*;
use http::StatusCode;

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

pub(crate) fn send_request_headers(ctx: &mut HttpEcho) {
    #[derive(Serialize)]
    struct Headers {
        #[serde(with = "tuple_vec_map")]
        headers: Vec<(String, String)>,
    }

    let headers = ctx.get_http_request_headers();
    ctx.send_json_response(StatusCode::OK, Some(Headers { headers }));
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
