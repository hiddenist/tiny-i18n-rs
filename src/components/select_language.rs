use web_sys::HtmlSelectElement;
use yew::prelude::*;

use crate::lang::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub lang: Lang,
    pub onselect: Callback<Lang>,
}

#[function_component]
pub fn SelectLanguage(props: &Props) -> Html {
    let items = select_language_options();

    let select_lang = {
        let onselect = props.onselect.clone();
        let items = items.clone();
        Callback::from(move |e: Event| {
            if let Some(select) = e.target_dyn_into::<HtmlSelectElement>() {
                let lang = items
                    .iter()
                    .find(|item| item.value == select.value().as_str())
                    .map(|item| item.lang)
                    .unwrap_or(Lang::En);
                onselect.emit(lang);
            }
        })
    };

    html! {
        <label>
            <span>{ m(props.lang, Message::SelectLanguageLabel) }</span>
            <select onchange={select_lang}>
                { for items.iter().map(|item| html! {
                    <option value={item.value} selected={item.lang == props.lang}>
                        { item.text }
                    </option>
                }) }
            </select>
        </label>
    }
}
