use super::lang_context;
use super::message::Message;
use crate::lang;
use web_sys::HtmlSelectElement;
use yew::prelude::{function_component, html, Callback, Event, Html, TargetCast};

#[function_component]
pub fn SelectLanguage() -> Html {
    let items = lang::select_language_options();
    let lang_context = lang_context::use_lang_context();

    let select_lang = {
        let items = items.clone();
        let reducer = lang_context.clone();
        Callback::from(move |e: Event| {
            if let Some(select) = e.target_dyn_into::<HtmlSelectElement>() {
                let lang = items
                    .iter()
                    .find(|item| item.value == select.value().as_str())
                    .map(|item| item.lang)
                    .unwrap_or(lang::Lang::En);
                reducer.dispatch(lang_context::LangAction::SetLang(lang));
            }
        })
    };

    html! {
        <label>
            <span><Message message={lang::Msg::SelectLanguageLabel} /></span>
            <select onchange={select_lang}>
                { for items.iter().map(|item| html! {
                    <option value={item.value} selected={item.lang == lang_context.lang}>
                        { item.text }
                    </option>
                }) }
            </select>
        </label>
    }
}
