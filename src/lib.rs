use yew::prelude::*;
mod components;
mod tauri;
use crate::components::form::*;
use yew_icons::{Icon, IconId};

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
        Menu::Management => html!(<Management />),
        Menu::Options => html!(<Options />),
    };

    html! {
        <main class="background">
            <main class="pattern main">
                {current_display}
            </main>
            <footer>
                <button onclick={set_home}>
                    <Icon icon_id={IconId::HeroiconsOutlineHome}/>
                </button>
                <button onclick={set_mng}>
                    <Icon icon_id={IconId::HeroiconsOutlineBars3}/>
                </button>
                <button onclick={set_options}>
                    <Icon icon_id={IconId::HeroiconsOutlineCog6Tooth}/>
                </button>
            </footer>
        </main>
    }
}

#[function_component(Management)]
fn management() -> Html{
    html!()
}

#[function_component(Options)]
fn options() -> Html{
    html!()
}

fn set_menu(option: Menu, handle: UseStateHandle<Menu>) -> Callback<MouseEvent> {
    Callback::from(move |_| handle.set(option))
}