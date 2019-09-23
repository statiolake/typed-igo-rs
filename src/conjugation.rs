pub mod conjugation {
    pub mod kahen_kuru {
        define_enum! {
            pub enum KahenKuru features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "未然ウ接続" => NegativeU,
                "連用形" => Continuous,
                "仮定形" => Conditional,
                "命令ｙｏ" => ImperativeYo,
                "命令ｉ" => ImperativeI,
                "仮定縮約１" => ConditionalContraction1,
                "体言接続特殊" => SpecialAttributive,
                "体言接続特殊２" => SpecialAttributive2,
            }
        }
    }
    pub use self::kahen_kuru::KahenKuru;

    pub mod kahen_kuru_kanji {
        define_enum! {
            pub enum KahenKuruKanji features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "未然ウ接続" => NegativeU,
                "連用形" => Continuous,
                "仮定形" => Conditional,
                "命令ｙｏ" => ImperativeYo,
                "命令ｉ" => ImperativeI,
                "仮定縮約１" => ConditionalContraction1,
                "体言接続特殊" => SpecialAttributive,
                "体言接続特殊２" => SpecialAttributive2,
            }
        }
    }
    pub use self::kahen_kuru_kanji::KahenKuruKanji;

    pub mod sahen_suru {
        define_enum! {
            pub enum SahenSuru features[5] {
                "基本形" => Basic,
                "文語基本形" => OldBasic,
                "未然形" => Negative,
                "未然ウ接続" => NegativeU,
                "未然レル接続" => NegativeReru,
                "未然ヌ接続" => NegativeNu,
                "連用形" => Continuous,
                "仮定形" => Conditional,
                "命令ｙｏ" => ImperativeYo,
                "命令ｒｏ" => ImperativeRo,
                "命令ｉ" => ImperativeI,
                "仮定縮約１" => ConditionalContraction1,
                "体言接続特殊" => SpecialAttributive,
                "体言接続特殊２" => SpecialAttributive2,
            }
        }
    }
    pub use self::sahen_suru::SahenSuru;

    pub mod sahen_suru_connected {
        define_enum! {
            pub enum SahenSuruConnected features[5] {
                "基本形" => Basic,
                "文語基本形" => OldBasic,
                "未然形" => Negative,
                "未然ウ接続" => NegativeU,
                "未然レル接続" => NegativeReru,
                "仮定形" => Conditional,
                "命令ｙｏ" => ImperativeYo,
                "命令ｒｏ" => ImperativeRo,
                "仮定縮約１" => ConditionalContraction1,
            }
        }
    }
    pub use self::sahen_suru_connected::SahenSuruConnected;

    pub mod sahen_zuru_connected {
        define_enum! {
            pub enum SahenZuruConnected features[5] {
                "基本形" => Basic,
                "文語基本形" => OldBasic,
                "未然形" => Negative,
                "未然ウ接続" => NegativeU,
                "仮定形" => Conditional,
                "命令ｙｏ" => ImperativeYo,
                "仮定縮約１" => ConditionalContraction1,
            }
        }
    }
    pub use self::sahen_zuru_connected::SahenZuruConnected;

    pub mod ichidan {
        define_enum! {
            pub enum Ichidan features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "未然ウ接続" => NegativeU,
                "連用形" => Continuous,
                "仮定形" => Conditional,
                "命令ｙｏ" => ImperativeYo,
                "命令ｒｏ" => ImperativeRo,
                "仮定縮約１" => ConditionalContraction1,
                "体言接続特殊" => SpecialAttributive,
            }
        }
    }
    pub use self::ichidan::Ichidan;

    pub mod ichidan_yameru {
        define_enum! {
            pub enum IchidanYameru features[5] {
                "基本形" => Basic,
            }
        }
    }
    pub use self::ichidan_yameru::IchidanYameru;

    pub mod godan_ka_ionbin {
        define_enum! {
            pub enum GodanKaIonbin features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "未然ウ接続" => NegativeU,
                "連用形" => Continuous,
                "連用タ接続" => ContinuousTa,
                "仮定形" => Conditional,
                "命令ｅ" => ImperativeE,
                "仮定縮約１" => ConditionalContraction1,
            }
        }
    }
    pub use self::godan_ka_ionbin::GodanKaIonbin;

    pub mod godan_ka_sokuonbin {
        define_enum! {
            pub enum GodanKaSokuonbin features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "未然ウ接続" => NegativeU,
                "連用形" => Continuous,
                "連用タ接続" => ContinuousTa,
                "仮定形" => Conditional,
                "命令ｅ" => ImperativeE,
                "仮定縮約１" => ConditionalContraction1,
            }
        }
    }
    pub use self::godan_ka_sokuonbin::GodanKaSokuonbin;

    pub mod godan_ka_sokuonbin_yuku {
        define_enum! {
            pub enum GodanKaSokuonbinYuku features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "未然ウ接続" => NegativeU,
                "連用形" => Continuous,
                "仮定形" => Conditional,
                "命令ｅ" => ImperativeE,
                "仮定縮約１" => ConditionalContraction1,
            }
        }
    }
    pub use self::godan_ka_sokuonbin_yuku::GodanKaSokuonbinYuku;

    pub mod godan_ga {
        define_enum! {
            pub enum GodanGa features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "未然ウ接続" => NegativeU,
                "連用形" => Continuous,
                "連用タ接続" => ContinuousTa,
                "仮定形" => Conditional,
                "命令ｅ" => ImperativeE,
                "仮定縮約１" => ConditionalContraction1,
            }
        }
    }
    pub use self::godan_ga::GodanGa;

    pub mod godan_sa {
        define_enum! {
            pub enum GodanSa features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "未然ウ接続" => NegativeU,
                "連用形" => Continuous,
                "仮定形" => Conditional,
                "命令ｅ" => ImperativeE,
                "仮定縮約１" => ConditionalContraction1,
            }
        }
    }
    pub use self::godan_sa::GodanSa;

    pub mod godan_ta {
        define_enum! {
            pub enum GodanTa features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "未然ウ接続" => NegativeU,
                "連用形" => Continuous,
                "連用タ接続" => ContinuousTa,
                "仮定形" => Conditional,
                "命令ｅ" => ImperativeE,
                "仮定縮約１" => ConditionalContraction1,
            }
        }
    }
    pub use self::godan_ta::GodanTa;

    pub mod godan_na {
        define_enum! {
            pub enum GodanNa features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "未然ウ接続" => NegativeU,
                "連用形" => Continuous,
                "連用タ接続" => ContinuousTa,
                "仮定形" => Conditional,
                "命令ｅ" => ImperativeE,
                "仮定縮約１" => ConditionalContraction1,
            }
        }
    }
    pub use self::godan_na::GodanNa;

    pub mod godan_ba {
        define_enum! {
            pub enum GodanBa features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "未然ウ接続" => NegativeU,
                "連用形" => Continuous,
                "連用タ接続" => ContinuousTa,
                "仮定形" => Conditional,
                "命令ｅ" => ImperativeE,
                "仮定縮約１" => ConditionalContraction1,
            }
        }
    }
    pub use self::godan_ba::GodanBa;

    pub mod godan_ma {
        define_enum! {
            pub enum GodanMa features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "未然ウ接続" => NegativeU,
                "連用形" => Continuous,
                "連用タ接続" => ContinuousTa,
                "仮定形" => Conditional,
                "命令ｅ" => ImperativeE,
                "仮定縮約１" => ConditionalContraction1,
            }
        }
    }
    pub use self::godan_ma::GodanMa;

    pub mod godan_ra {
        define_enum! {
            pub enum GodanRa features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "未然特殊" => NegativeSpecial,
                "未然ウ接続" => NegativeU,
                "連用形" => Continuous,
                "連用タ接続" => ContinuousTa,
                "仮定形" => Conditional,
                "命令ｅ" => ImperativeE,
                "仮定縮約１" => ConditionalContraction1,
                "体言接続特殊" => SpecialAttributive,
                "体言接続特殊２" => SpecialAttributive2,
            }
        }
    }
    pub use self::godan_ra::GodanRa;

    pub mod godan_ra_aru {
        define_enum! {
            pub enum GodanRaAru features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "未然ウ接続" => NegativeU,
                "連用形" => Continuous,
                "連用タ接続" => ContinuousTa,
                "仮定形" => Conditional,
                "命令ｅ" => ImperativeE,
                "仮定縮約１" => ConditionalContraction1,
                "体言接続特殊" => SpecialAttributive,
            }
        }
    }
    pub use self::godan_ra_aru::GodanRaAru;

    pub mod godan_ra_special {
        define_enum! {
            pub enum GodanRaSpecial features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "未然特殊" => NegativeSpecial,
                "未然ウ接続" => NegativeU,
                "連用形" => Continuous,
                "連用タ接続" => ContinuousTa,
                "仮定形" => Conditional,
                "命令ｅ" => ImperativeE,
                "命令ｉ" => ImperativeI,
                "仮定縮約１" => ConditionalContraction1,
            }
        }
    }
    pub use self::godan_ra_special::GodanRaSpecial;

    pub mod godan_wa_uonbin {
        define_enum! {
            pub enum GodanWaUonbin features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "未然ウ接続" => NegativeU,
                "連用形" => Continuous,
                "連用タ接続" => ContinuousTa,
                "仮定形" => Conditional,
                "命令ｅ" => ImperativeE,
            }
        }
    }
    pub use self::godan_wa_uonbin::GodanWaUonbin;

    pub mod godan_wa_sokuonbin {
        define_enum! {
            pub enum GodanWaSokuonbin features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "未然ウ接続" => NegativeU,
                "連用形" => Continuous,
                "連用タ接続" => ContinuousTa,
                "仮定形" => Conditional,
                "命令ｅ" => ImperativeE,
            }
        }
    }
    pub use self::godan_wa_sokuonbin::GodanWaSokuonbin;

    pub mod yodan_ka {
        define_enum! {
            pub enum YodanKa features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "連用形" => Continuous,
                "仮定形" => Conditional,
                "命令ｅ" => ImperativeE,
            }
        }
    }
    pub use self::yodan_ka::YodanKa;

    pub mod yodan_ga {
        define_enum! {
            pub enum YodanGa features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "連用形" => Continuous,
                "仮定形" => Conditional,
                "命令ｅ" => ImperativeE,
            }
        }
    }
    pub use self::yodan_ga::YodanGa;

    pub mod yodan_sa {
        define_enum! {
            pub enum YodanSa features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "連用形" => Continuous,
                "仮定形" => Conditional,
                "命令ｅ" => ImperativeE,
            }
        }
    }
    pub use self::yodan_sa::YodanSa;

    pub mod yodan_ta {
        define_enum! {
            pub enum YodanTa features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "連用形" => Continuous,
                "仮定形" => Conditional,
                "命令ｅ" => ImperativeE,
            }
        }
    }
    pub use self::yodan_ta::YodanTa;

    pub mod yodan_ba {
        define_enum! {
            pub enum YodanBa features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "連用形" => Continuous,
                "仮定形" => Conditional,
                "命令ｅ" => ImperativeE,
            }
        }
    }
    pub use self::yodan_ba::YodanBa;

    pub mod yodan_ma {
        define_enum! {
            pub enum YodanMa features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "連用形" => Continuous,
                "仮定形" => Conditional,
                "命令ｅ" => ImperativeE,
            }
        }
    }
    pub use self::yodan_ma::YodanMa;

    pub mod yodan_ra {
        define_enum! {
            pub enum YodanRa features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "連用形" => Continuous,
                "仮定形" => Conditional,
                "命令ｅ" => ImperativeE,
            }
        }
    }
    pub use self::yodan_ra::YodanRa;

    pub mod yodan_ha {
        define_enum! {
            pub enum YodanHa features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "連用形" => Continuous,
                "仮定形" => Conditional,
                "命令ｅ" => ImperativeE,
            }
        }
    }
    pub use self::yodan_ha::YodanHa;

    pub mod rahen {
        define_enum! {
            pub enum Rahen features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "連用形" => Continuous,
                "体言接続" => AttributiveConjunction,
                "仮定形" => Conditional,
                "命令ｅ" => ImperativeE,
            }
        }
    }
    pub use self::rahen::Rahen;

    pub mod kamini_da {
        define_enum! {
            pub enum KaminiDa features[5] {
                "基本形" => Basic,
                "現代基本形" => ModernBasic,
                "未然形" => Negative,
                "連用形" => Continuous,
                "体言接続" => AttributiveConjunction,
                "仮定形" => Conditional,
                "命令ｙｏ" => ImperativeYo,
            }
        }
    }
    pub use self::kamini_da::KaminiDa;

    pub mod kamini_ha {
        define_enum! {
            pub enum KaminiHa features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "連用形" => Continuous,
                "体言接続" => AttributiveConjunction,
                "仮定形" => Conditional,
                "命令ｙｏ" => ImperativeYo,
            }
        }
    }
    pub use self::kamini_ha::KaminiHa;

    pub mod shimoni_a {
        define_enum! {
            pub enum ShimoniA features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "連用形" => Continuous,
                "体言接続" => AttributiveConjunction,
                "仮定形" => Conditional,
                "命令ｙｏ" => ImperativeYo,
            }
        }
    }
    pub use self::shimoni_a::ShimoniA;

    pub mod shimoni_ka {
        define_enum! {
            pub enum ShimoniKa features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "連用形" => Continuous,
                "体言接続" => AttributiveConjunction,
                "仮定形" => Conditional,
                "命令ｙｏ" => ImperativeYo,
            }
        }
    }
    pub use self::shimoni_ka::ShimoniKa;

    pub mod shimoni_ga {
        define_enum! {
            pub enum ShimoniGa features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "連用形" => Continuous,
                "体言接続" => AttributiveConjunction,
                "仮定形" => Conditional,
                "命令ｙｏ" => ImperativeYo,
            }
        }
    }
    pub use self::shimoni_ga::ShimoniGa;

    pub mod shimoni_sa {
        define_enum! {
            pub enum ShimoniSa features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "連用形" => Continuous,
                "体言接続" => AttributiveConjunction,
                "仮定形" => Conditional,
                "命令ｙｏ" => ImperativeYo,
            }
        }
    }
    pub use self::shimoni_sa::ShimoniSa;

    pub mod shimoni_za {
        define_enum! {
            pub enum ShimoniZa features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "連用形" => Continuous,
                "体言接続" => AttributiveConjunction,
                "仮定形" => Conditional,
                "命令ｙｏ" => ImperativeYo,
            }
        }
    }
    pub use self::shimoni_za::ShimoniZa;

    pub mod shimoni_ta {
        define_enum! {
            pub enum ShimoniTa features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "連用形" => Continuous,
                "体言接続" => AttributiveConjunction,
                "仮定形" => Conditional,
                "命令ｙｏ" => ImperativeYo,
            }
        }
    }
    pub use self::shimoni_ta::ShimoniTa;

    pub mod shimoni_da {
        define_enum! {
            pub enum ShimoniDa features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "連用形" => Continuous,
                "体言接続" => AttributiveConjunction,
                "仮定形" => Conditional,
                "命令ｙｏ" => ImperativeYo,
            }
        }
    }
    pub use self::shimoni_da::ShimoniDa;

    pub mod shimoni_na {
        define_enum! {
            pub enum ShimoniNa features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "連用形" => Continuous,
                "体言接続" => AttributiveConjunction,
                "仮定形" => Conditional,
                "命令ｙｏ" => ImperativeYo,
            }
        }
    }
    pub use self::shimoni_na::ShimoniNa;

    pub mod shimoni_ha {
        define_enum! {
            pub enum ShimoniHa features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "連用形" => Continuous,
                "体言接続" => AttributiveConjunction,
                "仮定形" => Conditional,
                "命令ｙｏ" => ImperativeYo,
            }
        }
    }
    pub use self::shimoni_ha::ShimoniHa;

    pub mod shimoni_ba {
        define_enum! {
            pub enum ShimoniBa features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "連用形" => Continuous,
                "体言接続" => AttributiveConjunction,
                "仮定形" => Conditional,
                "命令ｙｏ" => ImperativeYo,
            }
        }
    }
    pub use self::shimoni_ba::ShimoniBa;

    pub mod shimoni_ma {
        define_enum! {
            pub enum ShimoniMa features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "連用形" => Continuous,
                "体言接続" => AttributiveConjunction,
                "仮定形" => Conditional,
                "命令ｙｏ" => ImperativeYo,
            }
        }
    }
    pub use self::shimoni_ma::ShimoniMa;

    pub mod shimoni_ya {
        define_enum! {
            pub enum ShimoniYa features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "連用形" => Continuous,
                "体言接続" => AttributiveConjunction,
                "仮定形" => Conditional,
                "命令ｙｏ" => ImperativeYo,
            }
        }
    }
    pub use self::shimoni_ya::ShimoniYa;

    pub mod shimoni_ra {
        define_enum! {
            pub enum ShimoniRa features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "連用形" => Continuous,
                "体言接続" => AttributiveConjunction,
                "仮定形" => Conditional,
                "命令ｙｏ" => ImperativeYo,
            }
        }
    }
    pub use self::shimoni_ra::ShimoniRa;

    pub mod shimoni_wa {
        define_enum! {
            pub enum ShimoniWa features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "連用形" => Continuous,
                "体言接続" => AttributiveConjunction,
                "仮定形" => Conditional,
                "命令ｙｏ" => ImperativeYo,
            }
        }
    }
    pub use self::shimoni_wa::ShimoniWa;

    pub mod shimoni_u {
        define_enum! {
            pub enum ShimoniU features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "未然ウ接続" => NegativeU,
                "連用形" => Continuous,
                "体言接続" => AttributiveConjunction,
                "仮定形" => Conditional,
                "命令ｙｏ" => ImperativeYo,
            }
        }
    }
    pub use self::shimoni_u::ShimoniU;

    pub mod ichidan_kureru {
        define_enum! {
            pub enum IchidanKureru features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "未然特殊" => NegativeSpecial,
                "未然ウ接続" => NegativeU,
                "連用形" => Continuous,
                "仮定形" => Conditional,
                "命令ｅ" => ImperativeE,
                "命令ｙｏ" => ImperativeYo,
                "命令ｒｏ" => ImperativeRo,
                "仮定縮約１" => ConditionalContraction1,
            }
        }
    }
    pub use self::ichidan_kureru::IchidanKureru;

    pub mod ichidan_ru {
        define_enum! {
            pub enum IchidanRu features[5] {
                "基本形" => Basic,
                "仮定形" => Conditional,
                "命令ｒｏ" => ImperativeRo,
                "仮定縮約１" => ConditionalContraction1,
            }
        }
    }
    pub use self::ichidan_ru::IchidanRu;

    pub mod adjective_auo {
        define_enum! {
            pub enum AdjectiveAUO features[5] {
                "基本形" => Basic,
                "文語基本形" => OldBasic,
                "未然ヌ接続" => NegativeNu,
                "未然ウ接続" => NegativeU,
                "連用タ接続" => ContinuousTa,
                "連用テ接続" => ContinuousTe,
                "連用ゴザイ接続" => ContinuousGozai,
                "体言接続" => AttributiveConjunction,
                "仮定形" => Conditional,
                "命令ｅ" => ImperativeE,
                "仮定縮約１" => ConditionalContraction1,
                "仮定縮約２" => ConditionalContraction2,
                "ガル接続" => Garu,
            }
        }
    }
    pub use self::adjective_auo::AdjectiveAUO;

    pub mod adjective_i {
        define_enum! {
            pub enum AdjectiveI features[5] {
                "基本形" => Basic,
                "文語基本形" => OldBasic,
                "未然ヌ接続" => NegativeNu,
                "未然ウ接続" => NegativeU,
                "連用タ接続" => ContinuousTa,
                "連用テ接続" => ContinuousTe,
                "連用ゴザイ接続" => ContinuousGozai,
                "体言接続" => AttributiveConjunction,
                "仮定形" => Conditional,
                "命令ｅ" => ImperativeE,
                "仮定縮約１" => ConditionalContraction1,
                "仮定縮約２" => ConditionalContraction2,
                "ガル接続" => Garu,
            }
        }
    }
    pub use self::adjective_i::AdjectiveI;

    pub mod special_nai {
        define_enum! {
            pub enum SpecialNai features[5] {
                "基本形" => Basic,
                "音便基本形" => OnbinBasic,
                "文語基本形" => OldBasic,
                "未然ヌ接続" => NegativeNu,
                "未然ウ接続" => NegativeU,
                "連用タ接続" => ContinuousTa,
                "連用テ接続" => ContinuousTe,
                "連用デ接続" => ContinuousDe,
                "連用ゴザイ接続" => ContinuousGozai,
                "体言接続" => AttributiveConjunction,
                "仮定形" => Conditional,
                "命令ｅ" => ImperativeE,
                "仮定縮約１" => ConditionalContraction1,
                "仮定縮約２" => ConditionalContraction2,
                "ガル接続" => Garu,
            }
        }
    }
    pub use self::special_nai::SpecialNai;

    pub mod special_tai {
        define_enum! {
            pub enum SpecialTai features[5] {
                "基本形" => Basic,
                "音便基本形" => OnbinBasic,
                "文語基本形" => OldBasic,
                "未然ヌ接続" => NegativeNu,
                "未然ウ接続" => NegativeU,
                "連用タ接続" => ContinuousTa,
                "連用テ接続" => ContinuousTe,
                "連用ゴザイ接続" => ContinuousGozai,
                "体言接続" => AttributiveConjunction,
                "仮定形" => Conditional,
                "仮定縮約１" => ConditionalContraction1,
                "仮定縮約２" => ConditionalContraction2,
                "ガル接続" => Garu,
            }
        }
    }
    pub use self::special_tai::SpecialTai;

    pub mod special_ta {
        define_enum! {
            pub enum SpecialTa features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "仮定形" => Conditional,
            }
        }
    }
    pub use self::special_ta::SpecialTa;

    pub mod special_da {
        define_enum! {
            pub enum SpecialDa features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "連用形" => Continuous,
                "連用タ接続" => ContinuousTa,
                "体言接続" => AttributiveConjunction,
                "仮定形" => Conditional,
                "命令ｅ" => ImperativeE,
            }
        }
    }
    pub use self::special_da::SpecialDa;

    pub mod special_desu {
        define_enum! {
            pub enum SpecialDesu features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "連用形" => Continuous,
            }
        }
    }
    pub use self::special_desu::SpecialDesu;

    pub mod special_ja {
        define_enum! {
            pub enum SpecialJa features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "連用形" => Continuous,
            }
        }
    }
    pub use self::special_ja::SpecialJa;

    pub mod specical_masu {
        define_enum! {
            pub enum SpecialMasu features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "未然ウ接続" => NegativeU,
                "連用形" => Continuous,
                "仮定形" => Conditional,
                "命令ｅ" => ImperativeE,
                "命令ｉ" => ImperativeI,
            }
        }
    }
    pub use self::specical_masu::SpecialMasu;

    pub mod special_nu {
        define_enum! {
            pub enum SpecialNu features[5] {
                "基本形" => Basic,
                "文語基本形" => OldBasic,
                "連用形" => Continuous,
                "連用ニ接続" => ContinuousNi,
                "体言接続" => AttributiveConjunction,
                "仮定形" => Conditional,
            }
        }
    }
    pub use self::special_nu::SpecialNu;

    pub mod old_beshi {
        define_enum! {
            pub enum OldBeshi features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "連用形" => Continuous,
                "体言接続" => AttributiveConjunction,
                "仮定形" => Conditional,
            }
        }
    }
    pub use self::old_beshi::OldBeshi;

    pub mod old_gotoshi {
        define_enum! {
            pub enum OldGotoshi features[5] {
                "基本形" => Basic,
                "連用形" => Continuous,
                "体言接続" => AttributiveConjunction,
            }
        }
    }
    pub use self::old_gotoshi::OldGotoshi;

    pub mod old_nari {
        define_enum! {
            pub enum OldNari features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "体言接続" => AttributiveConjunction,
                "仮定形" => Conditional,
                "命令ｅ" => ImperativeE,
            }
        }
    }
    pub use self::old_nari::OldNari;

    pub mod old_maji {
        define_enum! {
            pub enum OldMaji features[5] {
                "基本形" => Basic,
                "連用形" => Continuous,
                "体言接続" => AttributiveConjunction,
                "仮定形" => Conditional,
            }
        }
    }
    pub use self::old_maji::OldMaji;

    pub mod old_shimu {
        define_enum! {
            pub enum OldShimu features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "連用形" => Continuous,
                "体言接続" => AttributiveConjunction,
                "仮定形" => Conditional,
                "命令ｅ" => ImperativeE,
                "命令ｙｏ" => ImperativeYo,
            }
        }
    }
    pub use self::old_shimu::OldShimu;

    pub mod old_ki {
        define_enum! {
            pub enum OldKi features[5] {
                "基本形" => Basic,
                "体言接続" => AttributiveConjunction,
                "命令ｅ" => ImperativeE,
            }
        }
    }
    pub use self::old_ki::OldKi;

    pub mod old_keri {
        define_enum! {
            pub enum OldKeri features[5] {
                "基本形" => Basic,
                "体言接続" => AttributiveConjunction,
            }
        }
    }
    pub use self::old_keri::OldKeri;

    pub mod old_ri {
        define_enum! {
            pub enum OldRi features[5] {
                "基本形" => Basic,
                "体言接続" => AttributiveConjunction,
            }
        }
    }
    pub use self::old_ri::OldRi;

    pub mod old_ru {
        define_enum! {
            pub enum OldRu features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "連用形" => Continuous,
                "体言接続" => AttributiveConjunction,
                "仮定形" => Conditional,
                "命令ｅ" => ImperativeE,
                "命令ｙｏ" => ImperativeYo,
            }
        }
    }
    pub use self::old_ru::OldRu;

    pub mod no_conjugation {
        define_enum! {
            pub enum NoConjugation features[5] {
                "基本形" => Basic,
            }
        }
    }
    pub use self::no_conjugation::NoConjugation;

    pub mod adjective_ii {
        define_enum! {
            pub enum AdjectiveII features[5] {
                "基本形" => Basic,
                "基本形-促音便" => BasicSokuonbin,
            }
        }
    }
    pub use self::adjective_ii::AdjectiveII;

    pub mod special_dosu {
        define_enum! {
            pub enum SpecialDosu features[5] {
                "基本形" => Basic,
            }
        }
    }
    pub use self::special_dosu::SpecialDosu;

    pub mod ichidan_eru {
        define_enum! {
            pub enum IchidanEru features[5] {
                "基本形" => Basic,
                "仮定形" => Conditional,
            }
        }
    }
    pub use self::ichidan_eru::IchidanEru;

    pub mod special_ya {
        define_enum! {
            pub enum SpecialYa features[5] {
                "基本形" => Basic,
                "未然形" => Negative,
                "連用形" => Continuous,
            }
        }
    }
    pub use self::special_ya::SpecialYa;

    define_enum! {
        pub enum Conjugation features[4] {
            "*" => None,
            "カ変・クル" => KahenKuru(..),
            "カ変・来ル" => KahenKuruKanji(..),
            "サ変・スル" => SahenSuru(..),
            "サ変・－スル" => SahenSuruConnected(..),
            "サ変・－ズル" => SahenZuruConnected(..),
            "一段" => Ichidan(..),
            "一段・病メル" => IchidanYameru(..),
            "五段・カ行イ音便" => GodanKaIonbin(..),
            "五段・カ行促音便" => GodanKaSokuonbin(..),
            "五段・カ行促音便ユク" => GodanKaSokuonbinYuku(..),
            "五段・ガ行" => GodanGa(..),
            "五段・サ行" => GodanSa(..),
            "五段・タ行" => GodanTa(..),
            "五段・ナ行" => GodanNa(..),
            "五段・バ行" => GodanBa(..),
            "五段・マ行" => GodanMa(..),
            "五段・ラ行" => GodanRa(..),
            "五段・ラ行アル" => GodanRaAru(..),
            "五段・ラ行特殊" => GodanRaSpecial(..),
            "五段・ワ行ウ音便" => GodanWaUonbin(..),
            "五段・ワ行促音便" => GodanWaSokuonbin(..),
            "四段・カ行" => YodanKa(..),
            "四段・ガ行" => YodanGa(..),
            "四段・サ行" => YodanSa(..),
            "四段・タ行" => YodanTa(..),
            "四段・バ行" => YodanBa(..),
            "四段・マ行" => YodanMa(..),
            "四段・ラ行" => YodanRa(..),
            "四段・ハ行" => YodanHa(..),
            "ラ変" => Rahen(..),
            "上二・ダ行" => KaminiDa(..),
            "上二・ハ行" => KaminiHa(..),
            "下二・ア行" => ShimoniA(..),
            "下二・カ行" => ShimoniKa(..),
            "下二・ガ行" => ShimoniGa(..),
            "下二・サ行" => ShimoniSa(..),
            "下二・ザ行" => ShimoniZa(..),
            "下二・タ行" => ShimoniTa(..),
            "下二・ダ行" => ShimoniDa(..),
            "下二・ナ行" => ShimoniNa(..),
            "下二・ハ行" => ShimoniHa(..),
            "下二・バ行" => ShimoniBa(..),
            "下二・マ行" => ShimoniMa(..),
            "下二・ヤ行" => ShimoniYa(..),
            "下二・ラ行" => ShimoniRa(..),
            "下二・ワ行" => ShimoniWa(..),
            "下二・得" => ShimoniU(..),
            "一段・クレル" => IchidanKureru(..),
            "一段・ル" => IchidanRu(..),
            "形容詞・アウオ段" => AdjectiveAUO(..),
            "形容詞・イ段" => AdjectiveI(..),
            "特殊・ナイ" => SpecialNai(..),
            "特殊・タイ" => SpecialTai(..),
            "特殊・タ" => SpecialTa(..),
            "特殊・ダ" => SpecialDa(..),
            "特殊・デス" => SpecialDesu(..),
            "特殊・ジャ" => SpecialJa(..),
            "特殊・マス" => SpecialMasu(..),
            "特殊・ヌ" => SpecialNu(..),
            "文語・ベシ" => OldBeshi(..),
            "文語・ゴトシ" => OldGotoshi(..),
            "文語・ナリ" => OldNari(..),
            "文語・マジ" => OldMaji(..),
            "文語・シム" => OldShimu(..),
            "文語・キ" => OldKi(..),
            "文語・ケリ" => OldKeri(..),
            "文語・リ" => OldRi(..),
            "文語・ル" => OldRu(..),
            "不変化型" => NoConjugation(..),
            "形容詞・イイ" => AdjectiveII(..),
            "特殊・ドス" => SpecialDosu(..),
            "一段・得ル" => IchidanEru(..),
            "特殊・ヤ" => SpecialYa(..),
        }
    }
}
pub use self::conjugation::Conjugation;
