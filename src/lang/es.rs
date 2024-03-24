use super::Msg;

pub fn m(msg: Msg) -> &'static str {
    match msg {
        Msg::Welcome => "¡Bienvenido!",
        Msg::Introduction => "Actualmente estoy aprendiendo Rust. Esta es una pequeña aplicación web construida con Yew que demuestra un sistema de internacionalización simple.",
        Msg::SelectLanguageLabel => "Idioma:",
        Msg::Language(lang) => match lang {
            super::Lang::De => "Alemán",
            super::Lang::En => "Inglés",
            super::Lang::Es => "Español",
            super::Lang::Fr => "Francés",
            super::Lang::Ja => "Japonés",
        },
        Msg::ViewCodeLink => "Ver el código en GitHub",
    }
}
