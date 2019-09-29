#![recursion_limit = "1024"]

use igo::{Morpheme as IgoMorpheme, Tagger, ZipDir};
use std::io::Cursor;

macro_rules! define_enum {
    (pub enum $enum:ident $argfeatures:ident[$at:literal] { $($tts:tt)* }) => {
        define_enum! {
            @enum $enum
            @at $at
            @argfeatures $argfeatures
            @fields []
            @value ()
            @rest $($tts)*
        }
    };

    // field value parsing
    (@enum $enum:ident
     @at $at:literal
     @argfeatures $argfeatures:ident
     @fields [$($fields:tt)*]
     @value ($value:literal)
     @rest => $($rest:tt)*) => {
        define_enum! {
            @enum $enum
            @at $at
            @argfeatures $argfeatures
            @fields [$($fields)*]
            @value ($value)
            @ident ()
            @rest $($rest)*
        }
    };
    (@enum $enum:ident
     @at $at:literal
     @argfeatures $argfeatures:ident
     @fields [$($fields:tt)*]
     @value ()
     @rest $value:literal $($rest:tt)*) => {
        define_enum! {
            @enum $enum
            @at $at
            @argfeatures $argfeatures
            @fields [$($fields)*]
            @value ($value)
            @rest $($rest)*
        }
    };

    // field ident parsing
    (@enum $enum:ident
     @at $at:literal
     @argfeatures $argfeatures:ident
     @fields [$($fields:tt)*]
     @value ($value:literal)
     @ident ($($ident:tt)*)
     @rest , $($rest:tt)*) => {
        define_enum! {
            @enum $enum
            @at $at
            @argfeatures $argfeatures
            @fields [$($fields)* (@value ($value) @ident ($($ident)*) @child () @matchchild () @parsechild ())]
            @value ()
            @rest $($rest)*
        }
    };
    (@enum $enum:ident
     @at $at:literal
     @argfeatures $argfeatures:ident
     @fields [$($fields:tt)*]
     @value ($value:literal)
     @ident ($($ident:tt)*)
     @rest (..) , $($rest:tt)*) => {
        define_enum! {
            @enum $enum
            @at $at
            @argfeatures $argfeatures
            @fields [$($fields)* (@value ($value) @ident ($($ident)*) @child (($($ident)*)) @matchchild ((_)) @parsechild (($($ident)*::parse($argfeatures))))]
            @value ()
            @rest $($rest)*
        }
    };
    (@enum $enum:ident
     @at $at:literal
     @argfeatures $argfeatures:ident
     @fields [$($fields:tt)*]
     @value ($value:literal)
     @ident ($($ident:tt)*)
     @rest ($vartype:tt) , $($rest:tt)*) => {
        define_enum! {
            @enum $enum
            @at $at
            @argfeatures $argfeatures
            @fields [$($fields)* (@value ($value) @ident ($($ident)*) @child (($vartype)) @matchchild ((_)) @parsechild ((<$vartype>::parse($argfeatures))))]
            @value ()
            @rest $($rest)*
        }
    };
    (@enum $enum:ident
     @at $at:literal
     @argfeatures $argfeatures:ident
     @fields [$($fields:tt)*]
     @value ($value:literal)
     @ident ($($ident:tt)*)
     @rest $tt:tt $($rest:tt)*) => {
        define_enum! {
            @enum $enum
            @at $at
            @argfeatures $argfeatures
            @fields [$($fields)*]
            @value ($value)
            @ident ($($ident)* $tt)
            @rest $($rest)*
        }
    };
    (@enum $enum:ident
     @at $at:literal
     @argfeatures $argfeatures:ident
     @fields [$((@value ($value:literal) @ident ($($ident:tt)*) @child ($($child:tt)*) @matchchild ($($matchchild:tt)*) @parsechild ($($parsechild:tt)*)))*]
     @value ()
     @rest) => {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        pub enum $enum {
            $(
                #[doc=$value]
                $($ident)* $($child)*,
            )*
        }

        impl ::std::fmt::Display for $enum {
            fn fmt(&self, b: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                #[allow(unused_variables, non_snake_case)]
                match self {
                    $(
                        $enum::$($ident)* $($matchchild)* => ::std::fmt::Write::write_str(b, $value),
                    )*
                }
            }
        }

        impl $enum {
            pub fn parse($argfeatures: &[&str]) -> $enum {
                match $argfeatures[$at] {
                    $(
                        $value => $enum::$($ident)* $($parsechild)*,
                    )*
                    other => panic!(
                        "BUG: `{value}` is undefined value for type {enum};  maybe the author of `typed-igo-rs` forget to add this value to type {enum}.  please report this issue.",
                        value = other,
                        enum = stringify!($enum)
                    ),
                }
            }
        }
    };
}

include!("conjugation.rs");
include!("wordclass.rs");

#[derive(Debug)]
pub struct Morpheme<'input, 'dict> {
    pub surface: &'input str,
    pub wordclass: WordClass,
    pub conjugation: Conjugation,
    pub basic: &'dict str,
    pub reading: &'dict str,
    pub pronunciation: &'dict str,
    pub start: usize,
}

impl<'input, 'dict> From<IgoMorpheme<'dict, 'input>> for Morpheme<'input, 'dict> {
    fn from(from: IgoMorpheme<'dict, 'input>) -> Morpheme<'input, 'dict> {
        let surface = from.surface;
        let start = from.start;
        let features: Vec<_> = from.feature.split(',').collect();
        let wordclass = WordClass::parse(&*features);
        let conjugation = Conjugation::parse(&*features);
        let basic = features[6];
        let reading = features.get(7).unwrap_or(&"");
        let pronunciation = features.get(8).unwrap_or(&"");

        Morpheme {
            surface,
            wordclass,
            conjugation,
            basic,
            reading,
            pronunciation,
            start,
        }
    }
}

pub struct Parser {
    tagger: Tagger,
}

impl Parser {
    pub fn new() -> Parser {
        let dic = include_bytes!("ipadic.zip");
        let dic = Cursor::new(dic as &[u8]);
        let mut dic = ZipDir::new(dic).unwrap();
        let tagger = Tagger::load_from_dir(&mut dic).unwrap();
        Parser { tagger }
    }

    pub fn parse<'text, 'dict>(&'dict self, text: &'text str) -> Vec<Morpheme<'text, 'dict>> {
        self.tagger
            .parse(text)
            .into_iter()
            .map(Morpheme::from)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let parser = Parser::new();

        let morphemes = parser.parse("すもももももももものうち");
        println!("{:#?}", morphemes);

        let morphemes = parser
            .parse("日本の読者の皆様、こんにちは。日頃からウィキペディアをご利用いただきありがとうございます。 少し申し上げにくいのですが、この月曜日に、私たちには皆様のご支援が必要です。今年既にご寄付をくださった方には心から感謝しています。私たちは営業マンではありません。 平均 1,500円の寄付金が頼りですが、寄付してくださるのは全体の1%未満です。あなたが今日口にするコーヒー一杯分に相当する300円のご支援で、ウィキペディアは発展し続けられます。 どうぞよろしくお願いいたします。");
        println!("{:#?}", morphemes);
    }
}
