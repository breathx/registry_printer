mod util;

mod types {
    #![allow(unused)]

    use scale_info::TypeInfo;

    #[derive(TypeInfo)]
    pub struct X((u8, u16));
}

#[allow(unused_imports)]
use types::*;

fn main() {
    let registry = registry!(String, X);
    util::print_registry(registry);
}
