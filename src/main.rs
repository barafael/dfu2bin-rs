use structview::{u32_le, View};

#[derive(Debug, Clone, Copy, View)]
#[repr(C)]
struct DfuSEPrefix {
    sz_signature: [u8; 5],
    b_version: u8,
    dfu_image_size: u32_le,
    b_targets: u8,
}

/*
impl Default for DfuSEPrefix {
    fn default() -> Self {
        DfuSEPrefix {
            sz_signature: ['D' as u8, 'f' as u8, 'u' as u8, 'S' as u8, 'e' as u8],
            b_version: 1,
            dfu_image_size: 0,
            b_targets: 0,
        }
    }
}
*/

#[derive(Clone, Copy, View)]
#[repr(C)]
struct SzTargetName {
    name: [u8; 256],
}

impl std::fmt::Debug for SzTargetName {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("SzTargetName")
            .field("name", &&self.name[..])
            .finish()
    }
}

#[derive(Debug, Clone, Copy, View)]
#[repr(C)]
struct DfuSETargetPrefix {
    sz_signature: [u8; 6],
    b_alternate_setting: u8,
    b_target_named: u32_le,
    sz_target_name: SzTargetName,
}

#[derive(Debug, Clone, Copy, View)]
#[repr(C)]
struct DfuSEImage {
    target_prefix: DfuSETargetPrefix,
}

#[derive(Debug, Clone, Copy, View)]
#[repr(C)]
struct DfuSESuffix {
    bcd_device_lo: u8,
    bcd_device_hi: u8,
    id_product_lo: u8,
    id_product_hi: u8,
    id_vendor_lo: u8,
    id_vendor_hi: u8,
    bcd_dfu_lo: u8,
    bcd_dfu_hi: u8,
    uc_dfu_signature: [u8; 3],
    b_length: u8,
    dw_crc: u32_le,
}

/*
impl Default for DfuSESuffix {
    fn default() -> Self {
        assert_eq!(std::mem::size_of::<Self>(), 16);
        DfuSESuffix {
            bcd_device_lo: 0xFF,
            bcd_device_hi: 0xFF,
            id_product_lo: 0xFF,
            id_product_hi: 0xFF,
            id_vendor_lo: 0xFF,
            id_vendor_hi: 0xFF,
            bcd_dfu_lo: 0x1a,
            bcd_dfu_hi: 0x01,
            uc_dfu_signature: ['U' as u8, 'F' as u8, 'D' as u8],
            b_length: std::mem::size_of::<Self>() as u8,
            dw_crc: 0,
        }
    }
}
*/

#[derive(Debug, Clone, Copy, View)]
#[repr(C)]
struct DfuSESections {
    prefix: DfuSEPrefix,
    //images: Vec<DfuSEImage>,
    suffix: DfuSESuffix,
}

#[derive(Debug, Clone, Copy, View)]
#[repr(C)]
struct DfuImageElementHeader {
    dw_element_address: u32_le,
    dw_element_size: u32_le,
}

struct DfuSETargetPrefix {
    sz_signature: [u8; 6],
    b_alternate_setting: u8,
    b_target_named: [u8; 4],
    sz_target_name: [u8; 255],
    dw_nb_elements: u32,
}

fn main() -> std::io::Result<()> {
    let bytes = std::fs::read("test.dfu")?;
    let dfu = DfuSESuffix::view(&bytes).unwrap();
    dbg!(dfu);
    Ok(())
}

