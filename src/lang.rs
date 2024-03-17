#[derive(PartialEq, Copy, Clone)]
pub enum Lang {
    Jp,
    En,
    Es,
    Fr,
    De,
}

pub enum Message {
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
mod jp;

pub fn m(lang: Lang, msg: Message) -> &'static str {
    match lang {
        Lang::Jp => jp::m(msg),
        Lang::En => en_us::m(msg),
        Lang::Es => es::m(msg),
        Lang::Fr => fr::m(msg),
        Lang::De => de::m(msg),
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
            text: m(lang, Message::Language(lang)),
        }
    }
}

pub fn select_language_options() -> Vec<LangOption> {
    vec![
        LangOption::new(Lang::De, "de"),
        LangOption::new(Lang::En, "en"),
        LangOption::new(Lang::Es, "es"),
        LangOption::new(Lang::Fr, "fr"),
        LangOption::new(Lang::Jp, "jp"),
    ]
}
