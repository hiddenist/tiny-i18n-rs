use super::Message;

pub fn m(msg: Message) -> &'static str {
    match msg {
        Message::Welcome => "¡Bienvenido!",
        Message::Introduction => "Actualmente estoy aprendiendo Rust. Esta es una pequeña aplicación web construida con Yew que demuestra un sistema de internacionalización simple.",
        Message::SelectLanguageLabel => "Idioma:",
        Message::Language(lang) => match lang {
            super::Lang::De => "Alemán",
            super::Lang::En => "Inglés",
            super::Lang::Es => "Español",
            super::Lang::Fr => "Francés",
            super::Lang::Jp => "Japonés",
        },
        Message::ViewCodeLink => "Ver el código en GitHub",
    }
}
