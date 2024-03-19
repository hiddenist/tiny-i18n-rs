use super::lang::SelectLanguage;
use super::lang::{use_message, Message, Msg};
use crate::components::lang::LangContextProvider;
use yew::prelude::{function_component, html, Html};

#[function_component]
pub fn App() -> Html {
    html! {
        <LangContextProvider>
            <Main />
        </LangContextProvider>
    }
}

#[function_component]
fn Main() -> Html {
    let title = use_message(Msg::Welcome);
    html! {
        <main>
            <SelectLanguage />
            <hr />

            <h1>{title}</h1>
            <p><Message message={Msg::Introduction} /></p>

            <p>
                <a href="https://github.com/hiddenist/tiny-i18n-rs"><Message message={Msg::ViewCodeLink} /></a>
            </p>
        </main>
    }
}
