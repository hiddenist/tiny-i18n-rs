use super::Message;

pub fn m(msg: Message) -> &'static str {
    match msg {
        Message::Welcome => "Willkommen!",
        Message::Introduction => "Ich lerne derzeit Rust. Dies ist eine kleine Web-App, die mit Yew erstellt wurde und ein einfaches Internationalisierungssystem demonstriert.",
        Message::SelectLanguageLabel => "Sprache:",
        Message::Language(lang) => match lang {
            super::Lang::De => "Deutsch",
            super::Lang::En => "Englisch",
            super::Lang::Es => "Spanisch",
            super::Lang::Fr => "Französisch",
            super::Lang::Jp => "Japanisch",
        },
        Message::ViewCodeLink => "Code auf GitHub anzeigen",
    }
}
