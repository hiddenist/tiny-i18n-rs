use super::Msg;

pub fn m(msg: Msg) -> &'static str {
    match msg {
        Msg::Welcome => "Willkommen!",
        Msg::Introduction => "Ich lerne derzeit Rust. Dies ist eine kleine Web-App, die mit Yew erstellt wurde und ein einfaches Internationalisierungssystem demonstriert.",
        Msg::SelectLanguageLabel => "Sprache:",
        Msg::Language(lang) => match lang {
            super::Lang::De => "Deutsch",
            super::Lang::En => "Englisch",
            super::Lang::Es => "Spanisch",
            super::Lang::Fr => "Französisch",
            super::Lang::Jp => "Japanisch",
        },
        Msg::ViewCodeLink => "Code auf GitHub anzeigen",
    }
}
