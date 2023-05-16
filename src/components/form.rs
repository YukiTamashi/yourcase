use yew::prelude::*;
use serde::{Serialize, Deserialize};
use crate::components::input_field::*;
use crate::components::values::*;
use wasm_bindgen_futures::spawn_local;
use crate::tauri::insert;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FormData{
    store: String,
    promoter: String, 
    model: String,
    value: i32,
}

impl From<FormInternal> for FormData{
    fn from(value: FormInternal) -> Self {
        FormData { 
            store: (*value.store).clone(), 
            promoter: (*value.promoter).clone(), 
            model: (*value.model).clone(), 
            value: *value.value 
        }
    }
}

#[derive(PartialEq, Clone)]
struct FormInternal{
    store: UseStateHandle<String>,
    promoter: UseStateHandle<String>, 
    model: UseStateHandle<String>,
    value: UseStateHandle<i32>,
}

impl FormInternal{
    fn reset(&self) {
        self.store.set(Default::default());
        self.promoter.set(Default::default());
        self.model.set(Default::default());
        self.value.set(Default::default());
    }

    fn reset_into(&self) -> FormData{
        let form = FormData::from(self.clone());
        self.reset();
        form
    }
}

#[function_component(Form)]
pub fn form() -> Html{
    let data = FormInternal{
        store: use_state(Default::default),
        promoter: use_state(Default::default),
        model: use_state(Default::default),
        value: use_state(Default::default)
    };
    let onsubmit = on_submit(data.clone());

    html!(
        <form class="container form-box" {onsubmit}>
            <InputField name ="Loja" state= {data.store.clone()}/>
            <InputField name ="Promotor" state= {data.promoter.clone()}/>
            <InputField name ="Modelo" state= {data.model.clone()}/>
            <Values value = {data.value}/>
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
                //submit_form(form).await;
                insert(form).await;
            });
        })
}
