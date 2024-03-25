use crate::lang;
use std::rc::Rc;
use yew::{
    prelude::{
        function_component, hook, html, use_context, use_reducer_eq, ContextProvider, Html,
        Properties, Reducible,
    },
    UseReducerHandle,
};

#[derive(PartialEq, Clone)]
pub struct LangState {
    pub lang: lang::Lang,
}

pub enum LangAction {
    SetLang(lang::Lang),
}

impl Reducible for LangState {
    type Action = LangAction;
    fn reduce(self: Rc<Self>, msg: Self::Action) -> Rc<Self> {
        let new_lang = match msg {
            LangAction::SetLang(lang) => lang,
        };

        Self { lang: new_lang }.into()
    }
}

impl Default for LangState {
    fn default() -> Self {
        LangState {
            lang: get_navigator_lang().unwrap_or(lang::Lang::En),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
}

pub fn get_navigator_lang() -> Option<lang::Lang> {
    let window = web_sys::window().expect("no global `window` exists");

    if let Some(first_lang) = window.navigator().language() {
        if let Some(lang) = lang::lang_from_string(first_lang) {
            return Some(lang);
        }
    }

    let languages = window.navigator().languages();
    for i in 0..languages.length() {
        let lang_string = languages.get(i).as_string();

        match lang_string {
            Some(lang) => {
                if let Some(lang) = lang::lang_from_string(lang) {
                    return Some(lang);
                }
            }
            None => continue,
        }
    }

    None
}

#[function_component]
pub fn LangContextProvider(props: &Props) -> Html {
    let lang_reducer = use_reducer_eq(LangState::default);

    html! {
        <ContextProvider<UseReducerHandle<LangState>> context={lang_reducer.clone()}>
            { props.children.clone() }
        </ContextProvider<UseReducerHandle<LangState>>>
    }
}

#[hook]
pub fn use_lang_context() -> UseReducerHandle<LangState> {
    use_context::<UseReducerHandle<LangState>>()
        .expect("use_lang_context must be used within LangContextProvider")
}
