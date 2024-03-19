pub mod app;
pub mod lang {
    mod lang_context;
    mod message;
    mod select_language;
    mod use_message;

    pub use crate::lang::Msg;
    pub use lang_context::LangContextProvider;
    pub use message::Message;
    pub use select_language::SelectLanguage;
    pub use use_message::use_message;
}
