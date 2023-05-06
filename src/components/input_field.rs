use yew::prelude::*;


#[derive(Properties, PartialEq, Clone)]
pub struct Input{
    pub name: String,
    pub state: UseStateHandle<String>
}

#[macro_export]
macro_rules! create_input {
    ($name:expr) => {
        Input {
            name: String::from($name),
            state: use_state(String::new),
        }
    };
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
    let promoter = handle.clone();
    Callback::from(move |e: InputEvent| {
        if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
            promoter.set(input.value());
        }
    })
}