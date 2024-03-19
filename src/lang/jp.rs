use super::Msg;

pub fn m(msg: Msg) -> &'static str {
    match msg {
        Msg::Welcome => "ようこそ！",
        Msg::Introduction => "私は現在Rustを学んでいます。これはYewで構築された小さなWebアプリで、シンプルな国際化システムを実証しています。",
        Msg::SelectLanguageLabel => "言語：",
        Msg::Language(lang) => match lang {
            super::Lang::De => "ドイツ語",
            super::Lang::En => "英語",
            super::Lang::Es => "スペイン語",
            super::Lang::Fr => "フランス語",
            super::Lang::Jp => "日本語",
        },
        Msg::ViewCodeLink => "GitHubでコードを表示",
    }
}
