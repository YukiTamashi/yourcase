use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use yew::prelude::*;
mod components;
use crate::components::form::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="main">
            <Form />
        </main>
    }
}







