use deki::*;

// Spawning \\

    pub fn spawn_syntax(stream:TokenStream) -> TokenStream {
        let mut idx = 0;
        let mut spawn = qt!();
        let mut mutato = vec![];
        spawn_syntax_recursive(stream,Span::call_site(),None,vec![],&mut idx,&mut spawn,&mut mutato);
        let out = qt!{#spawn #(#mutato)*};
        out
    }

    fn spawn_syntax_recursive(
        stream: TokenStream,
        span: Span,
        custom_name: Option<Ident>,
        mut ancestors: Vec<Ident>,
        idx: &mut usize,
        spawn: &mut TokenStream,
        mutato: &mut Vec<TokenStream>
    ){
        let mut iter = stream.peek_iter();
        let e0_provided = iter.next_if(|t|t.is_punct('&')).yay();
        let this_provide = !e0_provided && iter.next_if(|t|t.is_punct('*')).yay();
        let mut split = iter.split_punct(';').into_iter();

       // prepare ancestors
        let ancestors_tokens = if ancestors.is_empty(){
            qt!{}
        } else {
            let ancestors_rev = ancestors.iter().rev();
            qt!{let ancestors = [#(#ancestors_rev),*];}
        };

        // handle naming & hierarchy
        let name = custom_name.unwrap_or(format!("e{idx}").ident_span(span));
        let name_tmp = name.to_string().ident();
 
        if *idx == 0 && this_provide {
            let e0 = split.next().unwrap();
            spawn.extend(qt!(
                let e0 = #e0.id();
                let mut world = #e0.commands();
            ));
        }
        else if 0 < *idx || !e0_provided {
            spawn.extend(qt!(let mut #name_tmp = world.spawn_empty();));
            if let Some(parent) = ancestors.last() {spawn.extend(qt!(
                #name_tmp.set_parent(#parent);        
            ))}
            spawn.extend(qt!(let #name = #name_tmp.id();));
        } else {
            let e0 = split.next().unwrap();
            spawn.extend(qt!(let e0 = #e0;));
        }

       *idx += 1;
        ancestors.push(name_tmp);

        // 
        let mut commands = qt!();
        let mut group_name = None;
        for row in split {
            let mut iter = row.peek_iter();
            next!{first = iter.peek()}
            match first {

                TokenTree::Ident(ident) => {
                    next!{first = ident.to_string().chars().next()}
                    let ident = iter.next().unwrap();
                    let is_macro = first.is_lowercase() && iter.peek_punct() == '!';
                    let mut tokens = TokenStream::from_iter(iter);
                    if !is_macro {
                        tokens = mevy_core_syntax::code(tokens);
                    }
                    commands.extend(qt!(this.insert(#ident #tokens);));
                }

                TokenTree::Group(g) if g.delimiter().is_parenthesis() => {
                    let mut tokens = TokenStream::from_iter(iter);
                    tokens = mevy_core_syntax::code(tokens);
                    commands.extend(qt!(this.insert(#tokens);));
                }

                TokenTree::Group(g) if g.delimiter().is_brace() => for group in iter {
                    next!{*TokenTree::Group(group) = group}
                    commands.extend(group.into_token_stream()); 
                }

                TokenTree::Group(g) if g.delimiter().is_bracked() => for group in iter {
                    next!{*TokenTree::Group(group) = group}
                    let mut check = group.stream().into_iter();
                    if let (Some(TokenTree::Ident(n)),None) = (check.next(),check.next()) {
                        group_name = Some(n);
                        continue
                    }

                    if !commands.is_empty() {
                        mutato.push(qt!(
                            #ancestors_tokens
                            let mut this = world.entity(#name);
                            #commands
                        ));
                        commands = qt!();
                    }

                    spawn_syntax_recursive(
                        group.stream(), group.span_open(), group_name.take(),
                        ancestors.clone(), idx, spawn, mutato
                    );
                }

                TokenTree::Punct(p) if p.as_char() == '.' => {
                    iter.next();
                    let mut tokens = TokenStream::from_iter(iter);
                    tokens = mevy_core_syntax::code(tokens);
                    commands.extend(qt!(this.#tokens;));
                }

                TokenTree::Punct(p) if p.as_char() == '>' => {
                    let span_entity = p.span();
                    iter.next();
                    let span_world = iter.next_if(|p|p.is_punct('>')).map(|p|p.span());
                    let mut event = iter.collect::<Vec<_>>();
                    next!{action = event.pop()}
                    match action {
                        TokenTree::Group(group) if group.delimiter().is_brace() => {
                            let trigger = "trigger".ident_span(span_entity);
                            let let_event = "event".ident_span(span_entity);
                            commands.extend(match span_world  { 
                                None => {
                                    let this = "this".ident_span(span_entity);
                                    qt!(this.observe(move|#trigger:Trigger<#(#event)*>,mut world: Commands|{
                                        let mut #this = world.entity(trigger.entity());
                                        let #let_event = trigger.event();
                                        #group
                                    });)
                                },
                                Some(span_world) => {
                                    let entity = "entity".ident_span(span_entity);
                                    let world = "world".ident_span(span_world);
                                    qt!(this.observe(move|#trigger:Trigger<#(#event)*>,mut world: Commands|{
                                        let #entity = trigger.entity();
                                        let #let_event = trigger.event().clone();
                                        world.queue(move|#world:&mut World|#group);
                                    });)
                                }
                            });
                        }
                        _ => {}
                    }
                }

                _ => {}
            }
        }

        if !commands.is_empty() {
            mutato.push(qt!(
                #ancestors_tokens
                let mut this = world.entity(#name);
                #commands
            ));
        }
    }

// EOF \\
