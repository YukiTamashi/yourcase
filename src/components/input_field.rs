use yew::prelude::*;


#[derive(Properties, PartialEq, Clone)]
pub struct Input{
    pub name: String,
    pub state: UseStateHandle<String>
}

#[function_component(InputField)]
pub fn input_field(props: &Input) -> Html{
    html!(
        <input 
        class="centered-text"
        id={props.name.clone()} 
        type="text" value={(*props.state).clone()} 
        placeholder={props.name.clone()} 
        oninput={on_input(props.state.clone())} />
    )
}

fn on_input(handle: UseStateHandle<String>) -> Callback<InputEvent>{
    Callback::from(move |e: InputEvent| {
        if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
            handle.set(input.value());
        }
    })
}