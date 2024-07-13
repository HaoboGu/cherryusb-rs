use cc::Build;
use std::{
    env,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

mod generate_cfg;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("Missing OUT_DIR"));

    {
        let mut f =
            File::create(out_dir.join("usb_config.h")).expect("Failed to create usb_config.h");
        f.write_all(generate_cfg::generate_cfg().as_bytes())
            .expect("Failed to write to usb_config.h");
    }

    let include_paths = String::from_utf8(
        Build::new()
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

    eprintln!("include_paths={:?}", include_paths);

    let mut build = Build::new();
    add_cherryusb_class_c_files(&mut build);
    add_all_c_files_in_dir(&mut build, "CherryUSB/core");
    add_all_c_files_in_dir(&mut build, "CherryUSB/common");

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
        .include(&out_dir);

    #[cfg(feature = "ehci")]
    build
        .include("CherryUSB/port/ehci")
        .file("CherryUSB/port/ehci/usb_hc_ehci.c");

    #[cfg(feature = "dwc2")]
    build
        .include("CherryUSB/port/dwc2")
        .file("CherryUSB/port/dwc2/usb_dc_dwc2.c")
        .file("CherryUSB/port/dwc2/usb_hc_dwc2.c");

    #[cfg(feature = "fsdev")]
    build
        .include("CherryUSB/port/fsdev")
        .file("CherryUSB/port/fsdev/usb_dc_fsdev.c");

    #[cfg(feature = "musb")]
    build
        .file("CherryUSB/port/musb/usb_dc_musb.c")
        .include("CherryUSB/port/musb")
        .file("CherryUSB/port/musb/usb_hc_musb.c");

    #[cfg(feature = "ohci")]
    build
        .include("CherryUSB/port/ohci")
        .file("CherryUSB/port/ohci/usb_hc_ohci.c");

    build.flag("-Wunused-parameter").compile("cherryusb");

    let target = env::var("TARGET").expect("Missing TARGET env var");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings = bindgen::Builder::default()
        // TODO: Which header file should be used?
        .header("usb_config.h")
        .rustified_enum(".*")
        .clang_arg(&format!("-I{}", &out_dir.display()))
        .derive_default(true)
        .layout_tests(false)
        .use_core()
        // .ctypes_prefix("cty")
        .clang_args(&vec![
            "-target",
            &target,
            "-fvisibility=default",
            "-fshort-enums",
        ])
        .clang_arg("-ICherryUSB/core")
        .clang_arg("-ICherryUSB/common")
        .clang_arg("-ICherryUSB/port/ehci")
        .clang_arg("-ICherryUSB/port/dwc2")
        .clang_arg("-ICherryUSB/port/fsdev")
        .clang_arg("-ICherryUSB/port/musb")
        .clang_arg("-ICherryUSB/port/ohci")
        .clang_args(&include_paths)
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Can't write bindings!");
}

fn add_all_c_files_in_dir(build: &mut Build, path: impl AsRef<Path>) {
    for entry in glob::glob(path.as_ref().join("**/*.c").to_str().unwrap()).unwrap() {
        let path = entry.unwrap();
        if path.extension().and_then(|s| s.to_str()) == Some("c") {
            build.file(&path);
        }
    }
}

fn add_cherryusb_class_c_files(build: &mut Build) {
    // TODO: use feature gate like `msc`, `hid`, etc
    add_all_c_files_in_dir(build, "CherryUSB/class/audio");
    add_all_c_files_in_dir(build, "CherryUSB/class/cdc");
    add_all_c_files_in_dir(build, "CherryUSB/class/dfu");
    add_all_c_files_in_dir(build, "CherryUSB/class/hid");
    add_all_c_files_in_dir(build, "CherryUSB/class/hub");
    add_all_c_files_in_dir(build, "CherryUSB/class/midi");
    add_all_c_files_in_dir(build, "CherryUSB/class/msc");
    add_all_c_files_in_dir(build, "CherryUSB/class/video");
    add_all_c_files_in_dir(build, "CherryUSB/class/vender/net");
    add_all_c_files_in_dir(build, "CherryUSB/class/vender/serial");
    add_all_c_files_in_dir(build, "CherryUSB/class/vender/wifi");
}
