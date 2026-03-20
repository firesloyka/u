use std::ops::Deref;

use gloo::console::log;
use gloo::timers::callback;
use stylist::yew::styled_component;
use yew::ContextProvider;
use yew::prelude::*;

mod components;
mod router;

use components::atoms::main_title::{Color, MainTitle};
use components::molecules::custom_form::CustomForm;

use crate::components::molecules::custom_form::Data;
use crate::router::Route;
use crate::router::switch;
use components::atoms::struct_hello::StructHello;
use yew_router::prelude::*;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct User {
    pub username: String,
    pub favorite_language: String,
}

#[styled_component(App)]
pub fn app() -> Html {
    let user_state = use_state(User::default);
    let main_title_load = Callback::from(|message: String| log!(message));
    let first_load = use_state(|| true);
    use_effect(move || {
        if *first_load {
            first_load.set(false);
        }
        || {}
    });
    let custom_form_submit = {
        let user_state = user_state.clone();
        Callback::from(move |data: Data| {
            let mut user = user_state.deref().clone();
            user.username = data.username;
            user.favorite_language = data.favorite_language;
            user_state.set(user);
        })
    };

    html! {
        <div>
            <StructHello message={ "Hello from lib.rs".to_owned()} />
        </div>

    }
}
