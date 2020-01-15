#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SCT configuration register"]
    pub config: CONFIG,
    #[doc = "0x04 - SCT control register"]
    pub ctrl: CTRL,
    #[doc = "0x08 - SCT limit event select register"]
    pub limit: LIMIT,
    #[doc = "0x0c - SCT halt event select register"]
    pub halt: HALT,
    #[doc = "0x10 - SCT stop event select register"]
    pub stop: STOP,
    #[doc = "0x14 - SCT start event select register"]
    pub start: START,
    _reserved6: [u8; 40usize],
    #[doc = "0x40 - SCT counter register"]
    pub count: COUNT,
    #[doc = "0x44 - SCT state register"]
    pub state: STATE,
    #[doc = "0x48 - SCT input register"]
    pub input: INPUT,
    #[doc = "0x4c - SCT match/capture mode register"]
    pub regmode: REGMODE,
    #[doc = "0x50 - SCT output register"]
    pub output: OUTPUT,
    #[doc = "0x54 - SCT output counter direction control register"]
    pub outputdirctrl: OUTPUTDIRCTRL,
    #[doc = "0x58 - SCT conflict resolution register"]
    pub res: RES,
    #[doc = "0x5c - SCT DMA request 0 register"]
    pub dmareq0: DMAREQ0,
    #[doc = "0x60 - SCT DMA request 1 register"]
    pub dmareq1: DMAREQ1,
    _reserved15: [u8; 140usize],
    #[doc = "0xf0 - SCT event interrupt enable register"]
    pub even: EVEN,
    #[doc = "0xf4 - SCT event flag register"]
    pub evflag: EVFLAG,
    #[doc = "0xf8 - SCT conflict interrupt enable register"]
    pub conen: CONEN,
    #[doc = "0xfc - SCT conflict flag register"]
    pub conflag: CONFLAG,
    _reserved_19_cap0: [u8; 4usize],
    _reserved_20_cap1: [u8; 4usize],
    _reserved_21_cap2: [u8; 4usize],
    _reserved_22_cap3: [u8; 4usize],
    _reserved_23_cap4: [u8; 4usize],
    _reserved_24_cap5: [u8; 4usize],
    _reserved_25_cap6: [u8; 4usize],
    _reserved_26_cap7: [u8; 4usize],
    _reserved_27_cap8: [u8; 4usize],
    _reserved_28_cap9: [u8; 4usize],
    _reserved_29_cap10: [u8; 4usize],
    _reserved_30_cap11: [u8; 4usize],
    _reserved_31_cap12: [u8; 4usize],
    _reserved_32_cap13: [u8; 4usize],
    _reserved_33_cap14: [u8; 4usize],
    _reserved_34_cap15: [u8; 4usize],
    _reserved35: [u8; 192usize],
    _reserved_35_capctrl0: [u8; 4usize],
    _reserved_36_capctrl1: [u8; 4usize],
    _reserved_37_capctrl2: [u8; 4usize],
    _reserved_38_capctrl3: [u8; 4usize],
    _reserved_39_capctrl4: [u8; 4usize],
    _reserved_40_capctrl5: [u8; 4usize],
    _reserved_41_capctrl6: [u8; 4usize],
    _reserved_42_capctrl7: [u8; 4usize],
    _reserved_43_capctrl8: [u8; 4usize],
    _reserved_44_capctrl9: [u8; 4usize],
    _reserved_45_capctrl10: [u8; 4usize],
    _reserved_46_capctrl11: [u8; 4usize],
    _reserved_47_capctrl12: [u8; 4usize],
    _reserved_48_capctrl13: [u8; 4usize],
    _reserved_49_capctrl14: [u8; 4usize],
    _reserved_50_capctrl15: [u8; 4usize],
    _reserved51: [u8; 192usize],
    #[doc = "0x300 - no description available"]
    pub ev: [EV; 16],
    _reserved52: [u8; 384usize],
    #[doc = "0x500 - no description available"]
    pub out: [OUT; 10],
}
impl RegisterBlock {
    #[doc = "0x100 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn match0(&self) -> &MATCH0 {
        unsafe { &*(((self as *const Self) as *const u8).add(256usize) as *const MATCH0) }
    }
    #[doc = "0x100 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn match0_mut(&self) -> &mut MATCH0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(256usize) as *mut MATCH0) }
    }
    #[doc = "0x100 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap0(&self) -> &CAP0 {
        unsafe { &*(((self as *const Self) as *const u8).add(256usize) as *const CAP0) }
    }
    #[doc = "0x100 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap0_mut(&self) -> &mut CAP0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(256usize) as *mut CAP0) }
    }
    #[doc = "0x104 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn match1(&self) -> &MATCH1 {
        unsafe { &*(((self as *const Self) as *const u8).add(260usize) as *const MATCH1) }
    }
    #[doc = "0x104 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn match1_mut(&self) -> &mut MATCH1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(260usize) as *mut MATCH1) }
    }
    #[doc = "0x104 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap1(&self) -> &CAP1 {
        unsafe { &*(((self as *const Self) as *const u8).add(260usize) as *const CAP1) }
    }
    #[doc = "0x104 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap1_mut(&self) -> &mut CAP1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(260usize) as *mut CAP1) }
    }
    #[doc = "0x108 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn match2(&self) -> &MATCH2 {
        unsafe { &*(((self as *const Self) as *const u8).add(264usize) as *const MATCH2) }
    }
    #[doc = "0x108 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn match2_mut(&self) -> &mut MATCH2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(264usize) as *mut MATCH2) }
    }
    #[doc = "0x108 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap2(&self) -> &CAP2 {
        unsafe { &*(((self as *const Self) as *const u8).add(264usize) as *const CAP2) }
    }
    #[doc = "0x108 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap2_mut(&self) -> &mut CAP2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(264usize) as *mut CAP2) }
    }
    #[doc = "0x10c - SCT match value register of match channels"]
    #[inline(always)]
    pub fn match3(&self) -> &MATCH3 {
        unsafe { &*(((self as *const Self) as *const u8).add(268usize) as *const MATCH3) }
    }
    #[doc = "0x10c - SCT match value register of match channels"]
    #[inline(always)]
    pub fn match3_mut(&self) -> &mut MATCH3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(268usize) as *mut MATCH3) }
    }
    #[doc = "0x10c - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap3(&self) -> &CAP3 {
        unsafe { &*(((self as *const Self) as *const u8).add(268usize) as *const CAP3) }
    }
    #[doc = "0x10c - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap3_mut(&self) -> &mut CAP3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(268usize) as *mut CAP3) }
    }
    #[doc = "0x110 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn match4(&self) -> &MATCH4 {
        unsafe { &*(((self as *const Self) as *const u8).add(272usize) as *const MATCH4) }
    }
    #[doc = "0x110 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn match4_mut(&self) -> &mut MATCH4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(272usize) as *mut MATCH4) }
    }
    #[doc = "0x110 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap4(&self) -> &CAP4 {
        unsafe { &*(((self as *const Self) as *const u8).add(272usize) as *const CAP4) }
    }
    #[doc = "0x110 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap4_mut(&self) -> &mut CAP4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(272usize) as *mut CAP4) }
    }
    #[doc = "0x114 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn match5(&self) -> &MATCH5 {
        unsafe { &*(((self as *const Self) as *const u8).add(276usize) as *const MATCH5) }
    }
    #[doc = "0x114 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn match5_mut(&self) -> &mut MATCH5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(276usize) as *mut MATCH5) }
    }
    #[doc = "0x114 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap5(&self) -> &CAP5 {
        unsafe { &*(((self as *const Self) as *const u8).add(276usize) as *const CAP5) }
    }
    #[doc = "0x114 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap5_mut(&self) -> &mut CAP5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(276usize) as *mut CAP5) }
    }
    #[doc = "0x118 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn match6(&self) -> &MATCH6 {
        unsafe { &*(((self as *const Self) as *const u8).add(280usize) as *const MATCH6) }
    }
    #[doc = "0x118 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn match6_mut(&self) -> &mut MATCH6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(280usize) as *mut MATCH6) }
    }
    #[doc = "0x118 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap6(&self) -> &CAP6 {
        unsafe { &*(((self as *const Self) as *const u8).add(280usize) as *const CAP6) }
    }
    #[doc = "0x118 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap6_mut(&self) -> &mut CAP6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(280usize) as *mut CAP6) }
    }
    #[doc = "0x11c - SCT match value register of match channels"]
    #[inline(always)]
    pub fn match7(&self) -> &MATCH7 {
        unsafe { &*(((self as *const Self) as *const u8).add(284usize) as *const MATCH7) }
    }
    #[doc = "0x11c - SCT match value register of match channels"]
    #[inline(always)]
    pub fn match7_mut(&self) -> &mut MATCH7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(284usize) as *mut MATCH7) }
    }
    #[doc = "0x11c - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap7(&self) -> &CAP7 {
        unsafe { &*(((self as *const Self) as *const u8).add(284usize) as *const CAP7) }
    }
    #[doc = "0x11c - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap7_mut(&self) -> &mut CAP7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(284usize) as *mut CAP7) }
    }
    #[doc = "0x120 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn match8(&self) -> &MATCH8 {
        unsafe { &*(((self as *const Self) as *const u8).add(288usize) as *const MATCH8) }
    }
    #[doc = "0x120 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn match8_mut(&self) -> &mut MATCH8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(288usize) as *mut MATCH8) }
    }
    #[doc = "0x120 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap8(&self) -> &CAP8 {
        unsafe { &*(((self as *const Self) as *const u8).add(288usize) as *const CAP8) }
    }
    #[doc = "0x120 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap8_mut(&self) -> &mut CAP8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(288usize) as *mut CAP8) }
    }
    #[doc = "0x124 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn match9(&self) -> &MATCH9 {
        unsafe { &*(((self as *const Self) as *const u8).add(292usize) as *const MATCH9) }
    }
    #[doc = "0x124 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn match9_mut(&self) -> &mut MATCH9 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(292usize) as *mut MATCH9) }
    }
    #[doc = "0x124 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap9(&self) -> &CAP9 {
        unsafe { &*(((self as *const Self) as *const u8).add(292usize) as *const CAP9) }
    }
    #[doc = "0x124 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap9_mut(&self) -> &mut CAP9 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(292usize) as *mut CAP9) }
    }
    #[doc = "0x128 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn match10(&self) -> &MATCH10 {
        unsafe { &*(((self as *const Self) as *const u8).add(296usize) as *const MATCH10) }
    }
    #[doc = "0x128 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn match10_mut(&self) -> &mut MATCH10 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(296usize) as *mut MATCH10) }
    }
    #[doc = "0x128 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap10(&self) -> &CAP10 {
        unsafe { &*(((self as *const Self) as *const u8).add(296usize) as *const CAP10) }
    }
    #[doc = "0x128 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap10_mut(&self) -> &mut CAP10 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(296usize) as *mut CAP10) }
    }
    #[doc = "0x12c - SCT match value register of match channels"]
    #[inline(always)]
    pub fn match11(&self) -> &MATCH11 {
        unsafe { &*(((self as *const Self) as *const u8).add(300usize) as *const MATCH11) }
    }
    #[doc = "0x12c - SCT match value register of match channels"]
    #[inline(always)]
    pub fn match11_mut(&self) -> &mut MATCH11 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(300usize) as *mut MATCH11) }
    }
    #[doc = "0x12c - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap11(&self) -> &CAP11 {
        unsafe { &*(((self as *const Self) as *const u8).add(300usize) as *const CAP11) }
    }
    #[doc = "0x12c - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap11_mut(&self) -> &mut CAP11 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(300usize) as *mut CAP11) }
    }
    #[doc = "0x130 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn match12(&self) -> &MATCH12 {
        unsafe { &*(((self as *const Self) as *const u8).add(304usize) as *const MATCH12) }
    }
    #[doc = "0x130 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn match12_mut(&self) -> &mut MATCH12 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(304usize) as *mut MATCH12) }
    }
    #[doc = "0x130 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap12(&self) -> &CAP12 {
        unsafe { &*(((self as *const Self) as *const u8).add(304usize) as *const CAP12) }
    }
    #[doc = "0x130 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap12_mut(&self) -> &mut CAP12 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(304usize) as *mut CAP12) }
    }
    #[doc = "0x134 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn match13(&self) -> &MATCH13 {
        unsafe { &*(((self as *const Self) as *const u8).add(308usize) as *const MATCH13) }
    }
    #[doc = "0x134 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn match13_mut(&self) -> &mut MATCH13 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(308usize) as *mut MATCH13) }
    }
    #[doc = "0x134 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap13(&self) -> &CAP13 {
        unsafe { &*(((self as *const Self) as *const u8).add(308usize) as *const CAP13) }
    }
    #[doc = "0x134 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap13_mut(&self) -> &mut CAP13 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(308usize) as *mut CAP13) }
    }
    #[doc = "0x138 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn match14(&self) -> &MATCH14 {
        unsafe { &*(((self as *const Self) as *const u8).add(312usize) as *const MATCH14) }
    }
    #[doc = "0x138 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn match14_mut(&self) -> &mut MATCH14 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(312usize) as *mut MATCH14) }
    }
    #[doc = "0x138 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap14(&self) -> &CAP14 {
        unsafe { &*(((self as *const Self) as *const u8).add(312usize) as *const CAP14) }
    }
    #[doc = "0x138 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap14_mut(&self) -> &mut CAP14 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(312usize) as *mut CAP14) }
    }
    #[doc = "0x13c - SCT match value register of match channels"]
    #[inline(always)]
    pub fn match15(&self) -> &MATCH15 {
        unsafe { &*(((self as *const Self) as *const u8).add(316usize) as *const MATCH15) }
    }
    #[doc = "0x13c - SCT match value register of match channels"]
    #[inline(always)]
    pub fn match15_mut(&self) -> &mut MATCH15 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(316usize) as *mut MATCH15) }
    }
    #[doc = "0x13c - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap15(&self) -> &CAP15 {
        unsafe { &*(((self as *const Self) as *const u8).add(316usize) as *const CAP15) }
    }
    #[doc = "0x13c - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap15_mut(&self) -> &mut CAP15 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(316usize) as *mut CAP15) }
    }
    #[doc = "0x200 - SCT match reload value register"]
    #[inline(always)]
    pub fn matchrel0(&self) -> &MATCHREL0 {
        unsafe { &*(((self as *const Self) as *const u8).add(512usize) as *const MATCHREL0) }
    }
    #[doc = "0x200 - SCT match reload value register"]
    #[inline(always)]
    pub fn matchrel0_mut(&self) -> &mut MATCHREL0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(512usize) as *mut MATCHREL0) }
    }
    #[doc = "0x200 - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl0(&self) -> &CAPCTRL0 {
        unsafe { &*(((self as *const Self) as *const u8).add(512usize) as *const CAPCTRL0) }
    }
    #[doc = "0x200 - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl0_mut(&self) -> &mut CAPCTRL0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(512usize) as *mut CAPCTRL0) }
    }
    #[doc = "0x204 - SCT match reload value register"]
    #[inline(always)]
    pub fn matchrel1(&self) -> &MATCHREL1 {
        unsafe { &*(((self as *const Self) as *const u8).add(516usize) as *const MATCHREL1) }
    }
    #[doc = "0x204 - SCT match reload value register"]
    #[inline(always)]
    pub fn matchrel1_mut(&self) -> &mut MATCHREL1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(516usize) as *mut MATCHREL1) }
    }
    #[doc = "0x204 - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl1(&self) -> &CAPCTRL1 {
        unsafe { &*(((self as *const Self) as *const u8).add(516usize) as *const CAPCTRL1) }
    }
    #[doc = "0x204 - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl1_mut(&self) -> &mut CAPCTRL1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(516usize) as *mut CAPCTRL1) }
    }
    #[doc = "0x208 - SCT match reload value register"]
    #[inline(always)]
    pub fn matchrel2(&self) -> &MATCHREL2 {
        unsafe { &*(((self as *const Self) as *const u8).add(520usize) as *const MATCHREL2) }
    }
    #[doc = "0x208 - SCT match reload value register"]
    #[inline(always)]
    pub fn matchrel2_mut(&self) -> &mut MATCHREL2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(520usize) as *mut MATCHREL2) }
    }
    #[doc = "0x208 - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl2(&self) -> &CAPCTRL2 {
        unsafe { &*(((self as *const Self) as *const u8).add(520usize) as *const CAPCTRL2) }
    }
    #[doc = "0x208 - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl2_mut(&self) -> &mut CAPCTRL2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(520usize) as *mut CAPCTRL2) }
    }
    #[doc = "0x20c - SCT match reload value register"]
    #[inline(always)]
    pub fn matchrel3(&self) -> &MATCHREL3 {
        unsafe { &*(((self as *const Self) as *const u8).add(524usize) as *const MATCHREL3) }
    }
    #[doc = "0x20c - SCT match reload value register"]
    #[inline(always)]
    pub fn matchrel3_mut(&self) -> &mut MATCHREL3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(524usize) as *mut MATCHREL3) }
    }
    #[doc = "0x20c - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl3(&self) -> &CAPCTRL3 {
        unsafe { &*(((self as *const Self) as *const u8).add(524usize) as *const CAPCTRL3) }
    }
    #[doc = "0x20c - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl3_mut(&self) -> &mut CAPCTRL3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(524usize) as *mut CAPCTRL3) }
    }
    #[doc = "0x210 - SCT match reload value register"]
    #[inline(always)]
    pub fn matchrel4(&self) -> &MATCHREL4 {
        unsafe { &*(((self as *const Self) as *const u8).add(528usize) as *const MATCHREL4) }
    }
    #[doc = "0x210 - SCT match reload value register"]
    #[inline(always)]
    pub fn matchrel4_mut(&self) -> &mut MATCHREL4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(528usize) as *mut MATCHREL4) }
    }
    #[doc = "0x210 - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl4(&self) -> &CAPCTRL4 {
        unsafe { &*(((self as *const Self) as *const u8).add(528usize) as *const CAPCTRL4) }
    }
    #[doc = "0x210 - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl4_mut(&self) -> &mut CAPCTRL4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(528usize) as *mut CAPCTRL4) }
    }
    #[doc = "0x214 - SCT match reload value register"]
    #[inline(always)]
    pub fn matchrel5(&self) -> &MATCHREL5 {
        unsafe { &*(((self as *const Self) as *const u8).add(532usize) as *const MATCHREL5) }
    }
    #[doc = "0x214 - SCT match reload value register"]
    #[inline(always)]
    pub fn matchrel5_mut(&self) -> &mut MATCHREL5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(532usize) as *mut MATCHREL5) }
    }
    #[doc = "0x214 - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl5(&self) -> &CAPCTRL5 {
        unsafe { &*(((self as *const Self) as *const u8).add(532usize) as *const CAPCTRL5) }
    }
    #[doc = "0x214 - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl5_mut(&self) -> &mut CAPCTRL5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(532usize) as *mut CAPCTRL5) }
    }
    #[doc = "0x218 - SCT match reload value register"]
    #[inline(always)]
    pub fn matchrel6(&self) -> &MATCHREL6 {
        unsafe { &*(((self as *const Self) as *const u8).add(536usize) as *const MATCHREL6) }
    }
    #[doc = "0x218 - SCT match reload value register"]
    #[inline(always)]
    pub fn matchrel6_mut(&self) -> &mut MATCHREL6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(536usize) as *mut MATCHREL6) }
    }
    #[doc = "0x218 - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl6(&self) -> &CAPCTRL6 {
        unsafe { &*(((self as *const Self) as *const u8).add(536usize) as *const CAPCTRL6) }
    }
    #[doc = "0x218 - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl6_mut(&self) -> &mut CAPCTRL6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(536usize) as *mut CAPCTRL6) }
    }
    #[doc = "0x21c - SCT match reload value register"]
    #[inline(always)]
    pub fn matchrel7(&self) -> &MATCHREL7 {
        unsafe { &*(((self as *const Self) as *const u8).add(540usize) as *const MATCHREL7) }
    }
    #[doc = "0x21c - SCT match reload value register"]
    #[inline(always)]
    pub fn matchrel7_mut(&self) -> &mut MATCHREL7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(540usize) as *mut MATCHREL7) }
    }
    #[doc = "0x21c - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl7(&self) -> &CAPCTRL7 {
        unsafe { &*(((self as *const Self) as *const u8).add(540usize) as *const CAPCTRL7) }
    }
    #[doc = "0x21c - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl7_mut(&self) -> &mut CAPCTRL7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(540usize) as *mut CAPCTRL7) }
    }
    #[doc = "0x220 - SCT match reload value register"]
    #[inline(always)]
    pub fn matchrel8(&self) -> &MATCHREL8 {
        unsafe { &*(((self as *const Self) as *const u8).add(544usize) as *const MATCHREL8) }
    }
    #[doc = "0x220 - SCT match reload value register"]
    #[inline(always)]
    pub fn matchrel8_mut(&self) -> &mut MATCHREL8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(544usize) as *mut MATCHREL8) }
    }
    #[doc = "0x220 - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl8(&self) -> &CAPCTRL8 {
        unsafe { &*(((self as *const Self) as *const u8).add(544usize) as *const CAPCTRL8) }
    }
    #[doc = "0x220 - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl8_mut(&self) -> &mut CAPCTRL8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(544usize) as *mut CAPCTRL8) }
    }
    #[doc = "0x224 - SCT match reload value register"]
    #[inline(always)]
    pub fn matchrel9(&self) -> &MATCHREL9 {
        unsafe { &*(((self as *const Self) as *const u8).add(548usize) as *const MATCHREL9) }
    }
    #[doc = "0x224 - SCT match reload value register"]
    #[inline(always)]
    pub fn matchrel9_mut(&self) -> &mut MATCHREL9 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(548usize) as *mut MATCHREL9) }
    }
    #[doc = "0x224 - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl9(&self) -> &CAPCTRL9 {
        unsafe { &*(((self as *const Self) as *const u8).add(548usize) as *const CAPCTRL9) }
    }
    #[doc = "0x224 - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl9_mut(&self) -> &mut CAPCTRL9 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(548usize) as *mut CAPCTRL9) }
    }
    #[doc = "0x228 - SCT match reload value register"]
    #[inline(always)]
    pub fn matchrel10(&self) -> &MATCHREL10 {
        unsafe { &*(((self as *const Self) as *const u8).add(552usize) as *const MATCHREL10) }
    }
    #[doc = "0x228 - SCT match reload value register"]
    #[inline(always)]
    pub fn matchrel10_mut(&self) -> &mut MATCHREL10 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(552usize) as *mut MATCHREL10) }
    }
    #[doc = "0x228 - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl10(&self) -> &CAPCTRL10 {
        unsafe { &*(((self as *const Self) as *const u8).add(552usize) as *const CAPCTRL10) }
    }
    #[doc = "0x228 - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl10_mut(&self) -> &mut CAPCTRL10 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(552usize) as *mut CAPCTRL10) }
    }
    #[doc = "0x22c - SCT match reload value register"]
    #[inline(always)]
    pub fn matchrel11(&self) -> &MATCHREL11 {
        unsafe { &*(((self as *const Self) as *const u8).add(556usize) as *const MATCHREL11) }
    }
    #[doc = "0x22c - SCT match reload value register"]
    #[inline(always)]
    pub fn matchrel11_mut(&self) -> &mut MATCHREL11 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(556usize) as *mut MATCHREL11) }
    }
    #[doc = "0x22c - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl11(&self) -> &CAPCTRL11 {
        unsafe { &*(((self as *const Self) as *const u8).add(556usize) as *const CAPCTRL11) }
    }
    #[doc = "0x22c - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl11_mut(&self) -> &mut CAPCTRL11 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(556usize) as *mut CAPCTRL11) }
    }
    #[doc = "0x230 - SCT match reload value register"]
    #[inline(always)]
    pub fn matchrel12(&self) -> &MATCHREL12 {
        unsafe { &*(((self as *const Self) as *const u8).add(560usize) as *const MATCHREL12) }
    }
    #[doc = "0x230 - SCT match reload value register"]
    #[inline(always)]
    pub fn matchrel12_mut(&self) -> &mut MATCHREL12 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(560usize) as *mut MATCHREL12) }
    }
    #[doc = "0x230 - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl12(&self) -> &CAPCTRL12 {
        unsafe { &*(((self as *const Self) as *const u8).add(560usize) as *const CAPCTRL12) }
    }
    #[doc = "0x230 - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl12_mut(&self) -> &mut CAPCTRL12 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(560usize) as *mut CAPCTRL12) }
    }
    #[doc = "0x234 - SCT match reload value register"]
    #[inline(always)]
    pub fn matchrel13(&self) -> &MATCHREL13 {
        unsafe { &*(((self as *const Self) as *const u8).add(564usize) as *const MATCHREL13) }
    }
    #[doc = "0x234 - SCT match reload value register"]
    #[inline(always)]
    pub fn matchrel13_mut(&self) -> &mut MATCHREL13 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(564usize) as *mut MATCHREL13) }
    }
    #[doc = "0x234 - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl13(&self) -> &CAPCTRL13 {
        unsafe { &*(((self as *const Self) as *const u8).add(564usize) as *const CAPCTRL13) }
    }
    #[doc = "0x234 - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl13_mut(&self) -> &mut CAPCTRL13 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(564usize) as *mut CAPCTRL13) }
    }
    #[doc = "0x238 - SCT match reload value register"]
    #[inline(always)]
    pub fn matchrel14(&self) -> &MATCHREL14 {
        unsafe { &*(((self as *const Self) as *const u8).add(568usize) as *const MATCHREL14) }
    }
    #[doc = "0x238 - SCT match reload value register"]
    #[inline(always)]
    pub fn matchrel14_mut(&self) -> &mut MATCHREL14 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(568usize) as *mut MATCHREL14) }
    }
    #[doc = "0x238 - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl14(&self) -> &CAPCTRL14 {
        unsafe { &*(((self as *const Self) as *const u8).add(568usize) as *const CAPCTRL14) }
    }
    #[doc = "0x238 - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl14_mut(&self) -> &mut CAPCTRL14 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(568usize) as *mut CAPCTRL14) }
    }
    #[doc = "0x23c - SCT match reload value register"]
    #[inline(always)]
    pub fn matchrel15(&self) -> &MATCHREL15 {
        unsafe { &*(((self as *const Self) as *const u8).add(572usize) as *const MATCHREL15) }
    }
    #[doc = "0x23c - SCT match reload value register"]
    #[inline(always)]
    pub fn matchrel15_mut(&self) -> &mut MATCHREL15 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(572usize) as *mut MATCHREL15) }
    }
    #[doc = "0x23c - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl15(&self) -> &CAPCTRL15 {
        unsafe { &*(((self as *const Self) as *const u8).add(572usize) as *const CAPCTRL15) }
    }
    #[doc = "0x23c - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl15_mut(&self) -> &mut CAPCTRL15 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(572usize) as *mut CAPCTRL15) }
    }
}
#[doc = r"Register block"]
#[repr(C)]
pub struct EV {
    #[doc = "0x00 - SCT event state register 0"]
    pub ev_state: self::ev::EV_STATE,
    #[doc = "0x04 - SCT event control register 0"]
    pub ev_ctrl: self::ev::EV_CTRL,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod ev;
#[doc = r"Register block"]
#[repr(C)]
pub struct OUT {
    #[doc = "0x00 - SCT output 0 set register"]
    pub out_set: self::out::OUT_SET,
    #[doc = "0x04 - SCT output 0 clear register"]
    pub out_clr: self::out::OUT_CLR,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod out;
#[doc = "SCT configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](config) module"]
pub type CONFIG = crate::Reg<u32, _CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG;
#[doc = "`read()` method returns [config::R](config::R) reader structure"]
impl crate::Readable for CONFIG {}
#[doc = "`write(|w| ..)` method takes [config::W](config::W) writer structure"]
impl crate::Writable for CONFIG {}
#[doc = "SCT configuration register"]
pub mod config;
#[doc = "SCT control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "SCT control register"]
pub mod ctrl;
#[doc = "SCT limit event select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [limit](limit) module"]
pub type LIMIT = crate::Reg<u32, _LIMIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LIMIT;
#[doc = "`read()` method returns [limit::R](limit::R) reader structure"]
impl crate::Readable for LIMIT {}
#[doc = "`write(|w| ..)` method takes [limit::W](limit::W) writer structure"]
impl crate::Writable for LIMIT {}
#[doc = "SCT limit event select register"]
pub mod limit;
#[doc = "SCT halt event select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [halt](halt) module"]
pub type HALT = crate::Reg<u32, _HALT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HALT;
#[doc = "`read()` method returns [halt::R](halt::R) reader structure"]
impl crate::Readable for HALT {}
#[doc = "`write(|w| ..)` method takes [halt::W](halt::W) writer structure"]
impl crate::Writable for HALT {}
#[doc = "SCT halt event select register"]
pub mod halt;
#[doc = "SCT stop event select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stop](stop) module"]
pub type STOP = crate::Reg<u32, _STOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STOP;
#[doc = "`read()` method returns [stop::R](stop::R) reader structure"]
impl crate::Readable for STOP {}
#[doc = "`write(|w| ..)` method takes [stop::W](stop::W) writer structure"]
impl crate::Writable for STOP {}
#[doc = "SCT stop event select register"]
pub mod stop;
#[doc = "SCT start event select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [start](start) module"]
pub type START = crate::Reg<u32, _START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _START;
#[doc = "`read()` method returns [start::R](start::R) reader structure"]
impl crate::Readable for START {}
#[doc = "`write(|w| ..)` method takes [start::W](start::W) writer structure"]
impl crate::Writable for START {}
#[doc = "SCT start event select register"]
pub mod start;
#[doc = "SCT counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count](count) module"]
pub type COUNT = crate::Reg<u32, _COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNT;
#[doc = "`read()` method returns [count::R](count::R) reader structure"]
impl crate::Readable for COUNT {}
#[doc = "`write(|w| ..)` method takes [count::W](count::W) writer structure"]
impl crate::Writable for COUNT {}
#[doc = "SCT counter register"]
pub mod count;
#[doc = "SCT state register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [state](state) module"]
pub type STATE = crate::Reg<u32, _STATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATE;
#[doc = "`read()` method returns [state::R](state::R) reader structure"]
impl crate::Readable for STATE {}
#[doc = "`write(|w| ..)` method takes [state::W](state::W) writer structure"]
impl crate::Writable for STATE {}
#[doc = "SCT state register"]
pub mod state;
#[doc = "SCT input register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [input](input) module"]
pub type INPUT = crate::Reg<u32, _INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INPUT;
#[doc = "`read()` method returns [input::R](input::R) reader structure"]
impl crate::Readable for INPUT {}
#[doc = "SCT input register"]
pub mod input;
#[doc = "SCT match/capture mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [regmode](regmode) module"]
pub type REGMODE = crate::Reg<u32, _REGMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGMODE;
#[doc = "`read()` method returns [regmode::R](regmode::R) reader structure"]
impl crate::Readable for REGMODE {}
#[doc = "`write(|w| ..)` method takes [regmode::W](regmode::W) writer structure"]
impl crate::Writable for REGMODE {}
#[doc = "SCT match/capture mode register"]
pub mod regmode;
#[doc = "SCT output register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [output](output) module"]
pub type OUTPUT = crate::Reg<u32, _OUTPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTPUT;
#[doc = "`read()` method returns [output::R](output::R) reader structure"]
impl crate::Readable for OUTPUT {}
#[doc = "`write(|w| ..)` method takes [output::W](output::W) writer structure"]
impl crate::Writable for OUTPUT {}
#[doc = "SCT output register"]
pub mod output;
#[doc = "SCT output counter direction control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outputdirctrl](outputdirctrl) module"]
pub type OUTPUTDIRCTRL = crate::Reg<u32, _OUTPUTDIRCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTPUTDIRCTRL;
#[doc = "`read()` method returns [outputdirctrl::R](outputdirctrl::R) reader structure"]
impl crate::Readable for OUTPUTDIRCTRL {}
#[doc = "`write(|w| ..)` method takes [outputdirctrl::W](outputdirctrl::W) writer structure"]
impl crate::Writable for OUTPUTDIRCTRL {}
#[doc = "SCT output counter direction control register"]
pub mod outputdirctrl;
#[doc = "SCT conflict resolution register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [res](res) module"]
pub type RES = crate::Reg<u32, _RES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RES;
#[doc = "`read()` method returns [res::R](res::R) reader structure"]
impl crate::Readable for RES {}
#[doc = "`write(|w| ..)` method takes [res::W](res::W) writer structure"]
impl crate::Writable for RES {}
#[doc = "SCT conflict resolution register"]
pub mod res;
#[doc = "SCT DMA request 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmareq0](dmareq0) module"]
pub type DMAREQ0 = crate::Reg<u32, _DMAREQ0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAREQ0;
#[doc = "`read()` method returns [dmareq0::R](dmareq0::R) reader structure"]
impl crate::Readable for DMAREQ0 {}
#[doc = "`write(|w| ..)` method takes [dmareq0::W](dmareq0::W) writer structure"]
impl crate::Writable for DMAREQ0 {}
#[doc = "SCT DMA request 0 register"]
pub mod dmareq0;
#[doc = "SCT DMA request 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmareq1](dmareq1) module"]
pub type DMAREQ1 = crate::Reg<u32, _DMAREQ1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAREQ1;
#[doc = "`read()` method returns [dmareq1::R](dmareq1::R) reader structure"]
impl crate::Readable for DMAREQ1 {}
#[doc = "`write(|w| ..)` method takes [dmareq1::W](dmareq1::W) writer structure"]
impl crate::Writable for DMAREQ1 {}
#[doc = "SCT DMA request 1 register"]
pub mod dmareq1;
#[doc = "SCT event interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [even](even) module"]
pub type EVEN = crate::Reg<u32, _EVEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVEN;
#[doc = "`read()` method returns [even::R](even::R) reader structure"]
impl crate::Readable for EVEN {}
#[doc = "`write(|w| ..)` method takes [even::W](even::W) writer structure"]
impl crate::Writable for EVEN {}
#[doc = "SCT event interrupt enable register"]
pub mod even;
#[doc = "SCT event flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evflag](evflag) module"]
pub type EVFLAG = crate::Reg<u32, _EVFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVFLAG;
#[doc = "`read()` method returns [evflag::R](evflag::R) reader structure"]
impl crate::Readable for EVFLAG {}
#[doc = "`write(|w| ..)` method takes [evflag::W](evflag::W) writer structure"]
impl crate::Writable for EVFLAG {}
#[doc = "SCT event flag register"]
pub mod evflag;
#[doc = "SCT conflict interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conen](conen) module"]
pub type CONEN = crate::Reg<u32, _CONEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONEN;
#[doc = "`read()` method returns [conen::R](conen::R) reader structure"]
impl crate::Readable for CONEN {}
#[doc = "`write(|w| ..)` method takes [conen::W](conen::W) writer structure"]
impl crate::Writable for CONEN {}
#[doc = "SCT conflict interrupt enable register"]
pub mod conen;
#[doc = "SCT conflict flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conflag](conflag) module"]
pub type CONFLAG = crate::Reg<u32, _CONFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFLAG;
#[doc = "`read()` method returns [conflag::R](conflag::R) reader structure"]
impl crate::Readable for CONFLAG {}
#[doc = "`write(|w| ..)` method takes [conflag::W](conflag::W) writer structure"]
impl crate::Writable for CONFLAG {}
#[doc = "SCT conflict flag register"]
pub mod conflag;
#[doc = "SCT capture register of capture channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap0](cap0) module"]
pub type CAP0 = crate::Reg<u32, _CAP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAP0;
#[doc = "`read()` method returns [cap0::R](cap0::R) reader structure"]
impl crate::Readable for CAP0 {}
#[doc = "`write(|w| ..)` method takes [cap0::W](cap0::W) writer structure"]
impl crate::Writable for CAP0 {}
#[doc = "SCT capture register of capture channel"]
pub mod cap0;
#[doc = "SCT match value register of match channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [match0](match0) module"]
pub type MATCH0 = crate::Reg<u32, _MATCH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCH0;
#[doc = "`read()` method returns [match0::R](match0::R) reader structure"]
impl crate::Readable for MATCH0 {}
#[doc = "`write(|w| ..)` method takes [match0::W](match0::W) writer structure"]
impl crate::Writable for MATCH0 {}
#[doc = "SCT match value register of match channels"]
pub mod match0;
#[doc = "SCT capture register of capture channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap1](cap1) module"]
pub type CAP1 = crate::Reg<u32, _CAP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAP1;
#[doc = "`read()` method returns [cap1::R](cap1::R) reader structure"]
impl crate::Readable for CAP1 {}
#[doc = "`write(|w| ..)` method takes [cap1::W](cap1::W) writer structure"]
impl crate::Writable for CAP1 {}
#[doc = "SCT capture register of capture channel"]
pub mod cap1;
#[doc = "SCT match value register of match channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [match1](match1) module"]
pub type MATCH1 = crate::Reg<u32, _MATCH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCH1;
#[doc = "`read()` method returns [match1::R](match1::R) reader structure"]
impl crate::Readable for MATCH1 {}
#[doc = "`write(|w| ..)` method takes [match1::W](match1::W) writer structure"]
impl crate::Writable for MATCH1 {}
#[doc = "SCT match value register of match channels"]
pub mod match1;
#[doc = "SCT capture register of capture channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap2](cap2) module"]
pub type CAP2 = crate::Reg<u32, _CAP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAP2;
#[doc = "`read()` method returns [cap2::R](cap2::R) reader structure"]
impl crate::Readable for CAP2 {}
#[doc = "`write(|w| ..)` method takes [cap2::W](cap2::W) writer structure"]
impl crate::Writable for CAP2 {}
#[doc = "SCT capture register of capture channel"]
pub mod cap2;
#[doc = "SCT match value register of match channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [match2](match2) module"]
pub type MATCH2 = crate::Reg<u32, _MATCH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCH2;
#[doc = "`read()` method returns [match2::R](match2::R) reader structure"]
impl crate::Readable for MATCH2 {}
#[doc = "`write(|w| ..)` method takes [match2::W](match2::W) writer structure"]
impl crate::Writable for MATCH2 {}
#[doc = "SCT match value register of match channels"]
pub mod match2;
#[doc = "SCT capture register of capture channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap3](cap3) module"]
pub type CAP3 = crate::Reg<u32, _CAP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAP3;
#[doc = "`read()` method returns [cap3::R](cap3::R) reader structure"]
impl crate::Readable for CAP3 {}
#[doc = "`write(|w| ..)` method takes [cap3::W](cap3::W) writer structure"]
impl crate::Writable for CAP3 {}
#[doc = "SCT capture register of capture channel"]
pub mod cap3;
#[doc = "SCT match value register of match channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [match3](match3) module"]
pub type MATCH3 = crate::Reg<u32, _MATCH3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCH3;
#[doc = "`read()` method returns [match3::R](match3::R) reader structure"]
impl crate::Readable for MATCH3 {}
#[doc = "`write(|w| ..)` method takes [match3::W](match3::W) writer structure"]
impl crate::Writable for MATCH3 {}
#[doc = "SCT match value register of match channels"]
pub mod match3;
#[doc = "SCT capture register of capture channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap4](cap4) module"]
pub type CAP4 = crate::Reg<u32, _CAP4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAP4;
#[doc = "`read()` method returns [cap4::R](cap4::R) reader structure"]
impl crate::Readable for CAP4 {}
#[doc = "`write(|w| ..)` method takes [cap4::W](cap4::W) writer structure"]
impl crate::Writable for CAP4 {}
#[doc = "SCT capture register of capture channel"]
pub mod cap4;
#[doc = "SCT match value register of match channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [match4](match4) module"]
pub type MATCH4 = crate::Reg<u32, _MATCH4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCH4;
#[doc = "`read()` method returns [match4::R](match4::R) reader structure"]
impl crate::Readable for MATCH4 {}
#[doc = "`write(|w| ..)` method takes [match4::W](match4::W) writer structure"]
impl crate::Writable for MATCH4 {}
#[doc = "SCT match value register of match channels"]
pub mod match4;
#[doc = "SCT capture register of capture channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap5](cap5) module"]
pub type CAP5 = crate::Reg<u32, _CAP5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAP5;
#[doc = "`read()` method returns [cap5::R](cap5::R) reader structure"]
impl crate::Readable for CAP5 {}
#[doc = "`write(|w| ..)` method takes [cap5::W](cap5::W) writer structure"]
impl crate::Writable for CAP5 {}
#[doc = "SCT capture register of capture channel"]
pub mod cap5;
#[doc = "SCT match value register of match channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [match5](match5) module"]
pub type MATCH5 = crate::Reg<u32, _MATCH5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCH5;
#[doc = "`read()` method returns [match5::R](match5::R) reader structure"]
impl crate::Readable for MATCH5 {}
#[doc = "`write(|w| ..)` method takes [match5::W](match5::W) writer structure"]
impl crate::Writable for MATCH5 {}
#[doc = "SCT match value register of match channels"]
pub mod match5;
#[doc = "SCT capture register of capture channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap6](cap6) module"]
pub type CAP6 = crate::Reg<u32, _CAP6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAP6;
#[doc = "`read()` method returns [cap6::R](cap6::R) reader structure"]
impl crate::Readable for CAP6 {}
#[doc = "`write(|w| ..)` method takes [cap6::W](cap6::W) writer structure"]
impl crate::Writable for CAP6 {}
#[doc = "SCT capture register of capture channel"]
pub mod cap6;
#[doc = "SCT match value register of match channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [match6](match6) module"]
pub type MATCH6 = crate::Reg<u32, _MATCH6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCH6;
#[doc = "`read()` method returns [match6::R](match6::R) reader structure"]
impl crate::Readable for MATCH6 {}
#[doc = "`write(|w| ..)` method takes [match6::W](match6::W) writer structure"]
impl crate::Writable for MATCH6 {}
#[doc = "SCT match value register of match channels"]
pub mod match6;
#[doc = "SCT capture register of capture channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap7](cap7) module"]
pub type CAP7 = crate::Reg<u32, _CAP7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAP7;
#[doc = "`read()` method returns [cap7::R](cap7::R) reader structure"]
impl crate::Readable for CAP7 {}
#[doc = "`write(|w| ..)` method takes [cap7::W](cap7::W) writer structure"]
impl crate::Writable for CAP7 {}
#[doc = "SCT capture register of capture channel"]
pub mod cap7;
#[doc = "SCT match value register of match channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [match7](match7) module"]
pub type MATCH7 = crate::Reg<u32, _MATCH7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCH7;
#[doc = "`read()` method returns [match7::R](match7::R) reader structure"]
impl crate::Readable for MATCH7 {}
#[doc = "`write(|w| ..)` method takes [match7::W](match7::W) writer structure"]
impl crate::Writable for MATCH7 {}
#[doc = "SCT match value register of match channels"]
pub mod match7;
#[doc = "SCT capture register of capture channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap8](cap8) module"]
pub type CAP8 = crate::Reg<u32, _CAP8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAP8;
#[doc = "`read()` method returns [cap8::R](cap8::R) reader structure"]
impl crate::Readable for CAP8 {}
#[doc = "`write(|w| ..)` method takes [cap8::W](cap8::W) writer structure"]
impl crate::Writable for CAP8 {}
#[doc = "SCT capture register of capture channel"]
pub mod cap8;
#[doc = "SCT match value register of match channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [match8](match8) module"]
pub type MATCH8 = crate::Reg<u32, _MATCH8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCH8;
#[doc = "`read()` method returns [match8::R](match8::R) reader structure"]
impl crate::Readable for MATCH8 {}
#[doc = "`write(|w| ..)` method takes [match8::W](match8::W) writer structure"]
impl crate::Writable for MATCH8 {}
#[doc = "SCT match value register of match channels"]
pub mod match8;
#[doc = "SCT capture register of capture channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap9](cap9) module"]
pub type CAP9 = crate::Reg<u32, _CAP9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAP9;
#[doc = "`read()` method returns [cap9::R](cap9::R) reader structure"]
impl crate::Readable for CAP9 {}
#[doc = "`write(|w| ..)` method takes [cap9::W](cap9::W) writer structure"]
impl crate::Writable for CAP9 {}
#[doc = "SCT capture register of capture channel"]
pub mod cap9;
#[doc = "SCT match value register of match channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [match9](match9) module"]
pub type MATCH9 = crate::Reg<u32, _MATCH9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCH9;
#[doc = "`read()` method returns [match9::R](match9::R) reader structure"]
impl crate::Readable for MATCH9 {}
#[doc = "`write(|w| ..)` method takes [match9::W](match9::W) writer structure"]
impl crate::Writable for MATCH9 {}
#[doc = "SCT match value register of match channels"]
pub mod match9;
#[doc = "SCT capture register of capture channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap10](cap10) module"]
pub type CAP10 = crate::Reg<u32, _CAP10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAP10;
#[doc = "`read()` method returns [cap10::R](cap10::R) reader structure"]
impl crate::Readable for CAP10 {}
#[doc = "`write(|w| ..)` method takes [cap10::W](cap10::W) writer structure"]
impl crate::Writable for CAP10 {}
#[doc = "SCT capture register of capture channel"]
pub mod cap10;
#[doc = "SCT match value register of match channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [match10](match10) module"]
pub type MATCH10 = crate::Reg<u32, _MATCH10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCH10;
#[doc = "`read()` method returns [match10::R](match10::R) reader structure"]
impl crate::Readable for MATCH10 {}
#[doc = "`write(|w| ..)` method takes [match10::W](match10::W) writer structure"]
impl crate::Writable for MATCH10 {}
#[doc = "SCT match value register of match channels"]
pub mod match10;
#[doc = "SCT capture register of capture channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap11](cap11) module"]
pub type CAP11 = crate::Reg<u32, _CAP11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAP11;
#[doc = "`read()` method returns [cap11::R](cap11::R) reader structure"]
impl crate::Readable for CAP11 {}
#[doc = "`write(|w| ..)` method takes [cap11::W](cap11::W) writer structure"]
impl crate::Writable for CAP11 {}
#[doc = "SCT capture register of capture channel"]
pub mod cap11;
#[doc = "SCT match value register of match channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [match11](match11) module"]
pub type MATCH11 = crate::Reg<u32, _MATCH11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCH11;
#[doc = "`read()` method returns [match11::R](match11::R) reader structure"]
impl crate::Readable for MATCH11 {}
#[doc = "`write(|w| ..)` method takes [match11::W](match11::W) writer structure"]
impl crate::Writable for MATCH11 {}
#[doc = "SCT match value register of match channels"]
pub mod match11;
#[doc = "SCT capture register of capture channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap12](cap12) module"]
pub type CAP12 = crate::Reg<u32, _CAP12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAP12;
#[doc = "`read()` method returns [cap12::R](cap12::R) reader structure"]
impl crate::Readable for CAP12 {}
#[doc = "`write(|w| ..)` method takes [cap12::W](cap12::W) writer structure"]
impl crate::Writable for CAP12 {}
#[doc = "SCT capture register of capture channel"]
pub mod cap12;
#[doc = "SCT match value register of match channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [match12](match12) module"]
pub type MATCH12 = crate::Reg<u32, _MATCH12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCH12;
#[doc = "`read()` method returns [match12::R](match12::R) reader structure"]
impl crate::Readable for MATCH12 {}
#[doc = "`write(|w| ..)` method takes [match12::W](match12::W) writer structure"]
impl crate::Writable for MATCH12 {}
#[doc = "SCT match value register of match channels"]
pub mod match12;
#[doc = "SCT capture register of capture channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap13](cap13) module"]
pub type CAP13 = crate::Reg<u32, _CAP13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAP13;
#[doc = "`read()` method returns [cap13::R](cap13::R) reader structure"]
impl crate::Readable for CAP13 {}
#[doc = "`write(|w| ..)` method takes [cap13::W](cap13::W) writer structure"]
impl crate::Writable for CAP13 {}
#[doc = "SCT capture register of capture channel"]
pub mod cap13;
#[doc = "SCT match value register of match channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [match13](match13) module"]
pub type MATCH13 = crate::Reg<u32, _MATCH13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCH13;
#[doc = "`read()` method returns [match13::R](match13::R) reader structure"]
impl crate::Readable for MATCH13 {}
#[doc = "`write(|w| ..)` method takes [match13::W](match13::W) writer structure"]
impl crate::Writable for MATCH13 {}
#[doc = "SCT match value register of match channels"]
pub mod match13;
#[doc = "SCT capture register of capture channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap14](cap14) module"]
pub type CAP14 = crate::Reg<u32, _CAP14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAP14;
#[doc = "`read()` method returns [cap14::R](cap14::R) reader structure"]
impl crate::Readable for CAP14 {}
#[doc = "`write(|w| ..)` method takes [cap14::W](cap14::W) writer structure"]
impl crate::Writable for CAP14 {}
#[doc = "SCT capture register of capture channel"]
pub mod cap14;
#[doc = "SCT match value register of match channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [match14](match14) module"]
pub type MATCH14 = crate::Reg<u32, _MATCH14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCH14;
#[doc = "`read()` method returns [match14::R](match14::R) reader structure"]
impl crate::Readable for MATCH14 {}
#[doc = "`write(|w| ..)` method takes [match14::W](match14::W) writer structure"]
impl crate::Writable for MATCH14 {}
#[doc = "SCT match value register of match channels"]
pub mod match14;
#[doc = "SCT capture register of capture channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap15](cap15) module"]
pub type CAP15 = crate::Reg<u32, _CAP15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAP15;
#[doc = "`read()` method returns [cap15::R](cap15::R) reader structure"]
impl crate::Readable for CAP15 {}
#[doc = "`write(|w| ..)` method takes [cap15::W](cap15::W) writer structure"]
impl crate::Writable for CAP15 {}
#[doc = "SCT capture register of capture channel"]
pub mod cap15;
#[doc = "SCT match value register of match channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [match15](match15) module"]
pub type MATCH15 = crate::Reg<u32, _MATCH15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCH15;
#[doc = "`read()` method returns [match15::R](match15::R) reader structure"]
impl crate::Readable for MATCH15 {}
#[doc = "`write(|w| ..)` method takes [match15::W](match15::W) writer structure"]
impl crate::Writable for MATCH15 {}
#[doc = "SCT match value register of match channels"]
pub mod match15;
#[doc = "SCT capture control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capctrl0](capctrl0) module"]
pub type CAPCTRL0 = crate::Reg<u32, _CAPCTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPCTRL0;
#[doc = "`read()` method returns [capctrl0::R](capctrl0::R) reader structure"]
impl crate::Readable for CAPCTRL0 {}
#[doc = "`write(|w| ..)` method takes [capctrl0::W](capctrl0::W) writer structure"]
impl crate::Writable for CAPCTRL0 {}
#[doc = "SCT capture control register"]
pub mod capctrl0;
#[doc = "SCT match reload value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matchrel0](matchrel0) module"]
pub type MATCHREL0 = crate::Reg<u32, _MATCHREL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCHREL0;
#[doc = "`read()` method returns [matchrel0::R](matchrel0::R) reader structure"]
impl crate::Readable for MATCHREL0 {}
#[doc = "`write(|w| ..)` method takes [matchrel0::W](matchrel0::W) writer structure"]
impl crate::Writable for MATCHREL0 {}
#[doc = "SCT match reload value register"]
pub mod matchrel0;
#[doc = "SCT capture control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capctrl1](capctrl1) module"]
pub type CAPCTRL1 = crate::Reg<u32, _CAPCTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPCTRL1;
#[doc = "`read()` method returns [capctrl1::R](capctrl1::R) reader structure"]
impl crate::Readable for CAPCTRL1 {}
#[doc = "`write(|w| ..)` method takes [capctrl1::W](capctrl1::W) writer structure"]
impl crate::Writable for CAPCTRL1 {}
#[doc = "SCT capture control register"]
pub mod capctrl1;
#[doc = "SCT match reload value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matchrel1](matchrel1) module"]
pub type MATCHREL1 = crate::Reg<u32, _MATCHREL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCHREL1;
#[doc = "`read()` method returns [matchrel1::R](matchrel1::R) reader structure"]
impl crate::Readable for MATCHREL1 {}
#[doc = "`write(|w| ..)` method takes [matchrel1::W](matchrel1::W) writer structure"]
impl crate::Writable for MATCHREL1 {}
#[doc = "SCT match reload value register"]
pub mod matchrel1;
#[doc = "SCT capture control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capctrl2](capctrl2) module"]
pub type CAPCTRL2 = crate::Reg<u32, _CAPCTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPCTRL2;
#[doc = "`read()` method returns [capctrl2::R](capctrl2::R) reader structure"]
impl crate::Readable for CAPCTRL2 {}
#[doc = "`write(|w| ..)` method takes [capctrl2::W](capctrl2::W) writer structure"]
impl crate::Writable for CAPCTRL2 {}
#[doc = "SCT capture control register"]
pub mod capctrl2;
#[doc = "SCT match reload value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matchrel2](matchrel2) module"]
pub type MATCHREL2 = crate::Reg<u32, _MATCHREL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCHREL2;
#[doc = "`read()` method returns [matchrel2::R](matchrel2::R) reader structure"]
impl crate::Readable for MATCHREL2 {}
#[doc = "`write(|w| ..)` method takes [matchrel2::W](matchrel2::W) writer structure"]
impl crate::Writable for MATCHREL2 {}
#[doc = "SCT match reload value register"]
pub mod matchrel2;
#[doc = "SCT capture control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capctrl3](capctrl3) module"]
pub type CAPCTRL3 = crate::Reg<u32, _CAPCTRL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPCTRL3;
#[doc = "`read()` method returns [capctrl3::R](capctrl3::R) reader structure"]
impl crate::Readable for CAPCTRL3 {}
#[doc = "`write(|w| ..)` method takes [capctrl3::W](capctrl3::W) writer structure"]
impl crate::Writable for CAPCTRL3 {}
#[doc = "SCT capture control register"]
pub mod capctrl3;
#[doc = "SCT match reload value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matchrel3](matchrel3) module"]
pub type MATCHREL3 = crate::Reg<u32, _MATCHREL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCHREL3;
#[doc = "`read()` method returns [matchrel3::R](matchrel3::R) reader structure"]
impl crate::Readable for MATCHREL3 {}
#[doc = "`write(|w| ..)` method takes [matchrel3::W](matchrel3::W) writer structure"]
impl crate::Writable for MATCHREL3 {}
#[doc = "SCT match reload value register"]
pub mod matchrel3;
#[doc = "SCT capture control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capctrl4](capctrl4) module"]
pub type CAPCTRL4 = crate::Reg<u32, _CAPCTRL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPCTRL4;
#[doc = "`read()` method returns [capctrl4::R](capctrl4::R) reader structure"]
impl crate::Readable for CAPCTRL4 {}
#[doc = "`write(|w| ..)` method takes [capctrl4::W](capctrl4::W) writer structure"]
impl crate::Writable for CAPCTRL4 {}
#[doc = "SCT capture control register"]
pub mod capctrl4;
#[doc = "SCT match reload value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matchrel4](matchrel4) module"]
pub type MATCHREL4 = crate::Reg<u32, _MATCHREL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCHREL4;
#[doc = "`read()` method returns [matchrel4::R](matchrel4::R) reader structure"]
impl crate::Readable for MATCHREL4 {}
#[doc = "`write(|w| ..)` method takes [matchrel4::W](matchrel4::W) writer structure"]
impl crate::Writable for MATCHREL4 {}
#[doc = "SCT match reload value register"]
pub mod matchrel4;
#[doc = "SCT capture control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capctrl5](capctrl5) module"]
pub type CAPCTRL5 = crate::Reg<u32, _CAPCTRL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPCTRL5;
#[doc = "`read()` method returns [capctrl5::R](capctrl5::R) reader structure"]
impl crate::Readable for CAPCTRL5 {}
#[doc = "`write(|w| ..)` method takes [capctrl5::W](capctrl5::W) writer structure"]
impl crate::Writable for CAPCTRL5 {}
#[doc = "SCT capture control register"]
pub mod capctrl5;
#[doc = "SCT match reload value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matchrel5](matchrel5) module"]
pub type MATCHREL5 = crate::Reg<u32, _MATCHREL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCHREL5;
#[doc = "`read()` method returns [matchrel5::R](matchrel5::R) reader structure"]
impl crate::Readable for MATCHREL5 {}
#[doc = "`write(|w| ..)` method takes [matchrel5::W](matchrel5::W) writer structure"]
impl crate::Writable for MATCHREL5 {}
#[doc = "SCT match reload value register"]
pub mod matchrel5;
#[doc = "SCT capture control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capctrl6](capctrl6) module"]
pub type CAPCTRL6 = crate::Reg<u32, _CAPCTRL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPCTRL6;
#[doc = "`read()` method returns [capctrl6::R](capctrl6::R) reader structure"]
impl crate::Readable for CAPCTRL6 {}
#[doc = "`write(|w| ..)` method takes [capctrl6::W](capctrl6::W) writer structure"]
impl crate::Writable for CAPCTRL6 {}
#[doc = "SCT capture control register"]
pub mod capctrl6;
#[doc = "SCT match reload value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matchrel6](matchrel6) module"]
pub type MATCHREL6 = crate::Reg<u32, _MATCHREL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCHREL6;
#[doc = "`read()` method returns [matchrel6::R](matchrel6::R) reader structure"]
impl crate::Readable for MATCHREL6 {}
#[doc = "`write(|w| ..)` method takes [matchrel6::W](matchrel6::W) writer structure"]
impl crate::Writable for MATCHREL6 {}
#[doc = "SCT match reload value register"]
pub mod matchrel6;
#[doc = "SCT capture control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capctrl7](capctrl7) module"]
pub type CAPCTRL7 = crate::Reg<u32, _CAPCTRL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPCTRL7;
#[doc = "`read()` method returns [capctrl7::R](capctrl7::R) reader structure"]
impl crate::Readable for CAPCTRL7 {}
#[doc = "`write(|w| ..)` method takes [capctrl7::W](capctrl7::W) writer structure"]
impl crate::Writable for CAPCTRL7 {}
#[doc = "SCT capture control register"]
pub mod capctrl7;
#[doc = "SCT match reload value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matchrel7](matchrel7) module"]
pub type MATCHREL7 = crate::Reg<u32, _MATCHREL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCHREL7;
#[doc = "`read()` method returns [matchrel7::R](matchrel7::R) reader structure"]
impl crate::Readable for MATCHREL7 {}
#[doc = "`write(|w| ..)` method takes [matchrel7::W](matchrel7::W) writer structure"]
impl crate::Writable for MATCHREL7 {}
#[doc = "SCT match reload value register"]
pub mod matchrel7;
#[doc = "SCT capture control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capctrl8](capctrl8) module"]
pub type CAPCTRL8 = crate::Reg<u32, _CAPCTRL8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPCTRL8;
#[doc = "`read()` method returns [capctrl8::R](capctrl8::R) reader structure"]
impl crate::Readable for CAPCTRL8 {}
#[doc = "`write(|w| ..)` method takes [capctrl8::W](capctrl8::W) writer structure"]
impl crate::Writable for CAPCTRL8 {}
#[doc = "SCT capture control register"]
pub mod capctrl8;
#[doc = "SCT match reload value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matchrel8](matchrel8) module"]
pub type MATCHREL8 = crate::Reg<u32, _MATCHREL8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCHREL8;
#[doc = "`read()` method returns [matchrel8::R](matchrel8::R) reader structure"]
impl crate::Readable for MATCHREL8 {}
#[doc = "`write(|w| ..)` method takes [matchrel8::W](matchrel8::W) writer structure"]
impl crate::Writable for MATCHREL8 {}
#[doc = "SCT match reload value register"]
pub mod matchrel8;
#[doc = "SCT capture control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capctrl9](capctrl9) module"]
pub type CAPCTRL9 = crate::Reg<u32, _CAPCTRL9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPCTRL9;
#[doc = "`read()` method returns [capctrl9::R](capctrl9::R) reader structure"]
impl crate::Readable for CAPCTRL9 {}
#[doc = "`write(|w| ..)` method takes [capctrl9::W](capctrl9::W) writer structure"]
impl crate::Writable for CAPCTRL9 {}
#[doc = "SCT capture control register"]
pub mod capctrl9;
#[doc = "SCT match reload value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matchrel9](matchrel9) module"]
pub type MATCHREL9 = crate::Reg<u32, _MATCHREL9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCHREL9;
#[doc = "`read()` method returns [matchrel9::R](matchrel9::R) reader structure"]
impl crate::Readable for MATCHREL9 {}
#[doc = "`write(|w| ..)` method takes [matchrel9::W](matchrel9::W) writer structure"]
impl crate::Writable for MATCHREL9 {}
#[doc = "SCT match reload value register"]
pub mod matchrel9;
#[doc = "SCT capture control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capctrl10](capctrl10) module"]
pub type CAPCTRL10 = crate::Reg<u32, _CAPCTRL10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPCTRL10;
#[doc = "`read()` method returns [capctrl10::R](capctrl10::R) reader structure"]
impl crate::Readable for CAPCTRL10 {}
#[doc = "`write(|w| ..)` method takes [capctrl10::W](capctrl10::W) writer structure"]
impl crate::Writable for CAPCTRL10 {}
#[doc = "SCT capture control register"]
pub mod capctrl10;
#[doc = "SCT match reload value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matchrel10](matchrel10) module"]
pub type MATCHREL10 = crate::Reg<u32, _MATCHREL10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCHREL10;
#[doc = "`read()` method returns [matchrel10::R](matchrel10::R) reader structure"]
impl crate::Readable for MATCHREL10 {}
#[doc = "`write(|w| ..)` method takes [matchrel10::W](matchrel10::W) writer structure"]
impl crate::Writable for MATCHREL10 {}
#[doc = "SCT match reload value register"]
pub mod matchrel10;
#[doc = "SCT capture control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capctrl11](capctrl11) module"]
pub type CAPCTRL11 = crate::Reg<u32, _CAPCTRL11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPCTRL11;
#[doc = "`read()` method returns [capctrl11::R](capctrl11::R) reader structure"]
impl crate::Readable for CAPCTRL11 {}
#[doc = "`write(|w| ..)` method takes [capctrl11::W](capctrl11::W) writer structure"]
impl crate::Writable for CAPCTRL11 {}
#[doc = "SCT capture control register"]
pub mod capctrl11;
#[doc = "SCT match reload value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matchrel11](matchrel11) module"]
pub type MATCHREL11 = crate::Reg<u32, _MATCHREL11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCHREL11;
#[doc = "`read()` method returns [matchrel11::R](matchrel11::R) reader structure"]
impl crate::Readable for MATCHREL11 {}
#[doc = "`write(|w| ..)` method takes [matchrel11::W](matchrel11::W) writer structure"]
impl crate::Writable for MATCHREL11 {}
#[doc = "SCT match reload value register"]
pub mod matchrel11;
#[doc = "SCT capture control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capctrl12](capctrl12) module"]
pub type CAPCTRL12 = crate::Reg<u32, _CAPCTRL12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPCTRL12;
#[doc = "`read()` method returns [capctrl12::R](capctrl12::R) reader structure"]
impl crate::Readable for CAPCTRL12 {}
#[doc = "`write(|w| ..)` method takes [capctrl12::W](capctrl12::W) writer structure"]
impl crate::Writable for CAPCTRL12 {}
#[doc = "SCT capture control register"]
pub mod capctrl12;
#[doc = "SCT match reload value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matchrel12](matchrel12) module"]
pub type MATCHREL12 = crate::Reg<u32, _MATCHREL12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCHREL12;
#[doc = "`read()` method returns [matchrel12::R](matchrel12::R) reader structure"]
impl crate::Readable for MATCHREL12 {}
#[doc = "`write(|w| ..)` method takes [matchrel12::W](matchrel12::W) writer structure"]
impl crate::Writable for MATCHREL12 {}
#[doc = "SCT match reload value register"]
pub mod matchrel12;
#[doc = "SCT capture control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capctrl13](capctrl13) module"]
pub type CAPCTRL13 = crate::Reg<u32, _CAPCTRL13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPCTRL13;
#[doc = "`read()` method returns [capctrl13::R](capctrl13::R) reader structure"]
impl crate::Readable for CAPCTRL13 {}
#[doc = "`write(|w| ..)` method takes [capctrl13::W](capctrl13::W) writer structure"]
impl crate::Writable for CAPCTRL13 {}
#[doc = "SCT capture control register"]
pub mod capctrl13;
#[doc = "SCT match reload value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matchrel13](matchrel13) module"]
pub type MATCHREL13 = crate::Reg<u32, _MATCHREL13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCHREL13;
#[doc = "`read()` method returns [matchrel13::R](matchrel13::R) reader structure"]
impl crate::Readable for MATCHREL13 {}
#[doc = "`write(|w| ..)` method takes [matchrel13::W](matchrel13::W) writer structure"]
impl crate::Writable for MATCHREL13 {}
#[doc = "SCT match reload value register"]
pub mod matchrel13;
#[doc = "SCT capture control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capctrl14](capctrl14) module"]
pub type CAPCTRL14 = crate::Reg<u32, _CAPCTRL14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPCTRL14;
#[doc = "`read()` method returns [capctrl14::R](capctrl14::R) reader structure"]
impl crate::Readable for CAPCTRL14 {}
#[doc = "`write(|w| ..)` method takes [capctrl14::W](capctrl14::W) writer structure"]
impl crate::Writable for CAPCTRL14 {}
#[doc = "SCT capture control register"]
pub mod capctrl14;
#[doc = "SCT match reload value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matchrel14](matchrel14) module"]
pub type MATCHREL14 = crate::Reg<u32, _MATCHREL14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCHREL14;
#[doc = "`read()` method returns [matchrel14::R](matchrel14::R) reader structure"]
impl crate::Readable for MATCHREL14 {}
#[doc = "`write(|w| ..)` method takes [matchrel14::W](matchrel14::W) writer structure"]
impl crate::Writable for MATCHREL14 {}
#[doc = "SCT match reload value register"]
pub mod matchrel14;
#[doc = "SCT capture control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capctrl15](capctrl15) module"]
pub type CAPCTRL15 = crate::Reg<u32, _CAPCTRL15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPCTRL15;
#[doc = "`read()` method returns [capctrl15::R](capctrl15::R) reader structure"]
impl crate::Readable for CAPCTRL15 {}
#[doc = "`write(|w| ..)` method takes [capctrl15::W](capctrl15::W) writer structure"]
impl crate::Writable for CAPCTRL15 {}
#[doc = "SCT capture control register"]
pub mod capctrl15;
#[doc = "SCT match reload value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matchrel15](matchrel15) module"]
pub type MATCHREL15 = crate::Reg<u32, _MATCHREL15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCHREL15;
#[doc = "`read()` method returns [matchrel15::R](matchrel15::R) reader structure"]
impl crate::Readable for MATCHREL15 {}
#[doc = "`write(|w| ..)` method takes [matchrel15::W](matchrel15::W) writer structure"]
impl crate::Writable for MATCHREL15 {}
#[doc = "SCT match reload value register"]
pub mod matchrel15;
