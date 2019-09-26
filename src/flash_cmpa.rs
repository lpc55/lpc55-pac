#[doc = r"Register block"]
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
    pub cc_socu_pin: CC_SOCU_PIN,
    #[doc = "0x14 - ."]
    pub cc_socu_dflt: CC_SOCU_DFLT,
    #[doc = "0x18 - ."]
    pub vendor_usage: VENDOR_USAGE,
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
    #[doc = "0x30 - Xtal 32kHz capabank triming."]
    pub xtal_32khz_capabank_trim: XTAL_32KHZ_CAPABANK_TRIM,
    #[doc = "0x34 - Xtal 16MHz capabank triming."]
    pub xtal_16mhz_capabank_trim: XTAL_16MHZ_CAPABANK_TRIM,
    _reserved14: [u8; 24usize],
    #[doc = "0x50 - ROTKH0 for Root of Trust Keys Table hash\\[255:224\\] ROTKH1 for Root of Trust Keys Table hash\\[223:192\\] ROTKH2 for Root of Trust Keys Table hash\\[191:160\\] ROTKH3 for Root of Trust Keys Table hash\\[159:128\\] ROTKH4 for Root of Trust Keys Table hash\\[127:96\\] ROTKH5 for Root of Trust Keys Table hash\\[95:64\\] ROTKH6 for Root of Trust Keys Table hash\\[63:32\\] ROTKH7 for Root of Trust Keys Table hash\\[31:0\\]"]
    pub rotkh: [ROTKH; 8],
    _reserved15: [u8; 144usize],
    #[doc = "0x100 - Customer Defined (Programable through ROM API)"]
    pub customer_defined: [CUSTOMER_DEFINED; 56],
    #[doc = "0x1e0 - SHA256_DIGEST0 for DIGEST\\[31:0\\] SHA256_DIGEST1 for DIGEST\\[63:32\\] SHA256_DIGEST2 for DIGEST\\[95:64\\] SHA256_DIGEST3 for DIGEST\\[127:96\\] SHA256_DIGEST4 for DIGEST\\[159:128\\] SHA256_DIGEST5 for DIGEST\\[191:160\\] SHA256_DIGEST6 for DIGEST\\[223:192\\] SHA256_DIGEST7 for DIGEST\\[255:224\\]"]
    pub sha256_digest: [SHA256_DIGEST; 8],
}
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [boot_cfg](boot_cfg) module"]
pub type BOOT_CFG = crate::Reg<u32, _BOOT_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BOOT_CFG;
#[doc = "`read()` method returns [boot_cfg::R](boot_cfg::R) reader structure"]
impl crate::Readable for BOOT_CFG {}
#[doc = "`write(|w| ..)` method takes [boot_cfg::W](boot_cfg::W) writer structure"]
impl crate::Writable for BOOT_CFG {}
#[doc = "."]
pub mod boot_cfg;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_flash_cfg](spi_flash_cfg) module"]
pub type SPI_FLASH_CFG = crate::Reg<u32, _SPI_FLASH_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_FLASH_CFG;
#[doc = "`read()` method returns [spi_flash_cfg::R](spi_flash_cfg::R) reader structure"]
impl crate::Readable for SPI_FLASH_CFG {}
#[doc = "`write(|w| ..)` method takes [spi_flash_cfg::W](spi_flash_cfg::W) writer structure"]
impl crate::Writable for SPI_FLASH_CFG {}
#[doc = "."]
pub mod spi_flash_cfg;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usb_id](usb_id) module"]
pub type USB_ID = crate::Reg<u32, _USB_ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_ID;
#[doc = "`read()` method returns [usb_id::R](usb_id::R) reader structure"]
impl crate::Readable for USB_ID {}
#[doc = "`write(|w| ..)` method takes [usb_id::W](usb_id::W) writer structure"]
impl crate::Writable for USB_ID {}
#[doc = "."]
pub mod usb_id;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sdio_cfg](sdio_cfg) module"]
pub type SDIO_CFG = crate::Reg<u32, _SDIO_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDIO_CFG;
#[doc = "`read()` method returns [sdio_cfg::R](sdio_cfg::R) reader structure"]
impl crate::Readable for SDIO_CFG {}
#[doc = "`write(|w| ..)` method takes [sdio_cfg::W](sdio_cfg::W) writer structure"]
impl crate::Writable for SDIO_CFG {}
#[doc = "."]
pub mod sdio_cfg;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_socu_pin](cc_socu_pin) module"]
pub type CC_SOCU_PIN = crate::Reg<u32, _CC_SOCU_PIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_SOCU_PIN;
#[doc = "`read()` method returns [cc_socu_pin::R](cc_socu_pin::R) reader structure"]
impl crate::Readable for CC_SOCU_PIN {}
#[doc = "`write(|w| ..)` method takes [cc_socu_pin::W](cc_socu_pin::W) writer structure"]
impl crate::Writable for CC_SOCU_PIN {}
#[doc = "."]
pub mod cc_socu_pin;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc_socu_dflt](cc_socu_dflt) module"]
pub type CC_SOCU_DFLT = crate::Reg<u32, _CC_SOCU_DFLT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_SOCU_DFLT;
#[doc = "`read()` method returns [cc_socu_dflt::R](cc_socu_dflt::R) reader structure"]
impl crate::Readable for CC_SOCU_DFLT {}
#[doc = "`write(|w| ..)` method takes [cc_socu_dflt::W](cc_socu_dflt::W) writer structure"]
impl crate::Writable for CC_SOCU_DFLT {}
#[doc = "."]
pub mod cc_socu_dflt;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [vendor_usage](vendor_usage) module"]
pub type VENDOR_USAGE = crate::Reg<u32, _VENDOR_USAGE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VENDOR_USAGE;
#[doc = "`read()` method returns [vendor_usage::R](vendor_usage::R) reader structure"]
impl crate::Readable for VENDOR_USAGE {}
#[doc = "`write(|w| ..)` method takes [vendor_usage::W](vendor_usage::W) writer structure"]
impl crate::Writable for VENDOR_USAGE {}
#[doc = "."]
pub mod vendor_usage;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [secure_boot_cfg](secure_boot_cfg) module"]
pub type SECURE_BOOT_CFG = crate::Reg<u32, _SECURE_BOOT_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECURE_BOOT_CFG;
#[doc = "`read()` method returns [secure_boot_cfg::R](secure_boot_cfg::R) reader structure"]
impl crate::Readable for SECURE_BOOT_CFG {}
#[doc = "`write(|w| ..)` method takes [secure_boot_cfg::W](secure_boot_cfg::W) writer structure"]
impl crate::Writable for SECURE_BOOT_CFG {}
#[doc = "."]
pub mod secure_boot_cfg;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_base_addr](prince_base_addr) module"]
pub type PRINCE_BASE_ADDR = crate::Reg<u32, _PRINCE_BASE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_BASE_ADDR;
#[doc = "`read()` method returns [prince_base_addr::R](prince_base_addr::R) reader structure"]
impl crate::Readable for PRINCE_BASE_ADDR {}
#[doc = "`write(|w| ..)` method takes [prince_base_addr::W](prince_base_addr::W) writer structure"]
impl crate::Writable for PRINCE_BASE_ADDR {}
#[doc = "."]
pub mod prince_base_addr;
#[doc = "Region 0, sub-region enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_sr_0](prince_sr_0) module"]
pub type PRINCE_SR_0 = crate::Reg<u32, _PRINCE_SR_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_SR_0;
#[doc = "`read()` method returns [prince_sr_0::R](prince_sr_0::R) reader structure"]
impl crate::Readable for PRINCE_SR_0 {}
#[doc = "`write(|w| ..)` method takes [prince_sr_0::W](prince_sr_0::W) writer structure"]
impl crate::Writable for PRINCE_SR_0 {}
#[doc = "Region 0, sub-region enable"]
pub mod prince_sr_0;
#[doc = "Region 1, sub-region enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_sr_1](prince_sr_1) module"]
pub type PRINCE_SR_1 = crate::Reg<u32, _PRINCE_SR_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_SR_1;
#[doc = "`read()` method returns [prince_sr_1::R](prince_sr_1::R) reader structure"]
impl crate::Readable for PRINCE_SR_1 {}
#[doc = "`write(|w| ..)` method takes [prince_sr_1::W](prince_sr_1::W) writer structure"]
impl crate::Writable for PRINCE_SR_1 {}
#[doc = "Region 1, sub-region enable"]
pub mod prince_sr_1;
#[doc = "Region 2, sub-region enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_sr_2](prince_sr_2) module"]
pub type PRINCE_SR_2 = crate::Reg<u32, _PRINCE_SR_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_SR_2;
#[doc = "`read()` method returns [prince_sr_2::R](prince_sr_2::R) reader structure"]
impl crate::Readable for PRINCE_SR_2 {}
#[doc = "`write(|w| ..)` method takes [prince_sr_2::W](prince_sr_2::W) writer structure"]
impl crate::Writable for PRINCE_SR_2 {}
#[doc = "Region 2, sub-region enable"]
pub mod prince_sr_2;
#[doc = "Xtal 32kHz capabank triming.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xtal_32khz_capabank_trim](xtal_32khz_capabank_trim) module"]
pub type XTAL_32KHZ_CAPABANK_TRIM = crate::Reg<u32, _XTAL_32KHZ_CAPABANK_TRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XTAL_32KHZ_CAPABANK_TRIM;
#[doc = "`read()` method returns [xtal_32khz_capabank_trim::R](xtal_32khz_capabank_trim::R) reader structure"]
impl crate::Readable for XTAL_32KHZ_CAPABANK_TRIM {}
#[doc = "`write(|w| ..)` method takes [xtal_32khz_capabank_trim::W](xtal_32khz_capabank_trim::W) writer structure"]
impl crate::Writable for XTAL_32KHZ_CAPABANK_TRIM {}
#[doc = "Xtal 32kHz capabank triming."]
pub mod xtal_32khz_capabank_trim;
#[doc = "Xtal 16MHz capabank triming.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xtal_16mhz_capabank_trim](xtal_16mhz_capabank_trim) module"]
pub type XTAL_16MHZ_CAPABANK_TRIM = crate::Reg<u32, _XTAL_16MHZ_CAPABANK_TRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XTAL_16MHZ_CAPABANK_TRIM;
#[doc = "`read()` method returns [xtal_16mhz_capabank_trim::R](xtal_16mhz_capabank_trim::R) reader structure"]
impl crate::Readable for XTAL_16MHZ_CAPABANK_TRIM {}
#[doc = "`write(|w| ..)` method takes [xtal_16mhz_capabank_trim::W](xtal_16mhz_capabank_trim::W) writer structure"]
impl crate::Writable for XTAL_16MHZ_CAPABANK_TRIM {}
#[doc = "Xtal 16MHz capabank triming."]
pub mod xtal_16mhz_capabank_trim;
#[doc = "ROTKH0 for Root of Trust Keys Table hash\\[255:224\\] ROTKH1 for Root of Trust Keys Table hash\\[223:192\\] ROTKH2 for Root of Trust Keys Table hash\\[191:160\\] ROTKH3 for Root of Trust Keys Table hash\\[159:128\\] ROTKH4 for Root of Trust Keys Table hash\\[127:96\\] ROTKH5 for Root of Trust Keys Table hash\\[95:64\\] ROTKH6 for Root of Trust Keys Table hash\\[63:32\\] ROTKH7 for Root of Trust Keys Table hash\\[31:0\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rotkh](rotkh) module"]
pub type ROTKH = crate::Reg<u32, _ROTKH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROTKH;
#[doc = "`read()` method returns [rotkh::R](rotkh::R) reader structure"]
impl crate::Readable for ROTKH {}
#[doc = "`write(|w| ..)` method takes [rotkh::W](rotkh::W) writer structure"]
impl crate::Writable for ROTKH {}
#[doc = "ROTKH0 for Root of Trust Keys Table hash\\[255:224\\] ROTKH1 for Root of Trust Keys Table hash\\[223:192\\] ROTKH2 for Root of Trust Keys Table hash\\[191:160\\] ROTKH3 for Root of Trust Keys Table hash\\[159:128\\] ROTKH4 for Root of Trust Keys Table hash\\[127:96\\] ROTKH5 for Root of Trust Keys Table hash\\[95:64\\] ROTKH6 for Root of Trust Keys Table hash\\[63:32\\] ROTKH7 for Root of Trust Keys Table hash\\[31:0\\]"]
pub mod rotkh;
#[doc = "Customer Defined (Programable through ROM API)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [customer_defined](customer_defined) module"]
pub type CUSTOMER_DEFINED = crate::Reg<u32, _CUSTOMER_DEFINED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUSTOMER_DEFINED;
#[doc = "`read()` method returns [customer_defined::R](customer_defined::R) reader structure"]
impl crate::Readable for CUSTOMER_DEFINED {}
#[doc = "`write(|w| ..)` method takes [customer_defined::W](customer_defined::W) writer structure"]
impl crate::Writable for CUSTOMER_DEFINED {}
#[doc = "Customer Defined (Programable through ROM API)"]
pub mod customer_defined;
#[doc = "SHA256_DIGEST0 for DIGEST\\[31:0\\] SHA256_DIGEST1 for DIGEST\\[63:32\\] SHA256_DIGEST2 for DIGEST\\[95:64\\] SHA256_DIGEST3 for DIGEST\\[127:96\\] SHA256_DIGEST4 for DIGEST\\[159:128\\] SHA256_DIGEST5 for DIGEST\\[191:160\\] SHA256_DIGEST6 for DIGEST\\[223:192\\] SHA256_DIGEST7 for DIGEST\\[255:224\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sha256_digest](sha256_digest) module"]
pub type SHA256_DIGEST = crate::Reg<u32, _SHA256_DIGEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHA256_DIGEST;
#[doc = "`read()` method returns [sha256_digest::R](sha256_digest::R) reader structure"]
impl crate::Readable for SHA256_DIGEST {}
#[doc = "`write(|w| ..)` method takes [sha256_digest::W](sha256_digest::W) writer structure"]
impl crate::Writable for SHA256_DIGEST {}
#[doc = "SHA256_DIGEST0 for DIGEST\\[31:0\\] SHA256_DIGEST1 for DIGEST\\[63:32\\] SHA256_DIGEST2 for DIGEST\\[95:64\\] SHA256_DIGEST3 for DIGEST\\[127:96\\] SHA256_DIGEST4 for DIGEST\\[159:128\\] SHA256_DIGEST5 for DIGEST\\[191:160\\] SHA256_DIGEST6 for DIGEST\\[223:192\\] SHA256_DIGEST7 for DIGEST\\[255:224\\]"]
pub mod sha256_digest;
