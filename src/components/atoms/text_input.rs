use gloo::file::ObjectUrl;
use serde::de::value;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use gloo::console::log;

#[derive(Properties, PartialEq)]
 
 pub struct Props {
    pub name: String,

 }

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let onchange = Callback::from(|event: Event| {
        let value = event
        .target()
        .unwrap()
        .unchecked_into::<HtmlInputElement>()
        .value();
        log!(value);
    

    });
    html! {
        <input type="text" name={props.name.clone()} onchange={onchange} />
    }
}