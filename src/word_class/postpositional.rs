use crate::define_enum;

define_enum! {
    pub enum Postpositional features[1] {
        "格助詞" => Nominative(..),
        "係助詞" => Dependency,
        "終助詞" => EndSentence,
        "接続助詞" => Conjunctive,
        "特殊" => Special,
        "副詞化" => MakeAdverb,
        "副助詞" => Supplementary,
        "副助詞／並立助詞／終助詞" => SupplementaryParallelEndSentence,
        "並立助詞" => Parallel,
        "連体化" => MakeDeterminer,
    }
}

define_enum! {
    pub enum Nominative features[2] {
        "一般" => General,
        "引用" => Quote,
        "連語" => Copula,
    }
}
