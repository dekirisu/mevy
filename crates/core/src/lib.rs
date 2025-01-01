use mevy_core_syntax::code as code_syntax;
use proc_macro::TokenStream as CompilerTokens;

// Macro \\

    /// Replaces the following patterns:
    /// - `#rgb`/`#rgba`/`#rrggbb`/`#rrggbbaa` -> `Color`
    /// - `0px`/`0%`/`0vw`/`0vh`/`0vmin`/`0vmax`/`@`(auto) -> `Val`
    /// - `[>0px]`/`[>0px 0px]`/`[>0px 0px 0px]`/`[>0px 0px 0px 0px]` -> UiRect
    #[proc_macro]
    pub fn code (stream:CompilerTokens) -> CompilerTokens {
        code_syntax(stream.into()).into()
    }


// EOF \\
