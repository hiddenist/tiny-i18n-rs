use super::Message;

pub fn m(msg: Message) -> &'static str {
    match msg {
        Message::Welcome => "Welcome!",
        Message::Introduction => "I'm currently learning Rust. This is a small web app build with Yew that demonstrates a simple internationalization system.",
        Message::SelectLanguageLabel => "Language:",
        Message::Language(lang) => match lang {
            super::Lang::De => "German",
            super::Lang::En => "English",
            super::Lang::Es => "Spanish",
            super::Lang::Fr => "French",
            super::Lang::Jp => "Japanese",
        },
        Message::ViewCodeLink => "View the code on GitHub",
    }
}
