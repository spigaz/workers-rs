use wasm_bindgen::prelude::*;

use crate::types::{DurableObjectId, DurableObjectStorage};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends=js_sys::Object)]
    pub type DurableObjectState;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &DurableObjectState) -> DurableObjectId;

    #[wasm_bindgen(method, getter)]
    pub fn storage(this: &DurableObjectState) -> DurableObjectStorage;

    #[wasm_bindgen(method, js_name=waitUntil)]
    pub fn wait_until(this: &DurableObjectState, promise: &js_sys::Promise);

    #[wasm_bindgen(method, js_name=acceptWebSocket)]
    pub fn accept_websocket(this: &DurableObjectState, ws: &web_sys::WebSocket, tags: &js_sys::Object);

    #[wasm_bindgen(method, js_name=getWebSockets)]
    pub fn get_websockets(this: &DurableObjectState, tag: wasm_bindgen::JsValue) -> wasm_bindgen::JsValue;

    /* not implemented yet.
    #[wasm_bindgen(extends=js_sys::Object)]
    pub type WebSocketRequestResponsePair;

    #[wasm_bindgen(method, js_name=setWebSocketAutoResponse)]
    pub fn set_websocket_auto_response(this: &DurableObjectState, request_response: WebSocketRequestResponsePair);
    */
}
