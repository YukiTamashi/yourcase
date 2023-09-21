use yew::prelude::*;
use daisyui::prelude::Input;
use daisyui::prelude::InputProps;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub name: String,
    pub state: UseStateHandle<String>,
}

#[function_component(InputField)]
pub fn input_field(props: &Props) -> Html {
    let a = InputProps{};
    html!(
        <>
        <Input />
        </>
        // //class to set horizontal size to 100%
        // <input
        // class="input w-full max-w-xs"
        // id={props.name.clone()}
        // type="text" value={(*props.state).clone()}
        // placeholder={props.name.clone()}
        // oninput={on_input(props.state.clone())} />
    )
}

fn on_input(handle: UseStateHandle<String>) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
            handle.set(input.value());
        }
    })
}
