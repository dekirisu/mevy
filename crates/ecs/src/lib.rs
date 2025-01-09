use mevy_ecs_syntax::*;
use proc_macro::TokenStream as CompilerTokens;

#[proc_macro]
pub fn spawn(stream:CompilerTokens) -> CompilerTokens {
    spawn_syntax(stream.into()).into()
}
