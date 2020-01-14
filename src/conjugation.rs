pub mod conjugation {
    define_enum! {
        pub enum ConjugationKind features[4] {
            "*" => None,
            "カ変・クル" => KahenKuru,
            "カ変・来ル" => KahenKuruKanji,
            "サ変・スル" => SahenSuru,
            "サ変・−スル" => SahenSuruConnected,
            "サ変・−ズル" => SahenZuruConnected,
            "一段" => Ichidan,
            "一段・病メル" => IchidanYameru,
            "五段・カ行イ音便" => GodanKaIonbin,
            "五段・カ行促音便" => GodanKaSokuonbin,
            "五段・カ行促音便ユク" => GodanKaSokuonbinYuku,
            "五段・ガ行" => GodanGa,
            "五段・サ行" => GodanSa,
            "五段・タ行" => GodanTa,
            "五段・ナ行" => GodanNa,
            "五段・バ行" => GodanBa,
            "五段・マ行" => GodanMa,
            "五段・ラ行" => GodanRa,
            "五段・ラ行アル" => GodanRaAru,
            "五段・ラ行特殊" => GodanRaSpecial,
            "五段・ワ行ウ音便" => GodanWaUonbin,
            "五段・ワ行促音便" => GodanWaSokuonbin,
            "四段・カ行" => YodanKa,
            "四段・ガ行" => YodanGa,
            "四段・サ行" => YodanSa,
            "四段・タ行" => YodanTa,
            "四段・バ行" => YodanBa,
            "四段・マ行" => YodanMa,
            "四段・ラ行" => YodanRa,
            "四段・ハ行" => YodanHa,
            "ラ変" => Rahen,
            "上二・ダ行" => KaminiDa,
            "上二・ハ行" => KaminiHa,
            "下二・ア行" => ShimoniA,
            "下二・カ行" => ShimoniKa,
            "下二・ガ行" => ShimoniGa,
            "下二・サ行" => ShimoniSa,
            "下二・ザ行" => ShimoniZa,
            "下二・タ行" => ShimoniTa,
            "下二・ダ行" => ShimoniDa,
            "下二・ナ行" => ShimoniNa,
            "下二・ハ行" => ShimoniHa,
            "下二・バ行" => ShimoniBa,
            "下二・マ行" => ShimoniMa,
            "下二・ヤ行" => ShimoniYa,
            "下二・ラ行" => ShimoniRa,
            "下二・ワ行" => ShimoniWa,
            "下二・得" => ShimoniU,
            "一段・クレル" => IchidanKureru,
            "一段・ル" => IchidanRu,
            "形容詞・アウオ段" => AdjectiveAUO,
            "形容詞・イ段" => AdjectiveI,
            "特殊・ナイ" => SpecialNai,
            "特殊・タイ" => SpecialTai,
            "特殊・タ" => SpecialTa,
            "特殊・ダ" => SpecialDa,
            "特殊・デス" => SpecialDesu,
            "特殊・ジャ" => SpecialJa,
            "特殊・マス" => SpecialMasu,
            "特殊・ヌ" => SpecialNu,
            "文語・ベシ" => OldBeshi,
            "文語・ゴトシ" => OldGotoshi,
            "文語・ナリ" => OldNari,
            "文語・マジ" => OldMaji,
            "文語・シム" => OldShimu,
            "文語・キ" => OldKi,
            "文語・ケリ" => OldKeri,
            "文語・リ" => OldRi,
            "文語・ル" => OldRu,
            "不変化型" => NoConjugation,
            "形容詞・イイ" => AdjectiveII,
            "特殊・ドス" => SpecialDosu,
            "一段・得ル" => IchidanEru,
            "特殊・ヤ" => SpecialYa,
        }
    }

    define_enum! {
        pub enum ConjugationForm features[5] {
            "*" => None,
            "ガル接続" => Garu,
            "仮定形" => Conditional,
            "仮定縮約１" => ConditionalContraction1,
            "仮定縮約２" => ConditionalContraction2,
            "体言接続" => AttributiveConjunction,
            "体言接続特殊" => SpecialAttributive,
            "体言接続特殊２" => SpecialAttributive2,
            "命令ｅ" => ImperativeE,
            "命令ｉ" => ImperativeI,
            "命令ｒｏ" => ImperativeRo,
            "命令ｙｏ" => ImperativeYo,
            "基本形" => Basic,
            "基本形-促音便" => BasicSokuonbin,
            "文語基本形" => OldBasic,
            "未然ウ接続" => NegativeU,
            "未然ヌ接続" => NegativeNu,
            "未然レル接続" => NegativeReru,
            "未然形" => Negative,
            "未然特殊" => NegativeSpecial,
            "現代基本形" => ModernBasic,
            "連用ゴザイ接続" => ContinuousGozai,
            "連用タ接続" => ContinuousTa,
            "連用テ接続" => ContinuousTe,
            "連用デ接続" => ContinuousDe,
            "連用ニ接続" => ContinuousNi,
            "連用形" => Continuous,
            "音便基本形" => OnbinBasic,
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    pub struct Conjugation {
        pub kind: ConjugationKind,
        pub form: ConjugationForm,
    }

    impl Conjugation {
        pub fn parse(features: &[&str]) -> Conjugation {
            Conjugation {
                kind: ConjugationKind::parse(features),
                form: ConjugationForm::parse(features),
            }
        }
    }
}
pub use self::conjugation::Conjugation;
