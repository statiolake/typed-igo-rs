pub mod dependent;
pub mod pronoun;
pub mod proper;
pub mod suffix;

pub use dependent::Dependent;
pub use pronoun::Pronoun;
pub use proper::Proper;
pub use suffix::Suffix;

use crate::define_enum;

define_enum! {
    pub enum Noun features[1] {
        "サ変接続" => SaColumnIrregularConjugation,
        "ナイ形容詞語幹" => NaiAdjectiveStem,
        "一般" => General,
        "引用文字列" => QuotationString,
        "形容動詞語幹" => AdverbStem,
        "固有名詞" => Proper(..),
        "数" => Number,
        "接続詞的" => LikeConjunction,
        "接尾" => Suffix(..),
        "代名詞" => Pronoun(..),
        "動詞非自立的" => VerbDependent,
        "特殊" => Special,
        "非自立" => Dependent(..),
        "副詞可能" => CanBeAdverb,
    }
}
