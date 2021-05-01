#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Encryption Enable register"]
    pub enc_enable: crate::Reg<enc_enable::ENC_ENABLE_SPEC>,
    #[doc = "0x04 - Data Mask register, 32 Least Significant Bits"]
    pub mask_lsb: crate::Reg<mask_lsb::MASK_LSB_SPEC>,
    #[doc = "0x08 - Data Mask register, 32 Most Significant Bits"]
    pub mask_msb: crate::Reg<mask_msb::MASK_MSB_SPEC>,
    #[doc = "0x0c - Lock register"]
    pub lock: crate::Reg<lock::LOCK_SPEC>,
    #[doc = "0x10 - Initial Vector register for region 0, Least Significant Bits"]
    pub iv_lsb0: crate::Reg<iv_lsb0::IV_LSB0_SPEC>,
    #[doc = "0x14 - Initial Vector register for region 0, Most Significant Bits"]
    pub iv_msb0: crate::Reg<iv_msb0::IV_MSB0_SPEC>,
    #[doc = "0x18 - Base Address for region 0 register"]
    pub base_addr0: crate::Reg<base_addr0::BASE_ADDR0_SPEC>,
    #[doc = "0x1c - Sub-Region Enable register for region 0"]
    pub sr_enable0: crate::Reg<sr_enable0::SR_ENABLE0_SPEC>,
    #[doc = "0x20 - Initial Vector register for region 1, Least Significant Bits"]
    pub iv_lsb1: crate::Reg<iv_lsb1::IV_LSB1_SPEC>,
    #[doc = "0x24 - Initial Vector register for region 1, Most Significant Bits"]
    pub iv_msb1: crate::Reg<iv_msb1::IV_MSB1_SPEC>,
    #[doc = "0x28 - Base Address for region 1 register"]
    pub base_addr1: crate::Reg<base_addr1::BASE_ADDR1_SPEC>,
    #[doc = "0x2c - Sub-Region Enable register for region 1"]
    pub sr_enable1: crate::Reg<sr_enable1::SR_ENABLE1_SPEC>,
    #[doc = "0x30 - Initial Vector register for region 2, Least Significant Bits"]
    pub iv_lsb2: crate::Reg<iv_lsb2::IV_LSB2_SPEC>,
    #[doc = "0x34 - Initial Vector register for region 2, Most Significant Bits"]
    pub iv_msb2: crate::Reg<iv_msb2::IV_MSB2_SPEC>,
    #[doc = "0x38 - Base Address for region 2 register"]
    pub base_addr2: crate::Reg<base_addr2::BASE_ADDR2_SPEC>,
    #[doc = "0x3c - Sub-Region Enable register for region 2"]
    pub sr_enable2: crate::Reg<sr_enable2::SR_ENABLE2_SPEC>,
}
#[doc = "ENC_ENABLE register accessor: an alias for `Reg<ENC_ENABLE_SPEC>`"]
pub type ENC_ENABLE = crate::Reg<enc_enable::ENC_ENABLE_SPEC>;
#[doc = "Encryption Enable register"]
pub mod enc_enable;
#[doc = "MASK_LSB register accessor: an alias for `Reg<MASK_LSB_SPEC>`"]
pub type MASK_LSB = crate::Reg<mask_lsb::MASK_LSB_SPEC>;
#[doc = "Data Mask register, 32 Least Significant Bits"]
pub mod mask_lsb;
#[doc = "MASK_MSB register accessor: an alias for `Reg<MASK_MSB_SPEC>`"]
pub type MASK_MSB = crate::Reg<mask_msb::MASK_MSB_SPEC>;
#[doc = "Data Mask register, 32 Most Significant Bits"]
pub mod mask_msb;
#[doc = "LOCK register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Lock register"]
pub mod lock;
#[doc = "IV_LSB0 register accessor: an alias for `Reg<IV_LSB0_SPEC>`"]
pub type IV_LSB0 = crate::Reg<iv_lsb0::IV_LSB0_SPEC>;
#[doc = "Initial Vector register for region 0, Least Significant Bits"]
pub mod iv_lsb0;
#[doc = "IV_MSB0 register accessor: an alias for `Reg<IV_MSB0_SPEC>`"]
pub type IV_MSB0 = crate::Reg<iv_msb0::IV_MSB0_SPEC>;
#[doc = "Initial Vector register for region 0, Most Significant Bits"]
pub mod iv_msb0;
#[doc = "BASE_ADDR0 register accessor: an alias for `Reg<BASE_ADDR0_SPEC>`"]
pub type BASE_ADDR0 = crate::Reg<base_addr0::BASE_ADDR0_SPEC>;
#[doc = "Base Address for region 0 register"]
pub mod base_addr0;
#[doc = "SR_ENABLE0 register accessor: an alias for `Reg<SR_ENABLE0_SPEC>`"]
pub type SR_ENABLE0 = crate::Reg<sr_enable0::SR_ENABLE0_SPEC>;
#[doc = "Sub-Region Enable register for region 0"]
pub mod sr_enable0;
#[doc = "IV_LSB1 register accessor: an alias for `Reg<IV_LSB1_SPEC>`"]
pub type IV_LSB1 = crate::Reg<iv_lsb1::IV_LSB1_SPEC>;
#[doc = "Initial Vector register for region 1, Least Significant Bits"]
pub mod iv_lsb1;
#[doc = "IV_MSB1 register accessor: an alias for `Reg<IV_MSB1_SPEC>`"]
pub type IV_MSB1 = crate::Reg<iv_msb1::IV_MSB1_SPEC>;
#[doc = "Initial Vector register for region 1, Most Significant Bits"]
pub mod iv_msb1;
#[doc = "BASE_ADDR1 register accessor: an alias for `Reg<BASE_ADDR1_SPEC>`"]
pub type BASE_ADDR1 = crate::Reg<base_addr1::BASE_ADDR1_SPEC>;
#[doc = "Base Address for region 1 register"]
pub mod base_addr1;
#[doc = "SR_ENABLE1 register accessor: an alias for `Reg<SR_ENABLE1_SPEC>`"]
pub type SR_ENABLE1 = crate::Reg<sr_enable1::SR_ENABLE1_SPEC>;
#[doc = "Sub-Region Enable register for region 1"]
pub mod sr_enable1;
#[doc = "IV_LSB2 register accessor: an alias for `Reg<IV_LSB2_SPEC>`"]
pub type IV_LSB2 = crate::Reg<iv_lsb2::IV_LSB2_SPEC>;
#[doc = "Initial Vector register for region 2, Least Significant Bits"]
pub mod iv_lsb2;
#[doc = "IV_MSB2 register accessor: an alias for `Reg<IV_MSB2_SPEC>`"]
pub type IV_MSB2 = crate::Reg<iv_msb2::IV_MSB2_SPEC>;
#[doc = "Initial Vector register for region 2, Most Significant Bits"]
pub mod iv_msb2;
#[doc = "BASE_ADDR2 register accessor: an alias for `Reg<BASE_ADDR2_SPEC>`"]
pub type BASE_ADDR2 = crate::Reg<base_addr2::BASE_ADDR2_SPEC>;
#[doc = "Base Address for region 2 register"]
pub mod base_addr2;
#[doc = "SR_ENABLE2 register accessor: an alias for `Reg<SR_ENABLE2_SPEC>`"]
pub type SR_ENABLE2 = crate::Reg<sr_enable2::SR_ENABLE2_SPEC>;
#[doc = "Sub-Region Enable register for region 2"]
pub mod sr_enable2;
