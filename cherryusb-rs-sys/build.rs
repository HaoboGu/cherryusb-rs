#[allow(dead_code)]
use cc::Build;
use core::panic;
use std::{
    env, fs,
    path::{Path, PathBuf},
    vec,
};

mod precompile_bindings;

fn main() {
    let target = env::var("TARGET").expect("Missing TARGET env var");
    println!("cargo:rustc-cfg=CURRENT_TARGET=\"{}\"", target);

    // generate_libs_for_all_targets();

    #[cfg(feature = "generate-bindings")]
    {
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        generate_lib(&target, out_path.clone());
        generate_bindings(&target, out_path.clone());
        println!("cargo:rustc-link-search={}", out_path.display());
    }

    #[cfg(not(feature = "generate-bindings"))]
    {
        // Just load generated librarys and write to target folder
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        let lib_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("src")
            .join("bindings")
            .join(target)
            .join("libcherryusb.a");

        match fs::read(lib_dir) {
            Ok(bytes) => {
                fs::write(out_dir.join("libcherryusb.a"), bytes).unwrap();
                println!("cargo:rustc-link-search={}", out_dir.display());
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    panic!(
                        "Cannot find pre-compiled library for target: {}\n",
                        env::var("TARGET").expect("Missing TARGET env var")
                    )
                } else {
                    panic!("Error reading pre-compiled library: {:?}", e);
                }
            }
        }
    }


}

fn add_all_c_files_in_dir_to_cc(build: &mut Build, path: impl AsRef<Path>) {
    for entry in glob::glob(path.as_ref().join("**/*.c").to_str().unwrap()).unwrap() {
        let path = entry.unwrap();
        if path.extension().and_then(|s| s.to_str()) == Some("c") {
            build.file(&path);
        }
    }
}

fn add_all_h_files_to_bindgen(
    mut builder: bindgen::Builder,
    path: impl AsRef<Path>,
) -> bindgen::Builder {
    for entry in glob::glob(path.as_ref().join("**/*.h").to_str().unwrap()).unwrap() {
        let path = entry.unwrap();
        if path.extension().and_then(|s| s.to_str()) == Some("h") {
            builder = builder.header(path.to_path_buf().to_str().unwrap());
        }
    }

    builder
}

fn generate_bindings(mut target: &str, out_path: PathBuf) {
    let mut build = Build::new();
    if target.contains("riscv") {
        // Get riscv toolchain path from environment variable
        let rv_path = env::var("RISCV_TOOLCHAIN_PATH").expect("Missing RISCV_TOOLCHAIN_PATH env");
        // let mut rv_path = String::new();
        // rv_path.push_str("E:\\Projects\\Rust\\sdk_env\\toolchains\\rv32imac_zicsr_zifencei_multilib_b_ext-win\\bin\\riscv32-unknown-elf-gcc.exe");
        build.compiler(rv_path);
        target = "riscv32-unknown-none-elf";
    }

    let include_paths = String::from_utf8(
        build
            .get_compiler()
            .to_command()
            .arg("-E")
            .arg("-Wp,-v")
            .arg("-xc")
            .arg("/dev/null")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to run the compiler to get paths")
            .wait_with_output()
            .expect("Failed to run the compiler to get paths")
            .stderr,
    )
    .unwrap()
    .lines()
    .filter_map(|line| line.strip_prefix(" "))
    .map(|path| format!("-I{}", path))
    .collect::<Vec<_>>();

    // Add headers that need to be included in the bindings
    let mut bindgen_builder = bindgen::Builder::default().header("usb_config.h");
    bindgen_builder = add_all_h_files_to_bindgen(bindgen_builder, "CherryUSB/core");
    bindgen_builder = add_all_h_files_to_bindgen(bindgen_builder, "CherryUSB/common");
    bindgen_builder = add_all_h_files_to_bindgen(bindgen_builder, "CherryUSB/class");

    bindgen_builder = add_all_h_files_to_bindgen(bindgen_builder, "CherryUSB/port/dwc2");


    // Other bindgen configurations
    bindgen_builder = bindgen_builder
        .rustified_enum(".*")
        .derive_default(true)
        .layout_tests(false)
        .use_core()
        .clang_args(&vec![
            "-target",
            &target,
            "-fvisibility=default",
            "-fshort-enums",
        ])
        .clang_arg(&format!("-I{}", &out_path.display()))
        .clang_arg("-ICherryUSB/core")
        .clang_arg("-ICherryUSB/common")
        .clang_arg("-ICherryUSB/port/ehci")
        .clang_arg("-ICherryUSB/port/dwc2")
        .clang_arg("-ICherryUSB/port/fsdev")
        .clang_arg("-ICherryUSB/port/musb")
        .clang_arg("-ICherryUSB/port/ohci")
        .clang_arg("-ICherryUSB/class/audio")
        .clang_arg("-ICherryUSB/class/cdc")
        .clang_arg("-ICherryUSB/class/dfu")
        .clang_arg("-ICherryUSB/class/hid")
        .clang_arg("-ICherryUSB/class/hub")
        .clang_arg("-ICherryUSB/class/midi")
        .clang_arg("-ICherryUSB/class/msc")
        .clang_arg("-ICherryUSB/class/video")
        .clang_arg("-ICherryUSB/class/vender/net")
        .clang_arg("-ICherryUSB/class/vender/serial")
        .clang_arg("-ICherryUSB/class/vender/wifi")
        .clang_args(&include_paths);

    // Generate bindings
    let bindings = bindgen_builder
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Can't write bindings!");
}

