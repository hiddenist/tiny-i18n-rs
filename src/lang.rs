#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Lang {
    De,
    En,
    Es,
    Fr,
    Ja,
}

impl std::fmt::Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum Msg {
    Welcome,
    Introduction,
    SelectLanguageLabel,
    Language(Lang),
    ViewCodeLink,
}

mod de;
mod en_us;
mod es;
mod fr;
mod ja;

pub fn get_translated_message(lang: Lang, msg: Msg) -> &'static str {
    match lang {
        Lang::Ja => ja::m(msg),
        Lang::En => en_us::m(msg),
        Lang::Es => es::m(msg),
        Lang::Fr => fr::m(msg),
        Lang::De => de::m(msg),
    }
}

pub fn lang_from_string(lang: String) -> Option<Lang> {
    let lang = lang.to_lowercase();
    let lang = &lang[..2];

    match lang {
        "de" => Some(Lang::De),
        "en" => Some(Lang::En),
        "es" => Some(Lang::Es),
        "fr" => Some(Lang::Fr),
        "ja" => Some(Lang::Ja),
        _ => None,
    }
}

#[derive(Clone)]
pub struct LangOption {
    pub lang: Lang,
    pub value: &'static str,
    pub text: &'static str,
}

impl LangOption {
    pub fn new(lang: Lang, value: &'static str) -> Self {
        LangOption {
            lang,
            value,
            text: get_translated_message(lang, Msg::Language(lang)),
        }
    }
}

pub fn select_language_options() -> Vec<LangOption> {
    vec![
        LangOption::new(Lang::De, "de"),
        LangOption::new(Lang::En, "en"),
        LangOption::new(Lang::Es, "es"),
        LangOption::new(Lang::Fr, "fr"),
        LangOption::new(Lang::Ja, "jp"),
    ]
}
