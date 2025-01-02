use deki::*;
use mevy_ui_syntax::*;
use proc_macro::TokenStream as CompilerTokens;

// Macro \\

    /// One macro to rule them all:
    /// - `ui!{( width: 1px; )}`: css syntax => tuple of mentioned UI components
    /// - `ui!{{ let v = 1px; }}`: (soon) edit by css syntax
    /// - ..more to come
    #[proc_macro]
    pub fn ui (tok:CompilerTokens) -> CompilerTokens {
        let tok: TokenStream = tok.into();
        let mut iter = tok.peek_iter();
    
        match iter.peek().unwrap() {
            TokenTree::Group(_) => {
                kill!{*Some(TokenTree::Group(group)) = iter.next()}
                match group.delimiter() {
                    Delimiter::Parenthesis => bundle(group.stream().peek_iter(),iter.next()),
                    _ => todo!{}
                }
            }
            _ => todo!{}
        }.into()
    }


// Component Getter \\

    fn bundle (iter:PeekIter,after:Option<TokenTree>) -> TokenStream {
        let after = after.map(|a|a.span()).unwrap_or(Span::call_site());
        let mut bundle = StackMap::<String,()>::new();
        let mut defaults = StackMap::<String,()>::new();
        let mut assign = qt!{};

        for stream in iter.split_punct(';') {
            let mut iter = stream.peek_iter();
            let field = iter.next().unwrap().risk_ident();
            let yuim = ui_style_sheet(field.clone().into(),&mut iter);

            if yuim.is_empty() {
                let varnm = field.to_string().chars()
                    .map(|c|if c.is_alphanumeric() {c} else {'_'}) 
                    .collect::<String>().to_case(Case::Snake).ident();
                let is_class = field.to_string().chars().next().unwrap().is_uppercase();
                let attr = code(TokenStream::from_iter(iter));
                let func = match is_class {
                    true => qt!{#field::new},
                    false => qt!{#field}
                };
                assign.extend(qt!{let #varnm = #func(#attr);});
                bundle.entry(varnm.to_string());
                continue
            }

            for (key,yuis) in yuim.into_iter() {
                let var = key.to_case(Case::Snake).ident();
                defaults.entry(key.to_string());
                bundle.entry(key.to_case(Case::Snake));
                for UiEntry { typ:_, fields, value, extra:_ } in yuis {
                    assign.extend(qt!{#var #fields = #value;});
                }
            }
        }

        let defaults = TokenStream::from_iter(defaults.keys.iter().map(|s|{
            let (var,typ) = (s.to_case(Case::Snake).ident(),s.ident());
            qt!{let mut #var = #typ::default();}
        }));
        let bundle = bundle.keys.iter().map(|s|s.ident());
        let out = "bundle".ident_span(after);

        qt!{{
            #defaults
            #assign
            let #out = (#(#bundle),*);
            bundle
        }}
    }


// EOF \\
