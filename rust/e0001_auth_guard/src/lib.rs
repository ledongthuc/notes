#![feature(async_closure)]

use std::future::Future;

pub struct Request {
    username: String,
    password: String,
    data: String,
}

pub struct FlowRespMsg {
    response_data: String,
}

/** SYNC func **/
pub fn auth_guard<F>(request_msg: Request, flow_run: F) -> Result<FlowRespMsg, &'static str>
where
    F: Fn(Request) -> FlowRespMsg,
{
    if auth_req(&request_msg) {
        return Ok(flow_run(request_msg));
    }
    Err("Auth failed")
}

fn auth_req(request_msg: &Request) -> bool {
    request_msg.username == "admin" && request_msg.password == "password"
}

/** ASYNC func **/
pub async fn auth_guard_async<F, Fut>(request_msg: Request, flow_run: F) -> Result<FlowRespMsg, &'static str>
where
    F: Fn(Request) -> Fut,
    Fut: Future<Output = FlowRespMsg>,
{
    if auth_req_async(&request_msg).await {
        return Ok(flow_run(request_msg).await);
    }
    Err("Auth failed")
}

async fn auth_req_async(request_msg: &Request) -> bool {
    request_msg.username == "admin" && request_msg.password == "password"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_guard() {
        let request_msg1 = Request {
            username: "admin".to_string(),
            password: "password".to_string(),
            data: "Successfull".to_string(),
        };
        let resp = auth_guard(request_msg1, |r| FlowRespMsg { response_data: r.data});
        assert_eq!(resp.unwrap().response_data, "Successfull");

        let request_msg2 = Request {
            username: "invalid username".to_string(),
            password: "invalid password".to_string(),
            data: "Failed".to_string(),
        };
        let resp = auth_guard(request_msg2, |r| FlowRespMsg { response_data: r.data});
        assert!(resp.is_err());
    }

    #[actix_rt::test]
    async fn test_auth_guard_async() {
        let request_msg1 = Request {
            username: "admin".to_string(),
            password: "password".to_string(),
            data: "Successfull".to_string(),
        };
        let resp = auth_guard(request_msg1, |r| FlowRespMsg { response_data: r.data});
        assert_eq!(resp.unwrap().response_data, "Successfull");

        let request_msg1 = Request {
            username: "invalid username".to_string(),
            password: "invalid password".to_string(),
            data: "Failed".to_string(),
        };
        let resp = auth_guard(request_msg1, |r| FlowRespMsg { response_data: r.data});
        assert!(resp.is_err());
    }
}
