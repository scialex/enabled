#![crate_type="dylib"]
#![crate_name="enabled"]
#![feature(plugin_registrar)]
#![doc(html_logo_url = "https://avatars.io/gravatar/d0ad9c6f37bb5aceac2d7ac95ba82607?size=large",
       html_favicon_url="https://avatars.io/gravatar/d0ad9c6f37bb5aceac2d7ac95ba82607?size=small")]

//! This crate defines an `is_enabled` and an `is_disabled` macro.
//!
//! The usage is `is_enabled!(TYPE -> NAME)` becomes `cfg!(TYPE_NAME)` and
//! `is_disabled!(TYPE -> NAME)` becomes `cfg!(NTYPE_NAME)`

#![feature(rustc_private)]
#![feature(collections)]
#![feature(core)]
extern crate syntax;
extern crate rustc;

use syntax::codemap::Span;
use syntax::parse::token;
use syntax::ast::{TokenTree, TtToken};
use syntax::ext::cfg;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult};
use rustc::plugin::Registry;

enum IDType {
    Titled(token::InternedString, token::InternedString),
    Normal(token::InternedString),
}

impl IDType {
    fn get_full(&self) -> String {
        match *self {
            IDType::Titled(ref t, ref n) => {
                let mut ret = t.get().to_string();
                ret.push('_');
                ret.push_str(n.get());
                ret
            },
            IDType::Normal(ref n) => n.get().to_string(),
        }
    }
}

fn expand(prefix: String, cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'static> {
    use IDType::*;
    let id = match args {
        [TtToken(_, token::Ident(title, _)), TtToken(_, token::RArrow), TtToken(_, token::Ident(name, _))] =>
            Titled(token::get_ident(title), token::get_ident(name)),
        [TtToken(_, token::Ident(name, _))] => Normal(token::get_ident(name)),
        _ => {
            cx.span_err(sp, "Argument should be 'module_name->option_name' or 'option_name'");
            return DummyResult::any(sp);
        }
    };
    let mut check_name = prefix;
    check_name.push_str(&id.get_full()[]);
    let outtok = token::gensym_ident(&check_name[]);
    let toktree = [TtToken(sp, token::Ident(outtok, token::Plain))];
    cfg::expand_cfg(cx, sp, &toktree)
}

fn expand_is_disabled(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'static> {
    expand("N".to_string(), cx, sp, args)
}

fn expand_is_enabled(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'static> {
    expand("".to_string(), cx, sp, args)
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("is_enabled", expand_is_enabled);
    reg.register_macro("is_disabled", expand_is_disabled);
}
