use crate::define_enum;

define_enum! {
    pub enum Conjugation features[4] {
        // 形容詞の変化形
        "形容詞・アウオ段" => AdjectiveAUO(..),
        "形容詞・イイ" => AdjectiveII(..),
        "形容詞・イ段" => AdjectiveI(..),
        "不変化型" => NoConjugation(..),

        // 動詞の変化形
        "カ変・来ル" => VerbKuru(..),
        "サ変・−スル" => VerbSuffixSuru(..),
        "サ変・−ズル" => VerbSuffixZuru(..),
        "サ変・スル" => VerbSuru(..),
        "ラ変" => VerbLaColumnIrregular(..),
        "一段" => VerbIchidan(..),
        "一段・クレル" => VerbIchidanKureru(..),
        "下二・ハ行" => VerbShimonidanHaColumn(..),
        "下二・得" => VerbShimonidanU(..),
        "五段・カ行促音便ユク" => VerbGodanKaColumnSoundChange(..),
        "五段・バ行" => VerbGodanBaColumn(..),
        "五段・マ行" => VerbGodanMaColumn(..),
        "五段・ラ行" => VerbGodanLaColumn(..),
        "五段・ラ行特殊" => VerbGodanLaColumnSpecial(..),
        "五段・ワ行促音便" => VerbGodanWaColumnSoundChane(..),

        // 助動詞の変化形
        "下二・タ行" => AuxiliaryVerbShimonidanTaColumn(..),
        // "形容詞・イ段" => ..(..),
        "五段・ラ行アル" => AuxiliaryVerbLaColumnAru(..),
        // "五段・ラ行特殊" => ..(..),
        "特殊・タ" => AuxiliaryVerbTa(..),
        "特殊・タイ" => AuxiliaryVerbTai(..),
        "特殊・ヌ" => AuxiliaryVerbNu(..),
        "特殊・マス" => AuxiliaryVerbMasu(..),
        // "不変化型" => ..(..),
        "文語・キ" => AuxiliaryVerbKi(..),
        "文語・ベシ" => AuxiliaryVerbBeshi(..),
        "文語・ル" => AuxiliaryVerbRu(..),
    }
}

// 形容詞の変化形
define_enum! {
    pub enum AdjectiveAUO features[5] {
        "ガル接続" => GaruConjunction,
        "基本形" => Base,
        "体言接続" => NoSubstantiveConjunction,
        "文語基本形" => OldBase,
        "連用ゴザイ接続" => DeclinableGozaiConjunction,
    }
}

define_enum! {
    pub enum AdjectiveII features[5] {
        "基本形-促音便" => BaseSoundChange,
    }
}

define_enum! {
    pub enum AdjectiveI features[5] {
        "ガル接続" => GaruConjunction,
        "基本形" => Base,
        "体言接続" => NoSubstantiveConjunction,
        "文語基本形" => OldBase,
        "連用ゴザイ接続" => DeclinableGozaiConjunction,
    }
}

define_enum! {
    pub enum NoConjugation features[5] {
        "基本形" => Base,
    }
}

// 動詞の変化形
define_enum! {
    pub enum VerbKuru features[5] {
        "命令ｙｏ" => Imperative,
    }
}

define_enum! {
    pub enum VerbSuffixSuru features[5] {
        "未然レル接続" => Negative,
    }
}

define_enum! {
    pub enum VerbSuffixZuru features[5] {
        "基本形" => Base,
        "文語基本形" => OldBase,
        "命令ｙｏ" => Imperative,
    }
}

define_enum! {
    pub enum VerbSuru features[5] {
        "未然レル接続" => Negative,
    }
}

define_enum! {
    pub enum VerbLaColumnIrregular features[5] {
        "体言接続" => Attributive,
    }
}

define_enum! {
    pub enum VerbIchidan features[5] {
        "基本形" => Base,
        "体言接続特殊" => Attributive,
        "未然ウ接続" => Negative,
        "命令ｙｏ" => Imperative,
    }
}

define_enum! {
    pub enum VerbIchidanKureru features[5] {
        "未然特殊" => Negative,
    }
}

define_enum! {
    pub enum VerbShimonidanHaColumn features[5] {
        "体現接続" => Attributive,
    }
}

define_enum! {
    pub enum VerbShimonidanU features[5] {
        "基本形" => Base,
        "体言接続" => Attributive,
        "未然ウ接続" => Negative,
        "命令ｙｏ" => Imperative,
    }
}

define_enum! {
    pub enum VerbGodanKaColumnSoundChange features[5] {
        "基本形" => Base,
        "未然ウ接続" => Negative,
    }
}

define_enum! {
    pub enum VerbGodanBaColumn features[5] {
        "基本形" => Base,
        "未然ウ接続" => Negative,
    }
}

define_enum! {
    pub enum VerbGodanMaColumn features[5] {
        "基本形" => Base,
        "未然ウ接続" => Negative,
    }
}

define_enum! {
    pub enum VerbGodanLaColumn features[5] {
        "体言接続特殊" => Attributive1,
        "体言接続特殊２" => Attributive2,
        "未然特殊" => Negative,
    }
}

define_enum! {
    pub enum VerbGodanLaColumnSpecial features[5] {
        "基本形" => Base,
        "未然ウ接続" => Negative,
        "未然特殊" => NegativeSpecial,
    }
}

define_enum! {
    pub enum VerbGodanWaColumnSoundChane features[5] {
        "基本形" => Base,
        "未然ウ接続" => Negative,
    }
}

// 助動詞の変化形
define_enum! {
    pub enum AuxiliaryVerbShimonidanTaColumn features[5] {
        "命令ｙｏ" => Imperative,
    }
}

define_enum! {
    pub enum AuxiliaryVerbLaColumnAru features[5] {
        "基本形" => Base,
        "体言接続特殊" => Attributive,
        "未然ウ接続" => Negative,
    }
}

define_enum! {
    pub enum AuxiliaryVerbTa features[5] {
        "基本形" => Base,
    }
}

define_enum! {
    pub enum AuxiliaryVerbTai features[5] {
        "ガル接続" => Garu,
        "体言接続" => Attributive,
        "連用ゴザイ接続" => Gozai,
    }
}

define_enum! {
    pub enum AuxiliaryVerbNu features[5] {
        "体言接続" => Attributive,
        "文語基本形" => OldBase,
    }
}

define_enum! {
    pub enum AuxiliaryVerbMasu features[5] {
        "基本形" => Base,
        "未然ウ接続" => Negative,
    }
}

// "不変化型" => ..(..),
define_enum! {
    pub enum AuxiliaryVerbKi features[5] {
        "体言接続" => Attributive,
    }
}

define_enum! {
    pub enum AuxiliaryVerbBeshi features[5] {
        "基本形" => Base,
        "体言接続" => Attributive,
    }
}

define_enum! {
    pub enum AuxiliaryVerbRu features[5] {
        "命令ｙｏ" => Imperative,
    }
}
