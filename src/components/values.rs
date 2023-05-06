use web_sys::{HtmlScriptElement, HtmlSelectElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ValuesProp{
    pub valor: UseStateHandle<String>
}

#[function_component(Values)]
pub fn values(value: &ValuesProp) -> Html {
    let on_switch = on_switch(value.valor.clone());
    html!(
        <div>
            <select id="value" value={(*value.valor).clone()} onchange={on_switch}>
                {for create_options()}
            </select>
        </div>
    )

}

fn on_switch(handle: UseStateHandle<String>) -> Callback<Event> {
    Callback::from(move |e: Event| {
        if let Some(input) = e.target_dyn_into::<HtmlSelectElement>() {
            handle.set(input.value());
        }
    })
}


fn create_options() -> Vec<Html> {
    vec![
        html! { <option value="">{"Valor"}</option> },
        html! { <option value="20">{"Virou (20)"}</option> },
        html! { <option value="25">{"Virou Extra (25)"}</option> },
    ]
}