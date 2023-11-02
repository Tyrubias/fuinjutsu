use fuinjutsu::{impl_supertrait_sealed, supertrait_sealed};

#[supertrait_sealed]
pub trait Washable {
    fn wash(&self, times: i32) -> bool;
}

pub struct Clothing {
    layers: i32,
}

#[impl_supertrait_sealed]
impl Washable for Clothing {
    fn wash(&self, times: i32) -> bool {
        self.layers * times > 0
    }
}

#[test]
pub fn wash_succeeds() {
    let clothing = Clothing { layers: 5 };

    assert!(clothing.wash(20));
}
