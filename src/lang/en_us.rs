use super::Msg;

pub fn m(msg: Msg) -> &'static str {
    match msg {
        Msg::Welcome => "Welcome!",
        Msg::Introduction => "I'm currently learning Rust. This is a small web app build with Yew that demonstrates a simple internationalization system.",
        Msg::SelectLanguageLabel => "Language:",
        Msg::Language(lang) => match lang {
            super::Lang::De => "German",
            super::Lang::En => "English",
            super::Lang::Es => "Spanish",
            super::Lang::Fr => "French",
            super::Lang::Ja => "Japanese",
        },
        Msg::ViewCodeLink => "View the code on GitHub",
    }
}
