use crate::define_enum;

define_enum! {
    pub enum Verb features[1] {
        "自立" => Independent,
        "接尾" => Suffix,
        "非自立" => Dependent,
    }
}
