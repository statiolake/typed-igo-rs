use failure::Fail;
use igo::Morpheme as IgoMorpheme;

pub mod conjugation;
pub mod word_class;

pub use conjugation::Conjugation;
pub use word_class::WordClass;

#[derive(Debug, Fail)]
#[fail(display = "Unknown value: {}", 0)]
pub struct UnknownValue(String);

#[macro_export]
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
            @fields [$($fields)* (@value ($value) @ident ($($ident)*) @child (($($ident)*)) @parsechild (($($ident)*::parse($argfeatures).expect("failed to parse child"))))]
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
                        $enum::$($ident)* $($child)* => ::std::fmt::Write::write_str(b, $value),
                    )*
                }
            }
        }

        impl $enum {
            pub fn parse<'a>($argfeatures: &'a [&'a str]) -> ::std::result::Result<$enum, $crate::UnknownValue> {
                match $argfeatures[$at] {
                    $(
                        $value => Ok($enum::$($ident)* $($parsechild)*),
                    )*
                    other => Err($crate::UnknownValue(other.to_string())),
                }
            }
        }
    };
}

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
        let word_class = WordClass::parse(&*features).expect("failed to parse WordClass");
        let conjugation = Conjugation::parse(&*features).expect("failed to parse Conjugation");
        let original_form = features[6];
        let reading = features[7];
        let pronunciation = features[8];

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
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
