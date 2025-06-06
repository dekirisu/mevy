pub use deki::*;
pub use mevy_core_syntax::*;
use std::{f32::consts::PI, iter::zip};
use syn::LitFloat;

// CSS -> Bundle \\

    pub fn bundle(iter:PeekIter,after:Option<TokenTree>) -> TokenStream {
        UiPrep::from_iter(iter).get_bundle(after)
    }

    #[derive(Default)]
    pub struct UiPrep {
        /// (variable,is_builtin)
        pub variables: StackMap<String,bool>,
        pub defaults: StackMap<String,()>,
        pub assign: TokenStream
    }

    impl UiPrep {
        pub fn from_stream(stream:TokenStream) -> Self {
            Self::from_iter(stream.peek_iter()) 
        }
        pub fn from_iter(iter:PeekIter) -> Self {
            let mut out = Self::default();
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
                    out.assign.extend(qt!{let #varnm = #func(#attr);});
                    *out.variables.entry(varnm.to_string()) = false;
                    continue
                }

                for (key,yuis) in yuim.into_iter() {
                    let var = key.to_case(Case::Snake).ident();
                    out.defaults.entry(key.to_string());
                    *out.variables.entry(key.to_case(Case::Snake)) = true;
                    for UiEntry { typ:_, fields, value, extra:_ } in yuis {
                        out.assign.extend(qt!{#var #fields = #value;});
                    }
                    next!{if key == "Node"}
                    let typ = key.ident_span(field.span());
                    out.assign.extend(qt!({type O = #typ;}));
                }
            }
            out
        }

        pub fn get_bundle(&self,after:Option<TokenTree>) -> TokenStream {
            let after = after.map(|a|a.span()).unwrap_or(Span::call_site());

            let defaults = TokenStream::from_iter(self.defaults.keys.iter().map(|s|{
                let (var,typ) = (s.to_case(Case::Snake).ident(),s.ident());
                match s.as_str() {
                    "BoxShadow" => {
                        #[cfg(feature="0.16")]
                        qt!{let mut #var = BoxShadow(vec![ShadowStyle::default()]);}
                        #[cfg(feature="0.15")]
                        qt!{let mut #var = #typ::default();}
                        #[cfg(not(feature="0.15"))]
                        #[cfg(not(feature="0.16"))]
                        compile_error_no_version()
                    }
                    _ => qt!{let mut #var = #typ::default();}
                }
            }));
            let bundle = self.variables.keys.iter().map(|s|s.ident());
            let out = "bundle".ident_span(after);
            let assign = self.assign.clone();

            qt!{{
                #defaults
                #assign
                let #out = (#(#bundle),*);
                bundle
            }} 
        }

        pub fn get_edits(&self) -> (Vec<(Ident,Ident)>,TokenStream) {
            let mut expected = vec![];
            let mut add = qt!();
            for (var,builtin) in self.variables.iter() {
                let var = var.ident();
                if *builtin {
                    let vart = var.to_case(Case::Pascal);
                    expected.push((var,vart));
                } 
                else {add.extend(qt!(ecmd.insert(#var);));}
            }
            if !add.is_empty() {
                expected.push(("ecmd".ident(),"EntityCommands".ident()));
            }
            let assign = self.assign.clone();

            (expected,qt!({
                #assign
                #add
            }))
        }

    }

// CSS-Like \\

    macro_rules! qar {($([$($tt:tt)*])*)=>{[$(qt!($($tt)*)),*]}}

    pub struct UiEntry {
        /// e.g: type of [Self::value]
        pub typ: Str,
        /// e.g: .width
        pub fields: TokenStream,
        // consider switching to Option<_> //<<
        /// e.g. Val::Px(300.)
        /// - can be empty, indicating the field shouldn't change
        pub value: TokenStream,
        /// any extra tokens passtru
        pub extra: Option<TokenStream>
    }

    type UiMap = StackMap<Str,Vec<UiEntry>>; 

    pub fn ui_style_sheet(field:TokenTree,iter:&mut PeekIter) -> UiMap {
        iter.skip_puncts("#-");
        let mut map = UiMap::new();

        macro_rules! out {
            ($main:ty => $sub:ty [$($ft:tt)*][$($vt:tt)*][$($extra:tt)*])=>{{
                let main = stringify!($main);
                let sub = stringify!($sub);
                out!{>main => sub [$($ft)*][$($vt)*][$($extra)*]}
            }};
            (>$main:tt => $sub:tt [$($ft:tt)*][$($vt:tt)*][$($extra:tt)*])=>{{
                map.entry($main).push(UiEntry{
                    typ: $sub,
                    fields: qt!{$($ft)*},
                    value: qt!{$($vt)*},
                    extra: $($extra)*
                });
            }}
        }

        match field.to_string().as_str() {

            "scale" => {
                let x = iter.next();
                let x_extra = iter.try_extra();
                let y = iter.next();
                let y_extra = iter.try_extra();
                exit!{x = x}
                out!{Transform => f32 [.scale.x][#x] [x_extra.clone()]}
                match y {
                    Some(y) => out!{Transform => f32 [.scale.y][#y] [y_extra]},
                    None => out!{Transform => f32 [.scale.y][#x] [x_extra]}
                }
            }

            "rotation" => {
                let iter = iter.map(|t|match t {
                    TokenTree::Literal(r) if r.is_numeric() => {
                        let r: LitFloat = r.into();
                        let val = r.base10_parse::<f32>().unwrap();
                        let radian = match r.suffix() {
                            "deg" => PI * val / 180.,
                            _ => val
                        };
                        TokenTree::Literal(Literal::f32_suffixed(radian))
                    }
                    _ => {t}
                });
                let mut vec = iter.collect::<Vec<_>>();
                let mut extra = None;
                if let Some(TokenTree::Group(grp)) = vec.last() {
                    if grp.delimiter().is_bracked(){
                        extra = Some(grp.stream()); 
                        vec.pop();
                    }
                }
                let token = TokenStream::from_iter(vec); 
                out!{Transform => Quat [.rotation][Quat::from_rotation_z(#token)] [extra]}
            }

            "color"|"font_color" => match iter.try_into_color().prepare() {
                Some((color,_,extra)) => out!{TextColor => Color [.0][#color] [extra]},
            _=>()}

            "font_size" => {
                kill!{val = iter.next()}
                out!{TextFont => f32 [.#field][#val as f32] [None]}
            }

            "background"|"background_color" => match iter.try_into_color().prepare() {
                Some((color,_,extra)) => out!{BackgroundColor => Color [.0][#color] [extra]},
            _=>()}

            "border_color" => match iter.try_into_color().prepare() {
                Some((color,_,extra)) => out!{BorderColor => Color [.0][#color] [extra]},
            _=>()}

            "border_radius" => {
                let vals = iter.into_rect_like(true);
                let fields = qar!([top_left][top_right][bottom_right][bottom_left]);
                for (field,oval) in zip(fields,vals) {
                    next!{val = oval.main}
                    let field = field.with_span(oval.span);
                    out!{BorderRadius => Val [.#field][#val] [oval.extra]}
                }
            }

            "outline" => {
                let fields = qar!([width][offset]);
                for (field,oval) in zip(fields,iter.into_vals()) {
                    next!{val = oval.main}
                    let field = field.with_span(oval.span);
                    out!{Outline => Val [.#field][#val] [oval.extra]}
                }
                if let Some((color,span,extra)) = iter.try_into_color().prepare() {
                    let field = "color".ident_span(span);
                    out!{Outline => Color [.#field][#color] [extra]}
                }
            }

            "box_shadow" => {
                let fields = qar!([x_offset][y_offset][blur_radius][spread_radius]);
                for (field,oval) in zip(fields,iter.into_vals()) {
                    next!{val = oval.main}
                    let field = field.with_span(oval.span);
                    #[cfg(feature="0.16")]
                    out!{BoxShadow => Val [[0].#field][#val] [oval.extra]}
                    #[cfg(feature="0.15")]
                    out!{BoxShadow => Val [.#field][#val] [oval.extra]}
                    #[cfg(not(feature="0.16"))]
                    #[cfg(not(feature="0.15"))]{
                        let err = compile_error_no_version();
                        out!{BoxShadow => Val [;][#err] [None]}
                    }
                }
                if let Some((color,span,extra)) = iter.try_into_color().prepare() {
                    let field = "color".ident_span(span);
                    #[cfg(feature="0.16")]
                    out!{BoxShadow => Color [[0].#field][#color] [extra]}
                    #[cfg(feature="0.15")]
                    out!{BoxShadow => Color [.#field][#color] [extra]}
                    #[cfg(not(feature="0.16"))]
                    #[cfg(not(feature="0.15"))]{
                        let err = compile_error_no_version();
                        out!{BoxShadow => Color [;][#err] [None]}
                    }
                }
            }

            "z_index"|"zindex" => {
                let pre = match iter.peek_punct() {
                    '-' => {iter.next();qt!(-)},
                    _ => qt!()
                };
                exit!{val = iter.next()}
                exit!{if !val.is_numeric()}
                let name = match iter.next() {
                    Some(tok) => match &tok.to_string()[..1] {
                        "g" => "GlobalZIndex",
                        _ => "ZIndex"
                    }_ => "ZIndex"
                };
                out!{>name => "_" [.0][#pre #val] [None]}
            }

            "interaction" => {map.entry("Interaction");}

            "relative_cursor_position"|"cursor_position"|"cursor_pos" => {
                map.entry("RelativeCursorPosition");
            }

            "focus"|"focus_policy" => {
                let var = iter.next().unwrap_or("Pass".ident().into());
                let var = var.risk_ident().to_case(Case::Pascal);
                out!{FocusPolicy => _ [][FocusPolicy::#var] [None]}
            }

            "scroll_position"|"scroll" => {
                let vals = iter.into_vals();
                if vals.is_empty(){
                    map.entry("ScrollPosition");
                } else {
                    let fields = qar!([x_offset][y_offset]);
                    for (field,oval) in zip(fields,iter.into_vals()) {
                        next!{val = oval.main}
                        let field = field.with_span(oval.span);
                        out!{ScrollPosition => Val [.#field][#val] [oval.extra]}
                    }
                }
            }

// Custom Groups \\

            "size" => {
                let fields = qar!([width][height]);
                for (field,oval) in zip(fields,iter.into_vals()) {
                    next!{val = oval.main}
                    let field = field.with_span(oval.span);
                    out!{Node => Val [.#field][#val] [oval.extra]}
                }
            }

            "gap" => {
                let fields = qar!([row_gap][column_gap]);
                for (field,oval) in zip(fields,iter.into_vals()) {
                    next!{val = oval.main}
                    let field = field.with_span(oval.span);
                    out!{Node => Val [.#field][#val] [oval.extra]}
                }
            }

            "position" => {
                let field = TokenTree::Ident("position_type".ident_span(field.span()));
                let enu = field.clone().risk_ident().to_case(Case::Pascal);
                kill!{val = iter.next(),risk_ident().to_case(Case::Pascal)}
                out!{Node => _ [.#field][#enu::#val] [None]}
            }

 //\\

            "grid_auto_rows"|"grid_auto_columns" => {
                let tracks = iter.into_grid_tracks();    
                out!{Node => _ [.#field] [vec![#(#tracks),*]] [None]}
            },

            "grid_template_rows"|"grid_template_columns" => {
                let tracks = iter.into_repeated_grid_tracks();
                out!{Node => _ [.#field] [vec![#(#tracks),*]] [None]}
            }

            "grid_row"|"grid_column" => {
                let mut vecy = iter.collect::<Vec<_>>();
                let val = match vecy.len() {
                    0|1 => qt!{GridPlacement::DEFAULT;},
                    2 => {
                        kill!{attr = vecy.pop()}
                        kill!{func = vecy.pop()}
                        qt!{GridPlacement::#func(#attr)}
                    }
                    _ => { 
                        kill!{attr2 = vecy.pop()}
                        kill!{attr1 = vecy.pop()}
                        kill!{func = vecy.pop()}
                        qt!{GridPlacement::#func(#attr1,#attr2)}
                    }
                };
                out!{Node => _ [.#field][#val] [None]}
            }

            "overflow_clip_margin" => {
                let mut vecy = iter.collect::<Vec<_>>();
                match vecy.len() {
                    0 => out!{Node => _ [.#field][OverflowClipMargin::DEFAULT] [None]},
                    1 => {
                        kill!{vbox = vecy.pop(),risk_ident().to_case(Case::Pascal)}
                        out!{Node => _ [.#field.visual_box][OverflowClipBox::#vbox] [None]}
                    }
                    _ => {
                        kill!{marg = vecy.pop()}
                        kill!{vbox = vecy.pop(),risk_ident().to_case(Case::Pascal)}
                        out!{Node => _ [.#field.visual_box][OverflowClipBox::#vbox] [None]}
                        out!{Node => f32 [.#field.margin][#marg as f32] [None]}
                    }
                };
            }

            "overflow" => {
                let mut vecy = iter.map(|p|p.risk_ident().to_case(Case::Pascal)).collect::<Vec<_>>();
                match vecy.len(){
                    0 => out!{Node => _ [.#field][Overflow::DEFAULT] [None]},
                    1 => {
                        let all = vecy.pop();
                        out!{Node => _ [.#field.x][OverflowAxis::#all] [None]}
                        out!{Node => _ [.#field.y][OverflowAxis::#all] [None]}
                    }
                    _ => {
                        let y = vecy.pop();
                        let x = vecy.pop();
                        out!{Node => _ [.#field.x][OverflowAxis::#x] [None]}
                        out!{Node => _ [.#field.y][OverflowAxis::#y] [None]}
                    }
                };
            }

            "display"|"position_type"|"align_items"|"justify_items"|"align_self"|"justify_self"|
            "align_content"|"justify_content"|"flex_direction"|"flex_wrap"|"grid_auto_flow" => {
                let enu = field.clone().risk_ident().to_case(Case::Pascal);
                kill!{val = iter.next(),risk_ident().to_case(Case::Pascal)}
                out!{Node => _ [.#field][#enu::#val] [None]}
            }

            "aspect_ratio" => {
                kill!{val = iter.next()}
                out!{Node => f32 [.#field][Some(#val as f32)] [None]}
            }

            "flex_grow"|"flex_shrink" => {
                kill!{val = iter.next()}
                out!{Node => f32 [.#field][#val as f32] [None]}
            }

            "border" => {
                let vals = iter.into_rect_like(false);
                let fields = qar!([top][right][bottom][left]);
                for (field2,oval) in zip(fields,vals) {
                    next!{val = oval.main}
                    let field2 = field2.with_span(oval.span);
                    out!{Node => Val [.#field.#field2] [#val] [oval.extra]}
                }
                if let Some((color,_,extra)) = iter.try_into_color().prepare() {
                    out!{BorderColor => Color [.0][#color] [extra]}
                }
            }

            "margin"|"padding" => {
                let vals = iter.into_rect_like(false);
                let fields = qar!([top][right][bottom][left]);
                for (field2,oval) in zip(fields,vals) {
                    next!{val = oval.main}
                    let field2 = field2.with_span(oval.span);
                    out!{Node => Val [.#field.#field2] [#val] [oval.extra]}
                }
            }

            "left"|"right"|"top"|"bottom"|"width"|"height"|
            "min_width"|"min_height"|"max_width"|"max_height"|
            "flex_basis"|"row_gap"|"column_gap" 
            => if let UiToken{main:Some(val),span:_,extra} = iter.into_val() {
                out!{Node => Val [.#field][#val] [extra]}
            }

            _ => {}
        }
        map
    }


// Ui Token \\

    #[derive(Constructor,Clone)]
    pub struct UiToken {
        main: Option<TokenStream>,
        span: Span,
        extra: Option<TokenStream>
    }

    impl UiToken {
        /// Val::Auto with dirty [Span]
        fn val() -> Self {Self::new(Some(qt!{Val::Auto}),Span::call_site(),None)}
    
        fn prepare(self) -> Option<(TokenStream,Span,Option<TokenStream>)> {
            match (self.main.yay(),self.extra.yay()) {
                (true,_) => Some((self.main.unwrap(),self.span,self.extra)),
                (_,true) => Some((self.main.unwrap_or(qt!{}),self.span,self.extra)),
                _ => None
            }
        }
    }



// Token Handling \\

    #[ext(trait DTreeExt)]
    impl TokenTree {

        fn is_keep(&self) -> bool {
            exit!{*TokenTree::Ident(id)=self}
            id.to_string().as_str() == "_"
        }

        fn is_valid_keep(&self,typ:Str) -> bool {
            exit!{*TokenTree::Ident(id)=self}
            ["_",typ].contains(&id.to_string().as_str())
        }

    }

    #[ext(trait DOptTreeExt)]
    impl <'a> Option<&'a TokenTree> {

        fn is_keep(&self) -> bool {
            exit!{tok = self}
            tok.is_keep()
        }

        fn is_valid_keep(&self,typ:Str) -> bool {
            exit!{tok = self}
            tok.is_valid_keep(typ)
        }

    }

    #[ext(pub trait DTreeIterExt)]
    impl PeekIter {

        fn into_rect_like(&mut self,corner_align:bool) -> [UiToken;4]{
            let default = UiToken::val();
            self.into_vals().into_rect_like(corner_align,default,|mut v|{
                v.main = v.main.map(|t|t.with_span(Span::call_site()));
                v
            })
        }

        /// tries to extract a numeric ui::Val, only progresses if first token is a number literal.
        fn into_val(&mut self) -> UiToken {
            self.try_into_val().unwrap_or(UiToken::val())
        }

        fn try_extra(&mut self) -> Option<TokenStream> {
            exit!{*Some(TokenTree::Group(grp)) = self.peek()}
            let out = Some(grp.stream());
            self.next();
            out
        }

        /// tries to extract a numeric ui::Val, only progresses if first token is a number literal.
        ///
        /// # Alternative
        /// use [Self::into_val] if you want [None] to be Val::Auto
        fn try_into_val(&mut self) -> Option<UiToken> { 
            if self.peek().is_valid_keep("v"){
                return Some(UiToken::new(None,self.next().unwrap().span(),self.try_extra()));
            }
            Some(match self.next_valvar() {
                Step::Base((_pct,lit)) => {
                    let var = qts!{lit.span()=>Px(#lit)};
                    UiToken::new(
                        Some(qt!{Val::#var}),
                        lit.span(),self.try_extra()
                    )
                }
                Step::Shift(var) => UiToken::new(
                    Some(qt!{Val::#var}),
                    var.span(),
                    self.try_extra()
                ),
                _ => exit!{}
            })
        }

        /// extracts a vec of Vals, stops at the first invalid one
        fn into_vals_limited(&mut self,mut limit:u32) -> Vec<UiToken> {
            let mut out = vec![];
            while let Some(val) = self.try_into_val() {
                out.push(val);
                limit -= 1;
                hold!{if limit == 0}
            }
            out
        }

        /// extracts a vec of Vals, stops at the first invalid one
        fn into_vals(&mut self) -> Vec<UiToken> {
            let mut out = vec![];
            while let Some(val) = self.try_into_val() {
                out.push(val);
            }
            out
        }

        fn into_grid_track(&mut self) -> TokenStream {
            self.try_into_grid_track().unwrap()
        }

        /// extracts a vec of GridTracks, stops at the first invalid one
        fn into_grid_tracks(&mut self) -> Vec<TokenStream> {
            let mut out = vec![];
            while let Some(val) = self.try_into_grid_track() {
                out.push(val);
            }
            out
        }

        fn into_repeated_grid_track(&mut self) -> TokenStream {
            self.try_into_repeated_grid_track().unwrap()
        }

        /// extracts a vec of GridTracks, stops at the first invalid one
        fn into_repeated_grid_tracks(&mut self) -> Vec<TokenStream> {
            let mut out = vec![];
            while let Some(val) = self.try_into_repeated_grid_track() {
                out.push(val);
            }
            out
        }

        fn try_into_track_sizing_function(&mut self) -> Step<(Option<Punct>,Literal),TokenStream> {
             match self.next_valvar() {
                Step::None => Step::Shift(match self.next() {
                    Some(TokenTree::Ident(idnt)) => match idnt.to_string().as_str() {
                        "min" => qt!{MinContent},
                        "max" => qt!{MaxContent},
                        "auto" => qt!{Auto},
                        _ => panic!("Failed into TrackSizingFunction")
                    }.with_span(idnt.span()),
                    Some(tok) => qts!{tok.span()=>Auto},
                    _ => qt!{Auto},
                }),
                Step::Shift(s) => Step::Shift(s),
                Step::Base(b) => Step::Base(b),
            }
        }

        fn try_into_track_sizing_function_max(&mut self) -> Step<(Option<Punct>,Literal),TokenStream> {
            match self.try_into_track_sizing_function(){
                Step::Shift(var) => Step::Shift(var),
                Step::Base((pct,lit)) => match LitFloat::from(lit.clone()).suffix() {
                    "fit_px" => Step::Shift(qts!{lit.span()=>FitContentPx(#lit)}),
                    "fit" => match self.peek_punct(){
                        '%' => {self.next();Step::Shift(qts!{lit.span()=>FitContentPercent(#lit)})}
                        '!' => {self.next();Step::Shift(qts!{lit.span()=>FitContentPx(#lit)})}
                        _ => Step::Base((pct,lit))
                    }
                    _ => Step::Base((pct,lit))
                }
                Step::None => Step::None
            }
        }

        fn try_into_grid_track(&mut self) -> Option<TokenStream> {
            self.try_into_grid_track_base(TokenStream::new()).map(|t|qts!{t.span()=>GridTrack #t})
        }

        fn try_into_grid_track_base(&mut self,fattr:TokenStream) -> Option<TokenStream> {
            exit!{peek = self.peek(),clone()}
            match peek {
                TokenTree::Ident(func) => {
                    self.next();
                    Some(match func.to_string().as_str() {
                        "auto"|"min_content"|"max_content" => qt!{::#func(#fattr)},
                        "min" => qt!{::min_content(#fattr)},
                        "max" => qt!{::max_content(#fattr)},
                        _ => panic!{}
                    }.with_span(func.span()))
                }
                TokenTree::Group(grp) => {
                    self.next();
                    let mut iter = grp.stream().into_iter().peekable();
                    let min = iter.try_into_track_sizing_function().shift_or(qt!{Auto});
                    iter.skip_puncts("#-");
                    let max = iter.try_into_track_sizing_function_max().risk_shift();
                    let min = qts!{min.span()=>MinTrackSizingFunction::#min};
                    let max = qts!{max.span()=>MaxTrackSizingFunction::#max};
                    Some(qts!{grp.span_close()=>::minmax(#fattr #min,#max)})
                }
                _ => {
                    kill!{val = self.next_if_num()}
                    let lit: LitFloat = val.clone().into();
                    let suffix = lit.suffix();
                    let func = match suffix {
                        "" => match self.peek_punct() {
                            '%' => {self.next();qt!{percent}}
                            '!'|'n' => {self.next();qt!{px}}
                            _ => panic!()
                        }
                        _ => suffix.ident().to_token_stream()
                    };
                    exit!{val = lit.base10_parse::<f32>()}
                    Some(qt!{::#func(#fattr #val)}.with_span(peek.span()))
                }
            }
        }

        fn try_into_repeated_grid_track(&mut self) -> Option<TokenStream> {
            exit!{rep = self.next()}
            let repe = match &rep {
                TokenTree::Literal(_) => if rep.is_numeric() {qt!{#rep}} else {panic!()}
                TokenTree::Ident(idn) => match idn.to_string().as_str() {
                    "fill"|"auto_fill" => qts!{idn.span()=>GridTrackRepetition::AutoFill},
                    "fit"|"auto_fit" => qts!{idn.span()=>GridTrackRepetition::AutoFit},
                    _ => panic!()
                }
                _ => panic!()
            };
            let mut spon = self.skip_puncts("#-");
            spon.insert(0,rep);
            let track = if !self.peek().is_ident(){
                self.try_into_grid_track_base(qt!{#repe,})
            } else {
                self.try_into_grid_track_base(repe)
            };
            track.map(|t|{
                let mut spon = TokenStream::from_iter(spon);spon.extend(t.clone());
                qts!{spon.span()=>RepeatedGridTrack #t}
            })
        }

        fn try_into_color(&mut self) -> UiToken {  
            if self.peek_punct() == '#' {
                self.next();
                kill!{(stream,span) = self.next_hex_color()}
                UiToken::new(Some(stream),span,self.try_extra())
            } 
            else if let Some(css) = self.next() {
                if css.is_valid_keep("c"){
                    UiToken::new(None,css.span(),self.try_extra())
                } else {
                    let css = css.risk_ident().to_case(Case::UpperSnake);
                    UiToken::new(Some(qt!{Color::Srgba(bevy::color::palettes::css::#css)}),css.span(),self.try_extra())
                }
            }
            else {UiToken::new(None,Span::call_site(),None)}
        }

    }

    fn compile_error_no_version() -> TokenStream {
        qt!{compile_error!{"Mevy: Missing bevy version!: Specify it in Cargo.toml! e.g. feature=[\"0.15\"])"}}
    }

// EOF \\
