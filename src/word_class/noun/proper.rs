use crate::define_enum;

define_enum! {
    pub enum Proper features[2] {
        "一般" => General,
        "人名" => Person(..),
        "組織" => Organization,
        "地域" => Regional(..),
    }
}

define_enum! {
    pub enum Person features[3] {
        "一般" => General,
        "性" => LastName,
        "名" => FirstName,
    }
}

define_enum! {
    pub enum Regional features[3] {
        "一般" => General,
        "国" => Country,
    }
}
