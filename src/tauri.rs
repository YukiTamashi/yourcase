use std::future::Future;
use serde::Serialize;
use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::to_value;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub trait DatabaseType: Serialize{}



#[derive(Serialize)]
struct Arguments<T: Serialize>{
    args: T
}

pub fn as_arg<T: Serialize>(arg: T) -> JsValue{
    to_value(&Arguments{args: arg}).unwrap()
}

pub fn submit_form(form: crate::components::form::FormData) -> impl Future<Output = JsValue>{
    invoke("submit_form", as_arg(form))
}

pub fn new_store(store: crate::components::db_management::Store) -> impl Future<Output = JsValue>{
    invoke("new_store", as_arg(store))
}

pub fn test<T: DatabaseType>(data: T) -> impl Future<Output = JsValue>{
    invoke("test", as_arg(data))
}