use std::{fs::File, io::Read};

pub fn generate_cfg() -> String {
    // Read cfg from `usb_config.h`
    let mut cfg_file = File::open("usb_config.h").expect("Failed to open usb_config.h");
    let mut cfg = String::new();
    cfg_file
        .read_to_string(&mut cfg)
        .expect("Failed to read usb_config.h");
    return cfg;
    // Check features
//     #[cfg(all(feature = "host", feature = "device"))]
//     compile_error!("choose only host or device");
//     #[cfg(not(any(feature = "host", feature = "device")))]
//     compile_error!("select mode host or device");

//     // Must select a ip core
//     #[cfg(not(any(
//         feature = "dwc2",
//         feature = "ehci",
//         feature = "fsdev",
//         feature = "musb",
//         feature = "ohci"
//     )))]
//     compile_error!("select an ip core");

//     #[cfg(feature = "ehci")]
//     {
//         cfg.push_str("#define CONFIG_USB_EHCI_HCCR_OFFSET (0x0)\n");
//         cfg.push_str("#define CONFIG_USB_EHCI_FRAME_LIST_SIZE 1024\n");
//         cfg.push_str("#define CONFIG_USB_EHCI_QH_NUM CONFIG_USBHOST_PIPE_NUM\n");
//         cfg.push_str("#define CONFIG_USB_EHCI_QTD_NUM 3\n");
//         cfg.push_str("#define CONFIG_USB_EHCI_ITD_NUM 20\n");
//     }

//     #[cfg(feature = "dwc2")]
//     {
//         cfg.push_str("#define CONFIG_USB_DWC2_RXALL_FIFO_SIZE (1024 / 4)\n");
//         cfg.push_str("#define CONFIG_USB_DWC2_TX0_FIFO_SIZE (64 / 4)\n");
//         cfg.push_str("#define CONFIG_USB_DWC2_TX1_FIFO_SIZE (512 / 4)\n");
//         cfg.push_str("#define CONFIG_USB_DWC2_TX2_FIFO_SIZE (64 / 4)\n");
//         cfg.push_str("#define CONFIG_USB_DWC2_TX3_FIFO_SIZE (64 / 4)\n");
//         cfg.push_str("#define CONFIG_USB_DWC2_TX4_FIFO_SIZE (0 / 4)\n");
//         cfg.push_str("#define CONFIG_USB_DWC2_TX5_FIFO_SIZE (0 / 4)\n");
//         cfg.push_str("#define CONFIG_USB_DWC2_TX6_FIFO_SIZE (0 / 4)\n");
//         cfg.push_str("#define CONFIG_USB_DWC2_TX7_FIFO_SIZE (0 / 4)\n");
//         cfg.push_str("#define CONFIG_USB_DWC2_TX8_FIFO_SIZE (0 / 4)\n");
//         cfg.push_str("#define CONFIG_USB_DWC2_NPTX_FIFO_SIZE (512 / 4)\n");
//         cfg.push_str("#define CONFIG_USB_DWC2_PTX_FIFO_SIZE (1024 / 4)\n");
//         cfg.push_str("#define CONFIG_USB_DWC2_RX_FIFO_SIZE ((1012 - CONFIG_USB_DWC2_NPTX_FIFO_SIZE - CONFIG_USB_DWC2_PTX_FIFO_SIZE) / 4)\n");
//     }

//     #[cfg(feature = "fsdev")]
//     // 1 or 2, depends on the chip
//     cfg.push_str("#define CONFIG_USBDEV_FSDEV_PMA_ACCESS 2\n");

//     #[cfg(feature = "musb")]
//     cfg.push_str("#define CONFIG_USB_MUSB_SUNXI\n");

//     #[cfg(feature = "ohci")]
//     cfg.push_str(" #define CONFIG_USB_OHCI_HCOR_OFFSET (0x0)\n");

//     cfg
}
