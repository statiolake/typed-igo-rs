pub mod adjective;
pub mod adverb;
pub mod noun;
pub mod other;
pub mod postpositional;
pub mod prefix;
pub mod symbol;
pub mod verb;

pub use adjective::Adjective;
pub use adverb::Adverb;
pub use noun::Noun;
pub use other::Other;
pub use postpositional::Postpositional;
pub use prefix::Prefix;
pub use symbol::Symbol;
pub use verb::Verb;

use crate::define_enum;

define_enum! {
    pub enum WordClass features[0] {
        "名詞" => Noun(..),
        "接頭詞" => Prefix(..),
        "動詞" => Verb(..),
        "形容詞" => Adjective(..),
        "副詞" => Adverb(..),
        "連体詞" => Determiner,
        "接続詞" => Conjunction,
        "助詞" => Postpositional(..),
        "助動詞" => AuxiliaryVerb,
        "感動詞" => Interjection,
        "記号" => Symbol(..),
        "フィラー" => Filler,
        "その他" => Other(..),
        "未知語" => Unknown,
    }
}
