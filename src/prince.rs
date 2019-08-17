#[doc = r"Register block"]
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
#[doc = "Encryption Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enc_enable](enc_enable) module"]
pub type ENC_ENABLE = crate::Reg<u32, _ENC_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENC_ENABLE;
#[doc = "`read()` method returns [enc_enable::R](enc_enable::R) reader structure"]
impl crate::Readable for ENC_ENABLE {}
#[doc = "`write(|w| ..)` method takes [enc_enable::W](enc_enable::W) writer structure"]
impl crate::Writable for ENC_ENABLE {}
#[doc = "Encryption Enable register"]
pub mod enc_enable;
#[doc = "Data Mask register, 32 Least Significant Bits\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mask_lsb](mask_lsb) module"]
pub type MASK_LSB = crate::Reg<u32, _MASK_LSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK_LSB;
#[doc = "`write(|w| ..)` method takes [mask_lsb::W](mask_lsb::W) writer structure"]
impl crate::Writable for MASK_LSB {}
#[doc = "Data Mask register, 32 Least Significant Bits"]
pub mod mask_lsb;
#[doc = "Data Mask register, 32 Most Significant Bits\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mask_msb](mask_msb) module"]
pub type MASK_MSB = crate::Reg<u32, _MASK_MSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK_MSB;
#[doc = "`write(|w| ..)` method takes [mask_msb::W](mask_msb::W) writer structure"]
impl crate::Writable for MASK_MSB {}
#[doc = "Data Mask register, 32 Most Significant Bits"]
pub mod mask_msb;
#[doc = "Lock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lock](lock) module"]
pub type LOCK = crate::Reg<u32, _LOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCK;
#[doc = "`read()` method returns [lock::R](lock::R) reader structure"]
impl crate::Readable for LOCK {}
#[doc = "`write(|w| ..)` method takes [lock::W](lock::W) writer structure"]
impl crate::Writable for LOCK {}
#[doc = "Lock register"]
pub mod lock;
#[doc = "Initial Vector register for region 0, Least Significant Bits\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iv_lsb0](iv_lsb0) module"]
pub type IV_LSB0 = crate::Reg<u32, _IV_LSB0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IV_LSB0;
#[doc = "`write(|w| ..)` method takes [iv_lsb0::W](iv_lsb0::W) writer structure"]
impl crate::Writable for IV_LSB0 {}
#[doc = "Initial Vector register for region 0, Least Significant Bits"]
pub mod iv_lsb0;
#[doc = "Initial Vector register for region 0, Most Significant Bits\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iv_msb0](iv_msb0) module"]
pub type IV_MSB0 = crate::Reg<u32, _IV_MSB0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IV_MSB0;
#[doc = "`write(|w| ..)` method takes [iv_msb0::W](iv_msb0::W) writer structure"]
impl crate::Writable for IV_MSB0 {}
#[doc = "Initial Vector register for region 0, Most Significant Bits"]
pub mod iv_msb0;
#[doc = "Base Address for region 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [base_addr0](base_addr0) module"]
pub type BASE_ADDR0 = crate::Reg<u32, _BASE_ADDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BASE_ADDR0;
#[doc = "`read()` method returns [base_addr0::R](base_addr0::R) reader structure"]
impl crate::Readable for BASE_ADDR0 {}
#[doc = "`write(|w| ..)` method takes [base_addr0::W](base_addr0::W) writer structure"]
impl crate::Writable for BASE_ADDR0 {}
#[doc = "Base Address for region 0 register"]
pub mod base_addr0;
#[doc = "Sub-Region Enable register for region 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sr_enable0](sr_enable0) module"]
pub type SR_ENABLE0 = crate::Reg<u32, _SR_ENABLE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR_ENABLE0;
#[doc = "`read()` method returns [sr_enable0::R](sr_enable0::R) reader structure"]
impl crate::Readable for SR_ENABLE0 {}
#[doc = "`write(|w| ..)` method takes [sr_enable0::W](sr_enable0::W) writer structure"]
impl crate::Writable for SR_ENABLE0 {}
#[doc = "Sub-Region Enable register for region 0"]
pub mod sr_enable0;
#[doc = "Initial Vector register for region 1, Least Significant Bits\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iv_lsb1](iv_lsb1) module"]
pub type IV_LSB1 = crate::Reg<u32, _IV_LSB1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IV_LSB1;
#[doc = "`write(|w| ..)` method takes [iv_lsb1::W](iv_lsb1::W) writer structure"]
impl crate::Writable for IV_LSB1 {}
#[doc = "Initial Vector register for region 1, Least Significant Bits"]
pub mod iv_lsb1;
#[doc = "Initial Vector register for region 1, Most Significant Bits\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iv_msb1](iv_msb1) module"]
pub type IV_MSB1 = crate::Reg<u32, _IV_MSB1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IV_MSB1;
#[doc = "`write(|w| ..)` method takes [iv_msb1::W](iv_msb1::W) writer structure"]
impl crate::Writable for IV_MSB1 {}
#[doc = "Initial Vector register for region 1, Most Significant Bits"]
pub mod iv_msb1;
#[doc = "Base Address for region 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [base_addr1](base_addr1) module"]
pub type BASE_ADDR1 = crate::Reg<u32, _BASE_ADDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BASE_ADDR1;
#[doc = "`read()` method returns [base_addr1::R](base_addr1::R) reader structure"]
impl crate::Readable for BASE_ADDR1 {}
#[doc = "`write(|w| ..)` method takes [base_addr1::W](base_addr1::W) writer structure"]
impl crate::Writable for BASE_ADDR1 {}
#[doc = "Base Address for region 1 register"]
pub mod base_addr1;
#[doc = "Sub-Region Enable register for region 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sr_enable1](sr_enable1) module"]
pub type SR_ENABLE1 = crate::Reg<u32, _SR_ENABLE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR_ENABLE1;
#[doc = "`read()` method returns [sr_enable1::R](sr_enable1::R) reader structure"]
impl crate::Readable for SR_ENABLE1 {}
#[doc = "`write(|w| ..)` method takes [sr_enable1::W](sr_enable1::W) writer structure"]
impl crate::Writable for SR_ENABLE1 {}
#[doc = "Sub-Region Enable register for region 1"]
pub mod sr_enable1;
#[doc = "Initial Vector register for region 2, Least Significant Bits\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iv_lsb2](iv_lsb2) module"]
pub type IV_LSB2 = crate::Reg<u32, _IV_LSB2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IV_LSB2;
#[doc = "`write(|w| ..)` method takes [iv_lsb2::W](iv_lsb2::W) writer structure"]
impl crate::Writable for IV_LSB2 {}
#[doc = "Initial Vector register for region 2, Least Significant Bits"]
pub mod iv_lsb2;
#[doc = "Initial Vector register for region 2, Most Significant Bits\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iv_msb2](iv_msb2) module"]
pub type IV_MSB2 = crate::Reg<u32, _IV_MSB2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IV_MSB2;
#[doc = "`write(|w| ..)` method takes [iv_msb2::W](iv_msb2::W) writer structure"]
impl crate::Writable for IV_MSB2 {}
#[doc = "Initial Vector register for region 2, Most Significant Bits"]
pub mod iv_msb2;
#[doc = "Base Address for region 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [base_addr2](base_addr2) module"]
pub type BASE_ADDR2 = crate::Reg<u32, _BASE_ADDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BASE_ADDR2;
#[doc = "`read()` method returns [base_addr2::R](base_addr2::R) reader structure"]
impl crate::Readable for BASE_ADDR2 {}
#[doc = "`write(|w| ..)` method takes [base_addr2::W](base_addr2::W) writer structure"]
impl crate::Writable for BASE_ADDR2 {}
#[doc = "Base Address for region 2 register"]
pub mod base_addr2;
#[doc = "Sub-Region Enable register for region 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sr_enable2](sr_enable2) module"]
pub type SR_ENABLE2 = crate::Reg<u32, _SR_ENABLE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR_ENABLE2;
#[doc = "`read()` method returns [sr_enable2::R](sr_enable2::R) reader structure"]
impl crate::Readable for SR_ENABLE2 {}
#[doc = "`write(|w| ..)` method takes [sr_enable2::W](sr_enable2::W) writer structure"]
impl crate::Writable for SR_ENABLE2 {}
#[doc = "Sub-Region Enable register for region 2"]
pub mod sr_enable2;
