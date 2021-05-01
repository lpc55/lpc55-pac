#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ."]
    pub boot_cfg: crate::Reg<boot_cfg::BOOT_CFG_SPEC>,
    #[doc = "0x04 - ."]
    pub spi_flash_cfg: crate::Reg<spi_flash_cfg::SPI_FLASH_CFG_SPEC>,
    #[doc = "0x08 - ."]
    pub usb_id: crate::Reg<usb_id::USB_ID_SPEC>,
    #[doc = "0x0c - ."]
    pub sdio_cfg: crate::Reg<sdio_cfg::SDIO_CFG_SPEC>,
    #[doc = "0x10 - ."]
    pub cc_socu_pin: crate::Reg<cc_socu_pin::CC_SOCU_PIN_SPEC>,
    #[doc = "0x14 - ."]
    pub cc_socu_dflt: crate::Reg<cc_socu_dflt::CC_SOCU_DFLT_SPEC>,
    #[doc = "0x18 - ."]
    pub vendor_usage: crate::Reg<vendor_usage::VENDOR_USAGE_SPEC>,
    #[doc = "0x1c - ."]
    pub secure_boot_cfg: crate::Reg<secure_boot_cfg::SECURE_BOOT_CFG_SPEC>,
    #[doc = "0x20 - ."]
    pub prince_base_addr: crate::Reg<prince_base_addr::PRINCE_BASE_ADDR_SPEC>,
    #[doc = "0x24 - Region 0, sub-region enable"]
    pub prince_sr_0: crate::Reg<prince_sr_0::PRINCE_SR_0_SPEC>,
    #[doc = "0x28 - Region 1, sub-region enable"]
    pub prince_sr_1: crate::Reg<prince_sr_1::PRINCE_SR_1_SPEC>,
    #[doc = "0x2c - Region 2, sub-region enable"]
    pub prince_sr_2: crate::Reg<prince_sr_2::PRINCE_SR_2_SPEC>,
    #[doc = "0x30 - Xtal 32kHz capabank triming."]
    pub xtal_32khz_capabank_trim:
        crate::Reg<xtal_32khz_capabank_trim::XTAL_32KHZ_CAPABANK_TRIM_SPEC>,
    #[doc = "0x34 - Xtal 16MHz capabank triming."]
    pub xtal_16mhz_capabank_trim:
        crate::Reg<xtal_16mhz_capabank_trim::XTAL_16MHZ_CAPABANK_TRIM_SPEC>,
    _reserved14: [u8; 24usize],
    #[doc = "0x50 - ROTKH0 for Root of Trust Keys Table hash\\[255:224\\]
ROTKH1 for Root of Trust Keys Table hash\\[223:192\\]
ROTKH2 for Root of Trust Keys Table hash\\[191:160\\]
ROTKH3 for Root of Trust Keys Table hash\\[159:128\\]
ROTKH4 for Root of Trust Keys Table hash\\[127:96\\]
ROTKH5 for Root of Trust Keys Table hash\\[95:64\\]
ROTKH6 for Root of Trust Keys Table hash\\[63:32\\]
ROTKH7 for Root of Trust Keys Table hash\\[31:0\\]"]
    pub rotkh: [crate::Reg<rotkh::ROTKH_SPEC>; 8],
    _reserved15: [u8; 144usize],
    #[doc = "0x100 - Customer Defined (Programable through ROM API)"]
    pub customer_defined: [crate::Reg<customer_defined::CUSTOMER_DEFINED_SPEC>; 56],
    #[doc = "0x1e0 - SHA256_DIGEST0 for DIGEST\\[31:0\\]
