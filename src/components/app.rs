use crate::lang::*;
use yew::prelude::*;

use crate::components::select_language::SelectLanguage;

#[function_component]
pub fn App() -> Html {
    let lang = use_state_eq(|| Lang::En);

    let select_lang = {
        let lang_value_handle = lang.clone();
        Callback::from(move |new_lang: Lang| lang_value_handle.set(new_lang))
    };

    html! {
        <main>
            <SelectLanguage lang={*lang} onselect={select_lang} />
            <hr />

            <h1>{ m(*lang, Message::Welcome) }</h1>
            <p>{ m(*lang, Message::Introduction)  }</p>

            <p>
                <a href="https://github.com/hiddenist/tiny-i18n-rs"> { m(*lang, Message::ViewCodeLink) }</a>
            </p>
        </main>
    }
}
