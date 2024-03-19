use super::use_message;
use crate::lang;
use yew::prelude::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub message: lang::Msg,
}

#[function_component]
pub fn Message(props: &Props) -> Html {
    let message = use_message(props.message);

    html! {
        <>{ message }</>
    }
}
