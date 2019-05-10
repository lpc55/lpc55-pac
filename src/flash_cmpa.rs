#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ."]
    pub boot_cfg: BOOT_CFG,
    #[doc = "0x04 - ."]
    pub spi_flash_cfg: SPI_FLASH_CFG,
    #[doc = "0x08 - ."]
    pub usb_id: USB_ID,
    #[doc = "0x0c - ."]
    pub sdio_cfg: SDIO_CFG,
    #[doc = "0x10 - ."]
    pub dcfg_cc_socu_pin: DCFG_CC_SOCU_PIN,
    #[doc = "0x14 - ."]
    pub dcfg_cc_socu_dflt: DCFG_CC_SOCU_DFLT,
    #[doc = "0x18 - ."]
    pub dap_vendor_usage_fixed: DAP_VENDOR_USAGE_FIXED,
    #[doc = "0x1c - ."]
    pub secure_boot_cfg: SECURE_BOOT_CFG,
    #[doc = "0x20 - ."]
    pub prince_base_addr: PRINCE_BASE_ADDR,
    #[doc = "0x24 - Region 0, sub-region enable"]
    pub prince_sr_0: PRINCE_SR_0,
    #[doc = "0x28 - Region 1, sub-region enable"]
    pub prince_sr_1: PRINCE_SR_1,
    #[doc = "0x2c - Region 2, sub-region enable"]
    pub prince_sr_2: PRINCE_SR_2,
    _reserved0: [u8; 32usize],
    #[doc = "0x50 - ROTKH0 for Root of Trust Keys Table hash\\[255:224\\] ROTKH1 for Root of Trust Keys Table hash\\[223:192\\] ROTKH2 for Root of Trust Keys Table hash\\[191:160\\] ROTKH3 for Root of Trust Keys Table hash\\[159:128\\] ROTKH4 for Root of Trust Keys Table hash\\[127:96\\] ROTKH5 for Root of Trust Keys Table hash\\[95:64\\] ROTKH6 for Root of Trust Keys Table hash\\[63:32\\] ROTKH7 for Root of Trust Keys Table hash\\[31:0\\]"]
    pub rotkh: [ROTKH; 8],
    _reserved1: [u8; 144usize],
    #[doc = "0x100 - Customer Defined (Programable through ROM API)"]
    pub customer_defined: [CUSTOMER_DEFINED; 56],
    #[doc = "0x1e0 - SHA256_DIGEST0 for DIGEST\\[31:0\\] SHA256_DIGEST1 for DIGEST\\[63:32\\] SHA256_DIGEST2 for DIGEST\\[95:64\\] SHA256_DIGEST3 for DIGEST\\[127:96\\] SHA256_DIGEST4 for DIGEST\\[159:128\\] SHA256_DIGEST5 for DIGEST\\[191:160\\] SHA256_DIGEST6 for DIGEST\\[223:192\\] SHA256_DIGEST7 for DIGEST\\[255:224\\]"]
    pub sha256_digest: [SHA256_DIGEST; 8],
}
#[doc = "."]
pub struct BOOT_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod boot_cfg;
#[doc = "."]
pub struct SPI_FLASH_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod spi_flash_cfg;
#[doc = "."]
pub struct USB_ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod usb_id;
#[doc = "."]
pub struct SDIO_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod sdio_cfg;
#[doc = "."]
pub struct DCFG_CC_SOCU_PIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod dcfg_cc_socu_pin;
#[doc = "."]
pub struct DCFG_CC_SOCU_DFLT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod dcfg_cc_socu_dflt;
#[doc = "."]
pub struct DAP_VENDOR_USAGE_FIXED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod dap_vendor_usage_fixed;
#[doc = "."]
pub struct SECURE_BOOT_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod secure_boot_cfg;
#[doc = "."]
pub struct PRINCE_BASE_ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_base_addr;
#[doc = "Region 0, sub-region enable"]
pub struct PRINCE_SR_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Region 0, sub-region enable"]
pub mod prince_sr_0;
#[doc = "Region 1, sub-region enable"]
pub struct PRINCE_SR_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Region 1, sub-region enable"]
pub mod prince_sr_1;
#[doc = "Region 2, sub-region enable"]
pub struct PRINCE_SR_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Region 2, sub-region enable"]
pub mod prince_sr_2;
#[doc = "ROTKH0 for Root of Trust Keys Table hash\\[255:224\\] ROTKH1 for Root of Trust Keys Table hash\\[223:192\\] ROTKH2 for Root of Trust Keys Table hash\\[191:160\\] ROTKH3 for Root of Trust Keys Table hash\\[159:128\\] ROTKH4 for Root of Trust Keys Table hash\\[127:96\\] ROTKH5 for Root of Trust Keys Table hash\\[95:64\\] ROTKH6 for Root of Trust Keys Table hash\\[63:32\\] ROTKH7 for Root of Trust Keys Table hash\\[31:0\\]"]
pub struct ROTKH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ROTKH0 for Root of Trust Keys Table hash\\[255:224\\] ROTKH1 for Root of Trust Keys Table hash\\[223:192\\] ROTKH2 for Root of Trust Keys Table hash\\[191:160\\] ROTKH3 for Root of Trust Keys Table hash\\[159:128\\] ROTKH4 for Root of Trust Keys Table hash\\[127:96\\] ROTKH5 for Root of Trust Keys Table hash\\[95:64\\] ROTKH6 for Root of Trust Keys Table hash\\[63:32\\] ROTKH7 for Root of Trust Keys Table hash\\[31:0\\]"]
pub mod rotkh;
#[doc = "Customer Defined (Programable through ROM API)"]
pub struct CUSTOMER_DEFINED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Customer Defined (Programable through ROM API)"]
pub mod customer_defined;
#[doc = "SHA256_DIGEST0 for DIGEST\\[31:0\\] SHA256_DIGEST1 for DIGEST\\[63:32\\] SHA256_DIGEST2 for DIGEST\\[95:64\\] SHA256_DIGEST3 for DIGEST\\[127:96\\] SHA256_DIGEST4 for DIGEST\\[159:128\\] SHA256_DIGEST5 for DIGEST\\[191:160\\] SHA256_DIGEST6 for DIGEST\\[223:192\\] SHA256_DIGEST7 for DIGEST\\[255:224\\]"]
pub struct SHA256_DIGEST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SHA256_DIGEST0 for DIGEST\\[31:0\\] SHA256_DIGEST1 for DIGEST\\[63:32\\] SHA256_DIGEST2 for DIGEST\\[95:64\\] SHA256_DIGEST3 for DIGEST\\[127:96\\] SHA256_DIGEST4 for DIGEST\\[159:128\\] SHA256_DIGEST5 for DIGEST\\[191:160\\] SHA256_DIGEST6 for DIGEST\\[223:192\\] SHA256_DIGEST7 for DIGEST\\[255:224\\]"]
pub mod sha256_digest;
