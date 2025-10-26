use deki::*;
use std::iter::zip;
use syn::LitFloat;


// Neat Ui Block \\

    /// simple in-code replacement of:
    /// - `#hexcode` => bevy color
    /// - `0px`, `3%` ... => bevy ui Vals
    /// - `[>0px 2px]` css-like notation => bevy UiRect
    pub fn code (stream:TokenStream) -> TokenStream {
        code_helper(stream,false)
    }

    fn code_helper(stream:TokenStream,in_group:bool) -> TokenStream {
        let mut list = stream.peek_iter();
        let mut out = qt!{};
        loop{match code_next(&mut list,in_group) {
            Step::Shift(stream) => out.extend(stream),
            Step::Base(tree) => match tree {
                TokenTree::Group(group) => out.extend([
                    TokenTree::Group(Group::new(group.delimiter(),code_helper(group.stream(),true)))
                ]),
                _ => out.extend([tree]),
            }
            _ => break
        }}
        out
    }

    fn code_next (iter:&mut PeekIter,in_group:bool) -> Step<TokenTree,TokenStream> {
        exit!{next = iter.next()}
        match next.try_val_variant(true) {
            Check::Some(stream) => return Step::Shift(qt!{Val::#stream}),
            Check::Maybe(num) => if let Some(stream) = iter.seek_val_variant(num) {
                let stream = stream.with_span(next.span());
                return Step::Shift(qt!{Val::#stream});
            } _ => {}
        }

        if next.is_punct('!') && iter.peek().nay() {
            let default = "default".ident_span(next.span());
            return Step::Shift(match in_group {
                true => qt!{..Default::#default()},
                false => qt!{::#default()}
            });
        }

        if next.is_punct('!') && iter.peek_punct() == ';' {
            let default = "default".ident_span(next.span());
            return Step::Shift(qt!{::#default()});
        }

        if next.is_punct('@'){
            let auto = "Auto".ident_span(next.span());
            return Step::Shift(qt!{Val::#auto});
        }

        if let TokenTree::Group(group) = &next {
            let mut iter = group.stream().peek_iter();
            if iter.next_if(|a|a.is_punct('>')).yay() {        
                let [rect,_] = iter.seek_rect_like();
                match rect {
                    Step::Shift([t,r,b,l]) => {
                        sandwich!{
                            let ^0 = stringify!{^0}.ident_span(l.span());
                            #left #right #top #bottom
                        }
                        let tok = qt!{UiRect{
                            #top:    Val::#t,
                            #right:  Val::#r,
                            #bottom: Val::#b,
                            #left:   Val::#l,
                        }};
                        return Step::Shift(tok);
                    } _ => {}
                }
            }
        } 

        if next.is_punct('#') {
            if let Some(out) = iter.next_hex_color() {
                return Step::Shift(out.0);
            }
        }

        Step::Base(next)
    }


// Peek Iter \\

    #[ext(pub trait UiPeekIter)]
    impl PeekIter {

        fn seek_val_variant(&mut self,num:f32) -> Option<TokenStream> {
            exit!{if self.peek_punct() != '%'}
            self.next();
            Some(qt!{Percent(#num)})
        }

        /// get hex color by next token, custom error or none if no token is available
        fn next_hex_color(&mut self) -> Option<(TokenStream,Span)> {
            if let Some(out) = self.seek_hex_color() {
                Some(out)
            } 
            else {
                exit!{hexy = self.next()}
                Some((qt!{compile_error!{"Invalid hex string!"}}.with_span(hexy.span()),hexy.span()))
            }
        }

        fn seek_hex_color(&mut self) -> Option<(TokenStream,Span)> {
            exit!{hex = self.peek().try_hex_color()}
            let tree = self.next().unwrap();
            let hex = format!("#{hex}");
            let hex = qt!{#hex}.with_span(tree.span());
            let fnc = "hex".ident_span(tree.span());
            Some((qt!{Color::Srgba(Srgba::#fnc(#hex).unwrap())},tree.span()))
        }

        /// get a rect by valid upcoming tokens - or a default one
        fn seek_rect_like(&mut self) -> [Step<(Option<Punct>,Literal),[TokenStream;4]>;2] {
            let mut rect = vec![];
            let mut last = Step::None;
            for _ in 0..4 { match self.next_valvar(){
                Step::Shift(v) => {rect.push(v);},
                Step::Base(b) => {
                    last = Step::Base(b); 
                    break;
                },
                Step::None => hold!{if self.peek().nay()}
            }}
            match rect.len() {               
                0 => [last,Step::None],
                _ => [Step::Shift(rect.into_rect_like(false,qt!{},|v|v.with_span(Span::call_site()))),last],
            }
        }

        /// # Returns
        /// - None: not a Val::_: Iterator hasn't progressed
        /// - Base: not a Val::_, but a number lit: Iterator has progressed
        /// - Shift: a Val::_/
        fn next_valvar(&mut self) -> Step<(Option<Punct>,Literal),TokenStream> {
            let (sign,punct) = match self.peek_punct() {
                '-' => (false,self.next().map(|t|t.unwrap_punct())),
                '+' => {self.next();(true,None)},
                _ => (true,None)
            };
            match self.peek().try_val_variant(sign) {
                Check::Maybe(m) => {
                    let base = self.next();
                    match self.seek_val_variant(m){
                        None => Step::Base((punct,base.unwrap().unwrap_literal())),
                        Some(v) => Step::Shift(v.with_span(base.span()))
                    }
                }
                Check::Some(v) => {self.next();Step::Shift(v)}
                Check::None => if self.peek_punct() == '@' {
                    let span = self.next().unwrap().span();
                    Step::Shift(qt!{Auto}.with_span(span))
                } 
                else {Step::None}
            }
        }
    }


// Token Handling \\

    fn hex_check(text:&str) -> bool {
        text.to_lowercase().chars().filter(|a|match a {
            '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|
            '8'|'9'|'a'|'b'|'c'|'d'|'e'|'f' => false,
            _ => true
        }).next().is_none()
    }

    #[ext(pub trait UiTokenTree)]
    impl TokenTree {
        fn try_val_variant(&self,sign:bool) -> Check<TokenStream,f32> {
            exit!{*TokenTree::Literal(lit) = self}
            lit.try_val_variant(sign)
        }
        fn try_hex_color(&self) -> Option<String> {
            let mut t = self.to_string();
            match t.len() {
                6|8 => {}
                5 => {
                    let vec = t.chars().collect::<Vec<_>>();
                    let [r,g,b,a0,a1] = vec.try_into().unwrap();
                    t = format!["{r}{r}{g}{g}{b}{b}{a0}{a1}"];
                }
                3|4 => t = zip(t.chars(),t.chars())
                    .map(|(a,b)|String::from_iter([a,b]))
                    .collect(),
                _ => exit!{},
            }
            hex_check(&t).then_some(t)
        }
    }

    impl UiTokenTree for Option<&TokenTree> {sandwich!{
        fn ^0 ^1 {
            exit!{tree = self}
            tree.^0 ^2 
        }
        #try_val_variant^(&self,sign:bool) -> Check<TokenStream,f32>^(sign)
        #try_hex_color^(&self) -> Option<String>^()
    }}

    #[ext(pub trait UiLiteral)]
    impl Literal {
        fn try_val_variant(&self,sign:bool) -> Check<TokenStream,f32> {
            exit!{if !self.is_numeric()}
            let lit: LitFloat = self.clone().into();
            kill!{num = lit.base10_parse::<f32>()}
            let num = if sign {num} else {-num};
            let val = match lit.suffix() {
                "px" => "Px",
                "vw" => "Vw",
                "vh" => "Vh",
                "vmin" => "VMin",
                "vmax" => "VMax",
                "" => return Check::Maybe(num),
                _ => return Check::None
            };
            let val = val.ident();
            Check::Some(qt!{#val(#num)}.with_span(lit.span()))
        }
    }

    #[ext(pub trait UiVec)]
    impl <T:Clone> Vec<T> {
        
        /// orders, removes & clones entries to fit a css-like rect
        /// - corner_align: use corner order logic (e.g. like css border-radius)
        /// - output: [top, right, bottom, left]
        fn into_rect_like(
            mut self,
            corner_align: bool,
            default: T,
            cloned_edit: fn(T)->T
        ) -> [T;4] {
            match self.len() {
                0 => [default.clone(),default.clone(),default.clone(),default],
                1 => {
                    let a = self.pop().unwrap();
                    let a0 = cloned_edit(a.clone());
                    [a0.clone(),a0.clone(),a0,a]
                }
                2 => unsafe {
                    let [v,h] = self.try_into().unwrap_unchecked();
                    let v0 = cloned_edit(v.clone());
                    let h0 = cloned_edit(h.clone());
                    if corner_align {[v0,v,h0,h]} else {[v0,h0,v,h]}
                }
                3 => unsafe {
                    let [t,h,b] = self.try_into().unwrap_unchecked();
                    if corner_align {
                        let b0 = cloned_edit(b.clone());
                        [t,h,b0,b]
                    } 
                    else {
                        let h0 = cloned_edit(h.clone());
                        [t,h0,b,h]
                    }
                }
                _ => unsafe {
                    self.set_len(4);
                    self.try_into().unwrap_unchecked()
                }
            }
        }
    }


// EOF \\
