pub mod wordclass {
    pub mod noun {
        pub mod proper {
            pub mod person {
                define_enum! {
                    pub enum Person features[3] {
                        "一般" => General,
                        "姓" => Last,
                        "名" => First,
                    }
                }
            }
            pub use self::person::Person;

            pub mod region {
                define_enum! {
                    pub enum Region features[3] {
                        "一般" => General,
                        "国" => Country,
                    }
                }
            }
            pub use self::region::Region;

            define_enum! {
                pub enum Proper features[2] {
                    "一般" => General,
                    "人名" => Person(..),
                    "組織" => Organization,
                    "地域" => Region(..),
                }
            }
        }
        pub use self::proper::Proper;

        pub mod pronoun {
            define_enum! {
                pub enum Pronoun features[2] {
                    "一般" => General,
                    "縮約" => Contraction,
                }
            }
        }
        pub use self::pronoun::Pronoun;

        pub mod dependent {
            define_enum! {
                pub enum Dependent features[2] {
                    "一般" => General,
                    "副詞可能" => CanBeAdverb,
                    "助動詞語幹" => AuxiliaryVerbStem,
                    "形容動詞語幹" => AdjectiveVerbStem,
                }
            }
        }
        pub use self::dependent::Dependent;

        pub mod special {
            define_enum! {
                pub enum Special features[2] {
                    "助動詞語幹" => AuxiliaryVerbStem,
                }
            }
        }
        pub use self::special::Special;

        pub mod suffix {
            define_enum! {
                pub enum Suffix features[2] {
                    "一般" => General,
                    "人名" => Person,
                    "地域" => Region,
                    "サ変接続" => SahenConjunction,
                    "助動詞語幹" => AuxiliaryVerbStem,
                    "形容動詞語幹" => AdjectiveVerbStem,
                    "副詞可能" => CanBeAdverb,
                    "助数詞" => Quantifier,
                    "特殊" => Special,
                }
            }
        }
        pub use self::suffix::Suffix;

        define_enum! {
            pub enum Noun features[1] {
                "一般" => General,
                "固有名詞" => Proper(..),
                "代名詞" => Pronoun(..),
                "副詞可能" => CanBeAdverb,
                "サ変接続" => SahenConjunction,
                "形容動詞語幹" => AdjectiveVerbStem,
                "数" => Number,
                "非自立" => Dependent(..),
                "特殊" => Special(..),
                "接尾" => Suffix(..),
                "接続詞的" => LineConjunction,
                "動詞非自立的" => VerbDependent,
                "引用文字列" => Quote,
                "ナイ形容詞語幹" => NaiAdjectiveStem,
            }
        }
    }
    pub use self::noun::Noun;

    pub mod prefix {
        define_enum! {
            pub enum Prefix features[1] {
                "名詞接続" => NounConjunction,
                "動詞接続" => VerbConjunction,
                "形容詞接続" => AdjectiveConjunction,
                "数接続" => NumberConjunction,
            }
        }
    }
    pub use self::prefix::Prefix;

    pub mod verb {
        define_enum! {
            pub enum Verb features[1] {
                "自立" => Independent,
                "非自立" => Dependent,
                "接尾" => Suffix,
            }
        }
    }
    pub use self::verb::Verb;

    pub mod adjective {
        define_enum! {
            pub enum Adjective features[1] {
                "自立" => Independent,
                "非自立" => Dependent,
                "接尾" => Suffix,
            }
        }
    }
    pub use self::adjective::Adjective;

    pub mod adverb {
        define_enum! {
            pub enum Adverb features[1] {
                "一般" => General,
                "助詞類接続" => AuxiliaryVerbConjunction,
            }
        }
    }
    pub use self::adverb::Adverb;

    pub mod postpositional {
        pub mod nominative {
            define_enum! {
                pub enum Nominative features[2] {
                    "一般" => General,
                    "引用" => Quote,
                    "連語" => Copula,
                }
            }
        }
        pub use self::nominative::Nominative;

        define_enum! {
            pub enum Postpositional features[1] {
                "格助詞" => Nominative(..),
                "接続助詞" => Conjunction,
                "係助詞" => Dependency,
                "副助詞" => Supplementary,
                "間投助詞" => Interjective,
                "並立助詞" => Parallel,
                "終助詞" => End,
                "副助詞／並立助詞／終助詞" => SupplementaryParallelEnd,
                "連体化" => MakeAttributive,
                "副詞化" => MakeAdverb,
                "特殊" => Special,
            }
        }
    }
    pub use self::postpositional::Postpositional;

    pub mod symbol {
        define_enum! {
            pub enum Symbol features[1] {
                "一般" => General,
                "句点" => Period,
                "読点" => Comma,
                "空白" => Space,
                "アルファベット" => Alphabet,
                "括弧開" => OpenParen,
                "括弧閉" => CloseParen,
            }
        }
    }
    pub use self::symbol::Symbol;

    pub mod other {
        define_enum! {
            pub enum Other features[1] {
                "間投" => Interjective,
            }
        }
    }
    pub use self::other::Other;

    define_enum! {
        pub enum WordClass features[0] {
            "名詞" => Noun(..),
            "接頭詞" => Prefix(..),
            "動詞" => Verb(..),
            "形容詞" => Adjective(..),
            "副詞" => Adverb(..),
            "連体詞" => Attributive,
            "接続詞" => Conjunction,
            "助詞" => Postpositional(..),
            "助動詞" => AuxiliaryVerb,
            "感動詞" => Interjection,
            "記号" => Symbol(..),
            "その他" => Other(..),
            "フィラー" => Filler,
            "非言語音" => NonLanguage,
            "語断片" => WordFragment,
        }
    }
}
pub use self::wordclass::WordClass;
