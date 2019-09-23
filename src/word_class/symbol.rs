use crate::define_enum;

define_enum! {
    pub enum Symbol features[1] {
        "句点" => Period,
        "読点" => Comma,
        "空白" => Whitespace,
        "アルファベット" => Alphabet,
        "一般" => General,
        "括弧開" => ParenOpen,
        "括弧閉" => ParenClose,
    }
}
