use crate::define_enum;

define_enum! {
    pub enum Pronoun features[2] {
        "一般" => General,
        "縮約" => Contraction,
    }
}
