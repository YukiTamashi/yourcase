use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct Input{
    pub name: String,
    pub state: UseStateHandle<String>
}

#[function_component(InputField)]
pub fn input_field(props: &Input) -> Html{
    html!(
        <input id={props.name.clone()} type="text" value={props.name.clone()} placeholder="Promotor" oninput={on_input(props.state.clone())} />
    )
}

fn on_input(handle: UseStateHandle<String>) -> Callback<InputEvent>{
    let promoter = handle.clone();
    Callback::from(move |e: InputEvent| {
        if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
            promoter.set(input.value());
        }
    })
}