use deki::*;

// Spawning \\

    pub fn spawn_syntax(stream:TokenStream) -> TokenStream {
        let mut idx = 0;
        let mut spawn = qt!();
        let mut mutato = qt!();
        spawn_syntax_recursive(stream,Span::call_site(),None,None,&mut idx,&mut spawn,&mut mutato);
        let out = qt!{#spawn #mutato};
        out
    }

    fn spawn_syntax_recursive(
        stream: TokenStream,
        span: Span,
        custom_name: Option<Ident>,
        parent: Option<Ident>,
        idx: &mut usize,
        spawn: &mut TokenStream,
        mutato: &mut TokenStream
    ){
        // handle naming & hierarchy
        let name = custom_name.unwrap_or(format!("e{idx}").ident_span(span));
        let name_tmp = name.to_string().ident();
        spawn.extend(qt!(let mut #name_tmp = world.spawn_empty();));
        if let Some(parent) = parent {spawn.extend(qt!(
            #name_tmp.set_parent(#parent);        
        ))}
        spawn.extend(qt!(let #name = #name_tmp.id();));
        *idx += 1;

        // 
        let mut components = qt!();
        let mut commands = qt!();
        let mut group_name = None;
        for row in stream.peek_iter().split_punct(';'){
            let mut iter = row.peek_iter();
            next!{first = iter.peek()}
            match first {

                TokenTree::Ident(ident) => {
                    let first = ident.to_string();
                    let mut tokens = TokenStream::from_iter(iter);
                    if first.as_str() != "ui" {
                        tokens = mevy_core_syntax::code(tokens);
                    }
                    components.extend(qt!(#tokens,));
                }

                TokenTree::Group(g) if g.delimiter().is_bracked() => for group in iter {
                    next!{*TokenTree::Group(group) = group}
                    let mut check = group.stream().into_iter();
                    if let (Some(TokenTree::Ident(n)),None) = (check.next(),check.next()) {
                        group_name = Some(n);
                        continue
                    }
                    spawn_syntax_recursive(
                        group.stream(), group.span_open(), group_name.take(),
                        Some(name.clone()), idx, spawn, mutato
                    );
                }

                TokenTree::Punct(p) if p.as_char() == '.' => {
                    iter.next();
                    commands.extend(qt!(ecmd.#(#iter)*;));
                }

                _ => {}
            }
        }

        mutato.extend(qt!(
            let mut ecmd = world.entity(#name);
            ecmd.insert((#components));
            #commands
        ));
    }

// EOF \\
