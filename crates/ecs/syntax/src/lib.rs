use deki::*;

// \\

    pub enum WorldEntry {
        EntityCommands{
            entry: TokenStream,
        },
        Commands{
            entry: TokenStream,
            entity: Option<TokenStream>,
        },
        DeferredWorld{
            entry: TokenStream,
            entity: Option<TokenStream>,
        },
        World{
            entry: TokenStream,
            entity: Option<TokenStream>,
        },        
        EntityWorldMut{
            entry: TokenStream,
        },
        ChildBuilder{
            entry: TokenStream,
        },        
    }

    impl WorldEntry {

        /// (.., Component-Redirect?)
        pub fn from_tokens(streams:&Option<Vec<TokenStream>>) -> Self {
            if streams.is_none(){
                return Self::Commands{entry:qt!(world),entity:None};
            }
            let streams = streams.as_ref().unwrap();
            let entity = streams.get(1).map(|a|if a.is_empty(){qt!{me}} else {a.clone()});
            let out = match streams.get(0) {
                None => Self::EntityCommands{entry:qt!{this}},
                Some(a) if a.is_empty() => Self::Commands{entry:qt!{world},entity},
                Some(a) => {
                    let mut a_iter = a.clone().peek_iter();
                    let first = a_iter.peek().cloned().unwrap();
                    match first {
                        TokenTree::Punct(pnc) => {
                            a_iter.next();
                            let mut entry = TokenStream::from_iter(a_iter);
                            if entry.is_empty() {entry = qt!{world};};
                            match pnc.as_char() {
                                '*' => Self::EntityCommands{entry},
                                '-' => Self::DeferredWorld{entry,entity},
                                '+' => {
                                    let mut a_iter = entry.peek_iter();
                                    match a_iter.peek_punct() {
                                        '*' => {
                                            a_iter.next();
                                            let mut entry = TokenStream::from_iter(a_iter);
                                            if entry.is_empty() {entry = qt!{world};};
                                            Self::EntityWorldMut{entry}
                                        }
                                        _ => Self::World{entry:TokenStream::from_iter(a_iter),entity},
                                    }
                                }
                                '^' => Self::ChildBuilder{entry},
                                _   => Self::Commands{entry,entity}
                            }
                        }
                        _ => Self::Commands{entry:a.clone(),entity}
                    }
                }
            };
            out
        }

        pub fn get_entity(&self) -> Option<TokenStream> {match self {
            Self::EntityCommands{entry} 
            |  Self::EntityWorldMut{entry}
            => Some(qt!{#entry.id()}),
            Self::Commands{entry:_,entity} 
            | Self::DeferredWorld {entry:_,entity}
            | Self::World {entry:_,entity} 
            => entity.clone(),
            Self::ChildBuilder{entry} => Some({
                #[cfg(feature="0.16-rc")]
                qt!{#entry.target_entity()}
                #[cfg(feature="0.15")]
                qt!{#entry.parent_entity()}
                #[cfg(not(feature="0.16-rc"))]
                #[cfg(not(feature="0.15"))]
                compile_error_no_version()
            }),
        }}

        pub fn has_entity(&self) -> bool {self.get_entity().is_some()}

        pub fn init_entity(&self) -> TokenStream {
            self.get_entity().map(|e|qt!{let me = #e;})
                .unwrap_or_else(||{
                    let entry = self.get_entry();
                    qt!(let me = #entry.spawn_empty().id();)
                })
        }

        pub fn get_entry(&self) -> TokenStream {match self {
            Self::EntityCommands { entry } 
            | Self::DeferredWorld { entry, entity:_ } 
            => qt!{#entry.commands()},
            Self::Commands { entry, entity:_ } 
            | Self::World { entry, entity:_ } 
            => qt!{#entry},
            Self::EntityWorldMut { entry }
            => qt!{unsafe{#entry.world_mut()}},
            Self::ChildBuilder{entry} => {
                #[cfg(feature="0.16-rc")]
                qt!{#entry.commands_mut()}
                #[cfg(feature="0.15")]
                qt!{#entry}
                #[cfg(not(feature="0.16-rc"))]
                #[cfg(not(feature="0.15"))]
                compile_error_no_version()
            }
        }}

        pub fn init_entry(&self) -> TokenStream{ match self {
            Self::EntityCommands { entry } 
            | Self::DeferredWorld { entry, entity:_ } 
            => qt!{let mut world = #entry.commands();},
            Self::EntityWorldMut { entry }
            => qt!{let world = unsafe{#entry.world_mut()};},
            Self::ChildBuilder { entry } => {
                #[cfg(feature="0.16-rc")]
                qt!{let mut world = #entry.commands_mut();}
                #[cfg(feature="0.15")]
                qt!{}
                #[cfg(not(feature="0.16-rc"))]
                #[cfg(not(feature="0.15"))]
                compile_error_no_version()
            },
             _ => qt!{} 
        }}

        pub fn use_entry(&self) -> TokenStream{ match self {
            Self::EntityCommands { entry:_ } 
            | Self::DeferredWorld { entry:_, entity:_ } 
            | Self::EntityWorldMut { entry:_ } 
            => qt!{world},
            Self::ChildBuilder { entry:_ } => {
                #[cfg(feature="0.16-rc")]
                qt!{world}
                #[cfg(feature="0.15")]
                {self.get_entry()}
                #[cfg(not(feature="0.16-rc"))]
                #[cfg(not(feature="0.15"))]
                compile_error_no_version()
            },
            _ => self.get_entry() 
        }}

        pub fn init(&self) -> TokenStream {
            let mut out = self.init_entity();
            out.extend(self.init_entry());
            out
        }

        pub fn world_wrap(&self,inner:TokenStream) -> TokenStream {match self {
            Self::Commands{entry:_,entity:_}
            | Self::EntityCommands{entry:_} 
            | Self::DeferredWorld {entry:_,entity:_} => { 
                let world = self.use_entry();
                qt!{#world.queue(move|world:&mut World|{#inner});}
            },
            Self::World{entry:_,entity:_} | Self::EntityWorldMut{entry:_} => qt!{
                #inner
            },
            Self::ChildBuilder{entry:_} => {
                #[allow(unused_variables)]
                let world = self.use_entry();
                #[cfg(feature="0.16-rc")]
                qt!{#world.queue(move|world:&mut World|{#inner});}
                #[cfg(feature="0.15")]
                qt!{#world.enqueue_command(move|world:&mut World|{#inner});}
                #[cfg(not(feature="0.16-rc"))]
                #[cfg(not(feature="0.15"))]
                compile_error_no_version()
            },
        }}

    }

    /// (.., Component-Redirect?)
    pub fn check_angled(iter:&mut PeekIter) -> Option<Vec<TokenStream>> {
        exit!{if iter.peek_punct() != '<'}
        iter.next();
        let mut streams = vec![qt!{}];
        for token in iter {
            hold!{if token.is_punct('>')}
            if token.is_punct('|') {
                streams.push(qt!{});
                continue
            }
            streams.last_mut().unwrap().extend([token]);
        }
        exit!{>if (streams.len()==1 && streams[0].is_empty()) Some(vec![])}
        Some(streams)
    }

//\\

    fn peek_split_punct_once(mut iter:PeekIter,punct:char) -> [TokenStream;2] {
        let mut out = [qt!{},qt!{}];
        let mut i = 0;
        while let Some(token) = iter.next() {
            if token.is_punct(punct){
                match iter.peek_punct() != punct {
                    true => if i == 0 { 
                        i += 1; 
                        continue
                    },
                    false => if let Some(t) = iter.next() {
                        out[i.min(1)].extend(match punct {
                            ':' => qt!{::},
                            _   => qt!(#t #t)
                        });
                        continue
                    }
                }
            }
            out[i.min(1)].extend(token.into_token_stream());
        }
        out
    }

    ///  bool: is forced?
    fn world_entity_init(vec:Option<&TokenStream>) -> Option<(TokenStream,bool)> {
        exit!{vec = vec}
        let mut iter = vec.clone().peek_iter();
        exit!{first = iter.next()}
        exit!{*TokenTree::Punct(punct) = first}
        match punct.as_char() {

            '@' => {
                let punct = iter.peek_punct();
                if punct == '!' || punct == '*' {iter.next();}
                let [typi,path] = peek_split_punct_once(iter,'.');
                match punct {

                    '!' => Some((qt!{let me = world.resource::<#typi>().#path;},true)),

                    '*' => Some((qt!{
                        let Some(data) = world.get_resource::<#typi>() else {return};
                        for me in data .#path.collect::<Vec<_>>()
                    },false)),

                    'n' => Some((qt!{
                        let Some(data) = world.get_resource::<#typi>() else {return};
                        #[allow(for_loops_over_fallibles)]
                        for me in data. #path
                    },false)),

                    _ => None
                }
            }

            '#' => {
                let punct = iter.peek_punct();
                if punct == '!' || punct == '*' {iter.next();}
                let [typi,path] = peek_split_punct_once(iter,'.');
                match punct {

                    '!' => Some((match path.is_empty(){
                        true => qt!{
                            let mut query = world.query_filtered::<Entity,With<#typi>>();
                            let me = query.single(world).unwrap();
                        },
                        false => qt!{
                            let mut query = world.query::<&#typi>();
                            let me = query.single(world).unwrap().#path;
                        }
                    },true)),

                    '*' => Some((qt!{
                        let mut query = world.query::<&#typi>();
                        for me in query.iter(world)
                            .map(|data|data .#path.collect::<Vec<_>>())
                            .flatten().collect::<Vec<_>>()
                    },false)),

                    'n' => Some((match path.is_empty(){
                        true => qt!{
                            let mut query = world.query_filtered::<Entity,With<#typi>>();
                            for me in query.iter(world).collect::<Vec<_>>()
                        },
                        false => qt!{
                            let mut query = world.query::<&#typi>();
                            for me in query.iter(world).filter_map(|data|data .#path).collect::<Vec<_>>()
                        }
                    },false)),

                    _ => None
                }
            }

            _   => None
        }
    }

    /// (_,is_forced?)
    fn query_to_redirect(query:TokenStream) -> TokenStream {
        let mut vec = query.into_iter().collect::<Vec<_>>();
        let mut post = qt!{.collect::<Vec<_>>()};
        if let Some(a) = vec.last(){if a.is_punct('!'){
            vec.pop();
            post = qt!{};
        }}

        let iter = TokenStream::from_iter(vec).peek_iter();
        let [typi,path] = peek_split_punct_once(iter,'.');
        qt!{
            let Some(data) = world.get::<#typi>(me) else {continue};
            #[allow(for_loops_over_fallibles)]
            for me in data. #path #post
        }
    }

// World Spawning \\

    pub fn world_spawn_syntax(stream:TokenStream) -> TokenStream {
        let mut idx = 0;
        let mut spawn = qt!();
        let mut parenting = qt!();
        let mut mutato = vec![];

        let mut iter = stream.peek_iter();
        let angled = check_angled(&mut iter);

        let mut chain = vec![];
        while let Some(mut query) = check_angled(&mut iter) {
            chain.push(query_to_redirect(query.swap_remove(0)));
        }
        chain.reverse();

        // NOTE: Mind the clone, try to dodge?
        let mut vec = iter.collect::<Vec<_>>();
        let [leak,retu] = vec.last().map(|t|
            [t.is_punct('>'),t.is_punct('<')]
        ).unwrap_or_default();
        if leak || retu {vec.pop();}

        let stream = TokenStream::from_iter(vec);

        let mut is_forced = true;
        let mut on_current = qt!{};
        let mut on_world = qt!{};
        let mut on_world_block = qt!{};

        let wentry = WorldEntry::from_tokens(&angled);
        let mut exec_on_world = match wentry {
            WorldEntry::World{entry:_,entity:_}
            | WorldEntry::ChildBuilder{entry:_} 
            | WorldEntry::EntityWorldMut { entry:_ }
            => true,
            _ => false
        };

        if let Some((winit,forced)) = world_entity_init(angled.as_ref().map(|a|a.get(1)).flatten()) {
            on_world.extend(winit);
            is_forced = forced;
            exec_on_world = true;
        } else {
            on_current.extend(wentry.init_entity());
            on_world.extend(qt!{
                #[allow(for_loops_over_fallibles)]
                for _ in Some(())
            });
        }

        if !chain.is_empty() {
            is_forced = false;
            exec_on_world = true;
        }

        let spawn_on_world = !is_forced && exec_on_world;
        on_current.extend(wentry.init_entry());

        world_spawn_syntax_recursive(
            stream,Span::call_site(),Some("me".ident()),vec![],
            &mut idx,&mut spawn,&mut parenting,&mut mutato,
            if spawn_on_world {qt!{world}} else {wentry.use_entry()},
            if exec_on_world {qt!{world}} else {wentry.use_entry()},
            if exec_on_world {qt!{entity_mut}} else {qt!{entity}}
        );

        if spawn_on_world {on_world_block.extend(spawn)} 
        else {on_current.extend(spawn)}

        if chain.is_empty() {
            on_world_block.extend(qt!(#parenting #(#mutato)*));
        } else {
            let mut chain = chain.into_iter();
            let header = chain.next().unwrap();
            let mut block = qt!{#header{ #parenting #(#mutato)* }};
            for header in chain {
                block = qt!{#header{#block}};
            }
            on_world_block.extend(block);
        }

        if exec_on_world{
            on_world.extend(qt!{{#on_world_block}});
            let wrap = wentry.world_wrap(on_world);
            on_current.extend(wrap);
        } else {
            on_current.extend(on_world_block);
        }

        match [leak,retu] {
            [true, _] => on_current,
            [_, true] => qt!{{#on_current me}},
            [ _ , _ ] => qt!{{#on_current}}
        }

    }

// Recursion \\

    fn world_spawn_syntax_recursive(
        stream: TokenStream,
        span: Span,
        custom_name: Option<Ident>,
        mut ancestors: Vec<Ident>,
        idx: &mut usize,
        spawn: &mut TokenStream,
        parenting: &mut TokenStream,
        mutato: &mut Vec<TokenStream>,
        spawn_token: TokenStream,
        world_token: TokenStream,
        entity_mut: TokenStream
    ){
        let iter = stream.peek_iter();
        let split = iter.split_punct(';').into_iter();

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
 
        if let Some(parent) = ancestors.last() {
            #[cfg(feature="0.16-rc")]
            parenting.extend(qt!{
                #world_token.#entity_mut(#name_tmp).insert(ChildOf(#parent));
            });
            #[cfg(feature="0.15")]
            parenting.extend(qt!{
                #world_token.#entity_mut(#name_tmp).set_parent(#parent);
            });
            #[cfg(not(feature="0.16-rc"))]
            #[cfg(not(feature="0.15"))]
            parenting.extend(compile_error_no_version());

            spawn.extend(qt!(
                let mut #name = #spawn_token.spawn_empty().id();
            ))
        }

       *idx += 1;
        ancestors.push(name_tmp);

        // 
        let mut commands = qt!();
        let mut group_name = None;
        for row in split {
            let mut iter = row.peek_iter();
            next!{first = iter.peek()}
            use mevy_core_syntax::code as mecode;
            match first {

                TokenTree::Ident(ident) => {
                    next!{first = ident.to_string().chars().next()}
                    let [func,mut attr] = peek_split_punct_once(iter,':');

                    let tokens = if first.is_uppercase() {
                        mecode(if attr.is_empty() {func} else {qt!{<#func>::new(#attr)}})
                    } else if !attr.is_empty() {
                        let is_macro = func.clone().into_iter().last().map(|p|p.is_punct('!')).unwrap_or_default();
                        if !is_macro {attr = mecode(attr)}
                        qt!{#func(#attr)}
                    } else {
                        let mut iter = func.peek_iter();
                        let ident = iter.next().unwrap();
                        let is_macro = first.is_lowercase() && iter.peek_punct() == '!';
                        let mut tokens = TokenStream::from_iter(iter);
                        if !is_macro { tokens = mecode(tokens); }
                        qt!{#ident #tokens}
                    };

                    commands.extend(qt!(this.insert(#tokens);));
                }

                TokenTree::Group(g) if g.delimiter().is_parenthesis() => {
                    let mut tokens = TokenStream::from_iter(iter);
                    tokens = mecode(tokens);
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
                        let world_token = world_token.clone();
                        mutato.push(qt!(
                            #ancestors_tokens
                            let mut this = #world_token.#entity_mut(#name);
                            #commands
                        ));
                        commands = qt!();
                    }

                    world_spawn_syntax_recursive(
                        group.stream(), group.span_open(), group_name.take(),
                        ancestors.clone(), idx, spawn, parenting, mutato, 
                        spawn_token.clone(), world_token.clone(), entity_mut.clone()
                    );
                }

                TokenTree::Punct(p) if p.as_char() == '.' => {
                    let [func,attr] = peek_split_punct_once(iter,':');
                    let tokens = if !attr.is_empty() {qt!{#func(#attr)}} else {func};
                    let tokens = mevy_core_syntax::code(tokens);
                    commands.extend(qt!(this #tokens;));
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
                            
                            #[cfg(feature="0.16-rc")]
                            let trigger_entity = qt!{trigger.target()};
                            #[cfg(feature="0.15")]
                            let trigger_entity = qt!{trigger.entity()};
                            #[cfg(not(feature="0.15"))]
                            #[cfg(not(feature="0.16-rc"))]
                            let trigger_entity = compile_error_no_version();

                            commands.extend(match span_world  { 
                                None => {
                                    let this = "this".ident_span(span_entity);
                                    qt!(this.observe(move|#trigger:Trigger<#(#event)*>,mut world: Commands|{
                                        #[allow(unused_variables)]
                                        let mut #this = world.entity(#trigger_entity);
                                        #[allow(unused_variables)]
                                        let #let_event = trigger.event();
                                        #group
                                    });)
                                },
                                Some(span_world) => {
                                    let entity = "entity".ident_span(span_entity);
                                    let world = "world".ident_span(span_world);
                                    qt!(this.observe(move|#trigger:Trigger<#(#event)*>,mut world: Commands|{
                                        #[allow(unused_variables)]
                                        let #entity = #trigger_entity;
                                        #[allow(unused_variables)]
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
            let entity_mut = entity_mut.clone();
            let world_token = world_token.clone();
            mutato.push(qt!(
                #ancestors_tokens
                let mut this = #world_token.#entity_mut(#name);
                #commands
            ));
        }
    }


    fn compile_error_no_version() -> TokenStream {
        qt!{compile_error!{"Mevy: Missing bevy version!: Specify it in Cargo.toml! e.g. feature=[\"0.15\"])"}}
    }

// EOF \\
