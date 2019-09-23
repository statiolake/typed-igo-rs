use crate::define_enum;

define_enum! {
    pub enum Dependent features[2] {
        "一般" => General,
        "形容動詞語幹" => AdverbStem,
        "助動詞語幹" => AuxiliaryVerbStem,
        "副詞可能" => CanBeAdverb,
    }
}
