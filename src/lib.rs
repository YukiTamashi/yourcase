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
        <main class="h-screen">
            {include_cdn()}
            <header class="flex flex-grow justify-evenly sticky top-0 py-8">
                <Icon icon_id={IconId::HeroiconsOutlineHome} onclick={set_home}/>
                <Icon icon_id={IconId::HeroiconsOutlineBars3} onclick={set_mng}/>
                <Icon icon_id={IconId::HeroiconsOutlineCog6Tooth} onclick={set_options}/>
            </header>
            <main class="bg-green-400 flex-grow flex items-center justify-center">
                {current_display}
            </main>
            {include_cdn_js()}
        </main>
    }
}

#[function_component(Management)]
fn management() -> Html {
    html!()
}

#[function_component(Options)]
fn options() -> Html {
    html!()
}

fn set_menu(option: Menu, handle: UseStateHandle<Menu>) -> Callback<MouseEvent> {
    Callback::from(move |_| handle.set(option))
}
