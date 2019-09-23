use crate::define_enum;

define_enum! {
    pub enum Prefix features[1] {
        "形容詞接続" => AdjectiveConjunction,
        "数接続" => NumberConjunction,
        "動詞接続" => VerbConjunction,
        "名詞接続" => NounConjunction,
    }
}
