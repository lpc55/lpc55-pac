#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version ID Register"]
    pub verid: VERID,
    #[doc = "0x04 - Parameter Register"]
    pub param: PARAM,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - ADC Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x14 - ADC Status Register"]
    pub stat: STAT,
    #[doc = "0x18 - Interrupt Enable Register"]
    pub ie: IE,
    #[doc = "0x1c - DMA Enable Register"]
    pub de: DE,
    #[doc = "0x20 - ADC Configuration Register"]
    pub cfg: CFG,
    #[doc = "0x24 - ADC Pause Register"]
    pub pause: PAUSE,
    _reserved8: [u8; 12usize],
    #[doc = "0x34 - Software Trigger Register"]
    pub swtrig: SWTRIG,
    #[doc = "0x38 - Trigger Status Register"]
    pub tstat: TSTAT,
    _reserved10: [u8; 4usize],
    #[doc = "0x40 - ADC Offset Trim Register"]
    pub ofstrim: OFSTRIM,
    _reserved11: [u8; 92usize],
    #[doc = "0xa0 - Trigger Control Register"]
    pub tctrl: [TCTRL; 16],
    #[doc = "0xe0 - FIFO Control Register"]
    pub fctrl: [FCTRL; 2],
    _reserved13: [u8; 8usize],
    #[doc = "0xf0 - Gain Calibration Control"]
    pub gcc: [GCC; 2],
    #[doc = "0xf8 - Gain Calculation Result"]
    pub gcr: [GCR; 2],
    #[doc = "0x100 - ADC Command Low Buffer Register"]
    pub cmdl1: CMDL1,
    #[doc = "0x104 - ADC Command High Buffer Register"]
    pub cmdh1: CMDH1,
    #[doc = "0x108 - ADC Command Low Buffer Register"]
    pub cmdl2: CMDL2,
    #[doc = "0x10c - ADC Command High Buffer Register"]
    pub cmdh2: CMDH2,
    #[doc = "0x110 - ADC Command Low Buffer Register"]
    pub cmdl3: CMDL3,
    #[doc = "0x114 - ADC Command High Buffer Register"]
    pub cmdh3: CMDH3,
    #[doc = "0x118 - ADC Command Low Buffer Register"]
    pub cmdl4: CMDL4,
    #[doc = "0x11c - ADC Command High Buffer Register"]
    pub cmdh4: CMDH4,
    #[doc = "0x120 - ADC Command Low Buffer Register"]
    pub cmdl5: CMDL5,
    #[doc = "0x124 - ADC Command High Buffer Register"]
    pub cmdh5: CMDH5,
    #[doc = "0x128 - ADC Command Low Buffer Register"]
    pub cmdl6: CMDL6,
    #[doc = "0x12c - ADC Command High Buffer Register"]
    pub cmdh6: CMDH6,
    #[doc = "0x130 - ADC Command Low Buffer Register"]
    pub cmdl7: CMDL7,
    #[doc = "0x134 - ADC Command High Buffer Register"]
    pub cmdh7: CMDH7,
    #[doc = "0x138 - ADC Command Low Buffer Register"]
    pub cmdl8: CMDL8,
    #[doc = "0x13c - ADC Command High Buffer Register"]
    pub cmdh8: CMDH8,
    #[doc = "0x140 - ADC Command Low Buffer Register"]
    pub cmdl9: CMDL9,
    #[doc = "0x144 - ADC Command High Buffer Register"]
    pub cmdh9: CMDH9,
    #[doc = "0x148 - ADC Command Low Buffer Register"]
    pub cmdl10: CMDL10,
    #[doc = "0x14c - ADC Command High Buffer Register"]
    pub cmdh10: CMDH10,
    #[doc = "0x150 - ADC Command Low Buffer Register"]
    pub cmdl11: CMDL11,
    #[doc = "0x154 - ADC Command High Buffer Register"]
    pub cmdh11: CMDH11,
    #[doc = "0x158 - ADC Command Low Buffer Register"]
    pub cmdl12: CMDL12,
    #[doc = "0x15c - ADC Command High Buffer Register"]
    pub cmdh12: CMDH12,
    #[doc = "0x160 - ADC Command Low Buffer Register"]
    pub cmdl13: CMDL13,
    #[doc = "0x164 - ADC Command High Buffer Register"]
    pub cmdh13: CMDH13,
    #[doc = "0x168 - ADC Command Low Buffer Register"]
    pub cmdl14: CMDL14,
    #[doc = "0x16c - ADC Command High Buffer Register"]
    pub cmdh14: CMDH14,
    #[doc = "0x170 - ADC Command Low Buffer Register"]
    pub cmdl15: CMDL15,
    #[doc = "0x174 - ADC Command High Buffer Register"]
    pub cmdh15: CMDH15,
    _reserved45: [u8; 136usize],
    #[doc = "0x200 - Compare Value Register"]
    pub cv1: CV,
    #[doc = "0x204 - Compare Value Register"]
    pub cv2: CV,
    #[doc = "0x208 - Compare Value Register"]
    pub cv3: CV,
    #[doc = "0x20c - Compare Value Register"]
    pub cv4: CV,
    _reserved49: [u8; 240usize],
    #[doc = "0x300 - ADC Data Result FIFO Register"]
    pub resfifo: [RESFIFO; 2],
    _reserved50: [u8; 248usize],
    #[doc = "0x400 - Calibration General A-Side Registers"]
    pub cal_gar: [CAL_GAR; 33],
    _reserved51: [u8; 124usize],
    #[doc = "0x500 - Calibration General B-Side Registers"]
    pub cal_gbr: [CAL_GBR; 33],
    _reserved52: [u8; 2680usize],
    #[doc = "0xffc - ADC Test Register"]
    pub tst: TST,
}
#[doc = "Version ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [verid](verid) module"]
pub type VERID = crate::Reg<u32, _VERID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERID;
#[doc = "`read()` method returns [verid::R](verid::R) reader structure"]
impl crate::Readable for VERID {}
#[doc = "Version ID Register"]
pub mod verid;
#[doc = "Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [param](param) module"]
pub type PARAM = crate::Reg<u32, _PARAM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PARAM;
#[doc = "`read()` method returns [param::R](param::R) reader structure"]
impl crate::Readable for PARAM {}
#[doc = "Parameter Register"]
pub mod param;
#[doc = "ADC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "ADC Control Register"]
pub mod ctrl;
#[doc = "ADC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "`write(|w| ..)` method takes [stat::W](stat::W) writer structure"]
impl crate::Writable for STAT {}
#[doc = "ADC Status Register"]
pub mod stat;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ie](ie) module"]
pub type IE = crate::Reg<u32, _IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IE;
#[doc = "`read()` method returns [ie::R](ie::R) reader structure"]
impl crate::Readable for IE {}
#[doc = "`write(|w| ..)` method takes [ie::W](ie::W) writer structure"]
impl crate::Writable for IE {}
#[doc = "Interrupt Enable Register"]
pub mod ie;
#[doc = "DMA Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [de](de) module"]
pub type DE = crate::Reg<u32, _DE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DE;
#[doc = "`read()` method returns [de::R](de::R) reader structure"]
impl crate::Readable for DE {}
#[doc = "`write(|w| ..)` method takes [de::W](de::W) writer structure"]
impl crate::Writable for DE {}
#[doc = "DMA Enable Register"]
pub mod de;
#[doc = "ADC Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "ADC Configuration Register"]
pub mod cfg;
#[doc = "ADC Pause Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pause](pause) module"]
pub type PAUSE = crate::Reg<u32, _PAUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PAUSE;
#[doc = "`read()` method returns [pause::R](pause::R) reader structure"]
impl crate::Readable for PAUSE {}
#[doc = "`write(|w| ..)` method takes [pause::W](pause::W) writer structure"]
impl crate::Writable for PAUSE {}
#[doc = "ADC Pause Register"]
pub mod pause;
#[doc = "Software Trigger Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [swtrig](swtrig) module"]
pub type SWTRIG = crate::Reg<u32, _SWTRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWTRIG;
#[doc = "`read()` method returns [swtrig::R](swtrig::R) reader structure"]
impl crate::Readable for SWTRIG {}
#[doc = "`write(|w| ..)` method takes [swtrig::W](swtrig::W) writer structure"]
impl crate::Writable for SWTRIG {}
#[doc = "Software Trigger Register"]
pub mod swtrig;
#[doc = "Trigger Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tstat](tstat) module"]
pub type TSTAT = crate::Reg<u32, _TSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSTAT;
#[doc = "`read()` method returns [tstat::R](tstat::R) reader structure"]
impl crate::Readable for TSTAT {}
#[doc = "`write(|w| ..)` method takes [tstat::W](tstat::W) writer structure"]
impl crate::Writable for TSTAT {}
#[doc = "Trigger Status Register"]
pub mod tstat;
#[doc = "ADC Offset Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ofstrim](ofstrim) module"]
pub type OFSTRIM = crate::Reg<u32, _OFSTRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OFSTRIM;
#[doc = "`read()` method returns [ofstrim::R](ofstrim::R) reader structure"]
impl crate::Readable for OFSTRIM {}
#[doc = "`write(|w| ..)` method takes [ofstrim::W](ofstrim::W) writer structure"]
impl crate::Writable for OFSTRIM {}
#[doc = "ADC Offset Trim Register"]
pub mod ofstrim;
#[doc = "Trigger Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tctrl](tctrl) module"]
pub type TCTRL = crate::Reg<u32, _TCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCTRL;
#[doc = "`read()` method returns [tctrl::R](tctrl::R) reader structure"]
impl crate::Readable for TCTRL {}
#[doc = "`write(|w| ..)` method takes [tctrl::W](tctrl::W) writer structure"]
impl crate::Writable for TCTRL {}
#[doc = "Trigger Control Register"]
pub mod tctrl;
#[doc = "FIFO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fctrl](fctrl) module"]
pub type FCTRL = crate::Reg<u32, _FCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCTRL;
#[doc = "`read()` method returns [fctrl::R](fctrl::R) reader structure"]
impl crate::Readable for FCTRL {}
#[doc = "`write(|w| ..)` method takes [fctrl::W](fctrl::W) writer structure"]
impl crate::Writable for FCTRL {}
#[doc = "FIFO Control Register"]
pub mod fctrl;
#[doc = "Gain Calibration Control\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gcc](gcc) module"]
pub type GCC = crate::Reg<u32, _GCC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GCC;
#[doc = "`read()` method returns [gcc::R](gcc::R) reader structure"]
impl crate::Readable for GCC {}
#[doc = "Gain Calibration Control"]
pub mod gcc;
#[doc = "Gain Calculation Result\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gcr](gcr) module"]
pub type GCR = crate::Reg<u32, _GCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GCR;
#[doc = "`read()` method returns [gcr::R](gcr::R) reader structure"]
impl crate::Readable for GCR {}
#[doc = "`write(|w| ..)` method takes [gcr::W](gcr::W) writer structure"]
impl crate::Writable for GCR {}
#[doc = "Gain Calculation Result"]
pub mod gcr;
#[doc = "ADC Command Low Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdl1](cmdl1) module"]
pub type CMDL1 = crate::Reg<u32, _CMDL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDL1;
#[doc = "`read()` method returns [cmdl1::R](cmdl1::R) reader structure"]
impl crate::Readable for CMDL1 {}
#[doc = "`write(|w| ..)` method takes [cmdl1::W](cmdl1::W) writer structure"]
impl crate::Writable for CMDL1 {}
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl1;
#[doc = "ADC Command High Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdh1](cmdh1) module"]
pub type CMDH1 = crate::Reg<u32, _CMDH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDH1;
#[doc = "`read()` method returns [cmdh1::R](cmdh1::R) reader structure"]
impl crate::Readable for CMDH1 {}
#[doc = "`write(|w| ..)` method takes [cmdh1::W](cmdh1::W) writer structure"]
impl crate::Writable for CMDH1 {}
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh1;
#[doc = "ADC Command Low Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdl2](cmdl2) module"]
pub type CMDL2 = crate::Reg<u32, _CMDL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDL2;
#[doc = "`read()` method returns [cmdl2::R](cmdl2::R) reader structure"]
impl crate::Readable for CMDL2 {}
#[doc = "`write(|w| ..)` method takes [cmdl2::W](cmdl2::W) writer structure"]
impl crate::Writable for CMDL2 {}
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl2;
#[doc = "ADC Command High Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdh2](cmdh2) module"]
pub type CMDH2 = crate::Reg<u32, _CMDH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDH2;
#[doc = "`read()` method returns [cmdh2::R](cmdh2::R) reader structure"]
impl crate::Readable for CMDH2 {}
#[doc = "`write(|w| ..)` method takes [cmdh2::W](cmdh2::W) writer structure"]
impl crate::Writable for CMDH2 {}
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh2;
#[doc = "ADC Command Low Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdl3](cmdl3) module"]
pub type CMDL3 = crate::Reg<u32, _CMDL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDL3;
#[doc = "`read()` method returns [cmdl3::R](cmdl3::R) reader structure"]
impl crate::Readable for CMDL3 {}
#[doc = "`write(|w| ..)` method takes [cmdl3::W](cmdl3::W) writer structure"]
impl crate::Writable for CMDL3 {}
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl3;
#[doc = "ADC Command High Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdh3](cmdh3) module"]
pub type CMDH3 = crate::Reg<u32, _CMDH3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDH3;
#[doc = "`read()` method returns [cmdh3::R](cmdh3::R) reader structure"]
impl crate::Readable for CMDH3 {}
#[doc = "`write(|w| ..)` method takes [cmdh3::W](cmdh3::W) writer structure"]
impl crate::Writable for CMDH3 {}
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh3;
#[doc = "ADC Command Low Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdl4](cmdl4) module"]
pub type CMDL4 = crate::Reg<u32, _CMDL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDL4;
#[doc = "`read()` method returns [cmdl4::R](cmdl4::R) reader structure"]
impl crate::Readable for CMDL4 {}
#[doc = "`write(|w| ..)` method takes [cmdl4::W](cmdl4::W) writer structure"]
impl crate::Writable for CMDL4 {}
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl4;
#[doc = "ADC Command High Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdh4](cmdh4) module"]
pub type CMDH4 = crate::Reg<u32, _CMDH4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDH4;
#[doc = "`read()` method returns [cmdh4::R](cmdh4::R) reader structure"]
impl crate::Readable for CMDH4 {}
#[doc = "`write(|w| ..)` method takes [cmdh4::W](cmdh4::W) writer structure"]
impl crate::Writable for CMDH4 {}
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh4;
#[doc = "ADC Command Low Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdl5](cmdl5) module"]
pub type CMDL5 = crate::Reg<u32, _CMDL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDL5;
#[doc = "`read()` method returns [cmdl5::R](cmdl5::R) reader structure"]
impl crate::Readable for CMDL5 {}
#[doc = "`write(|w| ..)` method takes [cmdl5::W](cmdl5::W) writer structure"]
impl crate::Writable for CMDL5 {}
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl5;
#[doc = "ADC Command High Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdh5](cmdh5) module"]
pub type CMDH5 = crate::Reg<u32, _CMDH5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDH5;
#[doc = "`read()` method returns [cmdh5::R](cmdh5::R) reader structure"]
impl crate::Readable for CMDH5 {}
#[doc = "`write(|w| ..)` method takes [cmdh5::W](cmdh5::W) writer structure"]
impl crate::Writable for CMDH5 {}
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh5;
#[doc = "ADC Command Low Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdl6](cmdl6) module"]
pub type CMDL6 = crate::Reg<u32, _CMDL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDL6;
#[doc = "`read()` method returns [cmdl6::R](cmdl6::R) reader structure"]
impl crate::Readable for CMDL6 {}
#[doc = "`write(|w| ..)` method takes [cmdl6::W](cmdl6::W) writer structure"]
impl crate::Writable for CMDL6 {}
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl6;
#[doc = "ADC Command High Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdh6](cmdh6) module"]
pub type CMDH6 = crate::Reg<u32, _CMDH6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDH6;
#[doc = "`read()` method returns [cmdh6::R](cmdh6::R) reader structure"]
impl crate::Readable for CMDH6 {}
#[doc = "`write(|w| ..)` method takes [cmdh6::W](cmdh6::W) writer structure"]
impl crate::Writable for CMDH6 {}
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh6;
#[doc = "ADC Command Low Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdl7](cmdl7) module"]
pub type CMDL7 = crate::Reg<u32, _CMDL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDL7;
#[doc = "`read()` method returns [cmdl7::R](cmdl7::R) reader structure"]
impl crate::Readable for CMDL7 {}
#[doc = "`write(|w| ..)` method takes [cmdl7::W](cmdl7::W) writer structure"]
impl crate::Writable for CMDL7 {}
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl7;
#[doc = "ADC Command High Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdh7](cmdh7) module"]
pub type CMDH7 = crate::Reg<u32, _CMDH7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDH7;
#[doc = "`read()` method returns [cmdh7::R](cmdh7::R) reader structure"]
impl crate::Readable for CMDH7 {}
#[doc = "`write(|w| ..)` method takes [cmdh7::W](cmdh7::W) writer structure"]
impl crate::Writable for CMDH7 {}
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh7;
#[doc = "ADC Command Low Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdl8](cmdl8) module"]
pub type CMDL8 = crate::Reg<u32, _CMDL8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDL8;
#[doc = "`read()` method returns [cmdl8::R](cmdl8::R) reader structure"]
impl crate::Readable for CMDL8 {}
#[doc = "`write(|w| ..)` method takes [cmdl8::W](cmdl8::W) writer structure"]
impl crate::Writable for CMDL8 {}
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl8;
#[doc = "ADC Command High Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdh8](cmdh8) module"]
pub type CMDH8 = crate::Reg<u32, _CMDH8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDH8;
#[doc = "`read()` method returns [cmdh8::R](cmdh8::R) reader structure"]
impl crate::Readable for CMDH8 {}
#[doc = "`write(|w| ..)` method takes [cmdh8::W](cmdh8::W) writer structure"]
impl crate::Writable for CMDH8 {}
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh8;
#[doc = "ADC Command Low Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdl9](cmdl9) module"]
pub type CMDL9 = crate::Reg<u32, _CMDL9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDL9;
#[doc = "`read()` method returns [cmdl9::R](cmdl9::R) reader structure"]
impl crate::Readable for CMDL9 {}
#[doc = "`write(|w| ..)` method takes [cmdl9::W](cmdl9::W) writer structure"]
impl crate::Writable for CMDL9 {}
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl9;
#[doc = "ADC Command High Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdh9](cmdh9) module"]
pub type CMDH9 = crate::Reg<u32, _CMDH9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDH9;
#[doc = "`read()` method returns [cmdh9::R](cmdh9::R) reader structure"]
impl crate::Readable for CMDH9 {}
#[doc = "`write(|w| ..)` method takes [cmdh9::W](cmdh9::W) writer structure"]
impl crate::Writable for CMDH9 {}
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh9;
#[doc = "ADC Command Low Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdl10](cmdl10) module"]
pub type CMDL10 = crate::Reg<u32, _CMDL10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDL10;
#[doc = "`read()` method returns [cmdl10::R](cmdl10::R) reader structure"]
impl crate::Readable for CMDL10 {}
#[doc = "`write(|w| ..)` method takes [cmdl10::W](cmdl10::W) writer structure"]
impl crate::Writable for CMDL10 {}
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl10;
#[doc = "ADC Command High Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdh10](cmdh10) module"]
pub type CMDH10 = crate::Reg<u32, _CMDH10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDH10;
#[doc = "`read()` method returns [cmdh10::R](cmdh10::R) reader structure"]
impl crate::Readable for CMDH10 {}
#[doc = "`write(|w| ..)` method takes [cmdh10::W](cmdh10::W) writer structure"]
impl crate::Writable for CMDH10 {}
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh10;
#[doc = "ADC Command Low Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdl11](cmdl11) module"]
pub type CMDL11 = crate::Reg<u32, _CMDL11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDL11;
#[doc = "`read()` method returns [cmdl11::R](cmdl11::R) reader structure"]
impl crate::Readable for CMDL11 {}
#[doc = "`write(|w| ..)` method takes [cmdl11::W](cmdl11::W) writer structure"]
impl crate::Writable for CMDL11 {}
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl11;
#[doc = "ADC Command High Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdh11](cmdh11) module"]
pub type CMDH11 = crate::Reg<u32, _CMDH11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDH11;
#[doc = "`read()` method returns [cmdh11::R](cmdh11::R) reader structure"]
impl crate::Readable for CMDH11 {}
#[doc = "`write(|w| ..)` method takes [cmdh11::W](cmdh11::W) writer structure"]
impl crate::Writable for CMDH11 {}
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh11;
#[doc = "ADC Command Low Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdl12](cmdl12) module"]
pub type CMDL12 = crate::Reg<u32, _CMDL12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDL12;
#[doc = "`read()` method returns [cmdl12::R](cmdl12::R) reader structure"]
impl crate::Readable for CMDL12 {}
#[doc = "`write(|w| ..)` method takes [cmdl12::W](cmdl12::W) writer structure"]
impl crate::Writable for CMDL12 {}
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl12;
#[doc = "ADC Command High Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdh12](cmdh12) module"]
pub type CMDH12 = crate::Reg<u32, _CMDH12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDH12;
#[doc = "`read()` method returns [cmdh12::R](cmdh12::R) reader structure"]
impl crate::Readable for CMDH12 {}
#[doc = "`write(|w| ..)` method takes [cmdh12::W](cmdh12::W) writer structure"]
impl crate::Writable for CMDH12 {}
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh12;
#[doc = "ADC Command Low Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdl13](cmdl13) module"]
pub type CMDL13 = crate::Reg<u32, _CMDL13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDL13;
#[doc = "`read()` method returns [cmdl13::R](cmdl13::R) reader structure"]
impl crate::Readable for CMDL13 {}
#[doc = "`write(|w| ..)` method takes [cmdl13::W](cmdl13::W) writer structure"]
impl crate::Writable for CMDL13 {}
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl13;
#[doc = "ADC Command High Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdh13](cmdh13) module"]
pub type CMDH13 = crate::Reg<u32, _CMDH13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDH13;
#[doc = "`read()` method returns [cmdh13::R](cmdh13::R) reader structure"]
impl crate::Readable for CMDH13 {}
#[doc = "`write(|w| ..)` method takes [cmdh13::W](cmdh13::W) writer structure"]
impl crate::Writable for CMDH13 {}
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh13;
#[doc = "ADC Command Low Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdl14](cmdl14) module"]
pub type CMDL14 = crate::Reg<u32, _CMDL14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDL14;
#[doc = "`read()` method returns [cmdl14::R](cmdl14::R) reader structure"]
impl crate::Readable for CMDL14 {}
#[doc = "`write(|w| ..)` method takes [cmdl14::W](cmdl14::W) writer structure"]
impl crate::Writable for CMDL14 {}
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl14;
#[doc = "ADC Command High Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdh14](cmdh14) module"]
pub type CMDH14 = crate::Reg<u32, _CMDH14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDH14;
#[doc = "`read()` method returns [cmdh14::R](cmdh14::R) reader structure"]
impl crate::Readable for CMDH14 {}
#[doc = "`write(|w| ..)` method takes [cmdh14::W](cmdh14::W) writer structure"]
impl crate::Writable for CMDH14 {}
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh14;
#[doc = "ADC Command Low Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdl15](cmdl15) module"]
pub type CMDL15 = crate::Reg<u32, _CMDL15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDL15;
#[doc = "`read()` method returns [cmdl15::R](cmdl15::R) reader structure"]
impl crate::Readable for CMDL15 {}
#[doc = "`write(|w| ..)` method takes [cmdl15::W](cmdl15::W) writer structure"]
impl crate::Writable for CMDL15 {}
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl15;
#[doc = "ADC Command High Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdh15](cmdh15) module"]
pub type CMDH15 = crate::Reg<u32, _CMDH15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDH15;
#[doc = "`read()` method returns [cmdh15::R](cmdh15::R) reader structure"]
impl crate::Readable for CMDH15 {}
#[doc = "`write(|w| ..)` method takes [cmdh15::W](cmdh15::W) writer structure"]
impl crate::Writable for CMDH15 {}
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh15;
#[doc = "Compare Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cv](cv) module"]
pub type CV = crate::Reg<u32, _CV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CV;
#[doc = "`read()` method returns [cv::R](cv::R) reader structure"]
impl crate::Readable for CV {}
#[doc = "`write(|w| ..)` method takes [cv::W](cv::W) writer structure"]
impl crate::Writable for CV {}
#[doc = "Compare Value Register"]
pub mod cv;
#[doc = "ADC Data Result FIFO Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [resfifo](resfifo) module"]
pub type RESFIFO = crate::Reg<u32, _RESFIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESFIFO;
#[doc = "`read()` method returns [resfifo::R](resfifo::R) reader structure"]
impl crate::Readable for RESFIFO {}
#[doc = "ADC Data Result FIFO Register"]
pub mod resfifo;
#[doc = "Calibration General A-Side Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cal_gar](cal_gar) module"]
pub type CAL_GAR = crate::Reg<u32, _CAL_GAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAL_GAR;
#[doc = "`read()` method returns [cal_gar::R](cal_gar::R) reader structure"]
impl crate::Readable for CAL_GAR {}
#[doc = "`write(|w| ..)` method takes [cal_gar::W](cal_gar::W) writer structure"]
impl crate::Writable for CAL_GAR {}
#[doc = "Calibration General A-Side Registers"]
pub mod cal_gar;
#[doc = "Calibration General B-Side Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cal_gbr](cal_gbr) module"]
pub type CAL_GBR = crate::Reg<u32, _CAL_GBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAL_GBR;
#[doc = "`read()` method returns [cal_gbr::R](cal_gbr::R) reader structure"]
impl crate::Readable for CAL_GBR {}
#[doc = "`write(|w| ..)` method takes [cal_gbr::W](cal_gbr::W) writer structure"]
impl crate::Writable for CAL_GBR {}
#[doc = "Calibration General B-Side Registers"]
pub mod cal_gbr;
#[doc = "ADC Test Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tst](tst) module"]
pub type TST = crate::Reg<u32, _TST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TST;
#[doc = "`read()` method returns [tst::R](tst::R) reader structure"]
impl crate::Readable for TST {}
#[doc = "`write(|w| ..)` method takes [tst::W](tst::W) writer structure"]
impl crate::Writable for TST {}
#[doc = "ADC Test Register"]
pub mod tst;
