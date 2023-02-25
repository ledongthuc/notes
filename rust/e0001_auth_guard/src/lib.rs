#![feature(async_closure)]

use std::future::Future;

pub struct Request {
    username: String,
    password: String,
    data: String,
}

/** SYNC func **/
pub fn auth_guard<F>(request_msg: Request, flow_run: F)
where
    F: Fn(Request),
{
    if auth_req(&request_msg) {
        flow_run(request_msg);
        return;
    }

    println!("Auth failed")
}

fn auth_req(request_msg: &Request) -> bool {
    request_msg.username == "admin" && request_msg.password == "password"
}

/** ASYNC func **/
pub async fn auth_guard_async<F, Fut>(request_msg: Request, flow_run: F)
where
    F: Fn(Request) -> Fut,
    Fut: Future,
{
    if auth_req_async(&request_msg).await {
        flow_run(request_msg).await;
        return;
    }

    println!("Auth failed")
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
            password: "admin".to_string(),
            data: "{}".to_string(),
        };
        auth_guard(request_msg1, |r| println!("Flow run 1: data: {}", r.data));

        let request_msg2 = Request {
            username: "admin".to_string(),
            password: "admin".to_string(),
            data: "{}".to_string(),
        };
        auth_guard(request_msg2, |r| println!("Flow run 2: data: {}", r.data));
    }

    #[test]
    fn test_auth_guard_async() {
        let request_msg1 = Request {
            username: "admin".to_string(),
            password: "admin".to_string(),
            data: "{}".to_string(),
        };
        auth_guard_async(request_msg1, async move |r| println!("Flow run 1: data: {}", r.data));

        let request_msg2 = Request {
            username: "admin".to_string(),
            password: "admin".to_string(),
            data: "{}".to_string(),
        };
        auth_guard_async(request_msg2, async move |r| println!("Flow run 2: data: {}", r.data));
    }
}
