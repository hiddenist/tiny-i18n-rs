use super::Message;

pub fn m(msg: Message) -> &'static str {
    match msg {
        Message::Welcome => "ようこそ！",
        Message::Introduction => "私は現在Rustを学んでいます。これはYewで構築された小さなWebアプリで、シンプルな国際化システムを実証しています。",
        Message::SelectLanguageLabel => "言語：",
        Message::Language(lang) => match lang {
            super::Lang::De => "ドイツ語",
            super::Lang::En => "英語",
            super::Lang::Es => "スペイン語",
            super::Lang::Fr => "フランス語",
            super::Lang::Jp => "日本語",
        },
        Message::ViewCodeLink => "GitHubでコードを表示",
    }
}
