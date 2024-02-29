use yew::prelude::*;

use wasm_bindgen::{
    JsCast,
    closure::Closure,
};

use serde::Serialize;
use serde::de::DeserializeOwned;

#[derive(Clone)]
#[derive(PartialEq)]
pub struct WebSocket {
    pub ws: web_sys::WebSocket,
}

impl WebSocket {
    pub fn new<SocketMessage>(url: &str, callback: Callback<SocketMessage>) -> Self
    where SocketMessage: DeserializeOwned + 'static,
    {
        let ws = web_sys::WebSocket::new(url).unwrap();

        let rc = std::rc::Rc::new(callback);
        let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: web_sys::MessageEvent| {
            let string = e.data().as_string().unwrap();
            crate::utils::log(string.as_str());
            let msg: SocketMessage = serde_json::from_str( string.as_str() ).unwrap();
            rc.emit(msg);
        });
        ws.set_onmessage( Some(onmessage_callback.as_ref().unchecked_ref()) );
        onmessage_callback.forget();

        Self { ws }
    }

    pub fn send<Data: Serialize>(&self, data: Data) {
        let jsonstr = serde_json::to_string(&data).unwrap();
        if let Err(e) = self.ws.send_with_str( jsonstr.as_str() ) {
            crate::utils::log(format!("{:?}", e).as_str());
        }
    }
}