fn generate_lib(target: &str, out_path: PathBuf) {
    if !target.contains("riscv") && !target.contains("thumb") {
        panic!("Unsupported target: {}", target);
    }
    // If the directory doesn't exist, create it
    fs::create_dir_all(&out_path).expect("Unable to create dir");

    // Read `usb_config.h` from environment variable
    let usb_config_path = PathBuf::from(env::var("CHERRYUSB_CONFIG_HEADER").expect(
        "CHERRYUSB_CONFIG_HEADER env var must be set when using generate-bindings feature",
    ));
    let usb_config =
        fs::read_to_string(usb_config_path).expect("Failed to read CHERRYUSB_CONFIG_HEADER");
    fs::write(out_path.join("usb_config.h"), usb_config.as_bytes())
        .expect("Failed to write to usb_config.h");

    let mut build = Build::new();
    if target.contains("riscv") {
        // Get riscv toolchain path from environment variable
        let rv_path = env::var("RISCV_TOOLCHAIN_PATH").expect("Missing RISCV_TOOLCHAIN_PATH env");
        // rv_path.push_str("E:\\Projects\\Rust\\sdk_env\\toolchains\\rv32imac_zicsr_zifencei_multilib_b_ext-win\\bin\\riscv32-unknown-elf-gcc.exe");
        build.compiler(rv_path);
    }
    build.target(target);
    add_cherryusb_class_c_files_to_cc(&mut build);
    add_all_c_files_in_dir_to_cc(&mut build, "CherryUSB/core");
    add_all_c_files_in_dir_to_cc(&mut build, "CherryUSB/common");

    build
        .include("CherryUSB/common")
        .include("CherryUSB/core")
        .include("CherryUSB/class/audio")
        .include("CherryUSB/class/cdc")
        .include("CherryUSB/class/dfu")
        .include("CherryUSB/class/hid")
        .include("CherryUSB/class/hub")
        .include("CherryUSB/class/midi")
        .include("CherryUSB/class/msc")
        .include("CherryUSB/class/video")
        .include("CherryUSB/class/vender/net")
        .include("CherryUSB/class/vender/serial")
        .include("CherryUSB/class/vender/wifi")
        .include(&out_path);

    #[cfg(feature = "ehci")]
    build
        .include("CherryUSB/port/ehci")
        .file("CherryUSB/port/ehci/usb_hc_ehci.c")
        .include("CherryUSB/port/ohci")
        .file("CherryUSB/port/ohci/usb_hc_ohci.c");

    #[cfg(feature = "dwc2")]
    build
        .include("CherryUSB/port/dwc2")
        .file("CherryUSB/port/dwc2/usb_dc_dwc2.c")
        .file("CherryUSB/port/dwc2/usb_hc_dwc2.c")
        .file("CherryUSB/port/dwc2/usb_glue_st.c");

    #[cfg(feature = "fsdev")]
    build
        .include("CherryUSB/port/fsdev")
        .file("CherryUSB/port/fsdev/usb_dc_fsdev.c");

    #[cfg(feature = "musb")]
    build
        .file("CherryUSB/port/musb/usb_dc_musb.c")
        .include("CherryUSB/port/musb")
        .file("CherryUSB/port/musb/usb_hc_musb.c");

    build
        .flag("-Wno-unused-but-set-variable")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-unused-variable")
        .out_dir(out_path)
        .compile("cherryusb");
}

// Generate library files for all targets, save them in `src/bindings`
fn generate_libs_for_all_targets() {
    let targets = vec![
        "riscv32imac-unknown-none-elf",
        "riscv32i-unknown-none-elf",
        "riscv32im-unknown-none-elf",
        "riscv32imc-unknown-none-elf",
        "riscv32imafc-unknown-none-elf",
        "thumbv6m-none-eabi",
        "thumbv7em-none-eabi",
        "thumbv7em-none-eabihf",
        "thumbv7m-none-eabi",
        "thumbv8m.base-none-eabi",
        "thumbv8m.main-none-eabi",
        "thumbv8m.main-none-eabihf",
    ];
    let out_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("src")
        .join("bindings");
    for target in targets {
        generate_lib(target, out_path.join(target));
    }
}

fn add_cherryusb_class_c_files_to_cc(build: &mut Build) {
    add_all_c_files_in_dir_to_cc(build, "CherryUSB/class/audio");
    add_all_c_files_in_dir_to_cc(build, "CherryUSB/class/cdc");
    add_all_c_files_in_dir_to_cc(build, "CherryUSB/class/dfu");
    add_all_c_files_in_dir_to_cc(build, "CherryUSB/class/hid");
    add_all_c_files_in_dir_to_cc(build, "CherryUSB/class/hub");
    add_all_c_files_in_dir_to_cc(build, "CherryUSB/class/midi");
    add_all_c_files_in_dir_to_cc(build, "CherryUSB/class/msc");
    add_all_c_files_in_dir_to_cc(build, "CherryUSB/class/video");
    add_all_c_files_in_dir_to_cc(build, "CherryUSB/class/vender/net");
    add_all_c_files_in_dir_to_cc(build, "CherryUSB/class/vender/serial");
    add_all_c_files_in_dir_to_cc(build, "CherryUSB/class/vender/wifi");
}
