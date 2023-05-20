use serde::Serialize;
use serde_wasm_bindgen::to_value;
use std::future::Future;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

mod catch {
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], catch)]
        pub async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
    }
}

#[derive(Serialize)]
struct Arguments<T: Serialize> {
    args: T,
}

//The struct plus this function provide a generic api for passing data to tauri
pub fn as_arg<T: Serialize>(arg: T) -> JsValue {
    to_value(&Arguments { args: arg }).unwrap()
}

#[derive(Serialize)]
pub enum Tables {
    Store(NewStore),
    Promoter(NewPromoter),
    Model(NewModel),
    Promotion(NewPromotion),
    Purchase(NewPurchase),
}

impl Tables {
    //This function handles the entire serialization into invoking the tauri command.
    pub fn insert(self) -> impl Future<Output = Result<JsValue, JsValue>> {
        catch::invoke("insert", as_arg(self))
    }
}

pub fn test() -> impl Future<Output = JsValue> {
    invoke("test", to_value(&()).unwrap())
}

#[derive(Serialize)]
pub struct NewStore {
    pub name: String,
}

#[derive(Serialize)]
pub struct NewPromoter {
    pub store_id: i32,
    pub name: String,
    pub bank_id: Option<String>,
}

#[derive(Serialize)]
pub struct NewModel {
    pub name: String,
}

#[derive(Serialize)]
pub struct NewPromotion {
    pub promoter_id: i32,
    pub model_id: i32,
}

#[derive(Serialize)]
pub struct NewPurchase {
    pub id: i32,
    pub promoter_id: i32,
    pub description: String,
}

#[derive(Serialize)]
pub struct NewPayment {
    pub promoter_id: i32,
    pub value: i32,
}
