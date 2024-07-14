#[cfg(CURRENT_TARGET = "riscv32i-unknown-none-elf")]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/bindings/riscv32i-unknown-none-elf/bindings.rs"
));
#[cfg(CURRENT_TARGET = "riscv32im-unknown-none-elf")]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/bindings/riscv32im-unknown-none-elf/bindings.rs"
));
#[cfg(CURRENT_TARGET = "riscv32imac-unknown-none-elf")]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/bindings/riscv32imac-unknown-none-elf/bindings.rs"
));
#[cfg(CURRENT_TARGET = "riscv32imafc-unknown-none-elf")]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/bindings/riscv32imafc-unknown-none-elf/bindings.rs"
));
#[cfg(CURRENT_TARGET = "riscv32imc-unknown-none-elf")]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/bindings/riscv32imc-unknown-none-elf/bindings.rs"
));
#[cfg(CURRENT_TARGET = "thumbv6m-none-eabi")]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/bindings/thumbv6m-none-eabi/bindings.rs"
));
#[cfg(CURRENT_TARGET = "thumbv7em-none-eabi")]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/bindings/thumbv7em-none-eabi/bindings.rs"
));
#[cfg(CURRENT_TARGET = "thumbv7em-none-eabihf")]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/bindings/thumbv7em-none-eabihf/bindings.rs"
));
#[cfg(CURRENT_TARGET = "thumbv7m-none-eabi")]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/bindings/thumbv7m-none-eabi/bindings.rs"
));
#[cfg(CURRENT_TARGET = "thumbv8m.base-none-eabi")]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/bindings/thumbv8m.base-none-eabi/bindings.rs"
));
#[cfg(CURRENT_TARGET = "thumbv8m.main-none-eabi")]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/bindings/thumbv8m.main-none-eabi/bindings.rs"
));
#[cfg(CURRENT_TARGET = "thumbv8m.main-none-eabihf")]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/bindings/thumbv8m.main-none-eabihf/bindings.rs"
));