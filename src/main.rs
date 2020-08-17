struct DfuPrefix {
    sz_signature: [u8; 5],
    b_version: u8,
    dfu_image_size: u32,
    b_targets: u8,
}

impl Default for DfuPrefix {
    fn default() -> Self {
        DfuPrefix {
            sz_signature: ['D' as u8, 'f' as u8, 'u' as u8, 'S' as u8, 'e' as u8],
            b_version: 1,
            dfu_image_size: 0,
            b_targets: 0,
        }
    }
}

struct DfuSuffix {
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
    dw_crc: u32,
}

impl Default for DfuSuffix {
    fn default() -> Self {
        assert_eq!(std::mem::size_of::<Self>(), 16);
        DfuSuffix {
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

struct DfuImageElementHeader {
    dw_element_address: u32,
    dw_element_size: u32,
}

struct DfuTargetPrefix {
    sz_signature: [u8; 6],
    b_alternate_setting: u8,
    b_target_named: [u8; 4],
    sz_target_name: [u8; 255],
    dw_nb_elements: u32,
}

struct DfuImage {
    target_prefix: DfuTargetPrefix,
}

fn main() {}
