#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register to enable and operate Hash and Crypto"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Indicates status of Hash peripheral."]
    pub status: STATUS,
    #[doc = "0x08 - Write 1 to enable interrupts; reads back with which are set."]
    pub intenset: INTENSET,
    #[doc = "0x0c - Write 1 to clear interrupts."]
    pub intenclr: INTENCLR,
    #[doc = "0x10 - Setup Master to access memory (if available)"]
    pub memctrl: MEMCTRL,
    #[doc = "0x14 - Address to start memory access from (if available)."]
    pub memaddr: MEMADDR,
    _reserved6: [u8; 8usize],
    #[doc = "0x20 - Input of 16 words at a time to load up buffer."]
    pub indata: INDATA,
    #[doc = "0x24 - no description available"]
    pub alias: [ALIAS; 7],
    #[doc = "0x40 - no description available"]
    pub digest0: [DIGEST0; 8],
    _reserved9: [u8; 32usize],
    #[doc = "0x80 - Crypto settings for AES and Salsa and ChaCha"]
    pub cryptcfg: CRYPTCFG,
    #[doc = "0x84 - Returns the configuration of this block in this chip - indicates what services are available."]
    pub config: CONFIG,
    _reserved11: [u8; 4usize],
    #[doc = "0x8c - Lock register allows locking to the current security level or unlocking by the lock holding level."]
    pub lock: LOCK,
    #[doc = "0x90 - no description available"]
    pub mask: [MASK; 4],
}
#[doc = "Control register to enable and operate Hash and Crypto\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control register to enable and operate Hash and Crypto"]
pub mod ctrl;
#[doc = "Indicates status of Hash peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "`write(|w| ..)` method takes [status::W](status::W) writer structure"]
impl crate::Writable for STATUS {}
#[doc = "Indicates status of Hash peripheral."]
pub mod status;
#[doc = "Write 1 to enable interrupts; reads back with which are set.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](intenset) module"]
pub type INTENSET = crate::Reg<u32, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`read()` method returns [intenset::R](intenset::R) reader structure"]
impl crate::Readable for INTENSET {}
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::Writable for INTENSET {}
#[doc = "Write 1 to enable interrupts; reads back with which are set."]
pub mod intenset;
#[doc = "Write 1 to clear interrupts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u32, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`read()` method returns [intenclr::R](intenclr::R) reader structure"]
impl crate::Readable for INTENCLR {}
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::Writable for INTENCLR {}
#[doc = "Write 1 to clear interrupts."]
pub mod intenclr;
#[doc = "Setup Master to access memory (if available)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [memctrl](memctrl) module"]
pub type MEMCTRL = crate::Reg<u32, _MEMCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEMCTRL;
#[doc = "`read()` method returns [memctrl::R](memctrl::R) reader structure"]
impl crate::Readable for MEMCTRL {}
#[doc = "`write(|w| ..)` method takes [memctrl::W](memctrl::W) writer structure"]
impl crate::Writable for MEMCTRL {}
#[doc = "Setup Master to access memory (if available)"]
pub mod memctrl;
#[doc = "Address to start memory access from (if available).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [memaddr](memaddr) module"]
pub type MEMADDR = crate::Reg<u32, _MEMADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEMADDR;
#[doc = "`read()` method returns [memaddr::R](memaddr::R) reader structure"]
impl crate::Readable for MEMADDR {}
#[doc = "`write(|w| ..)` method takes [memaddr::W](memaddr::W) writer structure"]
impl crate::Writable for MEMADDR {}
#[doc = "Address to start memory access from (if available)."]
pub mod memaddr;
#[doc = "Input of 16 words at a time to load up buffer.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [indata](indata) module"]
pub type INDATA = crate::Reg<u32, _INDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INDATA;
#[doc = "`write(|w| ..)` method takes [indata::W](indata::W) writer structure"]
impl crate::Writable for INDATA {}
#[doc = "Input of 16 words at a time to load up buffer."]
pub mod indata;
#[doc = "no description available\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alias](alias) module"]
pub type ALIAS = crate::Reg<u32, _ALIAS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALIAS;
#[doc = "`write(|w| ..)` method takes [alias::W](alias::W) writer structure"]
impl crate::Writable for ALIAS {}
#[doc = "no description available"]
pub mod alias;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [digest0](digest0) module"]
pub type DIGEST0 = crate::Reg<u32, _DIGEST0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIGEST0;
#[doc = "`read()` method returns [digest0::R](digest0::R) reader structure"]
impl crate::Readable for DIGEST0 {}
#[doc = "no description available"]
pub mod digest0;
#[doc = "Crypto settings for AES and Salsa and ChaCha\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryptcfg](cryptcfg) module"]
pub type CRYPTCFG = crate::Reg<u32, _CRYPTCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRYPTCFG;
#[doc = "`read()` method returns [cryptcfg::R](cryptcfg::R) reader structure"]
impl crate::Readable for CRYPTCFG {}
#[doc = "`write(|w| ..)` method takes [cryptcfg::W](cryptcfg::W) writer structure"]
impl crate::Writable for CRYPTCFG {}
#[doc = "Crypto settings for AES and Salsa and ChaCha"]
pub mod cryptcfg;
#[doc = "Returns the configuration of this block in this chip - indicates what services are available.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](config) module"]
pub type CONFIG = crate::Reg<u32, _CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG;
#[doc = "`read()` method returns [config::R](config::R) reader structure"]
impl crate::Readable for CONFIG {}
#[doc = "`write(|w| ..)` method takes [config::W](config::W) writer structure"]
impl crate::Writable for CONFIG {}
#[doc = "Returns the configuration of this block in this chip - indicates what services are available."]
pub mod config;
#[doc = "Lock register allows locking to the current security level or unlocking by the lock holding level.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lock](lock) module"]
pub type LOCK = crate::Reg<u32, _LOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCK;
#[doc = "`read()` method returns [lock::R](lock::R) reader structure"]
impl crate::Readable for LOCK {}
#[doc = "`write(|w| ..)` method takes [lock::W](lock::W) writer structure"]
impl crate::Writable for LOCK {}
#[doc = "Lock register allows locking to the current security level or unlocking by the lock holding level."]
pub mod lock;
#[doc = "no description available\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask](mask) module"]
pub type MASK = crate::Reg<u32, _MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK;
#[doc = "`write(|w| ..)` method takes [mask::W](mask::W) writer structure"]
impl crate::Writable for MASK {}
#[doc = "no description available"]
pub mod mask;