SHA256_DIGEST1 for DIGEST\\[63:32\\]
SHA256_DIGEST2 for DIGEST\\[95:64\\]
SHA256_DIGEST3 for DIGEST\\[127:96\\]
SHA256_DIGEST4 for DIGEST\\[159:128\\]
SHA256_DIGEST5 for DIGEST\\[191:160\\]
SHA256_DIGEST6 for DIGEST\\[223:192\\]
SHA256_DIGEST7 for DIGEST\\[255:224\\]"]
    pub sha256_digest: [crate::Reg<sha256_digest::SHA256_DIGEST_SPEC>; 8],
}
#[doc = "BOOT_CFG register accessor: an alias for `Reg<BOOT_CFG_SPEC>`"]
pub type BOOT_CFG = crate::Reg<boot_cfg::BOOT_CFG_SPEC>;
#[doc = "."]
pub mod boot_cfg;
#[doc = "SPI_FLASH_CFG register accessor: an alias for `Reg<SPI_FLASH_CFG_SPEC>`"]
pub type SPI_FLASH_CFG = crate::Reg<spi_flash_cfg::SPI_FLASH_CFG_SPEC>;
#[doc = "."]
pub mod spi_flash_cfg;
#[doc = "USB_ID register accessor: an alias for `Reg<USB_ID_SPEC>`"]
pub type USB_ID = crate::Reg<usb_id::USB_ID_SPEC>;
#[doc = "."]
pub mod usb_id;
#[doc = "SDIO_CFG register accessor: an alias for `Reg<SDIO_CFG_SPEC>`"]
pub type SDIO_CFG = crate::Reg<sdio_cfg::SDIO_CFG_SPEC>;
#[doc = "."]
pub mod sdio_cfg;
#[doc = "CC_SOCU_PIN register accessor: an alias for `Reg<CC_SOCU_PIN_SPEC>`"]
pub type CC_SOCU_PIN = crate::Reg<cc_socu_pin::CC_SOCU_PIN_SPEC>;
#[doc = "."]
pub mod cc_socu_pin;
#[doc = "CC_SOCU_DFLT register accessor: an alias for `Reg<CC_SOCU_DFLT_SPEC>`"]
pub type CC_SOCU_DFLT = crate::Reg<cc_socu_dflt::CC_SOCU_DFLT_SPEC>;
#[doc = "."]
pub mod cc_socu_dflt;
#[doc = "VENDOR_USAGE register accessor: an alias for `Reg<VENDOR_USAGE_SPEC>`"]
pub type VENDOR_USAGE = crate::Reg<vendor_usage::VENDOR_USAGE_SPEC>;
#[doc = "."]
pub mod vendor_usage;
#[doc = "SECURE_BOOT_CFG register accessor: an alias for `Reg<SECURE_BOOT_CFG_SPEC>`"]
pub type SECURE_BOOT_CFG = crate::Reg<secure_boot_cfg::SECURE_BOOT_CFG_SPEC>;
#[doc = "."]
pub mod secure_boot_cfg;
#[doc = "PRINCE_BASE_ADDR register accessor: an alias for `Reg<PRINCE_BASE_ADDR_SPEC>`"]
pub type PRINCE_BASE_ADDR = crate::Reg<prince_base_addr::PRINCE_BASE_ADDR_SPEC>;
#[doc = "."]
pub mod prince_base_addr;
#[doc = "PRINCE_SR_0 register accessor: an alias for `Reg<PRINCE_SR_0_SPEC>`"]
pub type PRINCE_SR_0 = crate::Reg<prince_sr_0::PRINCE_SR_0_SPEC>;
#[doc = "Region 0, sub-region enable"]
pub mod prince_sr_0;
#[doc = "PRINCE_SR_1 register accessor: an alias for `Reg<PRINCE_SR_1_SPEC>`"]
pub type PRINCE_SR_1 = crate::Reg<prince_sr_1::PRINCE_SR_1_SPEC>;
#[doc = "Region 1, sub-region enable"]
pub mod prince_sr_1;
#[doc = "PRINCE_SR_2 register accessor: an alias for `Reg<PRINCE_SR_2_SPEC>`"]
pub type PRINCE_SR_2 = crate::Reg<prince_sr_2::PRINCE_SR_2_SPEC>;
#[doc = "Region 2, sub-region enable"]
pub mod prince_sr_2;
#[doc = "XTAL_32KHZ_CAPABANK_TRIM register accessor: an alias for `Reg<XTAL_32KHZ_CAPABANK_TRIM_SPEC>`"]
pub type XTAL_32KHZ_CAPABANK_TRIM =
    crate::Reg<xtal_32khz_capabank_trim::XTAL_32KHZ_CAPABANK_TRIM_SPEC>;
#[doc = "Xtal 32kHz capabank triming."]
pub mod xtal_32khz_capabank_trim;
#[doc = "XTAL_16MHZ_CAPABANK_TRIM register accessor: an alias for `Reg<XTAL_16MHZ_CAPABANK_TRIM_SPEC>`"]
pub type XTAL_16MHZ_CAPABANK_TRIM =
    crate::Reg<xtal_16mhz_capabank_trim::XTAL_16MHZ_CAPABANK_TRIM_SPEC>;
#[doc = "Xtal 16MHz capabank triming."]
pub mod xtal_16mhz_capabank_trim;
#[doc = "ROTKH register accessor: an alias for `Reg<ROTKH_SPEC>`"]
pub type ROTKH = crate::Reg<rotkh::ROTKH_SPEC>;
#[doc = "ROTKH0 for Root of Trust Keys Table hash\\[255:224\\]
ROTKH1 for Root of Trust Keys Table hash\\[223:192\\]
ROTKH2 for Root of Trust Keys Table hash\\[191:160\\]
ROTKH3 for Root of Trust Keys Table hash\\[159:128\\]
ROTKH4 for Root of Trust Keys Table hash\\[127:96\\]
ROTKH5 for Root of Trust Keys Table hash\\[95:64\\]
ROTKH6 for Root of Trust Keys Table hash\\[63:32\\]
ROTKH7 for Root of Trust Keys Table hash\\[31:0\\]"]
pub mod rotkh;
#[doc = "CUSTOMER_DEFINED register accessor: an alias for `Reg<CUSTOMER_DEFINED_SPEC>`"]
pub type CUSTOMER_DEFINED = crate::Reg<customer_defined::CUSTOMER_DEFINED_SPEC>;
#[doc = "Customer Defined (Programable through ROM API)"]
pub mod customer_defined;
#[doc = "SHA256_DIGEST register accessor: an alias for `Reg<SHA256_DIGEST_SPEC>`"]
pub type SHA256_DIGEST = crate::Reg<sha256_digest::SHA256_DIGEST_SPEC>;
#[doc = "SHA256_DIGEST0 for DIGEST\\[31:0\\]
SHA256_DIGEST1 for DIGEST\\[63:32\\]
SHA256_DIGEST2 for DIGEST\\[95:64\\]
SHA256_DIGEST3 for DIGEST\\[127:96\\]
SHA256_DIGEST4 for DIGEST\\[159:128\\]
SHA256_DIGEST5 for DIGEST\\[191:160\\]
SHA256_DIGEST6 for DIGEST\\[223:192\\]
SHA256_DIGEST7 for DIGEST\\[255:224\\]"]
pub mod sha256_digest;
