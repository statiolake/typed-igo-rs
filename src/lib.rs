#![recursion_limit = "1024"]

use igo::Morpheme as IgoMorpheme;

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
            @fields [$($fields)* (@value ($value) @ident ($($ident)*) @child () @parsechild ())]
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
            @fields [$($fields)* (@value ($value) @ident ($($ident)*) @child (($($ident)*)) @parsechild (($($ident)*::parse($argfeatures))))]
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
            @fields [$($fields)* (@value ($value) @ident ($($ident)*) @child (($vartype)) @parsechild ((<$vartype>::parse($argfeatures))))]
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
     @fields [$((@value ($value:literal) @ident ($($ident:tt)*) @child ($($child:tt)*) @parsechild ($($parsechild:tt)*)))*]
     @value ()
     @rest) => {
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        pub enum $enum {
            $(
                #[doc=$value]
                $($ident)* $($child)*,
            )*

            #[doc="定義されていない分類"]
            Undefined(String),
        }

        impl ::std::fmt::Display for $enum {
            fn fmt(&self, b: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                #[allow(unused_variables, non_snake_case)]
                match self {
                    $(
                        $enum::$($ident)* $($child)* => ::std::fmt::Write::write_str(b, $value),
                    )*
                    $enum::Undefined(undef) => ::std::fmt::Write::write_str(b, undef),
                }
            }
        }

        impl $enum {
            pub fn parse<'a>($argfeatures: &'a [&'a str]) -> $enum {
                match $argfeatures[$at] {
                    $(
                        $value => $enum::$($ident)* $($parsechild)*,
                    )*
                    other => $enum::Undefined(other.to_string()),
                }
            }
        }
    };
}

include!("conjugation.rs");
include!("wordclass.rs");

#[derive(Debug)]
pub struct Morpheme<'s, 'f> {
    pub surface: &'s str,
    pub word_class: WordClass,
    pub conjugation: Conjugation,
    pub original_form: &'f str,
    pub reading: &'f str,
    pub pronunciation: &'f str,
    pub start: usize,
}

impl<'s, 'f> From<IgoMorpheme<'f, 's>> for Morpheme<'s, 'f> {
    fn from(from: IgoMorpheme<'f, 's>) -> Morpheme<'s, 'f> {
        let surface = from.surface;
        let start = from.start;
        let features: Vec<_> = from.feature.split(',').collect();
        let word_class = WordClass::parse(&*features);
        let conjugation = Conjugation::parse(&*features);
        let original_form = features[6];
        let reading = features.get(7).unwrap_or(&"");
        let pronunciation = features.get(8).unwrap_or(&"");

        Morpheme {
            surface,
            word_class,
            conjugation,
            original_form,
            reading,
            pronunciation,
            start,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use igo::{Tagger, ZipDir};
    use std::io::Cursor;

    #[test]
    fn it_works() {
        let dic = include_bytes!("ipadic.zip");
        let dic = Cursor::new(dic as &[u8]);
        let mut dic = ZipDir::new(dic).unwrap();
        let tagger = Tagger::load_from_dir(&mut dic).unwrap();

        let morphemes: Vec<_> = tagger
            .parse("すもももももももものうち")
            .into_iter()
            .map(Morpheme::from)
            .collect();
        println!("{:#?}", morphemes);

        let morphemes: Vec<_> = tagger
            .parse("日本の読者の皆様、こんにちは。日頃からウィキペディアをご利用いただきありがとうございます。 少し申し上げにくいのですが、この月曜日に、私たちには皆様のご支援が必要です。今年既にご寄付をくださった方には心から感謝しています。私たちは営業マンではありません。 平均 1,500円の寄付金が頼りですが、寄付してくださるのは全体の1%未満です。あなたが今日口にするコーヒー一杯分に相当する300円のご支援で、ウィキペディアは発展し続けられます。 どうぞよろしくお願いいたします。")
            .into_iter()
            .map(Morpheme::from)
            .collect();
        println!("{:#?}", morphemes);
    }
}
