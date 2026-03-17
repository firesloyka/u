use gloo::{console::log, history};
use yew::prelude::*;
use yew_router::{hooks::use_history, prelude::History};

use crate::router::Route;

#[function_component(Hello)]

pub fn home() -> Html {
    let history =use_history().unwrap();
    let onclick = Callback::from(move |_| {
        history.push(Route::Home);
    });
    html! {
        <div>
        <h1>{"Hello"}</h1>
        <button {onclick}>{"Go Home"}</button>
    </div>
    }
}
