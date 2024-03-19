use crate::lang;
use yew::prelude::hook;

use super::lang_context;

#[hook]
pub fn use_message(msg: lang::Msg) -> &str {
    let lang_context = lang_context::use_lang_context();
    lang::get_translated_message(lang_context.lang, msg)
}
