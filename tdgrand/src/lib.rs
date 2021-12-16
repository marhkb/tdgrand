// Copyright 2021 - developers of the `tdgrand` project.
// Copyright 2020 - developers of the `grammers` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
mod generated;
mod observer;
mod tdjson;

pub use generated::{enums, functions, types};

use enums::Update;
use once_cell::sync::Lazy;
use serde_json::Value;
use uuid::Uuid;

pub(crate) static OBSERVER: Lazy<observer::Observer> = Lazy::new(|| observer::Observer::new());

/// Create a TdLib client returning its id. Note that to start receiving
/// updates for a client you need to send at least a request with it first.
pub fn create_client() -> i32 {
    tdjson::create_client()
}

/// Sets the callback that will be called when a message is added to the internal TDLib log.
/// None of the TDLib methods can be called from the callback.
/// By default the callback is not set.
///
/// * `max_verbosity_level` - The maximum verbosity level of messages for which the callback will
///                           be called.
/// * `callback` - Callback that will be called when a message is added to the internal TDLib log.
///                Pass `None` to remove the callback.
pub fn set_log_message_callback(
    max_verbosity_level: i32,
    callback: Option<Box<dyn Fn(i32, &str) + Send>>,
) {
    tdjson::set_log_message_callback(max_verbosity_level, callback);
}

/// Receive a single update or response from TdLib. If it's an update, it
/// returns a tuple with the `Update` and the associated `client_id`.
/// Note that to start receiving updates for a client you need to send
/// at least a request with it first.
pub fn receive() -> Option<(Update, i32)> {
    let response = tdjson::receive(2.0);
    if let Some(response_str) = response {
        let response: Value = serde_json::from_str(&response_str).unwrap();

        match response["@extra"].as_str() {
            Some(_) => {
                OBSERVER.notify(response);
            }
            None => {
                let client_id = response["@client_id"].as_i64().unwrap() as i32;
                match serde_json::from_value(response) {
                    Ok(update) => {
                        return Some((update, client_id));
                    }
                    Err(e) => {
                        log::warn!(
                            "Received an unknown response: {}\nReason: {}",
                            response_str,
                            e
                        );
                    }
                }
            }
        }
    }

    None
}

pub(crate) async fn send_request(client_id: i32, mut request: Value) -> Value {
    let extra = Uuid::new_v4().to_string();
    request["@extra"] = serde_json::to_value(extra.clone()).unwrap();

    let receiver = OBSERVER.subscribe(extra);
    tdjson::send(client_id, request.to_string());

    receiver.await.unwrap()
}
