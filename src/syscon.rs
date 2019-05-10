#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Memory Remap control register"]
    pub memoryremap: MEMORYREMAP,
    _reserved0: [u8; 12usize],
    #[doc = "0x10 - AHB Matrix priority control register Priority values are 3 = highest, 0 = lowest"]
    pub ahbmatprio: AHBMATPRIO,
    _reserved1: [u8; 36usize],
    #[doc = "0x38 - System tick calibration for secure part of CPU0"]
    pub cpu0stckcal: CPU0STCKCAL,
    #[doc = "0x3c - System tick calibration for non-secure part of CPU0"]
    pub cpu0nstckcal: CPU0NSTCKCAL,
    #[doc = "0x40 - System tick calibration for CPU1"]
    pub cpu1tckcal: CPU1TCKCAL,
    _reserved2: [u8; 4usize],
    #[doc = "0x48 - NMI Source Select"]
    pub nmisrc: NMISRC,
    _reserved3: [u8; 180usize],
    #[doc = "0x100 - Peripheral reset control 0"]
    pub presetctrl0: PRESETCTRL0,
    #[doc = "0x104 - Peripheral reset control 1"]
    pub presetctrl1: PRESETCTRL1,
    #[doc = "0x108 - Peripheral reset control 2"]
    pub presetctrl2: PRESETCTRL2,
    _reserved4: [u8; 20usize],
    #[doc = "0x120 - Peripheral reset control set register"]
    pub presetctrlset: [PRESETCTRLSET; 3],
    _reserved5: [u8; 20usize],
    #[doc = "0x140 - Peripheral reset contro clearl register"]
    pub presetctrlclr: [PRESETCTRLCLR; 3],
    _reserved6: [u8; 20usize],
    #[doc = "0x160 - generate a software_reset"]
    pub swr_reset: SWR_RESET,
    _reserved7: [u8; 156usize],
    #[doc = "0x200 - AHB Clock control 0"]
    pub ahbclkctrl0: AHBCLKCTRL0,
    #[doc = "0x204 - AHB Clock control 1"]
    pub ahbclkctrl1: AHBCLKCTRL1,
    #[doc = "0x208 - AHB Clock control 2"]
    pub ahbclkctrl2: AHBCLKCTRL2,
    _reserved8: [u8; 20usize],
    #[doc = "0x220 - Peripheral reset control register"]
    pub ahbclkctrlset: [AHBCLKCTRLSET; 3],
    _reserved9: [u8; 20usize],
    #[doc = "0x240 - Peripheral reset control register"]
    pub ahbclkctrlclr: [AHBCLKCTRLCLR; 3],
    _reserved10: [u8; 20usize],
    #[doc = "0x260 - System Tick Timer for CPU0 source select"]
    pub systickclksel0: SYSTICKCLKSEL0,
    #[doc = "0x264 - System Tick Timer for CPU1 source select"]
    pub systickclksel1: SYSTICKCLKSEL1,
    #[doc = "0x268 - Trace clock source select"]
    pub traceclksel: TRACECLKSEL,
    #[doc = "0x26c - CTimer 0 clock source select"]
    pub ctimerclksel0: CTIMERCLKSEL0,
    #[doc = "0x270 - CTimer 1 clock source select"]
    pub ctimerclksel1: CTIMERCLKSEL1,
    #[doc = "0x274 - CTimer 2 clock source select"]
    pub ctimerclksel2: CTIMERCLKSEL2,
    #[doc = "0x278 - CTimer 3 clock source select"]
    pub ctimerclksel3: CTIMERCLKSEL3,
    #[doc = "0x27c - CTimer 4 clock source select"]
    pub ctimerclksel4: CTIMERCLKSEL4,
    #[doc = "0x280 - Main clock A source select"]
    pub mainclksela: MAINCLKSELA,
    #[doc = "0x284 - Main clock source select"]
    pub mainclkselb: MAINCLKSELB,
    #[doc = "0x288 - CLKOUT clock source select"]
    pub clkoutsel: CLKOUTSEL,
    _reserved11: [u8; 4usize],
    #[doc = "0x290 - PLL0 clock source select"]
    pub pll0clksel: PLL0CLKSEL,
    #[doc = "0x294 - PLL1 clock source select"]
    pub pll1clksel: PLL1CLKSEL,
    _reserved12: [u8; 12usize],
    #[doc = "0x2a4 - ADC clock source select"]
    pub adcclksel: ADCCLKSEL,
    #[doc = "0x2a8 - FS USB clock source select"]
    pub usb0clksel: USB0CLKSEL,
    #[doc = "0x2ac - HS USB clock source select - NOT USED"]
    pub usb1clksel: USB1CLKSEL,
    #[doc = "0x2b0 - Flexcomm Interface 0 clock source select for Fractional Rate Divider"]
    pub fcclksel0: FCCLKSEL0,
    #[doc = "0x2b4 - Flexcomm Interface 1 clock source select for Fractional Rate Divider"]
    pub fcclksel1: FCCLKSEL1,
    #[doc = "0x2b8 - Flexcomm Interface 2 clock source select for Fractional Rate Divider"]
    pub fcclksel2: FCCLKSEL2,
    #[doc = "0x2bc - Flexcomm Interface 3 clock source select for Fractional Rate Divider"]
    pub fcclksel3: FCCLKSEL3,
    #[doc = "0x2c0 - Flexcomm Interface 4 clock source select for Fractional Rate Divider"]
    pub fcclksel4: FCCLKSEL4,
    #[doc = "0x2c4 - Flexcomm Interface 5 clock source select for Fractional Rate Divider"]
    pub fcclksel5: FCCLKSEL5,
    #[doc = "0x2c8 - Flexcomm Interface 6 clock source select for Fractional Rate Divider"]
    pub fcclksel6: FCCLKSEL6,
    #[doc = "0x2cc - Flexcomm Interface 7 clock source select for Fractional Rate Divider"]
    pub fcclksel7: FCCLKSEL7,
    #[doc = "0x2d0 - HS LSPI clock source select"]
    pub hslspiclksel: HSLSPICLKSEL,
    _reserved13: [u8; 12usize],
    #[doc = "0x2e0 - MCLK clock source select"]
    pub mclkclksel: MCLKCLKSEL,
    _reserved14: [u8; 12usize],
    #[doc = "0x2f0 - SCTimer/PWM clock source select"]
    pub sctclksel: SCTCLKSEL,
    _reserved15: [u8; 4usize],
    #[doc = "0x2f8 - SDIO clock source select"]
    pub sdioclksel: SDIOCLKSEL,
    _reserved16: [u8; 4usize],
    #[doc = "0x300 - System Tick Timer divider for CPU0"]
    pub systickclkdiv0: SYSTICKCLKDIV0,
    #[doc = "0x304 - System Tick Timer divider for CPU1"]
    pub systickclkdiv1: SYSTICKCLKDIV1,
    #[doc = "0x308 - TRACE clock divider"]
    pub traceclkdiv: TRACECLKDIV,
    _reserved17: [u8; 20usize],
    #[doc = "0x320 - Fractional rate divider for flexcomm 0"]
    pub flexfrg0ctrl: FLEXFRG0CTRL,
    #[doc = "0x324 - Fractional rate divider for flexcomm 1"]
    pub flexfrg1ctrl: FLEXFRG1CTRL,
    #[doc = "0x328 - Fractional rate divider for flexcomm 2"]
    pub flexfrg2ctrl: FLEXFRG2CTRL,
    #[doc = "0x32c - Fractional rate divider for flexcomm 3"]
    pub flexfrg3ctrl: FLEXFRG3CTRL,
    #[doc = "0x330 - Fractional rate divider for flexcomm 4"]
    pub flexfrg4ctrl: FLEXFRG4CTRL,
    #[doc = "0x334 - Fractional rate divider for flexcomm 5"]
    pub flexfrg5ctrl: FLEXFRG5CTRL,
    #[doc = "0x338 - Fractional rate divider for flexcomm 6"]
    pub flexfrg6ctrl: FLEXFRG6CTRL,
    #[doc = "0x33c - Fractional rate divider for flexcomm 7"]
    pub flexfrg7ctrl: FLEXFRG7CTRL,
    _reserved18: [u8; 64usize],
    #[doc = "0x380 - System clock divider"]
    pub ahbclkdiv: AHBCLKDIV,
    #[doc = "0x384 - CLKOUT clock divider"]
    pub clkoutdiv: CLKOUTDIV,
    #[doc = "0x388 - FRO_HF (96MHz) clock divider"]
    pub frohfdiv: FROHFDIV,
    #[doc = "0x38c - WDT clock divider"]
    pub wdtclkdiv: WDTCLKDIV,
    _reserved19: [u8; 4usize],
    #[doc = "0x394 - ADC clock divider"]
    pub adcclkdiv: ADCCLKDIV,
    #[doc = "0x398 - USB0 Clock divider"]
    pub usb0clkdiv: USB0CLKDIV,
    _reserved20: [u8; 16usize],
    #[doc = "0x3ac - I2S MCLK clock divider"]
    pub mclkdiv: MCLKDIV,
    _reserved21: [u8; 4usize],
    #[doc = "0x3b4 - SCT/PWM clock divider"]
    pub sctclkdiv: SCTCLKDIV,
    _reserved22: [u8; 4usize],
    #[doc = "0x3bc - SDIO clock divider"]
    pub sdioclkdiv: SDIOCLKDIV,
    _reserved23: [u8; 4usize],
    #[doc = "0x3c4 - PLL0 clock divider"]
    pub pll0clkdiv: PLL0CLKDIV,
    _reserved24: [u8; 52usize],
    #[doc = "0x3fc - Control clock configuration registers access (like xxxDIV, xxxSEL)"]
    pub clockgenupdatelockout: CLOCKGENUPDATELOCKOUT,
    #[doc = "0x400 - FMC configuration register - INTERNAL USE ONLY"]
    pub fmccr: FMCCR,
    _reserved25: [u8; 8usize],
    #[doc = "0x40c - USB0 clock control"]
    pub usb0clkctrl: USB0CLKCTRL,
    #[doc = "0x410 - USB0 clock status"]
    pub usb0clkstat: USB0CLKSTAT,
    _reserved26: [u8; 8usize],
    #[doc = "0x41c - FMCflush control"]
    pub fmcflush: FMCFLUSH,
    #[doc = "0x420 - MCLK control"]
    pub mclkio: MCLKIO,
    #[doc = "0x424 - USB1 clock control"]
    pub usb1clkctrl: USB1CLKCTRL,
    #[doc = "0x428 - USB1 clock status"]
    pub usb1clkstat: USB1CLKSTAT,
    _reserved27: [u8; 36usize],
    #[doc = "0x450 - Flash Banks control"]
    pub flashbankenable: FLASHBANKENABLE,
    _reserved28: [u8; 12usize],
    #[doc = "0x460 - SDIO CCLKIN phase and delay control"]
    pub sdioclkctrl: SDIOCLKCTRL,
    _reserved29: [u8; 252usize],
    #[doc = "0x560 - PLL1 550m control"]
    pub pll1ctrl: PLL1CTRL,
    #[doc = "0x564 - PLL1 550m status"]
    pub pll1stat: PLL1STAT,
    #[doc = "0x568 - PLL1 550m N divider"]
    pub pll1ndec: PLL1NDEC,
    #[doc = "0x56c - PLL1 550m M divider"]
    pub pll1mdec: PLL1MDEC,
    #[doc = "0x570 - PLL1 550m P divider"]
    pub pll1pdec: PLL1PDEC,
    _reserved30: [u8; 12usize],
    #[doc = "0x580 - PLL0 550m control"]
    pub pll0ctrl: PLL0CTRL,
    #[doc = "0x584 - PLL0 550m status"]
    pub pll0stat: PLL0STAT,
    #[doc = "0x588 - PLL0 550m N divider"]
    pub pll0ndec: PLL0NDEC,
    #[doc = "0x58c - PLL0 550m P divider"]
    pub pll0pdec: PLL0PDEC,
    #[doc = "0x590 - PLL0 Spread Spectrum Wrapper control register 0"]
    pub pll0sscg0: PLL0SSCG0,
    #[doc = "0x594 - PLL0 Spread Spectrum Wrapper control register 1"]
    pub pll0sscg1: PLL0SSCG1,
    _reserved31: [u8; 52usize],
    #[doc = "0x5cc - eFUSE controller clock enable"]
    pub efuseclkctrl: EFUSECLKCTRL,
    _reserved32: [u8; 176usize],
    #[doc = "0x680 - Start logic wake-up enable register"]
    pub starter0: STARTER0,
    #[doc = "0x684 - Start logic wake-up enable register"]
    pub starter1: STARTER1,
    _reserved33: [u8; 24usize],
    #[doc = "0x6a0 - Set bits in STARTER"]
    pub starterset0: STARTERSET0,
    #[doc = "0x6a4 - Set bits in STARTER"]
    pub starterset1: STARTERSET1,
    _reserved34: [u8; 24usize],
    #[doc = "0x6c0 - Clear bits in STARTER"]
    pub starterclr0: STARTERCLR0,
    #[doc = "0x6c4 - Clear bits in STARTER"]
    pub starterclr1: STARTERCLR1,
    _reserved35: [u8; 184usize],
    #[doc = "0x780 - Hardware Sleep control"]
    pub hardwaresleep: HARDWARESLEEP,
    _reserved36: [u8; 124usize],
    #[doc = "0x800 - CPU Control for multiple processors"]
    pub cpuctrl: CPUCTRL,
    #[doc = "0x804 - Coprocessor Boot Address"]
    pub cpboot: CPBOOT,
    #[doc = "0x808 - Coprocessor Stack Address"]
    pub cpstack: CPSTACK,
    #[doc = "0x80c - CPU Status"]
    pub cpstat: CPSTAT,
    _reserved37: [u8; 240usize],
    #[doc = "0x900 - Composite Device Identifier"]
    pub dice_reg0: DICE_REG0,
    #[doc = "0x904 - Composite Device Identifier"]
    pub dice_reg1: DICE_REG1,
    #[doc = "0x908 - Composite Device Identifier"]
    pub dice_reg2: DICE_REG2,
    #[doc = "0x90c - Composite Device Identifier"]
    pub dice_reg3: DICE_REG3,
    #[doc = "0x910 - Composite Device Identifier"]
    pub dice_reg4: DICE_REG4,
    #[doc = "0x914 - Composite Device Identifier"]
    pub dice_reg5: DICE_REG5,
    #[doc = "0x918 - Composite Device Identifier"]
    pub dice_reg6: DICE_REG6,
    #[doc = "0x91c - Composite Device Identifier"]
    pub dice_reg7: DICE_REG7,
    _reserved38: [u8; 248usize],
    #[doc = "0xa18 - Various system clock controls : Flash clock (48 MHz) control, clocks to Frequency Measures"]
    pub clock_ctrl: CLOCK_CTRL,
    _reserved39: [u8; 244usize],
    #[doc = "0xb10 - Comparator Interrupt control"]
    pub comp_int_ctrl: COMP_INT_CTRL,
    #[doc = "0xb14 - Comparator Interrupt status"]
    pub comp_int_status: COMP_INT_STATUS,
    _reserved40: [u8; 748usize],
    #[doc = "0xe04 - Control automatic clock gating"]
    pub autoclkgateoverride: AUTOCLKGATEOVERRIDE,
    #[doc = "0xe08 - Enable bypass of the first stage of synchonization inside GPIO_INT module"]
    pub gpiopsync: GPIOPSYNC,
    _reserved41: [u8; 404usize],
    #[doc = "0xfa0 - Control write access to security registers -- FOR INTERNAl USE ONLY"]
    pub debug_lock_en: DEBUG_LOCK_EN,
    #[doc = "0xfa4 - Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control -- FOR INTERNAl USE ONLY"]
    pub debug_features: DEBUG_FEATURES,
    #[doc = "0xfa8 - Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control DUPLICATE register -- FOR INTERNAl USE ONLY"]
    pub debug_features_dp: DEBUG_FEATURES_DP,
    _reserved42: [u8; 4usize],
    #[doc = "0xfb0 - Security code to allow test (Design for Testability) access -- FOR INTERNAl USE ONLY"]
    pub codesecurityprottest: CODESECURITYPROTTEST,
    #[doc = "0xfb4 - Security code to allow CPU0 (CM33) Debug Access Port (DAP) -- FOR INTERNAl USE ONLY"]
    pub codesecurityprotcpu0: CODESECURITYPROTCPU0,
    #[doc = "0xfb8 - Security code to allow CPU1 (Micro CM33) Debug Access Port (DAP) -- FOR INTERNAl USE ONLY"]
    pub codesecurityprotcpu1: CODESECURITYPROTCPU1,
    #[doc = "0xfbc - block quiddikey/PUF all index. -- FOR INTERNAL USE ONLY"]
    pub key_block: KEY_BLOCK,
    #[doc = "0xfc0 - Debug authentication scratch registers -- FOR INTERNAL USE ONLY"]
    pub debug_auth_scratch: DEBUG_AUTH_SCRATCH,
    _reserved43: [u8; 16usize],
    #[doc = "0xfd4 - CPUs configuration register"]
    pub cpucfg: CPUCFG,
    _reserved44: [u8; 20usize],
    #[doc = "0xfec - peripheral enable configuration -- FOR INTERNAL USE ONLY"]
    pub periphencfg: PERIPHENCFG,
    _reserved45: [u8; 8usize],
    #[doc = "0xff8 - Device ID"]
    pub device_id0: DEVICE_ID0,
    #[doc = "0xffc - Chip revision ID and Number"]
    pub dieid: DIEID,
}
#[doc = "Memory Remap control register"]
pub struct MEMORYREMAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Remap control register"]
pub mod memoryremap;
#[doc = "AHB Matrix priority control register Priority values are 3 = highest, 0 = lowest"]
pub struct AHBMATPRIO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB Matrix priority control register Priority values are 3 = highest, 0 = lowest"]
pub mod ahbmatprio;
#[doc = "System tick calibration for secure part of CPU0"]
pub struct CPU0STCKCAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System tick calibration for secure part of CPU0"]
pub mod cpu0stckcal;
#[doc = "System tick calibration for non-secure part of CPU0"]
pub struct CPU0NSTCKCAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System tick calibration for non-secure part of CPU0"]
pub mod cpu0nstckcal;
#[doc = "System tick calibration for CPU1"]
pub struct CPU1TCKCAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System tick calibration for CPU1"]
pub mod cpu1tckcal;
#[doc = "NMI Source Select"]
pub struct NMISRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NMI Source Select"]
pub mod nmisrc;
#[doc = "Peripheral reset control 0"]
pub struct PRESETCTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control 0"]
pub mod presetctrl0;
#[doc = "Peripheral reset control register"]
pub struct PRESETCTRLX0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control register"]
pub mod presetctrlx0;
#[doc = "Peripheral reset control 1"]
pub struct PRESETCTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control 1"]
pub mod presetctrl1;
#[doc = "Peripheral reset control register"]
pub struct PRESETCTRLX1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control register"]
pub mod presetctrlx1;
#[doc = "Peripheral reset control 2"]
pub struct PRESETCTRL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control 2"]
pub mod presetctrl2;
#[doc = "Peripheral reset control register"]
pub struct PRESETCTRLX2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control register"]
pub mod presetctrlx2;
#[doc = "Peripheral reset control set register"]
pub struct PRESETCTRLSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control set register"]
pub mod presetctrlset;
#[doc = "Peripheral reset contro clearl register"]
pub struct PRESETCTRLCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset contro clearl register"]
pub mod presetctrlclr;
#[doc = "generate a software_reset"]
pub struct SWR_RESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "generate a software_reset"]
pub mod swr_reset;
#[doc = "AHB Clock control 0"]
pub struct AHBCLKCTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB Clock control 0"]
pub mod ahbclkctrl0;
#[doc = "Peripheral reset control register"]
pub struct AHBCLKCTRLX0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control register"]
pub mod ahbclkctrlx0;
#[doc = "AHB Clock control 1"]
pub struct AHBCLKCTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB Clock control 1"]
pub mod ahbclkctrl1;
#[doc = "Peripheral reset control register"]
pub struct AHBCLKCTRLX1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control register"]
pub mod ahbclkctrlx1;
#[doc = "AHB Clock control 2"]
pub struct AHBCLKCTRL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB Clock control 2"]
pub mod ahbclkctrl2;
#[doc = "Peripheral reset control register"]
pub struct AHBCLKCTRLX2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control register"]
pub mod ahbclkctrlx2;
#[doc = "Peripheral reset control register"]
pub struct AHBCLKCTRLSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control register"]
pub mod ahbclkctrlset;
#[doc = "Peripheral reset control register"]
pub struct AHBCLKCTRLCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control register"]
pub mod ahbclkctrlclr;
#[doc = "System Tick Timer for CPU0 source select"]
pub struct SYSTICKCLKSEL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Tick Timer for CPU0 source select"]
pub mod systickclksel0;
#[doc = "Peripheral reset control register"]
pub struct SYSTICKCLKSELX0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control register"]
pub mod systickclkselx0;
#[doc = "System Tick Timer for CPU1 source select"]
pub struct SYSTICKCLKSEL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Tick Timer for CPU1 source select"]
pub mod systickclksel1;
#[doc = "Peripheral reset control register"]
pub struct SYSTICKCLKSELX1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control register"]
pub mod systickclkselx1;
#[doc = "Trace clock source select"]
pub struct TRACECLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trace clock source select"]
pub mod traceclksel;
#[doc = "CTimer 0 clock source select"]
pub struct CTIMERCLKSEL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CTimer 0 clock source select"]
pub mod ctimerclksel0;
#[doc = "Peripheral reset control register"]
pub struct CTIMERCLKSELX0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control register"]
pub mod ctimerclkselx0;
#[doc = "CTimer 1 clock source select"]
pub struct CTIMERCLKSEL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CTimer 1 clock source select"]
pub mod ctimerclksel1;
#[doc = "Peripheral reset control register"]
pub struct CTIMERCLKSELX1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control register"]
pub mod ctimerclkselx1;
#[doc = "CTimer 2 clock source select"]
pub struct CTIMERCLKSEL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CTimer 2 clock source select"]
pub mod ctimerclksel2;
#[doc = "Peripheral reset control register"]
pub struct CTIMERCLKSELX2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control register"]
pub mod ctimerclkselx2;
#[doc = "CTimer 3 clock source select"]
pub struct CTIMERCLKSEL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CTimer 3 clock source select"]
pub mod ctimerclksel3;
#[doc = "Peripheral reset control register"]
pub struct CTIMERCLKSELX3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control register"]
pub mod ctimerclkselx3;
#[doc = "CTimer 4 clock source select"]
pub struct CTIMERCLKSEL4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CTimer 4 clock source select"]
pub mod ctimerclksel4;
#[doc = "Peripheral reset control register"]
pub struct CTIMERCLKSELX4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control register"]
pub mod ctimerclkselx4;
#[doc = "Main clock A source select"]
pub struct MAINCLKSELA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Main clock A source select"]
pub mod mainclksela;
#[doc = "Main clock source select"]
pub struct MAINCLKSELB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Main clock source select"]
pub mod mainclkselb;
#[doc = "CLKOUT clock source select"]
pub struct CLKOUTSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLKOUT clock source select"]
pub mod clkoutsel;
#[doc = "PLL0 clock source select"]
pub struct PLL0CLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL0 clock source select"]
pub mod pll0clksel;
#[doc = "PLL1 clock source select"]
pub struct PLL1CLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL1 clock source select"]
pub mod pll1clksel;
#[doc = "ADC clock source select"]
pub struct ADCCLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC clock source select"]
pub mod adcclksel;
#[doc = "FS USB clock source select"]
pub struct USB0CLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FS USB clock source select"]
pub mod usb0clksel;
#[doc = "HS USB clock source select - NOT USED"]
pub struct USB1CLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HS USB clock source select - NOT USED"]
pub mod usb1clksel;
#[doc = "Flexcomm Interface 0 clock source select for Fractional Rate Divider"]
pub struct FCCLKSEL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flexcomm Interface 0 clock source select for Fractional Rate Divider"]
pub mod fcclksel0;
#[doc = "Peripheral reset control register"]
pub struct FCCLKSELX0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control register"]
pub mod fcclkselx0;
#[doc = "Flexcomm Interface 1 clock source select for Fractional Rate Divider"]
pub struct FCCLKSEL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flexcomm Interface 1 clock source select for Fractional Rate Divider"]
pub mod fcclksel1;
#[doc = "Peripheral reset control register"]
pub struct FCCLKSELX1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control register"]
pub mod fcclkselx1;
#[doc = "Flexcomm Interface 2 clock source select for Fractional Rate Divider"]
pub struct FCCLKSEL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flexcomm Interface 2 clock source select for Fractional Rate Divider"]
pub mod fcclksel2;
#[doc = "Peripheral reset control register"]
pub struct FCCLKSELX2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control register"]
pub mod fcclkselx2;
#[doc = "Flexcomm Interface 3 clock source select for Fractional Rate Divider"]
pub struct FCCLKSEL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flexcomm Interface 3 clock source select for Fractional Rate Divider"]
pub mod fcclksel3;
#[doc = "Peripheral reset control register"]
pub struct FCCLKSELX3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control register"]
pub mod fcclkselx3;
#[doc = "Flexcomm Interface 4 clock source select for Fractional Rate Divider"]
pub struct FCCLKSEL4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flexcomm Interface 4 clock source select for Fractional Rate Divider"]
pub mod fcclksel4;
#[doc = "Peripheral reset control register"]
pub struct FCCLKSELX4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control register"]
pub mod fcclkselx4;
#[doc = "Flexcomm Interface 5 clock source select for Fractional Rate Divider"]
pub struct FCCLKSEL5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flexcomm Interface 5 clock source select for Fractional Rate Divider"]
pub mod fcclksel5;
#[doc = "Peripheral reset control register"]
pub struct FCCLKSELX5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control register"]
pub mod fcclkselx5;
#[doc = "Flexcomm Interface 6 clock source select for Fractional Rate Divider"]
pub struct FCCLKSEL6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flexcomm Interface 6 clock source select for Fractional Rate Divider"]
pub mod fcclksel6;
#[doc = "Peripheral reset control register"]
pub struct FCCLKSELX6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control register"]
pub mod fcclkselx6;
#[doc = "Flexcomm Interface 7 clock source select for Fractional Rate Divider"]
pub struct FCCLKSEL7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flexcomm Interface 7 clock source select for Fractional Rate Divider"]
pub mod fcclksel7;
#[doc = "Peripheral reset control register"]
pub struct FCCLKSELX7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control register"]
pub mod fcclkselx7;
#[doc = "HS LSPI clock source select"]
pub struct HSLSPICLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HS LSPI clock source select"]
pub mod hslspiclksel;
#[doc = "MCLK clock source select"]
pub struct MCLKCLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCLK clock source select"]
pub mod mclkclksel;
#[doc = "SCTimer/PWM clock source select"]
pub struct SCTCLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCTimer/PWM clock source select"]
pub mod sctclksel;
#[doc = "SDIO clock source select"]
pub struct SDIOCLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDIO clock source select"]
pub mod sdioclksel;
#[doc = "System Tick Timer divider for CPU0"]
pub struct SYSTICKCLKDIV0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Tick Timer divider for CPU0"]
pub mod systickclkdiv0;
#[doc = "System Tick Timer divider for CPU1"]
pub struct SYSTICKCLKDIV1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Tick Timer divider for CPU1"]
pub mod systickclkdiv1;
#[doc = "TRACE clock divider"]
pub struct TRACECLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRACE clock divider"]
pub mod traceclkdiv;
#[doc = "Fractional rate divider for flexcomm 0"]
pub struct FLEXFRG0CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fractional rate divider for flexcomm 0"]
pub mod flexfrg0ctrl;
#[doc = "Peripheral reset control register"]
pub struct FLEXFRGXCTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control register"]
pub mod flexfrgxctrl0;
#[doc = "Fractional rate divider for flexcomm 1"]
pub struct FLEXFRG1CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fractional rate divider for flexcomm 1"]
pub mod flexfrg1ctrl;
#[doc = "Peripheral reset control register"]
pub struct FLEXFRGXCTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control register"]
pub mod flexfrgxctrl1;
#[doc = "Fractional rate divider for flexcomm 2"]
pub struct FLEXFRG2CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fractional rate divider for flexcomm 2"]
pub mod flexfrg2ctrl;
#[doc = "Peripheral reset control register"]
pub struct FLEXFRGXCTRL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control register"]
pub mod flexfrgxctrl2;
#[doc = "Fractional rate divider for flexcomm 3"]
pub struct FLEXFRG3CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fractional rate divider for flexcomm 3"]
pub mod flexfrg3ctrl;
#[doc = "Peripheral reset control register"]
pub struct FLEXFRGXCTRL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control register"]
pub mod flexfrgxctrl3;
#[doc = "Fractional rate divider for flexcomm 4"]
pub struct FLEXFRG4CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fractional rate divider for flexcomm 4"]
pub mod flexfrg4ctrl;
#[doc = "Peripheral reset control register"]
pub struct FLEXFRGXCTRL4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control register"]
pub mod flexfrgxctrl4;
#[doc = "Fractional rate divider for flexcomm 5"]
pub struct FLEXFRG5CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fractional rate divider for flexcomm 5"]
pub mod flexfrg5ctrl;
#[doc = "Peripheral reset control register"]
pub struct FLEXFRGXCTRL5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control register"]
pub mod flexfrgxctrl5;
#[doc = "Fractional rate divider for flexcomm 6"]
pub struct FLEXFRG6CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fractional rate divider for flexcomm 6"]
pub mod flexfrg6ctrl;
#[doc = "Peripheral reset control register"]
pub struct FLEXFRGXCTRL6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control register"]
pub mod flexfrgxctrl6;
#[doc = "Fractional rate divider for flexcomm 7"]
pub struct FLEXFRG7CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fractional rate divider for flexcomm 7"]
pub mod flexfrg7ctrl;
#[doc = "Peripheral reset control register"]
pub struct FLEXFRGXCTRL7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control register"]
pub mod flexfrgxctrl7;
#[doc = "System clock divider"]
pub struct AHBCLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System clock divider"]
pub mod ahbclkdiv;
#[doc = "CLKOUT clock divider"]
pub struct CLKOUTDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLKOUT clock divider"]
pub mod clkoutdiv;
#[doc = "FRO_HF (96MHz) clock divider"]
pub struct FROHFDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FRO_HF (96MHz) clock divider"]
pub mod frohfdiv;
#[doc = "WDT clock divider"]
pub struct WDTCLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WDT clock divider"]
pub mod wdtclkdiv;
#[doc = "ADC clock divider"]
pub struct ADCCLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC clock divider"]
pub mod adcclkdiv;
#[doc = "USB0 Clock divider"]
pub struct USB0CLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB0 Clock divider"]
pub mod usb0clkdiv;
#[doc = "I2S MCLK clock divider"]
pub struct MCLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2S MCLK clock divider"]
pub mod mclkdiv;
#[doc = "SCT/PWM clock divider"]
pub struct SCTCLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT/PWM clock divider"]
pub mod sctclkdiv;
#[doc = "SDIO clock divider"]
pub struct SDIOCLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDIO clock divider"]
pub mod sdioclkdiv;
#[doc = "PLL0 clock divider"]
pub struct PLL0CLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL0 clock divider"]
pub mod pll0clkdiv;
#[doc = "Control clock configuration registers access (like xxxDIV, xxxSEL)"]
pub struct CLOCKGENUPDATELOCKOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control clock configuration registers access (like xxxDIV, xxxSEL)"]
pub mod clockgenupdatelockout;
#[doc = "FMC configuration register - INTERNAL USE ONLY"]
pub struct FMCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FMC configuration register - INTERNAL USE ONLY"]
pub mod fmccr;
#[doc = "USB0 clock control"]
pub struct USB0CLKCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB0 clock control"]
pub mod usb0clkctrl;
#[doc = "USB0 clock status"]
pub struct USB0CLKSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB0 clock status"]
pub mod usb0clkstat;
#[doc = "FMCflush control"]
pub struct FMCFLUSH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FMCflush control"]
pub mod fmcflush;
#[doc = "MCLK control"]
pub struct MCLKIO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCLK control"]
pub mod mclkio;
#[doc = "USB1 clock control"]
pub struct USB1CLKCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB1 clock control"]
pub mod usb1clkctrl;
#[doc = "USB1 clock status"]
pub struct USB1CLKSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB1 clock status"]
pub mod usb1clkstat;
#[doc = "Flash Banks control"]
pub struct FLASHBANKENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Banks control"]
pub mod flashbankenable;
#[doc = "SDIO CCLKIN phase and delay control"]
pub struct SDIOCLKCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDIO CCLKIN phase and delay control"]
pub mod sdioclkctrl;
#[doc = "PLL1 550m control"]
pub struct PLL1CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL1 550m control"]
pub mod pll1ctrl;
#[doc = "PLL1 550m status"]
pub struct PLL1STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL1 550m status"]
pub mod pll1stat;
#[doc = "PLL1 550m N divider"]
pub struct PLL1NDEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL1 550m N divider"]
pub mod pll1ndec;
#[doc = "PLL1 550m M divider"]
pub struct PLL1MDEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL1 550m M divider"]
pub mod pll1mdec;
#[doc = "PLL1 550m P divider"]
pub struct PLL1PDEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL1 550m P divider"]
pub mod pll1pdec;
#[doc = "PLL0 550m control"]
pub struct PLL0CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL0 550m control"]
pub mod pll0ctrl;
#[doc = "PLL0 550m status"]
pub struct PLL0STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL0 550m status"]
pub mod pll0stat;
#[doc = "PLL0 550m N divider"]
pub struct PLL0NDEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL0 550m N divider"]
pub mod pll0ndec;
#[doc = "PLL0 550m P divider"]
pub struct PLL0PDEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL0 550m P divider"]
pub mod pll0pdec;
#[doc = "PLL0 Spread Spectrum Wrapper control register 0"]
pub struct PLL0SSCG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL0 Spread Spectrum Wrapper control register 0"]
pub mod pll0sscg0;
#[doc = "PLL0 Spread Spectrum Wrapper control register 1"]
pub struct PLL0SSCG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL0 Spread Spectrum Wrapper control register 1"]
pub mod pll0sscg1;
#[doc = "eFUSE controller clock enable"]
pub struct EFUSECLKCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "eFUSE controller clock enable"]
pub mod efuseclkctrl;
#[doc = "Start logic wake-up enable register"]
pub struct STARTER0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start logic wake-up enable register"]
pub mod starter0;
#[doc = "Start logic wake-up enable register"]
pub struct STARTER1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start logic wake-up enable register"]
pub mod starter1;
#[doc = "Set bits in STARTER"]
pub struct STARTERSET0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set bits in STARTER"]
pub mod starterset0;
#[doc = "Set bits in STARTER"]
pub struct STARTERSET1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set bits in STARTER"]
pub mod starterset1;
#[doc = "Clear bits in STARTER"]
pub struct STARTERCLR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear bits in STARTER"]
pub mod starterclr0;
#[doc = "Clear bits in STARTER"]
pub struct STARTERCLR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear bits in STARTER"]
pub mod starterclr1;
#[doc = "Hardware Sleep control"]
pub struct HARDWARESLEEP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware Sleep control"]
pub mod hardwaresleep;
#[doc = "CPU Control for multiple processors"]
pub struct CPUCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU Control for multiple processors"]
pub mod cpuctrl;
#[doc = "Coprocessor Boot Address"]
pub struct CPBOOT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Coprocessor Boot Address"]
pub mod cpboot;
#[doc = "Coprocessor Stack Address"]
pub struct CPSTACK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Coprocessor Stack Address"]
pub mod cpstack;
#[doc = "CPU Status"]
pub struct CPSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU Status"]
pub mod cpstat;
#[doc = "Composite Device Identifier"]
pub struct DICE_REG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Composite Device Identifier"]
pub mod dice_reg0;
#[doc = "Composite Device Identifier"]
pub struct DICE_REG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Composite Device Identifier"]
pub mod dice_reg1;
#[doc = "Composite Device Identifier"]
pub struct DICE_REG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Composite Device Identifier"]
pub mod dice_reg2;
#[doc = "Composite Device Identifier"]
pub struct DICE_REG3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Composite Device Identifier"]
pub mod dice_reg3;
#[doc = "Composite Device Identifier"]
pub struct DICE_REG4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Composite Device Identifier"]
pub mod dice_reg4;
#[doc = "Composite Device Identifier"]
pub struct DICE_REG5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Composite Device Identifier"]
pub mod dice_reg5;
#[doc = "Composite Device Identifier"]
pub struct DICE_REG6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Composite Device Identifier"]
pub mod dice_reg6;
#[doc = "Composite Device Identifier"]
pub struct DICE_REG7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Composite Device Identifier"]
pub mod dice_reg7;
#[doc = "Various system clock controls : Flash clock (48 MHz) control, clocks to Frequency Measures"]
pub struct CLOCK_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Various system clock controls : Flash clock (48 MHz) control, clocks to Frequency Measures"]
pub mod clock_ctrl;
#[doc = "Comparator Interrupt control"]
pub struct COMP_INT_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Comparator Interrupt control"]
pub mod comp_int_ctrl;
#[doc = "Comparator Interrupt status"]
pub struct COMP_INT_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Comparator Interrupt status"]
pub mod comp_int_status;
#[doc = "Control automatic clock gating"]
pub struct AUTOCLKGATEOVERRIDE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control automatic clock gating"]
pub mod autoclkgateoverride;
#[doc = "Enable bypass of the first stage of synchonization inside GPIO_INT module"]
pub struct GPIOPSYNC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable bypass of the first stage of synchonization inside GPIO_INT module"]
pub mod gpiopsync;
#[doc = "Control write access to security registers -- FOR INTERNAl USE ONLY"]
pub struct DEBUG_LOCK_EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control write access to security registers -- FOR INTERNAl USE ONLY"]
pub mod debug_lock_en;
#[doc = "Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control -- FOR INTERNAl USE ONLY"]
pub struct DEBUG_FEATURES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control -- FOR INTERNAl USE ONLY"]
pub mod debug_features;
#[doc = "Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control DUPLICATE register -- FOR INTERNAl USE ONLY"]
pub struct DEBUG_FEATURES_DP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control DUPLICATE register -- FOR INTERNAl USE ONLY"]
pub mod debug_features_dp;
#[doc = "Security code to allow test (Design for Testability) access -- FOR INTERNAl USE ONLY"]
pub struct CODESECURITYPROTTEST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security code to allow test (Design for Testability) access -- FOR INTERNAl USE ONLY"]
pub mod codesecurityprottest;
#[doc = "Security code to allow CPU0 (CM33) Debug Access Port (DAP) -- FOR INTERNAl USE ONLY"]
pub struct CODESECURITYPROTCPU0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security code to allow CPU0 (CM33) Debug Access Port (DAP) -- FOR INTERNAl USE ONLY"]
pub mod codesecurityprotcpu0;
#[doc = "Security code to allow CPU1 (Micro CM33) Debug Access Port (DAP) -- FOR INTERNAl USE ONLY"]
pub struct CODESECURITYPROTCPU1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security code to allow CPU1 (Micro CM33) Debug Access Port (DAP) -- FOR INTERNAl USE ONLY"]
pub mod codesecurityprotcpu1;
#[doc = "block quiddikey/PUF all index. -- FOR INTERNAL USE ONLY"]
pub struct KEY_BLOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "block quiddikey/PUF all index. -- FOR INTERNAL USE ONLY"]
pub mod key_block;
#[doc = "Debug authentication scratch registers -- FOR INTERNAL USE ONLY"]
pub struct DEBUG_AUTH_SCRATCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debug authentication scratch registers -- FOR INTERNAL USE ONLY"]
pub mod debug_auth_scratch;
#[doc = "CPUs configuration register"]
pub struct CPUCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPUs configuration register"]
pub mod cpucfg;
#[doc = "peripheral enable configuration -- FOR INTERNAL USE ONLY"]
pub struct PERIPHENCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "peripheral enable configuration -- FOR INTERNAL USE ONLY"]
pub mod periphencfg;
#[doc = "Device ID"]
pub struct DEVICE_ID0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device ID"]
pub mod device_id0;
#[doc = "Chip revision ID and Number"]
pub struct DIEID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Chip revision ID and Number"]
pub mod dieid;
