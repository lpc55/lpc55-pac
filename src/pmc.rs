#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8usize],
    #[doc = "0x08 - Reset Control \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    pub resetctrl: RESETCTRL,
    #[doc = "0x0c - Reset Cause register \\[Reset by: PoR\\]"]
    pub resetcause: RESETCAUSE,
    _reserved1: [u8; 32usize],
    #[doc = "0x30 - VBAT Brown Out Dectector (BoD) control register \\[Reset by: PoR, Pin Reset, Software Reset\\]"]
    pub bodvbat: BODVBAT,
    _reserved2: [u8; 4usize],
    #[doc = "0x38 - Digital Core logic Brown Out Dectector control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    pub bodcore: BODCORE,
    _reserved3: [u8; 8usize],
    #[doc = "0x44 - 1 MHz Free Running Oscillator control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    pub fro1m: FRO1M,
    #[doc = "0x48 - 32 KHz Free Running Oscillator (FRO) control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
    pub fro32k: FRO32K,
    #[doc = "0x4c - 32 KHz Crystal oscillator (XTAL) control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
    pub xtal32k: XTAL32K,
    #[doc = "0x50 - Analog Comparator control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    pub comp: COMP,
    _reserved4: [u8; 20usize],
    #[doc = "0x68 - Allows to identify the Wake-up I/O source from Deep Power Down mode"]
    pub wakeiocause: WAKEIOCAUSE,
    _reserved5: [u8; 8usize],
    #[doc = "0x74 - FRO and XTAL status register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
    pub statusclk: STATUSCLK,
    _reserved6: [u8; 12usize],
    #[doc = "0x84 - General purpose always on domain data storage \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
    pub aoreg1: AOREG1,
    _reserved7: [u8; 16usize],
    #[doc = "0x98 - RTC 1 KHZ and 1 Hz clocks source control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
    pub rtcosc32k: RTCOSC32K,
    #[doc = "0x9c - OS Timer control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
    pub ostimer: OSTIMER,
    _reserved8: [u8; 16usize],
    #[doc = "0xb0 - Controls the power to various modules during Low Power modes - DEEP SLEEP, POWER DOWN and DEEP POWER DOWN \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Software Reset\\]"]
    pub pdsleepcfg0: PDSLEEPCFG0,
    _reserved9: [u8; 4usize],
    #[doc = "0xb8 - Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    pub pdruncfg0: PDRUNCFG0,
    _reserved10: [u8; 4usize],
    #[doc = "0xc0 - Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    pub pdruncfgset0: PDRUNCFGSET0,
    _reserved11: [u8; 4usize],
    #[doc = "0xc8 - Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    pub pdruncfgclr0: PDRUNCFGCLR0,
}
#[doc = "Reset Control \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub struct RESETCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset Control \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod resetctrl;
#[doc = "Reset Cause register \\[Reset by: PoR\\]"]
pub struct RESETCAUSE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset Cause register \\[Reset by: PoR\\]"]
pub mod resetcause;
#[doc = "VBAT Brown Out Dectector (BoD) control register \\[Reset by: PoR, Pin Reset, Software Reset\\]"]
pub struct BODVBAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VBAT Brown Out Dectector (BoD) control register \\[Reset by: PoR, Pin Reset, Software Reset\\]"]
pub mod bodvbat;
#[doc = "Digital Core logic Brown Out Dectector control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub struct BODCORE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital Core logic Brown Out Dectector control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod bodcore;
#[doc = "1 MHz Free Running Oscillator control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub struct FRO1M {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "1 MHz Free Running Oscillator control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod fro1m;
#[doc = "32 KHz Free Running Oscillator (FRO) control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
pub struct FRO32K {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "32 KHz Free Running Oscillator (FRO) control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
pub mod fro32k;
#[doc = "32 KHz Crystal oscillator (XTAL) control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
pub struct XTAL32K {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "32 KHz Crystal oscillator (XTAL) control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
pub mod xtal32k;
#[doc = "Analog Comparator control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub struct COMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Comparator control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod comp;
#[doc = "Allows to identify the Wake-up I/O source from Deep Power Down mode"]
pub struct WAKEIOCAUSE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Allows to identify the Wake-up I/O source from Deep Power Down mode"]
pub mod wakeiocause;
#[doc = "FRO and XTAL status register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
pub struct STATUSCLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FRO and XTAL status register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
pub mod statusclk;
#[doc = "General purpose always on domain data storage \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
pub struct AOREG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General purpose always on domain data storage \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
pub mod aoreg1;
#[doc = "RTC 1 KHZ and 1 Hz clocks source control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
pub struct RTCOSC32K {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC 1 KHZ and 1 Hz clocks source control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
pub mod rtcosc32k;
#[doc = "OS Timer control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
pub struct OSTIMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OS Timer control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
pub mod ostimer;
#[doc = "Controls the power to various modules during Low Power modes - DEEP SLEEP, POWER DOWN and DEEP POWER DOWN \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Software Reset\\]"]
pub struct PDSLEEPCFG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Controls the power to various modules during Low Power modes - DEEP SLEEP, POWER DOWN and DEEP POWER DOWN \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Software Reset\\]"]
pub mod pdsleepcfg0;
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub struct PDRUNCFG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod pdruncfg0;
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub struct PDRUNCFGSET0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod pdruncfgset0;
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub struct PDRUNCFGCLR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod pdruncfgclr0;
