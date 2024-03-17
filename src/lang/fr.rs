use super::Message;

pub fn m(msg: Message) -> &'static str {
    match msg {
        Message::Welcome => "Bienvenue!",
        Message::Introduction => "Je suis actuellement en train d'apprendre Rust. Ceci est une petite application web construite avec Yew qui démontre un système d'internationalisation simple.",
        Message::SelectLanguageLabel => "Langue :",
        Message::Language(lang) => match lang {
            super::Lang::De => "Allemand",
            super::Lang::En => "Anglais",
            super::Lang::Es => "Espagnol",
            super::Lang::Fr => "Français",
            super::Lang::Jp => "Japonais",
        },
        Message::ViewCodeLink => "Voir le code sur GitHub",
    }
}
