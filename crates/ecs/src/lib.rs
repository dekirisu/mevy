use mevy_ecs_syntax::*;
use proc_macro::TokenStream as CompilerTokens;
use deki::*;

#[proc_macro]
pub fn entity(stream:CompilerTokens) -> CompilerTokens {
    world_spawn_syntax(stream.into()).into()
}

#[proc_macro]
pub fn modify(stream:CompilerTokens) -> CompilerTokens {
    let stream: TokenStream = stream.into();
    world_spawn_syntax(qt!(<|> #stream)).into()
}
