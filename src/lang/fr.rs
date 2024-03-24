use super::Msg;

pub fn m(msg: Msg) -> &'static str {
    match msg {
        Msg::Welcome => "Bienvenue!",
        Msg::Introduction => "Je suis actuellement en train d'apprendre Rust. Ceci est une petite application web construite avec Yew qui démontre un système d'internationalisation simple.",
        Msg::SelectLanguageLabel => "Langue :",
        Msg::Language(lang) => match lang {
            super::Lang::De => "Allemand",
            super::Lang::En => "Anglais",
            super::Lang::Es => "Espagnol",
            super::Lang::Fr => "Français",
            super::Lang::Ja => "Japonais",
        },
        Msg::ViewCodeLink => "Voir le code sur GitHub",
    }
}
