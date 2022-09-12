/// Utility functions.
use codec::Encode;
use colored::*;
use scale_info::{PortableRegistry, Registry};

pub(crate) use scale_info::MetaType;

#[macro_export]
macro_rules! registry {
    ($t1:ty $(, $t:ty) *) => {
        $crate::util::to_registry(vec![
            $crate::util::MetaType::new::<$t1>() $(,
                $crate::util::MetaType::new::<$t>()
            ) *
        ])
    };
}

pub fn to_registry(meta_types: Vec<MetaType>) -> Registry {
    let mut registry = Registry::new();
    registry.register_types(meta_types);

    registry
}

pub fn print_registry(registry: Registry) {
    let registry: PortableRegistry = registry.into();

    let json = serde_json::to_string(&registry).expect("Failed to convert registry to JSON");
    let pretty_json =
        serde_json::to_string_pretty(&registry).expect("Failed to convert registry to pretty JSON");
    let bytes = registry.encode();
    let hex = hex::encode(bytes.clone());

    println!("\n    {}\n", "REGISTRY REPRESENTATIONS".blue().bold());

    println!("  {} {}\n", ">>".blue(), "CODEC HEX".white().italic());
    println!("{}\n", format!("0x{hex}").green().italic());

    println!("  {} {}\n", ">>".blue(), "CODEC BYTES".white().italic());
    println!("{}\n", format!("{bytes:?}").green().italic());

    println!("  {} {}\n", ">>".blue(), "JSON".white().italic());
    println!("{}\n", json.green().italic());

    println!("  {} {}\n", ">>".blue(), "KNOWN TYPES".white().italic());
    for ty in registry.types() {
        let res = ty.ty().path().segments().join("::");
        if !res.is_empty() {
            println!("{}", format!("'{res}'").green().italic());
        }
    }

    println!("\n  {} {}\n", ">>".blue(), "PRETTY JSON".white().italic());
    println!("{}\n", pretty_json.green().italic());
}
