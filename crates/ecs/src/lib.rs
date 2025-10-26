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


// Experimental \\
// Kinda My Sandbox!

// Resource \|

    /// Get Resource
    /// - required: mutable `world: World|DeferredWorld`
    /// - get: `let time = gere![Time].unwrap();`
    /// - get_mut: `let mut time = gere![mut Time].unwrap();`
    #[cfg(feature="experimental")]
    #[proc_macro]
    pub fn gere (item:CompilerTokens) -> CompilerTokens {
        let stream: TokenStream = item.into();
        let mut stream = stream.peek_iter();
        let get = stream.next_if(|a|a.is_string("mut"))
                        .map(|_|qt![get_resource_mut])
                        .unwrap_or(qt![get_resource]);
        let mut res = stream.collect::<Vec<_>>();
        let mut wrap = qt![];
        if res.last().unwrap().is_punct('!'){
            res.pop();
            wrap = qt![.unwrap()];
        }
        qt!{world.#get::<#(#res)*>() #wrap}.into()
    }

    /// Quickly Edit Resource (if available)
    /// - required: mutable `world: World|DeferredWorld`
    /// - usage: `gere![Struct.field = 100];`
    #[cfg(feature="experimental")]
    #[proc_macro]
    pub fn edre (item:CompilerTokens) -> CompilerTokens {
        let stream: TokenStream = item.into();
        let mut stream = stream.peek_iter();
        let comp = stream.next().unwrap();
        let left = TokenStream::from_iter(stream);
        qt!{if let Some(mut data) = world.get_resource_mut::<#comp>() {
            data #left;
        }}.into()
    }


// Component \|

    /// Get Component
    /// - required: mutable `world: World|DeferredWorld`
    /// - required: `me: Entity`
    /// - get: `let time = geco![Time].unwrap();`
    /// - get_mut: `let mut time = geco![mut Time].unwrap();`
    /// - get_cloned: `let time = geco![Time*].unwrap();`
    /// - has_component?: `if geco![Time?] {}`
    #[cfg(feature="experimental")]
    #[proc_macro]
    pub fn geco (item:CompilerTokens) -> CompilerTokens {
        let stream: TokenStream = item.into();
        let mut stream = stream.as_vec();
        let last = match stream.last().cloned() {
            Some(TokenTree::Punct(p)) if p.as_char() != '>' => {
                stream.pop();
                p.as_char()
            }
            _ => '-'
        };
        let post = match last {
            '?' => qt![.is_some()],
            '*' => qt![.cloned()],
            _ => qt![]
        };
        let get = match stream.get(0) {
            Some(t) if t.is_string("mut") => {
                stream.remove(0);
                qt![get_mut]
            }
            _ => qt![get]
        };
        let enty = match stream.last().cloned() {
            Some(TokenTree::Group(g)) => {
                stream.pop();
                g.stream()
            },
            _ => qt![me]
        };
        let comp = TokenStream::from_iter(stream.into_iter()); 
        qt!{world.#get::<#comp>(#enty)#post}.into()
    }

    /// Quickly Edit Components (if available)
    /// - required: mutable `world: World|DeferredWorld`
    /// - usage: `geco![Struct.field = 100];`
    #[cfg(feature="experimental")]
    #[proc_macro]
    pub fn edco (item:CompilerTokens) -> CompilerTokens {
        let stream: TokenStream = item.into();
        let mut stream = stream.peek_iter();
        let is_deref = stream.next_if(|a|a.is_punct('*')).is_some();
        let comp = stream.next().unwrap();
        let rest = TokenStream::from_iter(stream);
        let data = if is_deref {qt![*data]} else {qt![data]};
        qt!{if let Some(mut data) = world.get_mut::<#comp>(me) {
            #data #rest;
        }}.into()
    }


// EOF \\
