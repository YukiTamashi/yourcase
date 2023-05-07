use yew::prelude::*;
use serde::{Serialize, Deserialize};
use crate::components::input_field::*;
use crate::components::values::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[derive(Serialize, Deserialize, Debug)]
struct FormData{
    loja: String,
    promotor: String, 
    modelo: String,
    valor: i32,
}

struct FormDataBuilder{
    
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[function_component(Form)]
pub fn form() -> Html{
    let loja = Input{
        name: String::from("Loja"), 
        state: use_state(String::new)};
    let promotor = Input{
        name: String::from("Promotor"), 
        state: use_state(String::new)};
    let modelo = Input{
        name: String::from("Modelo"), 
        state: use_state(String::new)};
    let valor = use_state(i32::default);

    let states = vec!(
        loja.state.clone(), 
        promotor.state.clone(), 
        modelo.state.clone()
    );


    let on_submit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        spawn_local(async move{

            let args = serde_wasm_bindgen::to_value(&()).unwrap();
            let submit = invoke("submit_form", args).await.as_string().unwrap();
        });
        for state in &states{
            state.set(String::new());
        }
    });

    html!(
        <form class="container form-box" onsubmit={on_submit}>
            <InputField ..loja/>
            <InputField ..promotor/>
            <InputField ..modelo/>
            <Values valor ={valor}/>
            <button type="submit">{"Enviar"}</button>
        </form>
    )
}



