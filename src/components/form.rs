use std::future::Future;

use yew::prelude::*;
use serde::{Serialize, Deserialize};
use crate::components::input_field::*;
use crate::components::values::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use serde_wasm_bindgen::to_value;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FormData{
    loja: String,
    promotor: String, 
    modelo: String,
    valor: i32,
}

impl From<FormInternal> for FormData{
    fn from(value: FormInternal) -> Self {
        FormData { 
            loja: (*value.loja).clone(), 
            promotor: (*value.promotor).clone(), 
            modelo: (*value.modelo).clone(), 
            valor: *value.valor 
        }
    }
}

#[derive(PartialEq, Clone)]
struct FormInternal{
    loja: UseStateHandle<String>,
    promotor: UseStateHandle<String>, 
    modelo: UseStateHandle<String>,
    valor: UseStateHandle<i32>,
}

impl FormInternal{
    fn reset(&self) {
        self.loja.set(Default::default());
        self.promotor.set(Default::default());
        self.modelo.set(Default::default());
        self.valor.set(Default::default());
    }

    fn reset_into(&self) -> FormData{
        let form = FormData::from(self.clone());
        self.reset();
        form
    }
}


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[function_component(Form)]
pub fn form() -> Html{
    let data = FormInternal{
        loja: use_state(Default::default),
        promotor: use_state(Default::default),
        modelo: use_state(Default::default),
        valor: use_state(Default::default)
    };

    html!(
        <form class="container form-box" onsubmit={on_submit(data.clone())}>
            <InputField name ="Loja" state= {data.loja.clone()}/>
            <InputField name ="Promotor" state= {data.promotor.clone()}/>
            <InputField name ="Modelo" state= {data.modelo.clone()}/>
            <Values valor = {data.valor}/>
            <button type="submit">{"Enviar"}</button>
        </form>
    )
}

fn on_submit(data: FormInternal) -> Callback<SubmitEvent>{
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            //Needs to clone so it doesnt move the external data into the closure
            let form = data.clone().reset_into();
            spawn_local(async move{
                submit_form(form).await;
            });
        })
}

fn submit_form(form: FormData) -> impl Future<Output = JsValue>{
    let args = to_value(&form).unwrap();
    invoke("submit_form", args)
}