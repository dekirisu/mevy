use mevy_ecs_syntax::*;
use proc_macro::TokenStream as CompilerTokens;
use deki::*;

#[proc_macro]
pub fn entity(stream:CompilerTokens) -> CompilerTokens {
    world_spawn_syntax(stream.into()).into()
}

#[proc_macro]
pub fn spawn(stream:CompilerTokens) -> CompilerTokens {
    spawn_syntax(stream.into()).into()
}

#[proc_macro]
pub fn modify(stream:CompilerTokens) -> CompilerTokens {
    let stream: TokenStream = stream.into();
    let mut iter = stream.peek_iter();
    let pre = match iter.peek_punct() {
        '*' => {iter.next();qt!(*this;)},
        _ => qt!(&me;)
    };
    spawn_syntax(qt!(#pre #(#iter)*)).into()
}
