use yew::prelude::*;
use crate::components::input_field::*;
use crate::components::values::*;

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
    let valor = use_state(String::new);

    let states = vec!(
        loja.state.clone(), 
        promotor.state.clone(), 
        modelo.state.clone());
    let on_submit = on_form_submit(states);

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


fn on_form_submit(states: Vec<UseStateHandle<String>>) -> Callback<SubmitEvent>{
    Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        for state in &states{
            state.set(String::new());
        }
    })
}
