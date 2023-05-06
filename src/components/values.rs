use yew::prelude::*;

#[function_component(Values)]
pub fn values() -> Html {
    html!(
        <div>
            <label for="value">{"Virou: "}</label>
            <select id="value" value={"temp".to_string()} /*onchange={todo!()}*/>
                {for create_options()}
            </select>
        </div>
    )

}


fn create_options() -> Vec<Html> {
    vec![
        html! { <option value="20">{"Virou (20)"}</option> },
        html! { <option value="25">{"Virou Extra (25)"}</option> },
    ]
}