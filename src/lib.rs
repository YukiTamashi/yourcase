use yew::prelude::*;
mod components;
mod tauri;
use crate::components::form::*;

#[derive(Clone, Copy)]
enum Menu {
    Main,
    Management,
    Options,
}

#[function_component(App)]
pub fn app() -> Html {
    let menu_state = use_state(|| Menu::Main);
    let set_home = set_menu(Menu::Main, menu_state.clone());
    let set_mng = set_menu(Menu::Management, menu_state.clone());
    let set_options = set_menu(Menu::Options, menu_state.clone());

    let current_display = match *menu_state {
        Menu::Main => html!(<Form />),
        Menu::Management => html!(),
        Menu::Options => html!(),
    };

    html! {
        <>
            <main class="main">
                {current_display}
            </main>
            <footer>
                <button onclick={set_home}>{"Home"}</button>
                <button onclick={set_mng}>{"Management"}</button>
                <button onclick={set_options}>{"Options"}</button>
            </footer>
        </>
    }
}

fn set_menu(option: Menu, handle: UseStateHandle<Menu>) -> Callback<MouseEvent> {
    Callback::from(move |_| handle.set(option))
}
