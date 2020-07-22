use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Email {
    pub email: String,
}

#[wasm_bindgen]
pub async fn put_email(url: String, target_email: String) -> Result<JsValue, JsValue> {

    let email_address = Email { email: target_email };
    let email_json = json!(email_address).to_string();

    // set up the request options with web_sys::RequestInit
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.body(Some(&wasm_bindgen::JsValue::from_str(&email_json)));
    opts.mode(RequestMode::Cors);

    // Set our url - probably can pass this going forward?
    //let url = "http://localhost:8000/";

    let request = Request::new_with_str_and_init(&url, &opts)?;

    // need to call directly from our window
    // unwrap means that if this errors for some reason, we crash out - basically undoing error handling
    let window = web_sys::window().unwrap();

    //let's get our data
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // Make sure we got what we think we got - a response object
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    let api_response: ApiResponse = json.into_serde().unwrap();

    Ok(JsValue::from_serde(&api_response).unwrap())
}
