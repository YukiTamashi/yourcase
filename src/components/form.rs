use yew::prelude::*;
use crate::components::input_field::*;
use crate::components::values::*;

#[function_component(Form)]
pub fn form() -> Html{
    let on_form_submit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        // Handle form submission logic here.
    });
    let value = use_state(String::new);
    let test = Input{name: "".to_string(), state: use_state(|| String::new())};

    html!(
        <form class="container centered" onsubmit={on_form_submit}>
            <InputField ..test/>
            <Values />
            <button type="submit">{"Submit"}</button>
        </form>
    )
}


