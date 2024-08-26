use fuinjutsu::{impl_method_sealed, impl_supertrait_sealed, method_sealed, supertrait_sealed};
use private_trait_dryable::Token;

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

#[method_sealed]
pub trait Dryable {
    #[sealed]
    fn dry(&self, times: i32) -> bool;

    fn dry_quickly(&self, seconds: u32) -> bool;
}

#[impl_method_sealed]
impl Dryable for Clothing {
    #[sealed]
    fn dry(&self, times: i32) -> bool {
        self.layers * times > 0
    }

    fn dry_quickly(&self, seconds: u32) -> bool {
        self.layers * seconds as i32 > 0
    }
}

#[test]
pub fn wash_succeeds() {
    let clothing = Clothing { layers: 5 };

    assert!(clothing.wash(20));
}

#[test]
pub fn dry_succeeds() {
    let clothing = Clothing { layers: 5 };

    assert!(clothing.dry(20, Token));
    assert!(clothing.dry_quickly(20));
}
