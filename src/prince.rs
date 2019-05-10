#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Encryption Enable register"]
    pub enc_enable: ENC_ENABLE,
    #[doc = "0x04 - Data Mask register, 32 Least Significant Bits"]
    pub mask_lsb: MASK_LSB,
    #[doc = "0x08 - Data Mask register, 32 Most Significant Bits"]
    pub mask_msb: MASK_MSB,
    #[doc = "0x0c - Lock register"]
    pub lock: LOCK,
    #[doc = "0x10 - Initial Vector register for region 0, Least Significant Bits"]
    pub iv_lsb0: IV_LSB0,
    #[doc = "0x14 - Initial Vector register for region 0, Most Significant Bits"]
    pub iv_msb0: IV_MSB0,
    #[doc = "0x18 - Base Address for region 0 register"]
    pub base_addr0: BASE_ADDR0,
    #[doc = "0x1c - Sub-Region Enable register for region 0"]
    pub sr_enable0: SR_ENABLE0,
    #[doc = "0x20 - Initial Vector register for region 1, Least Significant Bits"]
    pub iv_lsb1: IV_LSB1,
    #[doc = "0x24 - Initial Vector register for region 1, Most Significant Bits"]
    pub iv_msb1: IV_MSB1,
    #[doc = "0x28 - Base Address for region 1 register"]
    pub base_addr1: BASE_ADDR1,
    #[doc = "0x2c - Sub-Region Enable register for region 1"]
    pub sr_enable1: SR_ENABLE1,
    #[doc = "0x30 - Initial Vector register for region 2, Least Significant Bits"]
    pub iv_lsb2: IV_LSB2,
    #[doc = "0x34 - Initial Vector register for region 2, Most Significant Bits"]
    pub iv_msb2: IV_MSB2,
    #[doc = "0x38 - Base Address for region 2 register"]
    pub base_addr2: BASE_ADDR2,
    #[doc = "0x3c - Sub-Region Enable register for region 2"]
    pub sr_enable2: SR_ENABLE2,
}
#[doc = "Encryption Enable register"]
pub struct ENC_ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Encryption Enable register"]
pub mod enc_enable;
#[doc = "Data Mask register, 32 Least Significant Bits"]
pub struct MASK_LSB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Mask register, 32 Least Significant Bits"]
pub mod mask_lsb;
#[doc = "Data Mask register, 32 Most Significant Bits"]
pub struct MASK_MSB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Mask register, 32 Most Significant Bits"]
pub mod mask_msb;
#[doc = "Lock register"]
pub struct LOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Lock register"]
pub mod lock;
#[doc = "Initial Vector register for region 0, Least Significant Bits"]
pub struct IV_LSB0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Initial Vector register for region 0, Least Significant Bits"]
pub mod iv_lsb0;
#[doc = "Initial Vector register for region 0, Most Significant Bits"]
pub struct IV_MSB0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Initial Vector register for region 0, Most Significant Bits"]
pub mod iv_msb0;
#[doc = "Base Address for region 0 register"]
pub struct BASE_ADDR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Base Address for region 0 register"]
pub mod base_addr0;
#[doc = "Sub-Region Enable register for region 0"]
pub struct SR_ENABLE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sub-Region Enable register for region 0"]
pub mod sr_enable0;
#[doc = "Initial Vector register for region 1, Least Significant Bits"]
pub struct IV_LSB1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Initial Vector register for region 1, Least Significant Bits"]
pub mod iv_lsb1;
#[doc = "Initial Vector register for region 1, Most Significant Bits"]
pub struct IV_MSB1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Initial Vector register for region 1, Most Significant Bits"]
pub mod iv_msb1;
#[doc = "Base Address for region 1 register"]
pub struct BASE_ADDR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Base Address for region 1 register"]
pub mod base_addr1;
#[doc = "Sub-Region Enable register for region 1"]
pub struct SR_ENABLE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sub-Region Enable register for region 1"]
pub mod sr_enable1;
#[doc = "Initial Vector register for region 2, Least Significant Bits"]
pub struct IV_LSB2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Initial Vector register for region 2, Least Significant Bits"]
pub mod iv_lsb2;
#[doc = "Initial Vector register for region 2, Most Significant Bits"]
pub struct IV_MSB2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Initial Vector register for region 2, Most Significant Bits"]
pub mod iv_msb2;
#[doc = "Base Address for region 2 register"]
pub struct BASE_ADDR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Base Address for region 2 register"]
pub mod base_addr2;
#[doc = "Sub-Region Enable register for region 2"]
pub struct SR_ENABLE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sub-Region Enable register for region 2"]
pub mod sr_enable2;
