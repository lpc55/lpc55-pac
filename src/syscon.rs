#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Memory Remap control register"]
    pub memoryremap: MEMORYREMAP,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - AHB Matrix priority control register Priority values are 3 = highest, 0 = lowest"]
    pub ahbmatprio: AHBMATPRIO,
    _reserved2: [u8; 36usize],
    #[doc = "0x38 - System tick calibration for secure part of CPU0"]
    pub cpu0stckcal: CPU0STCKCAL,
    #[doc = "0x3c - System tick calibration for non-secure part of CPU0"]
    pub cpu0nstckcal: CPU0NSTCKCAL,
    #[doc = "0x40 - System tick calibration for CPU1"]
    pub cpu1tckcal: CPU1TCKCAL,
    _reserved5: [u8; 4usize],
    #[doc = "0x48 - NMI Source Select"]
    pub nmisrc: NMISRC,
    _reserved6: [u8; 180usize],
    #[doc = "0x100 - Peripheral reset control 0"]
    pub presetctrl0: PRESETCTRL0,
    #[doc = "0x104 - Peripheral reset control 1"]
    pub presetctrl1: PRESETCTRL1,
    #[doc = "0x108 - Peripheral reset control 2"]
    pub presetctrl2: PRESETCTRL2,
    _reserved9: [u8; 20usize],
    #[doc = "0x120 - Peripheral reset control set register"]
    pub presetctrlset: [PRESETCTRLSET; 3],
    _reserved10: [u8; 20usize],
    #[doc = "0x140 - Peripheral reset contro clearl register"]
    pub presetctrlclr: [PRESETCTRLCLR; 3],
    _reserved11: [u8; 20usize],
    #[doc = "0x160 - generate a software_reset"]
    pub swr_reset: SWR_RESET,
    _reserved12: [u8; 156usize],
    #[doc = "0x200 - AHB Clock control 0"]
    pub ahbclkctrl0: AHBCLKCTRL0,
    #[doc = "0x204 - AHB Clock control 1"]
    pub ahbclkctrl1: AHBCLKCTRL1,
    #[doc = "0x208 - AHB Clock control 2"]
    pub ahbclkctrl2: AHBCLKCTRL2,
    _reserved15: [u8; 20usize],
    #[doc = "0x220 - Peripheral reset control register"]
    pub ahbclkctrlset: [AHBCLKCTRLSET; 3],
    _reserved16: [u8; 20usize],
    #[doc = "0x240 - Peripheral reset control register"]
    pub ahbclkctrlclr: [AHBCLKCTRLCLR; 3],
    _reserved17: [u8; 20usize],
    _reserved_17_systickclksel0: [u8; 4usize],
    _reserved_18_systickclksel1: [u8; 4usize],
    #[doc = "0x268 - Trace clock source select"]
    pub traceclksel: TRACECLKSEL,
    _reserved_20_ctimerclksel0: [u8; 4usize],
    _reserved_21_ctimerclksel1: [u8; 4usize],
    _reserved_22_ctimerclksel2: [u8; 4usize],
    _reserved_23_ctimerclksel3: [u8; 4usize],
    _reserved_24_ctimerclksel4: [u8; 4usize],
    #[doc = "0x280 - Main clock A source select"]
    pub mainclksela: MAINCLKSELA,
    #[doc = "0x284 - Main clock source select"]
    pub mainclkselb: MAINCLKSELB,
    #[doc = "0x288 - CLKOUT clock source select"]
    pub clkoutsel: CLKOUTSEL,
    _reserved28: [u8; 4usize],
    #[doc = "0x290 - PLL0 clock source select"]
    pub pll0clksel: PLL0CLKSEL,
    #[doc = "0x294 - PLL1 clock source select"]
    pub pll1clksel: PLL1CLKSEL,
    _reserved30: [u8; 12usize],
    #[doc = "0x2a4 - ADC clock source select"]
    pub adcclksel: ADCCLKSEL,
    #[doc = "0x2a8 - FS USB clock source select"]
    pub usb0clksel: USB0CLKSEL,
    #[doc = "0x2ac - HS USB clock source select - NOT USED"]
    pub usb1clksel: USB1CLKSEL,
    _reserved_33_fcclksel0: [u8; 4usize],
    _reserved_34_fcclksel1: [u8; 4usize],
    _reserved_35_fcclksel2: [u8; 4usize],
    _reserved_36_fcclksel3: [u8; 4usize],
    _reserved_37_fcclksel4: [u8; 4usize],
    _reserved_38_fcclksel5: [u8; 4usize],
    _reserved_39_fcclksel6: [u8; 4usize],
    _reserved_40_fcclksel7: [u8; 4usize],
    #[doc = "0x2d0 - HS LSPI clock source select"]
    pub hslspiclksel: HSLSPICLKSEL,
    _reserved42: [u8; 12usize],
    #[doc = "0x2e0 - MCLK clock source select"]
    pub mclkclksel: MCLKCLKSEL,
    _reserved43: [u8; 12usize],
    #[doc = "0x2f0 - SCTimer/PWM clock source select"]
    pub sctclksel: SCTCLKSEL,
    _reserved44: [u8; 4usize],
    #[doc = "0x2f8 - SDIO clock source select"]
    pub sdioclksel: SDIOCLKSEL,
    _reserved45: [u8; 4usize],
    #[doc = "0x300 - System Tick Timer divider for CPU0"]
    pub systickclkdiv0: SYSTICKCLKDIV0,
    #[doc = "0x304 - System Tick Timer divider for CPU1"]
    pub systickclkdiv1: SYSTICKCLKDIV1,
    #[doc = "0x308 - TRACE clock divider"]
    pub traceclkdiv: TRACECLKDIV,
    _reserved48: [u8; 20usize],
    _reserved_48_flexfrg0ctrl: [u8; 4usize],
    _reserved_49_flexfrg1ctrl: [u8; 4usize],
    _reserved_50_flexfrg2ctrl: [u8; 4usize],
    _reserved_51_flexfrg3ctrl: [u8; 4usize],
    _reserved_52_flexfrg4ctrl: [u8; 4usize],
    _reserved_53_flexfrg5ctrl: [u8; 4usize],
    _reserved_54_flexfrg6ctrl: [u8; 4usize],
    _reserved_55_flexfrg7ctrl: [u8; 4usize],
    _reserved56: [u8; 64usize],
    #[doc = "0x380 - System clock divider"]
    pub ahbclkdiv: AHBCLKDIV,
    #[doc = "0x384 - CLKOUT clock divider"]
    pub clkoutdiv: CLKOUTDIV,
    #[doc = "0x388 - FRO_HF (96MHz) clock divider"]
    pub frohfdiv: FROHFDIV,
    #[doc = "0x38c - WDT clock divider"]
    pub wdtclkdiv: WDTCLKDIV,
    _reserved60: [u8; 4usize],
    #[doc = "0x394 - ADC clock divider"]
    pub adcclkdiv: ADCCLKDIV,
    #[doc = "0x398 - USB0 Clock divider"]
    pub usb0clkdiv: USB0CLKDIV,
    _reserved62: [u8; 16usize],
    #[doc = "0x3ac - I2S MCLK clock divider"]
    pub mclkdiv: MCLKDIV,
    _reserved63: [u8; 4usize],
    #[doc = "0x3b4 - SCT/PWM clock divider"]
    pub sctclkdiv: SCTCLKDIV,
    _reserved64: [u8; 4usize],
    #[doc = "0x3bc - SDIO clock divider"]
    pub sdioclkdiv: SDIOCLKDIV,
    _reserved65: [u8; 4usize],
    #[doc = "0x3c4 - PLL0 clock divider"]
    pub pll0clkdiv: PLL0CLKDIV,
    _reserved66: [u8; 52usize],
    #[doc = "0x3fc - Control clock configuration registers access (like xxxDIV, xxxSEL)"]
    pub clockgenupdatelockout: CLOCKGENUPDATELOCKOUT,
    #[doc = "0x400 - FMC configuration register - INTERNAL USE ONLY"]
    pub fmccr: FMCCR,
    _reserved68: [u8; 8usize],
    #[doc = "0x40c - USB0 clock control"]
    pub usb0clkctrl: USB0CLKCTRL,
    #[doc = "0x410 - USB0 clock status"]
    pub usb0clkstat: USB0CLKSTAT,
    _reserved70: [u8; 8usize],
    #[doc = "0x41c - FMCflush control"]
    pub fmcflush: FMCFLUSH,
    #[doc = "0x420 - MCLK control"]
    pub mclkio: MCLKIO,
    #[doc = "0x424 - USB1 clock control"]
    pub usb1clkctrl: USB1CLKCTRL,
    #[doc = "0x428 - USB1 clock status"]
    pub usb1clkstat: USB1CLKSTAT,
    _reserved74: [u8; 36usize],
    #[doc = "0x450 - Flash Banks control"]
    pub flashbankenable: FLASHBANKENABLE,
    _reserved75: [u8; 12usize],
    #[doc = "0x460 - SDIO CCLKIN phase and delay control"]
    pub sdioclkctrl: SDIOCLKCTRL,
    _reserved76: [u8; 252usize],
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
    _reserved81: [u8; 12usize],
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
    _reserved87: [u8; 52usize],
    #[doc = "0x5cc - eFUSE controller clock enable"]
    pub efuseclkctrl: EFUSECLKCTRL,
    _reserved88: [u8; 176usize],
    #[doc = "0x680 - Start logic wake-up enable register"]
    pub starter0: STARTER0,
    #[doc = "0x684 - Start logic wake-up enable register"]
    pub starter1: STARTER1,
    _reserved90: [u8; 24usize],
    #[doc = "0x6a0 - Set bits in STARTER"]
    pub starterset0: STARTERSET0,
    #[doc = "0x6a4 - Set bits in STARTER"]
    pub starterset1: STARTERSET1,
    _reserved92: [u8; 24usize],
    #[doc = "0x6c0 - Clear bits in STARTER"]
    pub starterclr0: STARTERCLR0,
    #[doc = "0x6c4 - Clear bits in STARTER"]
    pub starterclr1: STARTERCLR1,
    _reserved94: [u8; 184usize],
    #[doc = "0x780 - Hardware Sleep control"]
    pub hardwaresleep: HARDWARESLEEP,
    _reserved95: [u8; 124usize],
    #[doc = "0x800 - CPU Control for multiple processors"]
    pub cpuctrl: CPUCTRL,
    #[doc = "0x804 - Coprocessor Boot Address"]
    pub cpboot: CPBOOT,
    #[doc = "0x808 - Coprocessor Stack Address"]
    pub cpstack: CPSTACK,
    #[doc = "0x80c - CPU Status"]
    pub cpstat: CPSTAT,
    _reserved99: [u8; 240usize],
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
    _reserved107: [u8; 248usize],
    #[doc = "0xa18 - Various system clock controls : Flash clock (48 MHz) control, clocks to Frequency Measures"]
    pub clock_ctrl: CLOCK_CTRL,
    _reserved108: [u8; 244usize],
    #[doc = "0xb10 - Comparator Interrupt control"]
    pub comp_int_ctrl: COMP_INT_CTRL,
    #[doc = "0xb14 - Comparator Interrupt status"]
    pub comp_int_status: COMP_INT_STATUS,
    _reserved110: [u8; 748usize],
    #[doc = "0xe04 - Control automatic clock gating"]
    pub autoclkgateoverride: AUTOCLKGATEOVERRIDE,
    #[doc = "0xe08 - Enable bypass of the first stage of synchonization inside GPIO_INT module"]
    pub gpiopsync: GPIOPSYNC,
    _reserved112: [u8; 404usize],
    #[doc = "0xfa0 - Control write access to security registers -- FOR INTERNAl USE ONLY"]
    pub debug_lock_en: DEBUG_LOCK_EN,
    #[doc = "0xfa4 - Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control -- FOR INTERNAl USE ONLY"]
    pub debug_features: DEBUG_FEATURES,
    #[doc = "0xfa8 - Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control DUPLICATE register -- FOR INTERNAl USE ONLY"]
    pub debug_features_dp: DEBUG_FEATURES_DP,
    _reserved115: [u8; 4usize],
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
    _reserved120: [u8; 16usize],
    #[doc = "0xfd4 - CPUs configuration register"]
    pub cpucfg: CPUCFG,
    _reserved121: [u8; 20usize],
    #[doc = "0xfec - peripheral enable configuration -- FOR INTERNAL USE ONLY"]
    pub periphencfg: PERIPHENCFG,
    _reserved122: [u8; 8usize],
    #[doc = "0xff8 - Device ID"]
    pub device_id0: DEVICE_ID0,
    #[doc = "0xffc - Chip revision ID and Number"]
    pub dieid: DIEID,
}
impl RegisterBlock {
    #[doc = "0x260 - Peripheral reset control register"]
    #[inline(always)]
    pub fn systickclkselx0(&self) -> &SYSTICKCLKSELX0 {
        unsafe { &*(((self as *const Self) as *const u8).add(608usize) as *const SYSTICKCLKSELX0) }
    }
    #[doc = "0x260 - Peripheral reset control register"]
    #[inline(always)]
    pub fn systickclkselx0_mut(&self) -> &mut SYSTICKCLKSELX0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(608usize) as *mut SYSTICKCLKSELX0) }
    }
    #[doc = "0x260 - System Tick Timer for CPU0 source select"]
    #[inline(always)]
    pub fn systickclksel0(&self) -> &SYSTICKCLKSEL0 {
        unsafe { &*(((self as *const Self) as *const u8).add(608usize) as *const SYSTICKCLKSEL0) }
    }
    #[doc = "0x260 - System Tick Timer for CPU0 source select"]
    #[inline(always)]
    pub fn systickclksel0_mut(&self) -> &mut SYSTICKCLKSEL0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(608usize) as *mut SYSTICKCLKSEL0) }
    }
    #[doc = "0x264 - Peripheral reset control register"]
    #[inline(always)]
    pub fn systickclkselx1(&self) -> &SYSTICKCLKSELX1 {
        unsafe { &*(((self as *const Self) as *const u8).add(612usize) as *const SYSTICKCLKSELX1) }
    }
    #[doc = "0x264 - Peripheral reset control register"]
    #[inline(always)]
    pub fn systickclkselx1_mut(&self) -> &mut SYSTICKCLKSELX1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(612usize) as *mut SYSTICKCLKSELX1) }
    }
    #[doc = "0x264 - System Tick Timer for CPU1 source select"]
    #[inline(always)]
    pub fn systickclksel1(&self) -> &SYSTICKCLKSEL1 {
        unsafe { &*(((self as *const Self) as *const u8).add(612usize) as *const SYSTICKCLKSEL1) }
    }
    #[doc = "0x264 - System Tick Timer for CPU1 source select"]
    #[inline(always)]
    pub fn systickclksel1_mut(&self) -> &mut SYSTICKCLKSEL1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(612usize) as *mut SYSTICKCLKSEL1) }
    }
    #[doc = "0x26c - Peripheral reset control register"]
    #[inline(always)]
    pub fn ctimerclkselx0(&self) -> &CTIMERCLKSELX0 {
        unsafe { &*(((self as *const Self) as *const u8).add(620usize) as *const CTIMERCLKSELX0) }
    }
    #[doc = "0x26c - Peripheral reset control register"]
    #[inline(always)]
    pub fn ctimerclkselx0_mut(&self) -> &mut CTIMERCLKSELX0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(620usize) as *mut CTIMERCLKSELX0) }
    }
    #[doc = "0x26c - CTimer 0 clock source select"]
    #[inline(always)]
    pub fn ctimerclksel0(&self) -> &CTIMERCLKSEL0 {
        unsafe { &*(((self as *const Self) as *const u8).add(620usize) as *const CTIMERCLKSEL0) }
    }
    #[doc = "0x26c - CTimer 0 clock source select"]
    #[inline(always)]
    pub fn ctimerclksel0_mut(&self) -> &mut CTIMERCLKSEL0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(620usize) as *mut CTIMERCLKSEL0) }
    }
    #[doc = "0x270 - Peripheral reset control register"]
    #[inline(always)]
    pub fn ctimerclkselx1(&self) -> &CTIMERCLKSELX1 {
        unsafe { &*(((self as *const Self) as *const u8).add(624usize) as *const CTIMERCLKSELX1) }
    }
    #[doc = "0x270 - Peripheral reset control register"]
    #[inline(always)]
    pub fn ctimerclkselx1_mut(&self) -> &mut CTIMERCLKSELX1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(624usize) as *mut CTIMERCLKSELX1) }
    }
    #[doc = "0x270 - CTimer 1 clock source select"]
    #[inline(always)]
    pub fn ctimerclksel1(&self) -> &CTIMERCLKSEL1 {
        unsafe { &*(((self as *const Self) as *const u8).add(624usize) as *const CTIMERCLKSEL1) }
    }
    #[doc = "0x270 - CTimer 1 clock source select"]
    #[inline(always)]
    pub fn ctimerclksel1_mut(&self) -> &mut CTIMERCLKSEL1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(624usize) as *mut CTIMERCLKSEL1) }
    }
    #[doc = "0x274 - Peripheral reset control register"]
    #[inline(always)]
    pub fn ctimerclkselx2(&self) -> &CTIMERCLKSELX2 {
        unsafe { &*(((self as *const Self) as *const u8).add(628usize) as *const CTIMERCLKSELX2) }
    }
    #[doc = "0x274 - Peripheral reset control register"]
    #[inline(always)]
    pub fn ctimerclkselx2_mut(&self) -> &mut CTIMERCLKSELX2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(628usize) as *mut CTIMERCLKSELX2) }
    }
    #[doc = "0x274 - CTimer 2 clock source select"]
    #[inline(always)]
    pub fn ctimerclksel2(&self) -> &CTIMERCLKSEL2 {
        unsafe { &*(((self as *const Self) as *const u8).add(628usize) as *const CTIMERCLKSEL2) }
    }
    #[doc = "0x274 - CTimer 2 clock source select"]
    #[inline(always)]
    pub fn ctimerclksel2_mut(&self) -> &mut CTIMERCLKSEL2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(628usize) as *mut CTIMERCLKSEL2) }
    }
    #[doc = "0x278 - Peripheral reset control register"]
    #[inline(always)]
    pub fn ctimerclkselx3(&self) -> &CTIMERCLKSELX3 {
        unsafe { &*(((self as *const Self) as *const u8).add(632usize) as *const CTIMERCLKSELX3) }
    }
    #[doc = "0x278 - Peripheral reset control register"]
    #[inline(always)]
    pub fn ctimerclkselx3_mut(&self) -> &mut CTIMERCLKSELX3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(632usize) as *mut CTIMERCLKSELX3) }
    }
    #[doc = "0x278 - CTimer 3 clock source select"]
    #[inline(always)]
    pub fn ctimerclksel3(&self) -> &CTIMERCLKSEL3 {
        unsafe { &*(((self as *const Self) as *const u8).add(632usize) as *const CTIMERCLKSEL3) }
    }
    #[doc = "0x278 - CTimer 3 clock source select"]
    #[inline(always)]
    pub fn ctimerclksel3_mut(&self) -> &mut CTIMERCLKSEL3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(632usize) as *mut CTIMERCLKSEL3) }
    }
    #[doc = "0x27c - Peripheral reset control register"]
    #[inline(always)]
    pub fn ctimerclkselx4(&self) -> &CTIMERCLKSELX4 {
        unsafe { &*(((self as *const Self) as *const u8).add(636usize) as *const CTIMERCLKSELX4) }
    }
    #[doc = "0x27c - Peripheral reset control register"]
    #[inline(always)]
    pub fn ctimerclkselx4_mut(&self) -> &mut CTIMERCLKSELX4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(636usize) as *mut CTIMERCLKSELX4) }
    }
    #[doc = "0x27c - CTimer 4 clock source select"]
    #[inline(always)]
    pub fn ctimerclksel4(&self) -> &CTIMERCLKSEL4 {
        unsafe { &*(((self as *const Self) as *const u8).add(636usize) as *const CTIMERCLKSEL4) }
    }
    #[doc = "0x27c - CTimer 4 clock source select"]
    #[inline(always)]
    pub fn ctimerclksel4_mut(&self) -> &mut CTIMERCLKSEL4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(636usize) as *mut CTIMERCLKSEL4) }
    }
    #[doc = "0x2b0 - Peripheral reset control register"]
    #[inline(always)]
    pub fn fcclkselx0(&self) -> &FCCLKSELX0 {
        unsafe { &*(((self as *const Self) as *const u8).add(688usize) as *const FCCLKSELX0) }
    }
    #[doc = "0x2b0 - Peripheral reset control register"]
    #[inline(always)]
    pub fn fcclkselx0_mut(&self) -> &mut FCCLKSELX0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(688usize) as *mut FCCLKSELX0) }
    }
    #[doc = "0x2b0 - Flexcomm Interface 0 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub fn fcclksel0(&self) -> &FCCLKSEL0 {
        unsafe { &*(((self as *const Self) as *const u8).add(688usize) as *const FCCLKSEL0) }
    }
    #[doc = "0x2b0 - Flexcomm Interface 0 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub fn fcclksel0_mut(&self) -> &mut FCCLKSEL0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(688usize) as *mut FCCLKSEL0) }
    }
    #[doc = "0x2b4 - Peripheral reset control register"]
    #[inline(always)]
    pub fn fcclkselx1(&self) -> &FCCLKSELX1 {
        unsafe { &*(((self as *const Self) as *const u8).add(692usize) as *const FCCLKSELX1) }
    }
    #[doc = "0x2b4 - Peripheral reset control register"]
    #[inline(always)]
    pub fn fcclkselx1_mut(&self) -> &mut FCCLKSELX1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(692usize) as *mut FCCLKSELX1) }
    }
    #[doc = "0x2b4 - Flexcomm Interface 1 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub fn fcclksel1(&self) -> &FCCLKSEL1 {
        unsafe { &*(((self as *const Self) as *const u8).add(692usize) as *const FCCLKSEL1) }
    }
    #[doc = "0x2b4 - Flexcomm Interface 1 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub fn fcclksel1_mut(&self) -> &mut FCCLKSEL1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(692usize) as *mut FCCLKSEL1) }
    }
    #[doc = "0x2b8 - Peripheral reset control register"]
    #[inline(always)]
    pub fn fcclkselx2(&self) -> &FCCLKSELX2 {
        unsafe { &*(((self as *const Self) as *const u8).add(696usize) as *const FCCLKSELX2) }
    }
    #[doc = "0x2b8 - Peripheral reset control register"]
    #[inline(always)]
    pub fn fcclkselx2_mut(&self) -> &mut FCCLKSELX2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(696usize) as *mut FCCLKSELX2) }
    }
    #[doc = "0x2b8 - Flexcomm Interface 2 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub fn fcclksel2(&self) -> &FCCLKSEL2 {
        unsafe { &*(((self as *const Self) as *const u8).add(696usize) as *const FCCLKSEL2) }
    }
    #[doc = "0x2b8 - Flexcomm Interface 2 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub fn fcclksel2_mut(&self) -> &mut FCCLKSEL2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(696usize) as *mut FCCLKSEL2) }
    }
    #[doc = "0x2bc - Peripheral reset control register"]
    #[inline(always)]
    pub fn fcclkselx3(&self) -> &FCCLKSELX3 {
        unsafe { &*(((self as *const Self) as *const u8).add(700usize) as *const FCCLKSELX3) }
    }
    #[doc = "0x2bc - Peripheral reset control register"]
    #[inline(always)]
    pub fn fcclkselx3_mut(&self) -> &mut FCCLKSELX3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(700usize) as *mut FCCLKSELX3) }
    }
    #[doc = "0x2bc - Flexcomm Interface 3 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub fn fcclksel3(&self) -> &FCCLKSEL3 {
        unsafe { &*(((self as *const Self) as *const u8).add(700usize) as *const FCCLKSEL3) }
    }
    #[doc = "0x2bc - Flexcomm Interface 3 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub fn fcclksel3_mut(&self) -> &mut FCCLKSEL3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(700usize) as *mut FCCLKSEL3) }
    }
    #[doc = "0x2c0 - Peripheral reset control register"]
    #[inline(always)]
    pub fn fcclkselx4(&self) -> &FCCLKSELX4 {
        unsafe { &*(((self as *const Self) as *const u8).add(704usize) as *const FCCLKSELX4) }
    }
    #[doc = "0x2c0 - Peripheral reset control register"]
    #[inline(always)]
    pub fn fcclkselx4_mut(&self) -> &mut FCCLKSELX4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(704usize) as *mut FCCLKSELX4) }
    }
    #[doc = "0x2c0 - Flexcomm Interface 4 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub fn fcclksel4(&self) -> &FCCLKSEL4 {
        unsafe { &*(((self as *const Self) as *const u8).add(704usize) as *const FCCLKSEL4) }
    }
    #[doc = "0x2c0 - Flexcomm Interface 4 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub fn fcclksel4_mut(&self) -> &mut FCCLKSEL4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(704usize) as *mut FCCLKSEL4) }
    }
    #[doc = "0x2c4 - Peripheral reset control register"]
    #[inline(always)]
    pub fn fcclkselx5(&self) -> &FCCLKSELX5 {
        unsafe { &*(((self as *const Self) as *const u8).add(708usize) as *const FCCLKSELX5) }
    }
    #[doc = "0x2c4 - Peripheral reset control register"]
    #[inline(always)]
    pub fn fcclkselx5_mut(&self) -> &mut FCCLKSELX5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(708usize) as *mut FCCLKSELX5) }
    }
    #[doc = "0x2c4 - Flexcomm Interface 5 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub fn fcclksel5(&self) -> &FCCLKSEL5 {
        unsafe { &*(((self as *const Self) as *const u8).add(708usize) as *const FCCLKSEL5) }
    }
    #[doc = "0x2c4 - Flexcomm Interface 5 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub fn fcclksel5_mut(&self) -> &mut FCCLKSEL5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(708usize) as *mut FCCLKSEL5) }
    }
    #[doc = "0x2c8 - Peripheral reset control register"]
    #[inline(always)]
    pub fn fcclkselx6(&self) -> &FCCLKSELX6 {
        unsafe { &*(((self as *const Self) as *const u8).add(712usize) as *const FCCLKSELX6) }
    }
    #[doc = "0x2c8 - Peripheral reset control register"]
    #[inline(always)]
    pub fn fcclkselx6_mut(&self) -> &mut FCCLKSELX6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(712usize) as *mut FCCLKSELX6) }
    }
    #[doc = "0x2c8 - Flexcomm Interface 6 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub fn fcclksel6(&self) -> &FCCLKSEL6 {
        unsafe { &*(((self as *const Self) as *const u8).add(712usize) as *const FCCLKSEL6) }
    }
    #[doc = "0x2c8 - Flexcomm Interface 6 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub fn fcclksel6_mut(&self) -> &mut FCCLKSEL6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(712usize) as *mut FCCLKSEL6) }
    }
    #[doc = "0x2cc - Peripheral reset control register"]
    #[inline(always)]
    pub fn fcclkselx7(&self) -> &FCCLKSELX7 {
        unsafe { &*(((self as *const Self) as *const u8).add(716usize) as *const FCCLKSELX7) }
    }
    #[doc = "0x2cc - Peripheral reset control register"]
    #[inline(always)]
    pub fn fcclkselx7_mut(&self) -> &mut FCCLKSELX7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(716usize) as *mut FCCLKSELX7) }
    }
    #[doc = "0x2cc - Flexcomm Interface 7 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub fn fcclksel7(&self) -> &FCCLKSEL7 {
        unsafe { &*(((self as *const Self) as *const u8).add(716usize) as *const FCCLKSEL7) }
    }
    #[doc = "0x2cc - Flexcomm Interface 7 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub fn fcclksel7_mut(&self) -> &mut FCCLKSEL7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(716usize) as *mut FCCLKSEL7) }
    }
    #[doc = "0x320 - Peripheral reset control register"]
    #[inline(always)]
    pub fn flexfrgxctrl0(&self) -> &FLEXFRGXCTRL0 {
        unsafe { &*(((self as *const Self) as *const u8).add(800usize) as *const FLEXFRGXCTRL0) }
    }
    #[doc = "0x320 - Peripheral reset control register"]
    #[inline(always)]
    pub fn flexfrgxctrl0_mut(&self) -> &mut FLEXFRGXCTRL0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(800usize) as *mut FLEXFRGXCTRL0) }
    }
    #[doc = "0x320 - Fractional rate divider for flexcomm 0"]
    #[inline(always)]
    pub fn flexfrg0ctrl(&self) -> &FLEXFRG0CTRL {
        unsafe { &*(((self as *const Self) as *const u8).add(800usize) as *const FLEXFRG0CTRL) }
    }
    #[doc = "0x320 - Fractional rate divider for flexcomm 0"]
    #[inline(always)]
    pub fn flexfrg0ctrl_mut(&self) -> &mut FLEXFRG0CTRL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(800usize) as *mut FLEXFRG0CTRL) }
    }
    #[doc = "0x324 - Peripheral reset control register"]
    #[inline(always)]
    pub fn flexfrgxctrl1(&self) -> &FLEXFRGXCTRL1 {
        unsafe { &*(((self as *const Self) as *const u8).add(804usize) as *const FLEXFRGXCTRL1) }
    }
    #[doc = "0x324 - Peripheral reset control register"]
    #[inline(always)]
    pub fn flexfrgxctrl1_mut(&self) -> &mut FLEXFRGXCTRL1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(804usize) as *mut FLEXFRGXCTRL1) }
    }
    #[doc = "0x324 - Fractional rate divider for flexcomm 1"]
    #[inline(always)]
    pub fn flexfrg1ctrl(&self) -> &FLEXFRG1CTRL {
        unsafe { &*(((self as *const Self) as *const u8).add(804usize) as *const FLEXFRG1CTRL) }
    }
    #[doc = "0x324 - Fractional rate divider for flexcomm 1"]
    #[inline(always)]
    pub fn flexfrg1ctrl_mut(&self) -> &mut FLEXFRG1CTRL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(804usize) as *mut FLEXFRG1CTRL) }
    }
    #[doc = "0x328 - Peripheral reset control register"]
    #[inline(always)]
    pub fn flexfrgxctrl2(&self) -> &FLEXFRGXCTRL2 {
        unsafe { &*(((self as *const Self) as *const u8).add(808usize) as *const FLEXFRGXCTRL2) }
    }
    #[doc = "0x328 - Peripheral reset control register"]
    #[inline(always)]
    pub fn flexfrgxctrl2_mut(&self) -> &mut FLEXFRGXCTRL2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(808usize) as *mut FLEXFRGXCTRL2) }
    }
    #[doc = "0x328 - Fractional rate divider for flexcomm 2"]
    #[inline(always)]
    pub fn flexfrg2ctrl(&self) -> &FLEXFRG2CTRL {
        unsafe { &*(((self as *const Self) as *const u8).add(808usize) as *const FLEXFRG2CTRL) }
    }
    #[doc = "0x328 - Fractional rate divider for flexcomm 2"]
    #[inline(always)]
    pub fn flexfrg2ctrl_mut(&self) -> &mut FLEXFRG2CTRL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(808usize) as *mut FLEXFRG2CTRL) }
    }
    #[doc = "0x32c - Peripheral reset control register"]
    #[inline(always)]
    pub fn flexfrgxctrl3(&self) -> &FLEXFRGXCTRL3 {
        unsafe { &*(((self as *const Self) as *const u8).add(812usize) as *const FLEXFRGXCTRL3) }
    }
    #[doc = "0x32c - Peripheral reset control register"]
    #[inline(always)]
    pub fn flexfrgxctrl3_mut(&self) -> &mut FLEXFRGXCTRL3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(812usize) as *mut FLEXFRGXCTRL3) }
    }
    #[doc = "0x32c - Fractional rate divider for flexcomm 3"]
    #[inline(always)]
    pub fn flexfrg3ctrl(&self) -> &FLEXFRG3CTRL {
        unsafe { &*(((self as *const Self) as *const u8).add(812usize) as *const FLEXFRG3CTRL) }
    }
    #[doc = "0x32c - Fractional rate divider for flexcomm 3"]
    #[inline(always)]
    pub fn flexfrg3ctrl_mut(&self) -> &mut FLEXFRG3CTRL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(812usize) as *mut FLEXFRG3CTRL) }
    }
    #[doc = "0x330 - Peripheral reset control register"]
    #[inline(always)]
    pub fn flexfrgxctrl4(&self) -> &FLEXFRGXCTRL4 {
        unsafe { &*(((self as *const Self) as *const u8).add(816usize) as *const FLEXFRGXCTRL4) }
    }
    #[doc = "0x330 - Peripheral reset control register"]
    #[inline(always)]
    pub fn flexfrgxctrl4_mut(&self) -> &mut FLEXFRGXCTRL4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(816usize) as *mut FLEXFRGXCTRL4) }
    }
    #[doc = "0x330 - Fractional rate divider for flexcomm 4"]
    #[inline(always)]
    pub fn flexfrg4ctrl(&self) -> &FLEXFRG4CTRL {
        unsafe { &*(((self as *const Self) as *const u8).add(816usize) as *const FLEXFRG4CTRL) }
    }
    #[doc = "0x330 - Fractional rate divider for flexcomm 4"]
    #[inline(always)]
    pub fn flexfrg4ctrl_mut(&self) -> &mut FLEXFRG4CTRL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(816usize) as *mut FLEXFRG4CTRL) }
    }
    #[doc = "0x334 - Peripheral reset control register"]
    #[inline(always)]
    pub fn flexfrgxctrl5(&self) -> &FLEXFRGXCTRL5 {
        unsafe { &*(((self as *const Self) as *const u8).add(820usize) as *const FLEXFRGXCTRL5) }
    }
    #[doc = "0x334 - Peripheral reset control register"]
    #[inline(always)]
    pub fn flexfrgxctrl5_mut(&self) -> &mut FLEXFRGXCTRL5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(820usize) as *mut FLEXFRGXCTRL5) }
    }
    #[doc = "0x334 - Fractional rate divider for flexcomm 5"]
    #[inline(always)]
    pub fn flexfrg5ctrl(&self) -> &FLEXFRG5CTRL {
        unsafe { &*(((self as *const Self) as *const u8).add(820usize) as *const FLEXFRG5CTRL) }
    }
    #[doc = "0x334 - Fractional rate divider for flexcomm 5"]
    #[inline(always)]
    pub fn flexfrg5ctrl_mut(&self) -> &mut FLEXFRG5CTRL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(820usize) as *mut FLEXFRG5CTRL) }
    }
    #[doc = "0x338 - Peripheral reset control register"]
    #[inline(always)]
    pub fn flexfrgxctrl6(&self) -> &FLEXFRGXCTRL6 {
        unsafe { &*(((self as *const Self) as *const u8).add(824usize) as *const FLEXFRGXCTRL6) }
    }
    #[doc = "0x338 - Peripheral reset control register"]
    #[inline(always)]
    pub fn flexfrgxctrl6_mut(&self) -> &mut FLEXFRGXCTRL6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(824usize) as *mut FLEXFRGXCTRL6) }
    }
    #[doc = "0x338 - Fractional rate divider for flexcomm 6"]
    #[inline(always)]
    pub fn flexfrg6ctrl(&self) -> &FLEXFRG6CTRL {
        unsafe { &*(((self as *const Self) as *const u8).add(824usize) as *const FLEXFRG6CTRL) }
    }
    #[doc = "0x338 - Fractional rate divider for flexcomm 6"]
    #[inline(always)]
    pub fn flexfrg6ctrl_mut(&self) -> &mut FLEXFRG6CTRL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(824usize) as *mut FLEXFRG6CTRL) }
    }
    #[doc = "0x33c - Peripheral reset control register"]
    #[inline(always)]
    pub fn flexfrgxctrl7(&self) -> &FLEXFRGXCTRL7 {
        unsafe { &*(((self as *const Self) as *const u8).add(828usize) as *const FLEXFRGXCTRL7) }
    }
    #[doc = "0x33c - Peripheral reset control register"]
    #[inline(always)]
    pub fn flexfrgxctrl7_mut(&self) -> &mut FLEXFRGXCTRL7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(828usize) as *mut FLEXFRGXCTRL7) }
    }
    #[doc = "0x33c - Fractional rate divider for flexcomm 7"]
    #[inline(always)]
    pub fn flexfrg7ctrl(&self) -> &FLEXFRG7CTRL {
        unsafe { &*(((self as *const Self) as *const u8).add(828usize) as *const FLEXFRG7CTRL) }
    }
    #[doc = "0x33c - Fractional rate divider for flexcomm 7"]
    #[inline(always)]
    pub fn flexfrg7ctrl_mut(&self) -> &mut FLEXFRG7CTRL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(828usize) as *mut FLEXFRG7CTRL) }
    }
}
#[doc = "Memory Remap control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [memoryremap](memoryremap) module"]
pub type MEMORYREMAP = crate::Reg<u32, _MEMORYREMAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEMORYREMAP;
#[doc = "`read()` method returns [memoryremap::R](memoryremap::R) reader structure"]
impl crate::Readable for MEMORYREMAP {}
#[doc = "`write(|w| ..)` method takes [memoryremap::W](memoryremap::W) writer structure"]
impl crate::Writable for MEMORYREMAP {}
#[doc = "Memory Remap control register"]
pub mod memoryremap;
#[doc = "AHB Matrix priority control register Priority values are 3 = highest, 0 = lowest\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahbmatprio](ahbmatprio) module"]
pub type AHBMATPRIO = crate::Reg<u32, _AHBMATPRIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBMATPRIO;
#[doc = "`read()` method returns [ahbmatprio::R](ahbmatprio::R) reader structure"]
impl crate::Readable for AHBMATPRIO {}
#[doc = "`write(|w| ..)` method takes [ahbmatprio::W](ahbmatprio::W) writer structure"]
impl crate::Writable for AHBMATPRIO {}
#[doc = "AHB Matrix priority control register Priority values are 3 = highest, 0 = lowest"]
pub mod ahbmatprio;
#[doc = "System tick calibration for secure part of CPU0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpu0stckcal](cpu0stckcal) module"]
pub type CPU0STCKCAL = crate::Reg<u32, _CPU0STCKCAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPU0STCKCAL;
#[doc = "`read()` method returns [cpu0stckcal::R](cpu0stckcal::R) reader structure"]
impl crate::Readable for CPU0STCKCAL {}
#[doc = "`write(|w| ..)` method takes [cpu0stckcal::W](cpu0stckcal::W) writer structure"]
impl crate::Writable for CPU0STCKCAL {}
#[doc = "System tick calibration for secure part of CPU0"]
pub mod cpu0stckcal;
#[doc = "System tick calibration for non-secure part of CPU0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpu0nstckcal](cpu0nstckcal) module"]
pub type CPU0NSTCKCAL = crate::Reg<u32, _CPU0NSTCKCAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPU0NSTCKCAL;
#[doc = "`read()` method returns [cpu0nstckcal::R](cpu0nstckcal::R) reader structure"]
impl crate::Readable for CPU0NSTCKCAL {}
#[doc = "`write(|w| ..)` method takes [cpu0nstckcal::W](cpu0nstckcal::W) writer structure"]
impl crate::Writable for CPU0NSTCKCAL {}
#[doc = "System tick calibration for non-secure part of CPU0"]
pub mod cpu0nstckcal;
#[doc = "System tick calibration for CPU1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpu1tckcal](cpu1tckcal) module"]
pub type CPU1TCKCAL = crate::Reg<u32, _CPU1TCKCAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPU1TCKCAL;
#[doc = "`read()` method returns [cpu1tckcal::R](cpu1tckcal::R) reader structure"]
impl crate::Readable for CPU1TCKCAL {}
#[doc = "`write(|w| ..)` method takes [cpu1tckcal::W](cpu1tckcal::W) writer structure"]
impl crate::Writable for CPU1TCKCAL {}
#[doc = "System tick calibration for CPU1"]
pub mod cpu1tckcal;
#[doc = "NMI Source Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nmisrc](nmisrc) module"]
pub type NMISRC = crate::Reg<u32, _NMISRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NMISRC;
#[doc = "`read()` method returns [nmisrc::R](nmisrc::R) reader structure"]
impl crate::Readable for NMISRC {}
#[doc = "`write(|w| ..)` method takes [nmisrc::W](nmisrc::W) writer structure"]
impl crate::Writable for NMISRC {}
#[doc = "NMI Source Select"]
pub mod nmisrc;
#[doc = "Peripheral reset control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [presetctrl0](presetctrl0) module"]
pub type PRESETCTRL0 = crate::Reg<u32, _PRESETCTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRESETCTRL0;
#[doc = "`read()` method returns [presetctrl0::R](presetctrl0::R) reader structure"]
impl crate::Readable for PRESETCTRL0 {}
#[doc = "`write(|w| ..)` method takes [presetctrl0::W](presetctrl0::W) writer structure"]
impl crate::Writable for PRESETCTRL0 {}
#[doc = "Peripheral reset control 0"]
pub mod presetctrl0;
#[doc = "Peripheral reset control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [presetctrl1](presetctrl1) module"]
pub type PRESETCTRL1 = crate::Reg<u32, _PRESETCTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRESETCTRL1;
#[doc = "`read()` method returns [presetctrl1::R](presetctrl1::R) reader structure"]
impl crate::Readable for PRESETCTRL1 {}
#[doc = "`write(|w| ..)` method takes [presetctrl1::W](presetctrl1::W) writer structure"]
impl crate::Writable for PRESETCTRL1 {}
#[doc = "Peripheral reset control 1"]
pub mod presetctrl1;
#[doc = "Peripheral reset control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [presetctrl2](presetctrl2) module"]
pub type PRESETCTRL2 = crate::Reg<u32, _PRESETCTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRESETCTRL2;
#[doc = "`read()` method returns [presetctrl2::R](presetctrl2::R) reader structure"]
impl crate::Readable for PRESETCTRL2 {}
#[doc = "`write(|w| ..)` method takes [presetctrl2::W](presetctrl2::W) writer structure"]
impl crate::Writable for PRESETCTRL2 {}
#[doc = "Peripheral reset control 2"]
pub mod presetctrl2;
#[doc = "Peripheral reset control set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [presetctrlset](presetctrlset) module"]
pub type PRESETCTRLSET = crate::Reg<u32, _PRESETCTRLSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRESETCTRLSET;
#[doc = "`read()` method returns [presetctrlset::R](presetctrlset::R) reader structure"]
impl crate::Readable for PRESETCTRLSET {}
#[doc = "`write(|w| ..)` method takes [presetctrlset::W](presetctrlset::W) writer structure"]
impl crate::Writable for PRESETCTRLSET {}
#[doc = "Peripheral reset control set register"]
pub mod presetctrlset;
#[doc = "Peripheral reset contro clearl register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [presetctrlclr](presetctrlclr) module"]
pub type PRESETCTRLCLR = crate::Reg<u32, _PRESETCTRLCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRESETCTRLCLR;
#[doc = "`read()` method returns [presetctrlclr::R](presetctrlclr::R) reader structure"]
impl crate::Readable for PRESETCTRLCLR {}
#[doc = "`write(|w| ..)` method takes [presetctrlclr::W](presetctrlclr::W) writer structure"]
impl crate::Writable for PRESETCTRLCLR {}
#[doc = "Peripheral reset contro clearl register"]
pub mod presetctrlclr;
#[doc = "generate a software_reset\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [swr_reset](swr_reset) module"]
pub type SWR_RESET = crate::Reg<u32, _SWR_RESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWR_RESET;
#[doc = "`write(|w| ..)` method takes [swr_reset::W](swr_reset::W) writer structure"]
impl crate::Writable for SWR_RESET {}
#[doc = "generate a software_reset"]
pub mod swr_reset;
#[doc = "AHB Clock control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahbclkctrl0](ahbclkctrl0) module"]
pub type AHBCLKCTRL0 = crate::Reg<u32, _AHBCLKCTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBCLKCTRL0;
#[doc = "`read()` method returns [ahbclkctrl0::R](ahbclkctrl0::R) reader structure"]
impl crate::Readable for AHBCLKCTRL0 {}
#[doc = "`write(|w| ..)` method takes [ahbclkctrl0::W](ahbclkctrl0::W) writer structure"]
impl crate::Writable for AHBCLKCTRL0 {}
#[doc = "AHB Clock control 0"]
pub mod ahbclkctrl0;
#[doc = "AHB Clock control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahbclkctrl1](ahbclkctrl1) module"]
pub type AHBCLKCTRL1 = crate::Reg<u32, _AHBCLKCTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBCLKCTRL1;
#[doc = "`read()` method returns [ahbclkctrl1::R](ahbclkctrl1::R) reader structure"]
impl crate::Readable for AHBCLKCTRL1 {}
#[doc = "`write(|w| ..)` method takes [ahbclkctrl1::W](ahbclkctrl1::W) writer structure"]
impl crate::Writable for AHBCLKCTRL1 {}
#[doc = "AHB Clock control 1"]
pub mod ahbclkctrl1;
#[doc = "AHB Clock control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahbclkctrl2](ahbclkctrl2) module"]
pub type AHBCLKCTRL2 = crate::Reg<u32, _AHBCLKCTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBCLKCTRL2;
#[doc = "`read()` method returns [ahbclkctrl2::R](ahbclkctrl2::R) reader structure"]
impl crate::Readable for AHBCLKCTRL2 {}
#[doc = "`write(|w| ..)` method takes [ahbclkctrl2::W](ahbclkctrl2::W) writer structure"]
impl crate::Writable for AHBCLKCTRL2 {}
#[doc = "AHB Clock control 2"]
pub mod ahbclkctrl2;
#[doc = "Peripheral reset control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahbclkctrlset](ahbclkctrlset) module"]
pub type AHBCLKCTRLSET = crate::Reg<u32, _AHBCLKCTRLSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBCLKCTRLSET;
#[doc = "`read()` method returns [ahbclkctrlset::R](ahbclkctrlset::R) reader structure"]
impl crate::Readable for AHBCLKCTRLSET {}
#[doc = "`write(|w| ..)` method takes [ahbclkctrlset::W](ahbclkctrlset::W) writer structure"]
impl crate::Writable for AHBCLKCTRLSET {}
#[doc = "Peripheral reset control register"]
pub mod ahbclkctrlset;
#[doc = "Peripheral reset control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahbclkctrlclr](ahbclkctrlclr) module"]
pub type AHBCLKCTRLCLR = crate::Reg<u32, _AHBCLKCTRLCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBCLKCTRLCLR;
#[doc = "`read()` method returns [ahbclkctrlclr::R](ahbclkctrlclr::R) reader structure"]
impl crate::Readable for AHBCLKCTRLCLR {}
#[doc = "`write(|w| ..)` method takes [ahbclkctrlclr::W](ahbclkctrlclr::W) writer structure"]
impl crate::Writable for AHBCLKCTRLCLR {}
#[doc = "Peripheral reset control register"]
pub mod ahbclkctrlclr;
#[doc = "System Tick Timer for CPU0 source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [systickclksel0](systickclksel0) module"]
pub type SYSTICKCLKSEL0 = crate::Reg<u32, _SYSTICKCLKSEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTICKCLKSEL0;
#[doc = "`read()` method returns [systickclksel0::R](systickclksel0::R) reader structure"]
impl crate::Readable for SYSTICKCLKSEL0 {}
#[doc = "`write(|w| ..)` method takes [systickclksel0::W](systickclksel0::W) writer structure"]
impl crate::Writable for SYSTICKCLKSEL0 {}
#[doc = "System Tick Timer for CPU0 source select"]
pub mod systickclksel0;
#[doc = "Peripheral reset control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [systickclkselx0](systickclkselx0) module"]
pub type SYSTICKCLKSELX0 = crate::Reg<u32, _SYSTICKCLKSELX0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTICKCLKSELX0;
#[doc = "`read()` method returns [systickclkselx0::R](systickclkselx0::R) reader structure"]
impl crate::Readable for SYSTICKCLKSELX0 {}
#[doc = "`write(|w| ..)` method takes [systickclkselx0::W](systickclkselx0::W) writer structure"]
impl crate::Writable for SYSTICKCLKSELX0 {}
#[doc = "Peripheral reset control register"]
pub mod systickclkselx0;
#[doc = "System Tick Timer for CPU1 source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [systickclksel1](systickclksel1) module"]
pub type SYSTICKCLKSEL1 = crate::Reg<u32, _SYSTICKCLKSEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTICKCLKSEL1;
#[doc = "`read()` method returns [systickclksel1::R](systickclksel1::R) reader structure"]
impl crate::Readable for SYSTICKCLKSEL1 {}
#[doc = "`write(|w| ..)` method takes [systickclksel1::W](systickclksel1::W) writer structure"]
impl crate::Writable for SYSTICKCLKSEL1 {}
#[doc = "System Tick Timer for CPU1 source select"]
pub mod systickclksel1;
#[doc = "Peripheral reset control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [systickclkselx1](systickclkselx1) module"]
pub type SYSTICKCLKSELX1 = crate::Reg<u32, _SYSTICKCLKSELX1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTICKCLKSELX1;
#[doc = "`read()` method returns [systickclkselx1::R](systickclkselx1::R) reader structure"]
impl crate::Readable for SYSTICKCLKSELX1 {}
#[doc = "`write(|w| ..)` method takes [systickclkselx1::W](systickclkselx1::W) writer structure"]
impl crate::Writable for SYSTICKCLKSELX1 {}
#[doc = "Peripheral reset control register"]
pub mod systickclkselx1;
#[doc = "Trace clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [traceclksel](traceclksel) module"]
pub type TRACECLKSEL = crate::Reg<u32, _TRACECLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRACECLKSEL;
#[doc = "`read()` method returns [traceclksel::R](traceclksel::R) reader structure"]
impl crate::Readable for TRACECLKSEL {}
#[doc = "`write(|w| ..)` method takes [traceclksel::W](traceclksel::W) writer structure"]
impl crate::Writable for TRACECLKSEL {}
#[doc = "Trace clock source select"]
pub mod traceclksel;
#[doc = "CTimer 0 clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctimerclksel0](ctimerclksel0) module"]
pub type CTIMERCLKSEL0 = crate::Reg<u32, _CTIMERCLKSEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTIMERCLKSEL0;
#[doc = "`read()` method returns [ctimerclksel0::R](ctimerclksel0::R) reader structure"]
impl crate::Readable for CTIMERCLKSEL0 {}
#[doc = "`write(|w| ..)` method takes [ctimerclksel0::W](ctimerclksel0::W) writer structure"]
impl crate::Writable for CTIMERCLKSEL0 {}
#[doc = "CTimer 0 clock source select"]
pub mod ctimerclksel0;
#[doc = "Peripheral reset control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctimerclkselx0](ctimerclkselx0) module"]
pub type CTIMERCLKSELX0 = crate::Reg<u32, _CTIMERCLKSELX0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTIMERCLKSELX0;
#[doc = "`read()` method returns [ctimerclkselx0::R](ctimerclkselx0::R) reader structure"]
impl crate::Readable for CTIMERCLKSELX0 {}
#[doc = "`write(|w| ..)` method takes [ctimerclkselx0::W](ctimerclkselx0::W) writer structure"]
impl crate::Writable for CTIMERCLKSELX0 {}
#[doc = "Peripheral reset control register"]
pub mod ctimerclkselx0;
#[doc = "CTimer 1 clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctimerclksel1](ctimerclksel1) module"]
pub type CTIMERCLKSEL1 = crate::Reg<u32, _CTIMERCLKSEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTIMERCLKSEL1;
#[doc = "`read()` method returns [ctimerclksel1::R](ctimerclksel1::R) reader structure"]
impl crate::Readable for CTIMERCLKSEL1 {}
#[doc = "`write(|w| ..)` method takes [ctimerclksel1::W](ctimerclksel1::W) writer structure"]
impl crate::Writable for CTIMERCLKSEL1 {}
#[doc = "CTimer 1 clock source select"]
pub mod ctimerclksel1;
#[doc = "Peripheral reset control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctimerclkselx1](ctimerclkselx1) module"]
pub type CTIMERCLKSELX1 = crate::Reg<u32, _CTIMERCLKSELX1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTIMERCLKSELX1;
#[doc = "`read()` method returns [ctimerclkselx1::R](ctimerclkselx1::R) reader structure"]
impl crate::Readable for CTIMERCLKSELX1 {}
#[doc = "`write(|w| ..)` method takes [ctimerclkselx1::W](ctimerclkselx1::W) writer structure"]
impl crate::Writable for CTIMERCLKSELX1 {}
#[doc = "Peripheral reset control register"]
pub mod ctimerclkselx1;
#[doc = "CTimer 2 clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctimerclksel2](ctimerclksel2) module"]
pub type CTIMERCLKSEL2 = crate::Reg<u32, _CTIMERCLKSEL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTIMERCLKSEL2;
#[doc = "`read()` method returns [ctimerclksel2::R](ctimerclksel2::R) reader structure"]
impl crate::Readable for CTIMERCLKSEL2 {}
#[doc = "`write(|w| ..)` method takes [ctimerclksel2::W](ctimerclksel2::W) writer structure"]
impl crate::Writable for CTIMERCLKSEL2 {}
#[doc = "CTimer 2 clock source select"]
pub mod ctimerclksel2;
#[doc = "Peripheral reset control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctimerclkselx2](ctimerclkselx2) module"]
pub type CTIMERCLKSELX2 = crate::Reg<u32, _CTIMERCLKSELX2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTIMERCLKSELX2;
#[doc = "`read()` method returns [ctimerclkselx2::R](ctimerclkselx2::R) reader structure"]
impl crate::Readable for CTIMERCLKSELX2 {}
#[doc = "`write(|w| ..)` method takes [ctimerclkselx2::W](ctimerclkselx2::W) writer structure"]
impl crate::Writable for CTIMERCLKSELX2 {}
#[doc = "Peripheral reset control register"]
pub mod ctimerclkselx2;
#[doc = "CTimer 3 clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctimerclksel3](ctimerclksel3) module"]
pub type CTIMERCLKSEL3 = crate::Reg<u32, _CTIMERCLKSEL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTIMERCLKSEL3;
#[doc = "`read()` method returns [ctimerclksel3::R](ctimerclksel3::R) reader structure"]
impl crate::Readable for CTIMERCLKSEL3 {}
#[doc = "`write(|w| ..)` method takes [ctimerclksel3::W](ctimerclksel3::W) writer structure"]
impl crate::Writable for CTIMERCLKSEL3 {}
#[doc = "CTimer 3 clock source select"]
pub mod ctimerclksel3;
#[doc = "Peripheral reset control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctimerclkselx3](ctimerclkselx3) module"]
pub type CTIMERCLKSELX3 = crate::Reg<u32, _CTIMERCLKSELX3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTIMERCLKSELX3;
#[doc = "`read()` method returns [ctimerclkselx3::R](ctimerclkselx3::R) reader structure"]
impl crate::Readable for CTIMERCLKSELX3 {}
#[doc = "`write(|w| ..)` method takes [ctimerclkselx3::W](ctimerclkselx3::W) writer structure"]
impl crate::Writable for CTIMERCLKSELX3 {}
#[doc = "Peripheral reset control register"]
pub mod ctimerclkselx3;
#[doc = "CTimer 4 clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctimerclksel4](ctimerclksel4) module"]
pub type CTIMERCLKSEL4 = crate::Reg<u32, _CTIMERCLKSEL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTIMERCLKSEL4;
#[doc = "`read()` method returns [ctimerclksel4::R](ctimerclksel4::R) reader structure"]
impl crate::Readable for CTIMERCLKSEL4 {}
#[doc = "`write(|w| ..)` method takes [ctimerclksel4::W](ctimerclksel4::W) writer structure"]
impl crate::Writable for CTIMERCLKSEL4 {}
#[doc = "CTimer 4 clock source select"]
pub mod ctimerclksel4;
#[doc = "Peripheral reset control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctimerclkselx4](ctimerclkselx4) module"]
pub type CTIMERCLKSELX4 = crate::Reg<u32, _CTIMERCLKSELX4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTIMERCLKSELX4;
#[doc = "`read()` method returns [ctimerclkselx4::R](ctimerclkselx4::R) reader structure"]
impl crate::Readable for CTIMERCLKSELX4 {}
#[doc = "`write(|w| ..)` method takes [ctimerclkselx4::W](ctimerclkselx4::W) writer structure"]
impl crate::Writable for CTIMERCLKSELX4 {}
#[doc = "Peripheral reset control register"]
pub mod ctimerclkselx4;
#[doc = "Main clock A source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mainclksela](mainclksela) module"]
pub type MAINCLKSELA = crate::Reg<u32, _MAINCLKSELA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAINCLKSELA;
#[doc = "`read()` method returns [mainclksela::R](mainclksela::R) reader structure"]
impl crate::Readable for MAINCLKSELA {}
#[doc = "`write(|w| ..)` method takes [mainclksela::W](mainclksela::W) writer structure"]
impl crate::Writable for MAINCLKSELA {}
#[doc = "Main clock A source select"]
pub mod mainclksela;
#[doc = "Main clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mainclkselb](mainclkselb) module"]
pub type MAINCLKSELB = crate::Reg<u32, _MAINCLKSELB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAINCLKSELB;
#[doc = "`read()` method returns [mainclkselb::R](mainclkselb::R) reader structure"]
impl crate::Readable for MAINCLKSELB {}
#[doc = "`write(|w| ..)` method takes [mainclkselb::W](mainclkselb::W) writer structure"]
impl crate::Writable for MAINCLKSELB {}
#[doc = "Main clock source select"]
pub mod mainclkselb;
#[doc = "CLKOUT clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clkoutsel](clkoutsel) module"]
pub type CLKOUTSEL = crate::Reg<u32, _CLKOUTSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKOUTSEL;
#[doc = "`read()` method returns [clkoutsel::R](clkoutsel::R) reader structure"]
impl crate::Readable for CLKOUTSEL {}
#[doc = "`write(|w| ..)` method takes [clkoutsel::W](clkoutsel::W) writer structure"]
impl crate::Writable for CLKOUTSEL {}
#[doc = "CLKOUT clock source select"]
pub mod clkoutsel;
#[doc = "PLL0 clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll0clksel](pll0clksel) module"]
pub type PLL0CLKSEL = crate::Reg<u32, _PLL0CLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL0CLKSEL;
#[doc = "`read()` method returns [pll0clksel::R](pll0clksel::R) reader structure"]
impl crate::Readable for PLL0CLKSEL {}
#[doc = "`write(|w| ..)` method takes [pll0clksel::W](pll0clksel::W) writer structure"]
impl crate::Writable for PLL0CLKSEL {}
#[doc = "PLL0 clock source select"]
pub mod pll0clksel;
#[doc = "PLL1 clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll1clksel](pll1clksel) module"]
pub type PLL1CLKSEL = crate::Reg<u32, _PLL1CLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL1CLKSEL;
#[doc = "`read()` method returns [pll1clksel::R](pll1clksel::R) reader structure"]
impl crate::Readable for PLL1CLKSEL {}
#[doc = "`write(|w| ..)` method takes [pll1clksel::W](pll1clksel::W) writer structure"]
impl crate::Writable for PLL1CLKSEL {}
#[doc = "PLL1 clock source select"]
pub mod pll1clksel;
#[doc = "ADC clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [adcclksel](adcclksel) module"]
pub type ADCCLKSEL = crate::Reg<u32, _ADCCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCCLKSEL;
#[doc = "`read()` method returns [adcclksel::R](adcclksel::R) reader structure"]
impl crate::Readable for ADCCLKSEL {}
#[doc = "`write(|w| ..)` method takes [adcclksel::W](adcclksel::W) writer structure"]
impl crate::Writable for ADCCLKSEL {}
#[doc = "ADC clock source select"]
pub mod adcclksel;
#[doc = "FS USB clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usb0clksel](usb0clksel) module"]
pub type USB0CLKSEL = crate::Reg<u32, _USB0CLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB0CLKSEL;
#[doc = "`read()` method returns [usb0clksel::R](usb0clksel::R) reader structure"]
impl crate::Readable for USB0CLKSEL {}
#[doc = "`write(|w| ..)` method takes [usb0clksel::W](usb0clksel::W) writer structure"]
impl crate::Writable for USB0CLKSEL {}
#[doc = "FS USB clock source select"]
pub mod usb0clksel;
#[doc = "HS USB clock source select - NOT USED\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usb1clksel](usb1clksel) module"]
pub type USB1CLKSEL = crate::Reg<u32, _USB1CLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB1CLKSEL;
#[doc = "`read()` method returns [usb1clksel::R](usb1clksel::R) reader structure"]
impl crate::Readable for USB1CLKSEL {}
#[doc = "`write(|w| ..)` method takes [usb1clksel::W](usb1clksel::W) writer structure"]
impl crate::Writable for USB1CLKSEL {}
#[doc = "HS USB clock source select - NOT USED"]
pub mod usb1clksel;
#[doc = "Flexcomm Interface 0 clock source select for Fractional Rate Divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fcclksel0](fcclksel0) module"]
pub type FCCLKSEL0 = crate::Reg<u32, _FCCLKSEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCLKSEL0;
#[doc = "`read()` method returns [fcclksel0::R](fcclksel0::R) reader structure"]
impl crate::Readable for FCCLKSEL0 {}
#[doc = "`write(|w| ..)` method takes [fcclksel0::W](fcclksel0::W) writer structure"]
impl crate::Writable for FCCLKSEL0 {}
#[doc = "Flexcomm Interface 0 clock source select for Fractional Rate Divider"]
pub mod fcclksel0;
#[doc = "Peripheral reset control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fcclkselx0](fcclkselx0) module"]
pub type FCCLKSELX0 = crate::Reg<u32, _FCCLKSELX0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCLKSELX0;
#[doc = "`read()` method returns [fcclkselx0::R](fcclkselx0::R) reader structure"]
impl crate::Readable for FCCLKSELX0 {}
#[doc = "`write(|w| ..)` method takes [fcclkselx0::W](fcclkselx0::W) writer structure"]
impl crate::Writable for FCCLKSELX0 {}
#[doc = "Peripheral reset control register"]
pub mod fcclkselx0;
#[doc = "Flexcomm Interface 1 clock source select for Fractional Rate Divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fcclksel1](fcclksel1) module"]
pub type FCCLKSEL1 = crate::Reg<u32, _FCCLKSEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCLKSEL1;
#[doc = "`read()` method returns [fcclksel1::R](fcclksel1::R) reader structure"]
impl crate::Readable for FCCLKSEL1 {}
#[doc = "`write(|w| ..)` method takes [fcclksel1::W](fcclksel1::W) writer structure"]
impl crate::Writable for FCCLKSEL1 {}
#[doc = "Flexcomm Interface 1 clock source select for Fractional Rate Divider"]
pub mod fcclksel1;
#[doc = "Peripheral reset control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fcclkselx1](fcclkselx1) module"]
pub type FCCLKSELX1 = crate::Reg<u32, _FCCLKSELX1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCLKSELX1;
#[doc = "`read()` method returns [fcclkselx1::R](fcclkselx1::R) reader structure"]
impl crate::Readable for FCCLKSELX1 {}
#[doc = "`write(|w| ..)` method takes [fcclkselx1::W](fcclkselx1::W) writer structure"]
impl crate::Writable for FCCLKSELX1 {}
#[doc = "Peripheral reset control register"]
pub mod fcclkselx1;
#[doc = "Flexcomm Interface 2 clock source select for Fractional Rate Divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fcclksel2](fcclksel2) module"]
pub type FCCLKSEL2 = crate::Reg<u32, _FCCLKSEL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCLKSEL2;
#[doc = "`read()` method returns [fcclksel2::R](fcclksel2::R) reader structure"]
impl crate::Readable for FCCLKSEL2 {}
#[doc = "`write(|w| ..)` method takes [fcclksel2::W](fcclksel2::W) writer structure"]
impl crate::Writable for FCCLKSEL2 {}
#[doc = "Flexcomm Interface 2 clock source select for Fractional Rate Divider"]
pub mod fcclksel2;
#[doc = "Peripheral reset control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fcclkselx2](fcclkselx2) module"]
pub type FCCLKSELX2 = crate::Reg<u32, _FCCLKSELX2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCLKSELX2;
#[doc = "`read()` method returns [fcclkselx2::R](fcclkselx2::R) reader structure"]
impl crate::Readable for FCCLKSELX2 {}
#[doc = "`write(|w| ..)` method takes [fcclkselx2::W](fcclkselx2::W) writer structure"]
impl crate::Writable for FCCLKSELX2 {}
#[doc = "Peripheral reset control register"]
pub mod fcclkselx2;
#[doc = "Flexcomm Interface 3 clock source select for Fractional Rate Divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fcclksel3](fcclksel3) module"]
pub type FCCLKSEL3 = crate::Reg<u32, _FCCLKSEL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCLKSEL3;
#[doc = "`read()` method returns [fcclksel3::R](fcclksel3::R) reader structure"]
impl crate::Readable for FCCLKSEL3 {}
#[doc = "`write(|w| ..)` method takes [fcclksel3::W](fcclksel3::W) writer structure"]
impl crate::Writable for FCCLKSEL3 {}
#[doc = "Flexcomm Interface 3 clock source select for Fractional Rate Divider"]
pub mod fcclksel3;
#[doc = "Peripheral reset control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fcclkselx3](fcclkselx3) module"]
pub type FCCLKSELX3 = crate::Reg<u32, _FCCLKSELX3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCLKSELX3;
#[doc = "`read()` method returns [fcclkselx3::R](fcclkselx3::R) reader structure"]
impl crate::Readable for FCCLKSELX3 {}
#[doc = "`write(|w| ..)` method takes [fcclkselx3::W](fcclkselx3::W) writer structure"]
impl crate::Writable for FCCLKSELX3 {}
#[doc = "Peripheral reset control register"]
pub mod fcclkselx3;
#[doc = "Flexcomm Interface 4 clock source select for Fractional Rate Divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fcclksel4](fcclksel4) module"]
pub type FCCLKSEL4 = crate::Reg<u32, _FCCLKSEL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCLKSEL4;
#[doc = "`read()` method returns [fcclksel4::R](fcclksel4::R) reader structure"]
impl crate::Readable for FCCLKSEL4 {}
#[doc = "`write(|w| ..)` method takes [fcclksel4::W](fcclksel4::W) writer structure"]
impl crate::Writable for FCCLKSEL4 {}
#[doc = "Flexcomm Interface 4 clock source select for Fractional Rate Divider"]
pub mod fcclksel4;
#[doc = "Peripheral reset control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fcclkselx4](fcclkselx4) module"]
pub type FCCLKSELX4 = crate::Reg<u32, _FCCLKSELX4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCLKSELX4;
#[doc = "`read()` method returns [fcclkselx4::R](fcclkselx4::R) reader structure"]
impl crate::Readable for FCCLKSELX4 {}
#[doc = "`write(|w| ..)` method takes [fcclkselx4::W](fcclkselx4::W) writer structure"]
impl crate::Writable for FCCLKSELX4 {}
#[doc = "Peripheral reset control register"]
pub mod fcclkselx4;
#[doc = "Flexcomm Interface 5 clock source select for Fractional Rate Divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fcclksel5](fcclksel5) module"]
pub type FCCLKSEL5 = crate::Reg<u32, _FCCLKSEL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCLKSEL5;
#[doc = "`read()` method returns [fcclksel5::R](fcclksel5::R) reader structure"]
impl crate::Readable for FCCLKSEL5 {}
#[doc = "`write(|w| ..)` method takes [fcclksel5::W](fcclksel5::W) writer structure"]
impl crate::Writable for FCCLKSEL5 {}
#[doc = "Flexcomm Interface 5 clock source select for Fractional Rate Divider"]
pub mod fcclksel5;
#[doc = "Peripheral reset control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fcclkselx5](fcclkselx5) module"]
pub type FCCLKSELX5 = crate::Reg<u32, _FCCLKSELX5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCLKSELX5;
#[doc = "`read()` method returns [fcclkselx5::R](fcclkselx5::R) reader structure"]
impl crate::Readable for FCCLKSELX5 {}
#[doc = "`write(|w| ..)` method takes [fcclkselx5::W](fcclkselx5::W) writer structure"]
impl crate::Writable for FCCLKSELX5 {}
#[doc = "Peripheral reset control register"]
pub mod fcclkselx5;
#[doc = "Flexcomm Interface 6 clock source select for Fractional Rate Divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fcclksel6](fcclksel6) module"]
pub type FCCLKSEL6 = crate::Reg<u32, _FCCLKSEL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCLKSEL6;
#[doc = "`read()` method returns [fcclksel6::R](fcclksel6::R) reader structure"]
impl crate::Readable for FCCLKSEL6 {}
#[doc = "`write(|w| ..)` method takes [fcclksel6::W](fcclksel6::W) writer structure"]
impl crate::Writable for FCCLKSEL6 {}
#[doc = "Flexcomm Interface 6 clock source select for Fractional Rate Divider"]
pub mod fcclksel6;
#[doc = "Peripheral reset control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fcclkselx6](fcclkselx6) module"]
pub type FCCLKSELX6 = crate::Reg<u32, _FCCLKSELX6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCLKSELX6;
#[doc = "`read()` method returns [fcclkselx6::R](fcclkselx6::R) reader structure"]
impl crate::Readable for FCCLKSELX6 {}
#[doc = "`write(|w| ..)` method takes [fcclkselx6::W](fcclkselx6::W) writer structure"]
impl crate::Writable for FCCLKSELX6 {}
#[doc = "Peripheral reset control register"]
pub mod fcclkselx6;
#[doc = "Flexcomm Interface 7 clock source select for Fractional Rate Divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fcclksel7](fcclksel7) module"]
pub type FCCLKSEL7 = crate::Reg<u32, _FCCLKSEL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCLKSEL7;
#[doc = "`read()` method returns [fcclksel7::R](fcclksel7::R) reader structure"]
impl crate::Readable for FCCLKSEL7 {}
#[doc = "`write(|w| ..)` method takes [fcclksel7::W](fcclksel7::W) writer structure"]
impl crate::Writable for FCCLKSEL7 {}
#[doc = "Flexcomm Interface 7 clock source select for Fractional Rate Divider"]
pub mod fcclksel7;
#[doc = "Peripheral reset control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fcclkselx7](fcclkselx7) module"]
pub type FCCLKSELX7 = crate::Reg<u32, _FCCLKSELX7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCLKSELX7;
#[doc = "`read()` method returns [fcclkselx7::R](fcclkselx7::R) reader structure"]
impl crate::Readable for FCCLKSELX7 {}
#[doc = "`write(|w| ..)` method takes [fcclkselx7::W](fcclkselx7::W) writer structure"]
impl crate::Writable for FCCLKSELX7 {}
#[doc = "Peripheral reset control register"]
pub mod fcclkselx7;
#[doc = "HS LSPI clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hslspiclksel](hslspiclksel) module"]
pub type HSLSPICLKSEL = crate::Reg<u32, _HSLSPICLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSLSPICLKSEL;
#[doc = "`read()` method returns [hslspiclksel::R](hslspiclksel::R) reader structure"]
impl crate::Readable for HSLSPICLKSEL {}
#[doc = "`write(|w| ..)` method takes [hslspiclksel::W](hslspiclksel::W) writer structure"]
impl crate::Writable for HSLSPICLKSEL {}
#[doc = "HS LSPI clock source select"]
pub mod hslspiclksel;
#[doc = "MCLK clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mclkclksel](mclkclksel) module"]
pub type MCLKCLKSEL = crate::Reg<u32, _MCLKCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCLKCLKSEL;
#[doc = "`read()` method returns [mclkclksel::R](mclkclksel::R) reader structure"]
impl crate::Readable for MCLKCLKSEL {}
#[doc = "`write(|w| ..)` method takes [mclkclksel::W](mclkclksel::W) writer structure"]
impl crate::Writable for MCLKCLKSEL {}
#[doc = "MCLK clock source select"]
pub mod mclkclksel;
#[doc = "SCTimer/PWM clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sctclksel](sctclksel) module"]
pub type SCTCLKSEL = crate::Reg<u32, _SCTCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTCLKSEL;
#[doc = "`read()` method returns [sctclksel::R](sctclksel::R) reader structure"]
impl crate::Readable for SCTCLKSEL {}
#[doc = "`write(|w| ..)` method takes [sctclksel::W](sctclksel::W) writer structure"]
impl crate::Writable for SCTCLKSEL {}
#[doc = "SCTimer/PWM clock source select"]
pub mod sctclksel;
#[doc = "SDIO clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sdioclksel](sdioclksel) module"]
pub type SDIOCLKSEL = crate::Reg<u32, _SDIOCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDIOCLKSEL;
#[doc = "`read()` method returns [sdioclksel::R](sdioclksel::R) reader structure"]
impl crate::Readable for SDIOCLKSEL {}
#[doc = "`write(|w| ..)` method takes [sdioclksel::W](sdioclksel::W) writer structure"]
impl crate::Writable for SDIOCLKSEL {}
#[doc = "SDIO clock source select"]
pub mod sdioclksel;
#[doc = "System Tick Timer divider for CPU0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [systickclkdiv0](systickclkdiv0) module"]
pub type SYSTICKCLKDIV0 = crate::Reg<u32, _SYSTICKCLKDIV0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTICKCLKDIV0;
#[doc = "`read()` method returns [systickclkdiv0::R](systickclkdiv0::R) reader structure"]
impl crate::Readable for SYSTICKCLKDIV0 {}
#[doc = "`write(|w| ..)` method takes [systickclkdiv0::W](systickclkdiv0::W) writer structure"]
impl crate::Writable for SYSTICKCLKDIV0 {}
#[doc = "System Tick Timer divider for CPU0"]
pub mod systickclkdiv0;
#[doc = "System Tick Timer divider for CPU1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [systickclkdiv1](systickclkdiv1) module"]
pub type SYSTICKCLKDIV1 = crate::Reg<u32, _SYSTICKCLKDIV1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTICKCLKDIV1;
#[doc = "`read()` method returns [systickclkdiv1::R](systickclkdiv1::R) reader structure"]
impl crate::Readable for SYSTICKCLKDIV1 {}
#[doc = "`write(|w| ..)` method takes [systickclkdiv1::W](systickclkdiv1::W) writer structure"]
impl crate::Writable for SYSTICKCLKDIV1 {}
#[doc = "System Tick Timer divider for CPU1"]
pub mod systickclkdiv1;
#[doc = "TRACE clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [traceclkdiv](traceclkdiv) module"]
pub type TRACECLKDIV = crate::Reg<u32, _TRACECLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRACECLKDIV;
#[doc = "`read()` method returns [traceclkdiv::R](traceclkdiv::R) reader structure"]
impl crate::Readable for TRACECLKDIV {}
#[doc = "`write(|w| ..)` method takes [traceclkdiv::W](traceclkdiv::W) writer structure"]
impl crate::Writable for TRACECLKDIV {}
#[doc = "TRACE clock divider"]
pub mod traceclkdiv;
#[doc = "Fractional rate divider for flexcomm 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexfrg0ctrl](flexfrg0ctrl) module"]
pub type FLEXFRG0CTRL = crate::Reg<u32, _FLEXFRG0CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXFRG0CTRL;
#[doc = "`read()` method returns [flexfrg0ctrl::R](flexfrg0ctrl::R) reader structure"]
impl crate::Readable for FLEXFRG0CTRL {}
#[doc = "`write(|w| ..)` method takes [flexfrg0ctrl::W](flexfrg0ctrl::W) writer structure"]
impl crate::Writable for FLEXFRG0CTRL {}
#[doc = "Fractional rate divider for flexcomm 0"]
pub mod flexfrg0ctrl;
#[doc = "Peripheral reset control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexfrgxctrl0](flexfrgxctrl0) module"]
pub type FLEXFRGXCTRL0 = crate::Reg<u32, _FLEXFRGXCTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXFRGXCTRL0;
#[doc = "`read()` method returns [flexfrgxctrl0::R](flexfrgxctrl0::R) reader structure"]
impl crate::Readable for FLEXFRGXCTRL0 {}
#[doc = "`write(|w| ..)` method takes [flexfrgxctrl0::W](flexfrgxctrl0::W) writer structure"]
impl crate::Writable for FLEXFRGXCTRL0 {}
#[doc = "Peripheral reset control register"]
pub mod flexfrgxctrl0;
#[doc = "Fractional rate divider for flexcomm 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexfrg1ctrl](flexfrg1ctrl) module"]
pub type FLEXFRG1CTRL = crate::Reg<u32, _FLEXFRG1CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXFRG1CTRL;
#[doc = "`read()` method returns [flexfrg1ctrl::R](flexfrg1ctrl::R) reader structure"]
impl crate::Readable for FLEXFRG1CTRL {}
#[doc = "`write(|w| ..)` method takes [flexfrg1ctrl::W](flexfrg1ctrl::W) writer structure"]
impl crate::Writable for FLEXFRG1CTRL {}
#[doc = "Fractional rate divider for flexcomm 1"]
pub mod flexfrg1ctrl;
#[doc = "Peripheral reset control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexfrgxctrl1](flexfrgxctrl1) module"]
pub type FLEXFRGXCTRL1 = crate::Reg<u32, _FLEXFRGXCTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXFRGXCTRL1;
#[doc = "`read()` method returns [flexfrgxctrl1::R](flexfrgxctrl1::R) reader structure"]
impl crate::Readable for FLEXFRGXCTRL1 {}
#[doc = "`write(|w| ..)` method takes [flexfrgxctrl1::W](flexfrgxctrl1::W) writer structure"]
impl crate::Writable for FLEXFRGXCTRL1 {}
#[doc = "Peripheral reset control register"]
pub mod flexfrgxctrl1;
#[doc = "Fractional rate divider for flexcomm 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexfrg2ctrl](flexfrg2ctrl) module"]
pub type FLEXFRG2CTRL = crate::Reg<u32, _FLEXFRG2CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXFRG2CTRL;
#[doc = "`read()` method returns [flexfrg2ctrl::R](flexfrg2ctrl::R) reader structure"]
impl crate::Readable for FLEXFRG2CTRL {}
#[doc = "`write(|w| ..)` method takes [flexfrg2ctrl::W](flexfrg2ctrl::W) writer structure"]
impl crate::Writable for FLEXFRG2CTRL {}
#[doc = "Fractional rate divider for flexcomm 2"]
pub mod flexfrg2ctrl;
#[doc = "Peripheral reset control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexfrgxctrl2](flexfrgxctrl2) module"]
pub type FLEXFRGXCTRL2 = crate::Reg<u32, _FLEXFRGXCTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXFRGXCTRL2;
#[doc = "`read()` method returns [flexfrgxctrl2::R](flexfrgxctrl2::R) reader structure"]
impl crate::Readable for FLEXFRGXCTRL2 {}
#[doc = "`write(|w| ..)` method takes [flexfrgxctrl2::W](flexfrgxctrl2::W) writer structure"]
impl crate::Writable for FLEXFRGXCTRL2 {}
#[doc = "Peripheral reset control register"]
pub mod flexfrgxctrl2;
#[doc = "Fractional rate divider for flexcomm 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexfrg3ctrl](flexfrg3ctrl) module"]
pub type FLEXFRG3CTRL = crate::Reg<u32, _FLEXFRG3CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXFRG3CTRL;
#[doc = "`read()` method returns [flexfrg3ctrl::R](flexfrg3ctrl::R) reader structure"]
impl crate::Readable for FLEXFRG3CTRL {}
#[doc = "`write(|w| ..)` method takes [flexfrg3ctrl::W](flexfrg3ctrl::W) writer structure"]
impl crate::Writable for FLEXFRG3CTRL {}
#[doc = "Fractional rate divider for flexcomm 3"]
pub mod flexfrg3ctrl;
#[doc = "Peripheral reset control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexfrgxctrl3](flexfrgxctrl3) module"]
pub type FLEXFRGXCTRL3 = crate::Reg<u32, _FLEXFRGXCTRL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXFRGXCTRL3;
#[doc = "`read()` method returns [flexfrgxctrl3::R](flexfrgxctrl3::R) reader structure"]
impl crate::Readable for FLEXFRGXCTRL3 {}
#[doc = "`write(|w| ..)` method takes [flexfrgxctrl3::W](flexfrgxctrl3::W) writer structure"]
impl crate::Writable for FLEXFRGXCTRL3 {}
#[doc = "Peripheral reset control register"]
pub mod flexfrgxctrl3;
#[doc = "Fractional rate divider for flexcomm 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexfrg4ctrl](flexfrg4ctrl) module"]
pub type FLEXFRG4CTRL = crate::Reg<u32, _FLEXFRG4CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXFRG4CTRL;
#[doc = "`read()` method returns [flexfrg4ctrl::R](flexfrg4ctrl::R) reader structure"]
impl crate::Readable for FLEXFRG4CTRL {}
#[doc = "`write(|w| ..)` method takes [flexfrg4ctrl::W](flexfrg4ctrl::W) writer structure"]
impl crate::Writable for FLEXFRG4CTRL {}
#[doc = "Fractional rate divider for flexcomm 4"]
pub mod flexfrg4ctrl;
#[doc = "Peripheral reset control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexfrgxctrl4](flexfrgxctrl4) module"]
pub type FLEXFRGXCTRL4 = crate::Reg<u32, _FLEXFRGXCTRL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXFRGXCTRL4;
#[doc = "`read()` method returns [flexfrgxctrl4::R](flexfrgxctrl4::R) reader structure"]
impl crate::Readable for FLEXFRGXCTRL4 {}
#[doc = "`write(|w| ..)` method takes [flexfrgxctrl4::W](flexfrgxctrl4::W) writer structure"]
impl crate::Writable for FLEXFRGXCTRL4 {}
#[doc = "Peripheral reset control register"]
pub mod flexfrgxctrl4;
#[doc = "Fractional rate divider for flexcomm 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexfrg5ctrl](flexfrg5ctrl) module"]
pub type FLEXFRG5CTRL = crate::Reg<u32, _FLEXFRG5CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXFRG5CTRL;
#[doc = "`read()` method returns [flexfrg5ctrl::R](flexfrg5ctrl::R) reader structure"]
impl crate::Readable for FLEXFRG5CTRL {}
#[doc = "`write(|w| ..)` method takes [flexfrg5ctrl::W](flexfrg5ctrl::W) writer structure"]
impl crate::Writable for FLEXFRG5CTRL {}
#[doc = "Fractional rate divider for flexcomm 5"]
pub mod flexfrg5ctrl;
#[doc = "Peripheral reset control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexfrgxctrl5](flexfrgxctrl5) module"]
pub type FLEXFRGXCTRL5 = crate::Reg<u32, _FLEXFRGXCTRL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXFRGXCTRL5;
#[doc = "`read()` method returns [flexfrgxctrl5::R](flexfrgxctrl5::R) reader structure"]
impl crate::Readable for FLEXFRGXCTRL5 {}
#[doc = "`write(|w| ..)` method takes [flexfrgxctrl5::W](flexfrgxctrl5::W) writer structure"]
impl crate::Writable for FLEXFRGXCTRL5 {}
#[doc = "Peripheral reset control register"]
pub mod flexfrgxctrl5;
#[doc = "Fractional rate divider for flexcomm 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexfrg6ctrl](flexfrg6ctrl) module"]
pub type FLEXFRG6CTRL = crate::Reg<u32, _FLEXFRG6CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXFRG6CTRL;
#[doc = "`read()` method returns [flexfrg6ctrl::R](flexfrg6ctrl::R) reader structure"]
impl crate::Readable for FLEXFRG6CTRL {}
#[doc = "`write(|w| ..)` method takes [flexfrg6ctrl::W](flexfrg6ctrl::W) writer structure"]
impl crate::Writable for FLEXFRG6CTRL {}
#[doc = "Fractional rate divider for flexcomm 6"]
pub mod flexfrg6ctrl;
#[doc = "Peripheral reset control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexfrgxctrl6](flexfrgxctrl6) module"]
pub type FLEXFRGXCTRL6 = crate::Reg<u32, _FLEXFRGXCTRL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXFRGXCTRL6;
#[doc = "`read()` method returns [flexfrgxctrl6::R](flexfrgxctrl6::R) reader structure"]
impl crate::Readable for FLEXFRGXCTRL6 {}
#[doc = "`write(|w| ..)` method takes [flexfrgxctrl6::W](flexfrgxctrl6::W) writer structure"]
impl crate::Writable for FLEXFRGXCTRL6 {}
#[doc = "Peripheral reset control register"]
pub mod flexfrgxctrl6;
#[doc = "Fractional rate divider for flexcomm 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexfrg7ctrl](flexfrg7ctrl) module"]
pub type FLEXFRG7CTRL = crate::Reg<u32, _FLEXFRG7CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXFRG7CTRL;
#[doc = "`read()` method returns [flexfrg7ctrl::R](flexfrg7ctrl::R) reader structure"]
impl crate::Readable for FLEXFRG7CTRL {}
#[doc = "`write(|w| ..)` method takes [flexfrg7ctrl::W](flexfrg7ctrl::W) writer structure"]
impl crate::Writable for FLEXFRG7CTRL {}
#[doc = "Fractional rate divider for flexcomm 7"]
pub mod flexfrg7ctrl;
#[doc = "Peripheral reset control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flexfrgxctrl7](flexfrgxctrl7) module"]
pub type FLEXFRGXCTRL7 = crate::Reg<u32, _FLEXFRGXCTRL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLEXFRGXCTRL7;
#[doc = "`read()` method returns [flexfrgxctrl7::R](flexfrgxctrl7::R) reader structure"]
impl crate::Readable for FLEXFRGXCTRL7 {}
#[doc = "`write(|w| ..)` method takes [flexfrgxctrl7::W](flexfrgxctrl7::W) writer structure"]
impl crate::Writable for FLEXFRGXCTRL7 {}
#[doc = "Peripheral reset control register"]
pub mod flexfrgxctrl7;
#[doc = "System clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ahbclkdiv](ahbclkdiv) module"]
pub type AHBCLKDIV = crate::Reg<u32, _AHBCLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBCLKDIV;
#[doc = "`read()` method returns [ahbclkdiv::R](ahbclkdiv::R) reader structure"]
impl crate::Readable for AHBCLKDIV {}
#[doc = "`write(|w| ..)` method takes [ahbclkdiv::W](ahbclkdiv::W) writer structure"]
impl crate::Writable for AHBCLKDIV {}
#[doc = "System clock divider"]
pub mod ahbclkdiv;
#[doc = "CLKOUT clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clkoutdiv](clkoutdiv) module"]
pub type CLKOUTDIV = crate::Reg<u32, _CLKOUTDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKOUTDIV;
#[doc = "`read()` method returns [clkoutdiv::R](clkoutdiv::R) reader structure"]
impl crate::Readable for CLKOUTDIV {}
#[doc = "`write(|w| ..)` method takes [clkoutdiv::W](clkoutdiv::W) writer structure"]
impl crate::Writable for CLKOUTDIV {}
#[doc = "CLKOUT clock divider"]
pub mod clkoutdiv;
#[doc = "FRO_HF (96MHz) clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [frohfdiv](frohfdiv) module"]
pub type FROHFDIV = crate::Reg<u32, _FROHFDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FROHFDIV;
#[doc = "`read()` method returns [frohfdiv::R](frohfdiv::R) reader structure"]
impl crate::Readable for FROHFDIV {}
#[doc = "`write(|w| ..)` method takes [frohfdiv::W](frohfdiv::W) writer structure"]
impl crate::Writable for FROHFDIV {}
#[doc = "FRO_HF (96MHz) clock divider"]
pub mod frohfdiv;
#[doc = "WDT clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdtclkdiv](wdtclkdiv) module"]
pub type WDTCLKDIV = crate::Reg<u32, _WDTCLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTCLKDIV;
#[doc = "`read()` method returns [wdtclkdiv::R](wdtclkdiv::R) reader structure"]
impl crate::Readable for WDTCLKDIV {}
#[doc = "`write(|w| ..)` method takes [wdtclkdiv::W](wdtclkdiv::W) writer structure"]
impl crate::Writable for WDTCLKDIV {}
#[doc = "WDT clock divider"]
pub mod wdtclkdiv;
#[doc = "ADC clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [adcclkdiv](adcclkdiv) module"]
pub type ADCCLKDIV = crate::Reg<u32, _ADCCLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCCLKDIV;
#[doc = "`read()` method returns [adcclkdiv::R](adcclkdiv::R) reader structure"]
impl crate::Readable for ADCCLKDIV {}
#[doc = "`write(|w| ..)` method takes [adcclkdiv::W](adcclkdiv::W) writer structure"]
impl crate::Writable for ADCCLKDIV {}
#[doc = "ADC clock divider"]
pub mod adcclkdiv;
#[doc = "USB0 Clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usb0clkdiv](usb0clkdiv) module"]
pub type USB0CLKDIV = crate::Reg<u32, _USB0CLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB0CLKDIV;
#[doc = "`read()` method returns [usb0clkdiv::R](usb0clkdiv::R) reader structure"]
impl crate::Readable for USB0CLKDIV {}
#[doc = "`write(|w| ..)` method takes [usb0clkdiv::W](usb0clkdiv::W) writer structure"]
impl crate::Writable for USB0CLKDIV {}
#[doc = "USB0 Clock divider"]
pub mod usb0clkdiv;
#[doc = "I2S MCLK clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mclkdiv](mclkdiv) module"]
pub type MCLKDIV = crate::Reg<u32, _MCLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCLKDIV;
#[doc = "`read()` method returns [mclkdiv::R](mclkdiv::R) reader structure"]
impl crate::Readable for MCLKDIV {}
#[doc = "`write(|w| ..)` method takes [mclkdiv::W](mclkdiv::W) writer structure"]
impl crate::Writable for MCLKDIV {}
#[doc = "I2S MCLK clock divider"]
pub mod mclkdiv;
#[doc = "SCT/PWM clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sctclkdiv](sctclkdiv) module"]
pub type SCTCLKDIV = crate::Reg<u32, _SCTCLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTCLKDIV;
#[doc = "`read()` method returns [sctclkdiv::R](sctclkdiv::R) reader structure"]
impl crate::Readable for SCTCLKDIV {}
#[doc = "`write(|w| ..)` method takes [sctclkdiv::W](sctclkdiv::W) writer structure"]
impl crate::Writable for SCTCLKDIV {}
#[doc = "SCT/PWM clock divider"]
pub mod sctclkdiv;
#[doc = "SDIO clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sdioclkdiv](sdioclkdiv) module"]
pub type SDIOCLKDIV = crate::Reg<u32, _SDIOCLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDIOCLKDIV;
#[doc = "`read()` method returns [sdioclkdiv::R](sdioclkdiv::R) reader structure"]
impl crate::Readable for SDIOCLKDIV {}
#[doc = "`write(|w| ..)` method takes [sdioclkdiv::W](sdioclkdiv::W) writer structure"]
impl crate::Writable for SDIOCLKDIV {}
#[doc = "SDIO clock divider"]
pub mod sdioclkdiv;
#[doc = "PLL0 clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll0clkdiv](pll0clkdiv) module"]
pub type PLL0CLKDIV = crate::Reg<u32, _PLL0CLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL0CLKDIV;
#[doc = "`read()` method returns [pll0clkdiv::R](pll0clkdiv::R) reader structure"]
impl crate::Readable for PLL0CLKDIV {}
#[doc = "`write(|w| ..)` method takes [pll0clkdiv::W](pll0clkdiv::W) writer structure"]
impl crate::Writable for PLL0CLKDIV {}
#[doc = "PLL0 clock divider"]
pub mod pll0clkdiv;
#[doc = "Control clock configuration registers access (like xxxDIV, xxxSEL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clockgenupdatelockout](clockgenupdatelockout) module"]
pub type CLOCKGENUPDATELOCKOUT = crate::Reg<u32, _CLOCKGENUPDATELOCKOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLOCKGENUPDATELOCKOUT;
#[doc = "`read()` method returns [clockgenupdatelockout::R](clockgenupdatelockout::R) reader structure"]
impl crate::Readable for CLOCKGENUPDATELOCKOUT {}
#[doc = "`write(|w| ..)` method takes [clockgenupdatelockout::W](clockgenupdatelockout::W) writer structure"]
impl crate::Writable for CLOCKGENUPDATELOCKOUT {}
#[doc = "Control clock configuration registers access (like xxxDIV, xxxSEL)"]
pub mod clockgenupdatelockout;
#[doc = "FMC configuration register - INTERNAL USE ONLY\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fmccr](fmccr) module"]
pub type FMCCR = crate::Reg<u32, _FMCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMCCR;
#[doc = "`read()` method returns [fmccr::R](fmccr::R) reader structure"]
impl crate::Readable for FMCCR {}
#[doc = "`write(|w| ..)` method takes [fmccr::W](fmccr::W) writer structure"]
impl crate::Writable for FMCCR {}
#[doc = "FMC configuration register - INTERNAL USE ONLY"]
pub mod fmccr;
#[doc = "USB0 clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usb0clkctrl](usb0clkctrl) module"]
pub type USB0CLKCTRL = crate::Reg<u32, _USB0CLKCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB0CLKCTRL;
#[doc = "`read()` method returns [usb0clkctrl::R](usb0clkctrl::R) reader structure"]
impl crate::Readable for USB0CLKCTRL {}
#[doc = "`write(|w| ..)` method takes [usb0clkctrl::W](usb0clkctrl::W) writer structure"]
impl crate::Writable for USB0CLKCTRL {}
#[doc = "USB0 clock control"]
pub mod usb0clkctrl;
#[doc = "USB0 clock status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usb0clkstat](usb0clkstat) module"]
pub type USB0CLKSTAT = crate::Reg<u32, _USB0CLKSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB0CLKSTAT;
#[doc = "`read()` method returns [usb0clkstat::R](usb0clkstat::R) reader structure"]
impl crate::Readable for USB0CLKSTAT {}
#[doc = "`write(|w| ..)` method takes [usb0clkstat::W](usb0clkstat::W) writer structure"]
impl crate::Writable for USB0CLKSTAT {}
#[doc = "USB0 clock status"]
pub mod usb0clkstat;
#[doc = "FMCflush control\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fmcflush](fmcflush) module"]
pub type FMCFLUSH = crate::Reg<u32, _FMCFLUSH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMCFLUSH;
#[doc = "`write(|w| ..)` method takes [fmcflush::W](fmcflush::W) writer structure"]
impl crate::Writable for FMCFLUSH {}
#[doc = "FMCflush control"]
pub mod fmcflush;
#[doc = "MCLK control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mclkio](mclkio) module"]
pub type MCLKIO = crate::Reg<u32, _MCLKIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCLKIO;
#[doc = "`read()` method returns [mclkio::R](mclkio::R) reader structure"]
impl crate::Readable for MCLKIO {}
#[doc = "`write(|w| ..)` method takes [mclkio::W](mclkio::W) writer structure"]
impl crate::Writable for MCLKIO {}
#[doc = "MCLK control"]
pub mod mclkio;
#[doc = "USB1 clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usb1clkctrl](usb1clkctrl) module"]
pub type USB1CLKCTRL = crate::Reg<u32, _USB1CLKCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB1CLKCTRL;
#[doc = "`read()` method returns [usb1clkctrl::R](usb1clkctrl::R) reader structure"]
impl crate::Readable for USB1CLKCTRL {}
#[doc = "`write(|w| ..)` method takes [usb1clkctrl::W](usb1clkctrl::W) writer structure"]
impl crate::Writable for USB1CLKCTRL {}
#[doc = "USB1 clock control"]
pub mod usb1clkctrl;
#[doc = "USB1 clock status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usb1clkstat](usb1clkstat) module"]
pub type USB1CLKSTAT = crate::Reg<u32, _USB1CLKSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB1CLKSTAT;
#[doc = "`read()` method returns [usb1clkstat::R](usb1clkstat::R) reader structure"]
impl crate::Readable for USB1CLKSTAT {}
#[doc = "`write(|w| ..)` method takes [usb1clkstat::W](usb1clkstat::W) writer structure"]
impl crate::Writable for USB1CLKSTAT {}
#[doc = "USB1 clock status"]
pub mod usb1clkstat;
#[doc = "Flash Banks control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flashbankenable](flashbankenable) module"]
pub type FLASHBANKENABLE = crate::Reg<u32, _FLASHBANKENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASHBANKENABLE;
#[doc = "`read()` method returns [flashbankenable::R](flashbankenable::R) reader structure"]
impl crate::Readable for FLASHBANKENABLE {}
#[doc = "`write(|w| ..)` method takes [flashbankenable::W](flashbankenable::W) writer structure"]
impl crate::Writable for FLASHBANKENABLE {}
#[doc = "Flash Banks control"]
pub mod flashbankenable;
#[doc = "SDIO CCLKIN phase and delay control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sdioclkctrl](sdioclkctrl) module"]
pub type SDIOCLKCTRL = crate::Reg<u32, _SDIOCLKCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDIOCLKCTRL;
#[doc = "`read()` method returns [sdioclkctrl::R](sdioclkctrl::R) reader structure"]
impl crate::Readable for SDIOCLKCTRL {}
#[doc = "`write(|w| ..)` method takes [sdioclkctrl::W](sdioclkctrl::W) writer structure"]
impl crate::Writable for SDIOCLKCTRL {}
#[doc = "SDIO CCLKIN phase and delay control"]
pub mod sdioclkctrl;
#[doc = "PLL1 550m control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll1ctrl](pll1ctrl) module"]
pub type PLL1CTRL = crate::Reg<u32, _PLL1CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL1CTRL;
#[doc = "`read()` method returns [pll1ctrl::R](pll1ctrl::R) reader structure"]
impl crate::Readable for PLL1CTRL {}
#[doc = "`write(|w| ..)` method takes [pll1ctrl::W](pll1ctrl::W) writer structure"]
impl crate::Writable for PLL1CTRL {}
#[doc = "PLL1 550m control"]
pub mod pll1ctrl;
#[doc = "PLL1 550m status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll1stat](pll1stat) module"]
pub type PLL1STAT = crate::Reg<u32, _PLL1STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL1STAT;
#[doc = "`read()` method returns [pll1stat::R](pll1stat::R) reader structure"]
impl crate::Readable for PLL1STAT {}
#[doc = "`write(|w| ..)` method takes [pll1stat::W](pll1stat::W) writer structure"]
impl crate::Writable for PLL1STAT {}
#[doc = "PLL1 550m status"]
pub mod pll1stat;
#[doc = "PLL1 550m N divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll1ndec](pll1ndec) module"]
pub type PLL1NDEC = crate::Reg<u32, _PLL1NDEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL1NDEC;
#[doc = "`read()` method returns [pll1ndec::R](pll1ndec::R) reader structure"]
impl crate::Readable for PLL1NDEC {}
#[doc = "`write(|w| ..)` method takes [pll1ndec::W](pll1ndec::W) writer structure"]
impl crate::Writable for PLL1NDEC {}
#[doc = "PLL1 550m N divider"]
pub mod pll1ndec;
#[doc = "PLL1 550m M divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll1mdec](pll1mdec) module"]
pub type PLL1MDEC = crate::Reg<u32, _PLL1MDEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL1MDEC;
#[doc = "`read()` method returns [pll1mdec::R](pll1mdec::R) reader structure"]
impl crate::Readable for PLL1MDEC {}
#[doc = "`write(|w| ..)` method takes [pll1mdec::W](pll1mdec::W) writer structure"]
impl crate::Writable for PLL1MDEC {}
#[doc = "PLL1 550m M divider"]
pub mod pll1mdec;
#[doc = "PLL1 550m P divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll1pdec](pll1pdec) module"]
pub type PLL1PDEC = crate::Reg<u32, _PLL1PDEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL1PDEC;
#[doc = "`read()` method returns [pll1pdec::R](pll1pdec::R) reader structure"]
impl crate::Readable for PLL1PDEC {}
#[doc = "`write(|w| ..)` method takes [pll1pdec::W](pll1pdec::W) writer structure"]
impl crate::Writable for PLL1PDEC {}
#[doc = "PLL1 550m P divider"]
pub mod pll1pdec;
#[doc = "PLL0 550m control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll0ctrl](pll0ctrl) module"]
pub type PLL0CTRL = crate::Reg<u32, _PLL0CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL0CTRL;
#[doc = "`read()` method returns [pll0ctrl::R](pll0ctrl::R) reader structure"]
impl crate::Readable for PLL0CTRL {}
#[doc = "`write(|w| ..)` method takes [pll0ctrl::W](pll0ctrl::W) writer structure"]
impl crate::Writable for PLL0CTRL {}
#[doc = "PLL0 550m control"]
pub mod pll0ctrl;
#[doc = "PLL0 550m status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll0stat](pll0stat) module"]
pub type PLL0STAT = crate::Reg<u32, _PLL0STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL0STAT;
#[doc = "`read()` method returns [pll0stat::R](pll0stat::R) reader structure"]
impl crate::Readable for PLL0STAT {}
#[doc = "`write(|w| ..)` method takes [pll0stat::W](pll0stat::W) writer structure"]
impl crate::Writable for PLL0STAT {}
#[doc = "PLL0 550m status"]
pub mod pll0stat;
#[doc = "PLL0 550m N divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll0ndec](pll0ndec) module"]
pub type PLL0NDEC = crate::Reg<u32, _PLL0NDEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL0NDEC;
#[doc = "`read()` method returns [pll0ndec::R](pll0ndec::R) reader structure"]
impl crate::Readable for PLL0NDEC {}
#[doc = "`write(|w| ..)` method takes [pll0ndec::W](pll0ndec::W) writer structure"]
impl crate::Writable for PLL0NDEC {}
#[doc = "PLL0 550m N divider"]
pub mod pll0ndec;
#[doc = "PLL0 550m P divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll0pdec](pll0pdec) module"]
pub type PLL0PDEC = crate::Reg<u32, _PLL0PDEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL0PDEC;
#[doc = "`read()` method returns [pll0pdec::R](pll0pdec::R) reader structure"]
impl crate::Readable for PLL0PDEC {}
#[doc = "`write(|w| ..)` method takes [pll0pdec::W](pll0pdec::W) writer structure"]
impl crate::Writable for PLL0PDEC {}
#[doc = "PLL0 550m P divider"]
pub mod pll0pdec;
#[doc = "PLL0 Spread Spectrum Wrapper control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll0sscg0](pll0sscg0) module"]
pub type PLL0SSCG0 = crate::Reg<u32, _PLL0SSCG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL0SSCG0;
#[doc = "`read()` method returns [pll0sscg0::R](pll0sscg0::R) reader structure"]
impl crate::Readable for PLL0SSCG0 {}
#[doc = "`write(|w| ..)` method takes [pll0sscg0::W](pll0sscg0::W) writer structure"]
impl crate::Writable for PLL0SSCG0 {}
#[doc = "PLL0 Spread Spectrum Wrapper control register 0"]
pub mod pll0sscg0;
#[doc = "PLL0 Spread Spectrum Wrapper control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll0sscg1](pll0sscg1) module"]
pub type PLL0SSCG1 = crate::Reg<u32, _PLL0SSCG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL0SSCG1;
#[doc = "`read()` method returns [pll0sscg1::R](pll0sscg1::R) reader structure"]
impl crate::Readable for PLL0SSCG1 {}
#[doc = "`write(|w| ..)` method takes [pll0sscg1::W](pll0sscg1::W) writer structure"]
impl crate::Writable for PLL0SSCG1 {}
#[doc = "PLL0 Spread Spectrum Wrapper control register 1"]
pub mod pll0sscg1;
#[doc = "eFUSE controller clock enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [efuseclkctrl](efuseclkctrl) module"]
pub type EFUSECLKCTRL = crate::Reg<u32, _EFUSECLKCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSECLKCTRL;
#[doc = "`read()` method returns [efuseclkctrl::R](efuseclkctrl::R) reader structure"]
impl crate::Readable for EFUSECLKCTRL {}
#[doc = "`write(|w| ..)` method takes [efuseclkctrl::W](efuseclkctrl::W) writer structure"]
impl crate::Writable for EFUSECLKCTRL {}
#[doc = "eFUSE controller clock enable"]
pub mod efuseclkctrl;
#[doc = "Start logic wake-up enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [starter0](starter0) module"]
pub type STARTER0 = crate::Reg<u32, _STARTER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STARTER0;
#[doc = "`read()` method returns [starter0::R](starter0::R) reader structure"]
impl crate::Readable for STARTER0 {}
#[doc = "`write(|w| ..)` method takes [starter0::W](starter0::W) writer structure"]
impl crate::Writable for STARTER0 {}
#[doc = "Start logic wake-up enable register"]
pub mod starter0;
#[doc = "Start logic wake-up enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [starter1](starter1) module"]
pub type STARTER1 = crate::Reg<u32, _STARTER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STARTER1;
#[doc = "`read()` method returns [starter1::R](starter1::R) reader structure"]
impl crate::Readable for STARTER1 {}
#[doc = "`write(|w| ..)` method takes [starter1::W](starter1::W) writer structure"]
impl crate::Writable for STARTER1 {}
#[doc = "Start logic wake-up enable register"]
pub mod starter1;
#[doc = "Set bits in STARTER\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [starterset0](starterset0) module"]
pub type STARTERSET0 = crate::Reg<u32, _STARTERSET0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STARTERSET0;
#[doc = "`write(|w| ..)` method takes [starterset0::W](starterset0::W) writer structure"]
impl crate::Writable for STARTERSET0 {}
#[doc = "Set bits in STARTER"]
pub mod starterset0;
#[doc = "Set bits in STARTER\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [starterset1](starterset1) module"]
pub type STARTERSET1 = crate::Reg<u32, _STARTERSET1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STARTERSET1;
#[doc = "`write(|w| ..)` method takes [starterset1::W](starterset1::W) writer structure"]
impl crate::Writable for STARTERSET1 {}
#[doc = "Set bits in STARTER"]
pub mod starterset1;
#[doc = "Clear bits in STARTER\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [starterclr0](starterclr0) module"]
pub type STARTERCLR0 = crate::Reg<u32, _STARTERCLR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STARTERCLR0;
#[doc = "`write(|w| ..)` method takes [starterclr0::W](starterclr0::W) writer structure"]
impl crate::Writable for STARTERCLR0 {}
#[doc = "Clear bits in STARTER"]
pub mod starterclr0;
#[doc = "Clear bits in STARTER\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [starterclr1](starterclr1) module"]
pub type STARTERCLR1 = crate::Reg<u32, _STARTERCLR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STARTERCLR1;
#[doc = "`write(|w| ..)` method takes [starterclr1::W](starterclr1::W) writer structure"]
impl crate::Writable for STARTERCLR1 {}
#[doc = "Clear bits in STARTER"]
pub mod starterclr1;
#[doc = "Hardware Sleep control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hardwaresleep](hardwaresleep) module"]
pub type HARDWARESLEEP = crate::Reg<u32, _HARDWARESLEEP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HARDWARESLEEP;
#[doc = "`read()` method returns [hardwaresleep::R](hardwaresleep::R) reader structure"]
impl crate::Readable for HARDWARESLEEP {}
#[doc = "`write(|w| ..)` method takes [hardwaresleep::W](hardwaresleep::W) writer structure"]
impl crate::Writable for HARDWARESLEEP {}
#[doc = "Hardware Sleep control"]
pub mod hardwaresleep;
#[doc = "CPU Control for multiple processors\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpuctrl](cpuctrl) module"]
pub type CPUCTRL = crate::Reg<u32, _CPUCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUCTRL;
#[doc = "`read()` method returns [cpuctrl::R](cpuctrl::R) reader structure"]
impl crate::Readable for CPUCTRL {}
#[doc = "`write(|w| ..)` method takes [cpuctrl::W](cpuctrl::W) writer structure"]
impl crate::Writable for CPUCTRL {}
#[doc = "CPU Control for multiple processors"]
pub mod cpuctrl;
#[doc = "Coprocessor Boot Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpboot](cpboot) module"]
pub type CPBOOT = crate::Reg<u32, _CPBOOT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPBOOT;
#[doc = "`read()` method returns [cpboot::R](cpboot::R) reader structure"]
impl crate::Readable for CPBOOT {}
#[doc = "`write(|w| ..)` method takes [cpboot::W](cpboot::W) writer structure"]
impl crate::Writable for CPBOOT {}
#[doc = "Coprocessor Boot Address"]
pub mod cpboot;
#[doc = "Coprocessor Stack Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpstack](cpstack) module"]
pub type CPSTACK = crate::Reg<u32, _CPSTACK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPSTACK;
#[doc = "`read()` method returns [cpstack::R](cpstack::R) reader structure"]
impl crate::Readable for CPSTACK {}
#[doc = "`write(|w| ..)` method takes [cpstack::W](cpstack::W) writer structure"]
impl crate::Writable for CPSTACK {}
#[doc = "Coprocessor Stack Address"]
pub mod cpstack;
#[doc = "CPU Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpstat](cpstat) module"]
pub type CPSTAT = crate::Reg<u32, _CPSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPSTAT;
#[doc = "`read()` method returns [cpstat::R](cpstat::R) reader structure"]
impl crate::Readable for CPSTAT {}
#[doc = "`write(|w| ..)` method takes [cpstat::W](cpstat::W) writer structure"]
impl crate::Writable for CPSTAT {}
#[doc = "CPU Status"]
pub mod cpstat;
#[doc = "Composite Device Identifier\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dice_reg0](dice_reg0) module"]
pub type DICE_REG0 = crate::Reg<u32, _DICE_REG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DICE_REG0;
#[doc = "`read()` method returns [dice_reg0::R](dice_reg0::R) reader structure"]
impl crate::Readable for DICE_REG0 {}
#[doc = "`write(|w| ..)` method takes [dice_reg0::W](dice_reg0::W) writer structure"]
impl crate::Writable for DICE_REG0 {}
#[doc = "Composite Device Identifier"]
pub mod dice_reg0;
#[doc = "Composite Device Identifier\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dice_reg1](dice_reg1) module"]
pub type DICE_REG1 = crate::Reg<u32, _DICE_REG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DICE_REG1;
#[doc = "`read()` method returns [dice_reg1::R](dice_reg1::R) reader structure"]
impl crate::Readable for DICE_REG1 {}
#[doc = "`write(|w| ..)` method takes [dice_reg1::W](dice_reg1::W) writer structure"]
impl crate::Writable for DICE_REG1 {}
#[doc = "Composite Device Identifier"]
pub mod dice_reg1;
#[doc = "Composite Device Identifier\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dice_reg2](dice_reg2) module"]
pub type DICE_REG2 = crate::Reg<u32, _DICE_REG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DICE_REG2;
#[doc = "`read()` method returns [dice_reg2::R](dice_reg2::R) reader structure"]
impl crate::Readable for DICE_REG2 {}
#[doc = "`write(|w| ..)` method takes [dice_reg2::W](dice_reg2::W) writer structure"]
impl crate::Writable for DICE_REG2 {}
#[doc = "Composite Device Identifier"]
pub mod dice_reg2;
#[doc = "Composite Device Identifier\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dice_reg3](dice_reg3) module"]
pub type DICE_REG3 = crate::Reg<u32, _DICE_REG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DICE_REG3;
#[doc = "`read()` method returns [dice_reg3::R](dice_reg3::R) reader structure"]
impl crate::Readable for DICE_REG3 {}
#[doc = "`write(|w| ..)` method takes [dice_reg3::W](dice_reg3::W) writer structure"]
impl crate::Writable for DICE_REG3 {}
#[doc = "Composite Device Identifier"]
pub mod dice_reg3;
#[doc = "Composite Device Identifier\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dice_reg4](dice_reg4) module"]
pub type DICE_REG4 = crate::Reg<u32, _DICE_REG4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DICE_REG4;
#[doc = "`read()` method returns [dice_reg4::R](dice_reg4::R) reader structure"]
impl crate::Readable for DICE_REG4 {}
#[doc = "`write(|w| ..)` method takes [dice_reg4::W](dice_reg4::W) writer structure"]
impl crate::Writable for DICE_REG4 {}
#[doc = "Composite Device Identifier"]
pub mod dice_reg4;
#[doc = "Composite Device Identifier\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dice_reg5](dice_reg5) module"]
pub type DICE_REG5 = crate::Reg<u32, _DICE_REG5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DICE_REG5;
#[doc = "`read()` method returns [dice_reg5::R](dice_reg5::R) reader structure"]
impl crate::Readable for DICE_REG5 {}
#[doc = "`write(|w| ..)` method takes [dice_reg5::W](dice_reg5::W) writer structure"]
impl crate::Writable for DICE_REG5 {}
#[doc = "Composite Device Identifier"]
pub mod dice_reg5;
#[doc = "Composite Device Identifier\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dice_reg6](dice_reg6) module"]
pub type DICE_REG6 = crate::Reg<u32, _DICE_REG6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DICE_REG6;
#[doc = "`read()` method returns [dice_reg6::R](dice_reg6::R) reader structure"]
impl crate::Readable for DICE_REG6 {}
#[doc = "`write(|w| ..)` method takes [dice_reg6::W](dice_reg6::W) writer structure"]
impl crate::Writable for DICE_REG6 {}
#[doc = "Composite Device Identifier"]
pub mod dice_reg6;
#[doc = "Composite Device Identifier\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dice_reg7](dice_reg7) module"]
pub type DICE_REG7 = crate::Reg<u32, _DICE_REG7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DICE_REG7;
#[doc = "`read()` method returns [dice_reg7::R](dice_reg7::R) reader structure"]
impl crate::Readable for DICE_REG7 {}
#[doc = "`write(|w| ..)` method takes [dice_reg7::W](dice_reg7::W) writer structure"]
impl crate::Writable for DICE_REG7 {}
#[doc = "Composite Device Identifier"]
pub mod dice_reg7;
#[doc = "Various system clock controls : Flash clock (48 MHz) control, clocks to Frequency Measures\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clock_ctrl](clock_ctrl) module"]
pub type CLOCK_CTRL = crate::Reg<u32, _CLOCK_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLOCK_CTRL;
#[doc = "`read()` method returns [clock_ctrl::R](clock_ctrl::R) reader structure"]
impl crate::Readable for CLOCK_CTRL {}
#[doc = "`write(|w| ..)` method takes [clock_ctrl::W](clock_ctrl::W) writer structure"]
impl crate::Writable for CLOCK_CTRL {}
#[doc = "Various system clock controls : Flash clock (48 MHz) control, clocks to Frequency Measures"]
pub mod clock_ctrl;
#[doc = "Comparator Interrupt control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [comp_int_ctrl](comp_int_ctrl) module"]
pub type COMP_INT_CTRL = crate::Reg<u32, _COMP_INT_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP_INT_CTRL;
#[doc = "`read()` method returns [comp_int_ctrl::R](comp_int_ctrl::R) reader structure"]
impl crate::Readable for COMP_INT_CTRL {}
#[doc = "`write(|w| ..)` method takes [comp_int_ctrl::W](comp_int_ctrl::W) writer structure"]
impl crate::Writable for COMP_INT_CTRL {}
#[doc = "Comparator Interrupt control"]
pub mod comp_int_ctrl;
#[doc = "Comparator Interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [comp_int_status](comp_int_status) module"]
pub type COMP_INT_STATUS = crate::Reg<u32, _COMP_INT_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP_INT_STATUS;
#[doc = "`read()` method returns [comp_int_status::R](comp_int_status::R) reader structure"]
impl crate::Readable for COMP_INT_STATUS {}
#[doc = "`write(|w| ..)` method takes [comp_int_status::W](comp_int_status::W) writer structure"]
impl crate::Writable for COMP_INT_STATUS {}
#[doc = "Comparator Interrupt status"]
pub mod comp_int_status;
#[doc = "Control automatic clock gating\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [autoclkgateoverride](autoclkgateoverride) module"]
pub type AUTOCLKGATEOVERRIDE = crate::Reg<u32, _AUTOCLKGATEOVERRIDE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUTOCLKGATEOVERRIDE;
#[doc = "`read()` method returns [autoclkgateoverride::R](autoclkgateoverride::R) reader structure"]
impl crate::Readable for AUTOCLKGATEOVERRIDE {}
#[doc = "`write(|w| ..)` method takes [autoclkgateoverride::W](autoclkgateoverride::W) writer structure"]
impl crate::Writable for AUTOCLKGATEOVERRIDE {}
#[doc = "Control automatic clock gating"]
pub mod autoclkgateoverride;
#[doc = "Enable bypass of the first stage of synchonization inside GPIO_INT module\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiopsync](gpiopsync) module"]
pub type GPIOPSYNC = crate::Reg<u32, _GPIOPSYNC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOPSYNC;
#[doc = "`read()` method returns [gpiopsync::R](gpiopsync::R) reader structure"]
impl crate::Readable for GPIOPSYNC {}
#[doc = "`write(|w| ..)` method takes [gpiopsync::W](gpiopsync::W) writer structure"]
impl crate::Writable for GPIOPSYNC {}
#[doc = "Enable bypass of the first stage of synchonization inside GPIO_INT module"]
pub mod gpiopsync;
#[doc = "Control write access to security registers -- FOR INTERNAl USE ONLY\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [debug_lock_en](debug_lock_en) module"]
pub type DEBUG_LOCK_EN = crate::Reg<u32, _DEBUG_LOCK_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEBUG_LOCK_EN;
#[doc = "`read()` method returns [debug_lock_en::R](debug_lock_en::R) reader structure"]
impl crate::Readable for DEBUG_LOCK_EN {}
#[doc = "`write(|w| ..)` method takes [debug_lock_en::W](debug_lock_en::W) writer structure"]
impl crate::Writable for DEBUG_LOCK_EN {}
#[doc = "Control write access to security registers -- FOR INTERNAl USE ONLY"]
pub mod debug_lock_en;
#[doc = "Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control -- FOR INTERNAl USE ONLY\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [debug_features](debug_features) module"]
pub type DEBUG_FEATURES = crate::Reg<u32, _DEBUG_FEATURES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEBUG_FEATURES;
#[doc = "`read()` method returns [debug_features::R](debug_features::R) reader structure"]
impl crate::Readable for DEBUG_FEATURES {}
#[doc = "`write(|w| ..)` method takes [debug_features::W](debug_features::W) writer structure"]
impl crate::Writable for DEBUG_FEATURES {}
#[doc = "Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control -- FOR INTERNAl USE ONLY"]
pub mod debug_features;
#[doc = "Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control DUPLICATE register -- FOR INTERNAl USE ONLY\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [debug_features_dp](debug_features_dp) module"]
pub type DEBUG_FEATURES_DP = crate::Reg<u32, _DEBUG_FEATURES_DP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEBUG_FEATURES_DP;
#[doc = "`read()` method returns [debug_features_dp::R](debug_features_dp::R) reader structure"]
impl crate::Readable for DEBUG_FEATURES_DP {}
#[doc = "`write(|w| ..)` method takes [debug_features_dp::W](debug_features_dp::W) writer structure"]
impl crate::Writable for DEBUG_FEATURES_DP {}
#[doc = "Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control DUPLICATE register -- FOR INTERNAl USE ONLY"]
pub mod debug_features_dp;
#[doc = "Security code to allow test (Design for Testability) access -- FOR INTERNAl USE ONLY\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [codesecurityprottest](codesecurityprottest) module"]
pub type CODESECURITYPROTTEST = crate::Reg<u32, _CODESECURITYPROTTEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CODESECURITYPROTTEST;
#[doc = "`write(|w| ..)` method takes [codesecurityprottest::W](codesecurityprottest::W) writer structure"]
impl crate::Writable for CODESECURITYPROTTEST {}
#[doc = "Security code to allow test (Design for Testability) access -- FOR INTERNAl USE ONLY"]
pub mod codesecurityprottest;
#[doc = "Security code to allow CPU0 (CM33) Debug Access Port (DAP) -- FOR INTERNAl USE ONLY\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [codesecurityprotcpu0](codesecurityprotcpu0) module"]
pub type CODESECURITYPROTCPU0 = crate::Reg<u32, _CODESECURITYPROTCPU0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CODESECURITYPROTCPU0;
#[doc = "`write(|w| ..)` method takes [codesecurityprotcpu0::W](codesecurityprotcpu0::W) writer structure"]
impl crate::Writable for CODESECURITYPROTCPU0 {}
#[doc = "Security code to allow CPU0 (CM33) Debug Access Port (DAP) -- FOR INTERNAl USE ONLY"]
pub mod codesecurityprotcpu0;
#[doc = "Security code to allow CPU1 (Micro CM33) Debug Access Port (DAP) -- FOR INTERNAl USE ONLY\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [codesecurityprotcpu1](codesecurityprotcpu1) module"]
pub type CODESECURITYPROTCPU1 = crate::Reg<u32, _CODESECURITYPROTCPU1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CODESECURITYPROTCPU1;
#[doc = "`write(|w| ..)` method takes [codesecurityprotcpu1::W](codesecurityprotcpu1::W) writer structure"]
impl crate::Writable for CODESECURITYPROTCPU1 {}
#[doc = "Security code to allow CPU1 (Micro CM33) Debug Access Port (DAP) -- FOR INTERNAl USE ONLY"]
pub mod codesecurityprotcpu1;
#[doc = "block quiddikey/PUF all index. -- FOR INTERNAL USE ONLY\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [key_block](key_block) module"]
pub type KEY_BLOCK = crate::Reg<u32, _KEY_BLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY_BLOCK;
#[doc = "`write(|w| ..)` method takes [key_block::W](key_block::W) writer structure"]
impl crate::Writable for KEY_BLOCK {}
#[doc = "block quiddikey/PUF all index. -- FOR INTERNAL USE ONLY"]
pub mod key_block;
#[doc = "Debug authentication scratch registers -- FOR INTERNAL USE ONLY\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [debug_auth_scratch](debug_auth_scratch) module"]
pub type DEBUG_AUTH_SCRATCH = crate::Reg<u32, _DEBUG_AUTH_SCRATCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEBUG_AUTH_SCRATCH;
#[doc = "`read()` method returns [debug_auth_scratch::R](debug_auth_scratch::R) reader structure"]
impl crate::Readable for DEBUG_AUTH_SCRATCH {}
#[doc = "`write(|w| ..)` method takes [debug_auth_scratch::W](debug_auth_scratch::W) writer structure"]
impl crate::Writable for DEBUG_AUTH_SCRATCH {}
#[doc = "Debug authentication scratch registers -- FOR INTERNAL USE ONLY"]
pub mod debug_auth_scratch;
#[doc = "CPUs configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpucfg](cpucfg) module"]
pub type CPUCFG = crate::Reg<u32, _CPUCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUCFG;
#[doc = "`read()` method returns [cpucfg::R](cpucfg::R) reader structure"]
impl crate::Readable for CPUCFG {}
#[doc = "`write(|w| ..)` method takes [cpucfg::W](cpucfg::W) writer structure"]
impl crate::Writable for CPUCFG {}
#[doc = "CPUs configuration register"]
pub mod cpucfg;
#[doc = "peripheral enable configuration -- FOR INTERNAL USE ONLY\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [periphencfg](periphencfg) module"]
pub type PERIPHENCFG = crate::Reg<u32, _PERIPHENCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERIPHENCFG;
#[doc = "`read()` method returns [periphencfg::R](periphencfg::R) reader structure"]
impl crate::Readable for PERIPHENCFG {}
#[doc = "`write(|w| ..)` method takes [periphencfg::W](periphencfg::W) writer structure"]
impl crate::Writable for PERIPHENCFG {}
#[doc = "peripheral enable configuration -- FOR INTERNAL USE ONLY"]
pub mod periphencfg;
#[doc = "Device ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [device_id0](device_id0) module"]
pub type DEVICE_ID0 = crate::Reg<u32, _DEVICE_ID0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVICE_ID0;
#[doc = "`read()` method returns [device_id0::R](device_id0::R) reader structure"]
impl crate::Readable for DEVICE_ID0 {}
#[doc = "Device ID"]
pub mod device_id0;
#[doc = "Chip revision ID and Number\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dieid](dieid) module"]
pub type DIEID = crate::Reg<u32, _DIEID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEID;
#[doc = "`read()` method returns [dieid::R](dieid::R) reader structure"]
impl crate::Readable for DIEID {}
#[doc = "Chip revision ID and Number"]
pub mod dieid;
