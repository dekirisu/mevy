use mevy_ecs_syntax::*;
use proc_macro::TokenStream as Cokens;
use TokenStream as Tokens;
use deki::*;

#[proc_macro]
pub fn entity(stream:Cokens) -> Cokens {
    world_spawn_syntax(stream.into()).into()
}

#[proc_macro]
pub fn modify(stream:Cokens) -> Cokens {
    let stream: Tokens = stream.into();
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
    pub fn gere (item:Cokens) -> Cokens {
        let stream: Tokens = item.into();
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
    pub fn edre (item:Cokens) -> Cokens {
        let stream: Tokens = item.into();
        let mut stream = stream.peek_iter();
        let comp = stream.next().unwrap();
        let left = Tokens::from_iter(stream);
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
    pub fn geco (item:Cokens) -> Cokens {
        let stream: Tokens = item.into();
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
        let comp = Tokens::from_iter(stream.into_iter()); 
        qt!{world.#get::<#comp>(#enty)#post}.into()
    }

    /// Quickly Edit Components (if available)
    /// - required: mutable `world: World|DeferredWorld`
    /// - usage: `geco![Struct.field = 100];`
    #[cfg(feature="experimental")]
    #[proc_macro]
    pub fn edco (item:Cokens) -> Cokens {
        let stream: Tokens = item.into();
        let mut stream = stream.peek_iter();
        let is_deref = stream.next_if(|a|a.is_punct('*')).is_some();
        let comp = stream.next().unwrap();
        let rest = Tokens::from_iter(stream);
        let data = if is_deref {qt![*data]} else {qt![data]};
        qt!{if let Some(mut data) = world.get_mut::<#comp>(me) {
            #data #rest;
        }}.into()
    }

// Spawners \|

    /// "D(eferred) En(tity)" 
    /// Alternative `entity!` for `world: DeferredWorld`
    /// - `den![..]`: spawn a `me: Entity`
    /// - `den![&..]`: edit a `me: Entity`
    /// - `den![*..]: edit a `world: EntityCommands`
    /// - `den![#Marker|..]: edit all Entities with `Marker` component
    #[proc_macro]
    pub fn den(stream:Cokens) -> Cokens {
        let stream: Tokens = stream.into();
        let mut iter = stream.peek_iter();
        let dir = en_translate(qt![-],&mut iter);
        let stream = Tokens::from_iter(iter);
        mevy_ecs_syntax::world_spawn_syntax(qt!(#dir #stream)).into()
    }

    /// "W(orld) En(tity)" 
    /// Alternative `entity!` for `world: World`
    /// - `wen![..]`: spawn a `me: Entity`
    /// - `wen![&..]`: edit a `me: Entity`
    /// - `wen![*..]: edit a `world: EntityWorldMut`
    /// - `wen![#Marker|..]: edit all Entities with `Marker` component
    #[proc_macro]
    pub fn wen(stream:Cokens) -> Cokens {
        let stream: Tokens = stream.into();
        let mut iter = stream.peek_iter();
        let dir = en_translate(qt![+],&mut iter);
        let stream = Tokens::from_iter(iter);
        mevy_ecs_syntax::world_spawn_syntax(qt!(#dir #stream)).into()
    }

    /// "C(ommand) En(tity)" 
    /// Alternative `entity!` for `world: Commands`
    /// - `cen![..]`: spawn a `me: Entity`
    /// - `cen![&..]`: edit a `me: Entity`
    /// - `cen![*..]: edit a `world: EntityWorldMut`
    /// - `cen![#Marker|..]: edit all Entities with `Marker` component
     #[proc_macro]
    pub fn cen(stream:Cokens) -> Cokens {
        let stream: Tokens = stream.into();
        let mut iter = stream.peek_iter();
        let dir = en_translate(qt![],&mut iter);
        let stream = Tokens::from_iter(iter);
        mevy_ecs_syntax::world_spawn_syntax(qt!(#dir #stream)).into()
    }


// Spawners Helper \|

    fn en_translate(left:Tokens,iter:&mut PeekIter) -> Tokens {match iter.peek_punct(){
        '#' => {
            let me = iter.collect_til_punct('|');
            iter.next();
            qt![<#left|#(#me)*>]
        }
        '*' => {
            iter.next();
            let me = if iter.peek_punct() != ':' {qt!()} else {
                iter.next();
                let this = iter.next().unwrap();
                qt!{#this}
            };
            qt!{<#left*#me>}
        }
        '&' => {
            iter.next();
            let me = if iter.peek_punct() != ':' {qt!()} else {
                iter.next();
                let this = iter.next().unwrap();
                qt!{#this}
            };
            qt!{<#left|#me>}
        }
        _ => if left.is_empty() {qt!{}} else {qt!{<#left>}}
    }}


// EOF \\
