use crate::item::{CareerItem, Link};

#[derive(Clone, Debug)]
pub struct Section {
    pub name: String,
    pub items: Vec<CareerItem>,
}

#[derive(Clone, Debug)]
pub struct Resume {
    pub links: Vec<Link>,
    pub sections: Section,
}
