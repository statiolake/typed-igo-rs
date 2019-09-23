use crate::define_enum;

define_enum! {
    pub enum Suffix features[2] {
        "サ変接続" => SaColumnIrregularConjugation,
        "一般" => General,
        "形容動詞語幹" => AdverbStem,
        "助数詞" => Quantifier,
        "助動詞語幹" => AuxiliaryVerbStem,
        "人名" => Person,
        "地域" => Regional,
        "特殊" => Special,
        "副詞可能" => CanBeAdverb,
    }
}
