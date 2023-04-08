use anyhow::{anyhow, Result};
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

/// A simple Spin HTTP component.
#[http_component]
fn handle_amtsschimmel(req: Request) -> Result<Response> {
    println!("{:?}", req.headers());

    let warten_req = http::Request::builder()
        .method("GET")
        .uri("https://warten.stuttgart.de/warten/update")
        .body(None)?;

    let warten_res = spin_sdk::outbound_http::send_request(warten_req)?;

    let body_bytes = warten_res.body().as_ref().ok_or(anyhow!("error"))?;
    let body = String::from_utf8(body_bytes.to_vec())?;

    println!("body: {:?}", body);

    let components = body.split("|").collect::<Vec<_>>();
    let numbers = components
        .get(0)
        .ok_or(anyhow!("no numbers component"))?
        .split(",")
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;
    let keys = components
        .get(1)
        .ok_or(anyhow!("no keys component"))?
        .split(",")
        .map(|s| s.trim_start())
        .collect::<Vec<_>>();
    let office_map = keys.iter().zip(numbers).collect::<Vec<_>>();

    let msg = format!("values: {:?}", &office_map);

    let request = http::Response::builder()
        .status(200)
        .header("Content-Type", "text/plain;charset=utf8");

    Ok(request.body(Some(msg.into()))?)
}
