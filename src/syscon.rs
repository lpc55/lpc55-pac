#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Memory Remap control register"]
    pub memoryremap: crate::Reg<memoryremap::MEMORYREMAP_SPEC>,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - AHB Matrix priority control register Priority values are 3 = highest, 0 = lowest"]
    pub ahbmatprio: crate::Reg<ahbmatprio::AHBMATPRIO_SPEC>,
    _reserved2: [u8; 36usize],
    #[doc = "0x38 - System tick calibration for secure part of CPU0"]
    pub cpu0stckcal: crate::Reg<cpu0stckcal::CPU0STCKCAL_SPEC>,
    #[doc = "0x3c - System tick calibration for non-secure part of CPU0"]
    pub cpu0nstckcal: crate::Reg<cpu0nstckcal::CPU0NSTCKCAL_SPEC>,
    #[doc = "0x40 - System tick calibration for CPU1"]
    pub cpu1stckcal: crate::Reg<cpu1stckcal::CPU1STCKCAL_SPEC>,
    _reserved5: [u8; 4usize],
    #[doc = "0x48 - NMI Source Select"]
    pub nmisrc: crate::Reg<nmisrc::NMISRC_SPEC>,
    _reserved6: [u8; 180usize],
    #[doc = "0x100 - Peripheral reset control 0"]
    pub presetctrl0: crate::Reg<presetctrl0::PRESETCTRL0_SPEC>,
    #[doc = "0x104 - Peripheral reset control 1"]
    pub presetctrl1: crate::Reg<presetctrl1::PRESETCTRL1_SPEC>,
    #[doc = "0x108 - Peripheral reset control 2"]
    pub presetctrl2: crate::Reg<presetctrl2::PRESETCTRL2_SPEC>,
    _reserved9: [u8; 20usize],
    #[doc = "0x120 - Peripheral reset control set register"]
    pub presetctrlset: [crate::Reg<presetctrlset::PRESETCTRLSET_SPEC>; 3],
    _reserved10: [u8; 20usize],
    #[doc = "0x140 - Peripheral reset control clear register"]
    pub presetctrlclr: [crate::Reg<presetctrlclr::PRESETCTRLCLR_SPEC>; 3],
    _reserved11: [u8; 20usize],
    #[doc = "0x160 - generate a software_reset"]
    pub swr_reset: crate::Reg<swr_reset::SWR_RESET_SPEC>,
    _reserved12: [u8; 156usize],
    #[doc = "0x200 - AHB Clock control 0"]
    pub ahbclkctrl0: crate::Reg<ahbclkctrl0::AHBCLKCTRL0_SPEC>,
    #[doc = "0x204 - AHB Clock control 1"]
    pub ahbclkctrl1: crate::Reg<ahbclkctrl1::AHBCLKCTRL1_SPEC>,
    #[doc = "0x208 - AHB Clock control 2"]
    pub ahbclkctrl2: crate::Reg<ahbclkctrl2::AHBCLKCTRL2_SPEC>,
    _reserved15: [u8; 20usize],
    #[doc = "0x220 - Peripheral reset control register"]
    pub ahbclkctrlset: [crate::Reg<ahbclkctrlset::AHBCLKCTRLSET_SPEC>; 3],
    _reserved16: [u8; 20usize],
    #[doc = "0x240 - Peripheral reset control register"]
    pub ahbclkctrlclr: [crate::Reg<ahbclkctrlclr::AHBCLKCTRLCLR_SPEC>; 3],
    _reserved17: [u8; 20usize],
    _reserved_17_systickclksel0: [u8; 4usize],
    _reserved_18_systickclksel1: [u8; 4usize],
    #[doc = "0x268 - Trace clock source select"]
    pub traceclksel: crate::Reg<traceclksel::TRACECLKSEL_SPEC>,
    _reserved_20_ctimerclksel0: [u8; 4usize],
    _reserved_21_ctimerclksel1: [u8; 4usize],
    _reserved_22_ctimerclksel2: [u8; 4usize],
    _reserved_23_ctimerclksel3: [u8; 4usize],
    _reserved_24_ctimerclksel4: [u8; 4usize],
    #[doc = "0x280 - Main clock A source select"]
    pub mainclksela: crate::Reg<mainclksela::MAINCLKSELA_SPEC>,
    #[doc = "0x284 - Main clock source select"]
    pub mainclkselb: crate::Reg<mainclkselb::MAINCLKSELB_SPEC>,
    #[doc = "0x288 - CLKOUT clock source select"]
    pub clkoutsel: crate::Reg<clkoutsel::CLKOUTSEL_SPEC>,
    _reserved28: [u8; 4usize],
    #[doc = "0x290 - PLL0 clock source select"]
    pub pll0clksel: crate::Reg<pll0clksel::PLL0CLKSEL_SPEC>,
    #[doc = "0x294 - PLL1 clock source select"]
    pub pll1clksel: crate::Reg<pll1clksel::PLL1CLKSEL_SPEC>,
    _reserved30: [u8; 12usize],
    #[doc = "0x2a4 - ADC clock source select"]
    pub adcclksel: crate::Reg<adcclksel::ADCCLKSEL_SPEC>,
    #[doc = "0x2a8 - FS USB clock source select"]
    pub usb0clksel: crate::Reg<usb0clksel::USB0CLKSEL_SPEC>,
    _reserved32: [u8; 4usize],
    _reserved_32_fcclksel0: [u8; 4usize],
    _reserved_33_fcclksel1: [u8; 4usize],
    _reserved_34_fcclksel2: [u8; 4usize],
    _reserved_35_fcclksel3: [u8; 4usize],
    _reserved_36_fcclksel4: [u8; 4usize],
    _reserved_37_fcclksel5: [u8; 4usize],
    _reserved_38_fcclksel6: [u8; 4usize],
    _reserved_39_fcclksel7: [u8; 4usize],
    #[doc = "0x2d0 - HS LSPI clock source select"]
    pub hslspiclksel: crate::Reg<hslspiclksel::HSLSPICLKSEL_SPEC>,
    _reserved41: [u8; 12usize],
    #[doc = "0x2e0 - MCLK clock source select"]
    pub mclkclksel: crate::Reg<mclkclksel::MCLKCLKSEL_SPEC>,
    _reserved42: [u8; 12usize],
    #[doc = "0x2f0 - SCTimer/PWM clock source select"]
    pub sctclksel: crate::Reg<sctclksel::SCTCLKSEL_SPEC>,
    _reserved43: [u8; 4usize],
    #[doc = "0x2f8 - SDIO clock source select"]
    pub sdioclksel: crate::Reg<sdioclksel::SDIOCLKSEL_SPEC>,
    _reserved44: [u8; 4usize],
    #[doc = "0x300 - System Tick Timer divider for CPU0"]
    pub systickclkdiv0: crate::Reg<systickclkdiv0::SYSTICKCLKDIV0_SPEC>,
    #[doc = "0x304 - System Tick Timer divider for CPU1"]
    pub systickclkdiv1: crate::Reg<systickclkdiv1::SYSTICKCLKDIV1_SPEC>,
    #[doc = "0x308 - TRACE clock divider"]
    pub traceclkdiv: crate::Reg<traceclkdiv::TRACECLKDIV_SPEC>,
    _reserved47: [u8; 20usize],
    _reserved_47_flexfrg0ctrl: [u8; 4usize],
    _reserved_48_flexfrg1ctrl: [u8; 4usize],
    _reserved_49_flexfrg2ctrl: [u8; 4usize],
    _reserved_50_flexfrg3ctrl: [u8; 4usize],
    _reserved_51_flexfrg4ctrl: [u8; 4usize],
    _reserved_52_flexfrg5ctrl: [u8; 4usize],
    _reserved_53_flexfrg6ctrl: [u8; 4usize],
    _reserved_54_flexfrg7ctrl: [u8; 4usize],
    _reserved55: [u8; 64usize],
    #[doc = "0x380 - System clock divider"]
    pub ahbclkdiv: crate::Reg<ahbclkdiv::AHBCLKDIV_SPEC>,
    #[doc = "0x384 - CLKOUT clock divider"]
    pub clkoutdiv: crate::Reg<clkoutdiv::CLKOUTDIV_SPEC>,
    #[doc = "0x388 - FRO_HF (96MHz) clock divider"]
    pub frohfdiv: crate::Reg<frohfdiv::FROHFDIV_SPEC>,
    #[doc = "0x38c - WDT clock divider"]
    pub wdtclkdiv: crate::Reg<wdtclkdiv::WDTCLKDIV_SPEC>,
    _reserved59: [u8; 4usize],
    #[doc = "0x394 - ADC clock divider"]
    pub adcclkdiv: crate::Reg<adcclkdiv::ADCCLKDIV_SPEC>,
    #[doc = "0x398 - USB0 Clock divider"]
    pub usb0clkdiv: crate::Reg<usb0clkdiv::USB0CLKDIV_SPEC>,
    _reserved61: [u8; 16usize],
    #[doc = "0x3ac - I2S MCLK clock divider"]
    pub mclkdiv: crate::Reg<mclkdiv::MCLKDIV_SPEC>,
    _reserved62: [u8; 4usize],
    #[doc = "0x3b4 - SCT/PWM clock divider"]
    pub sctclkdiv: crate::Reg<sctclkdiv::SCTCLKDIV_SPEC>,
    _reserved63: [u8; 4usize],
    #[doc = "0x3bc - SDIO clock divider"]
    pub sdioclkdiv: crate::Reg<sdioclkdiv::SDIOCLKDIV_SPEC>,
    _reserved64: [u8; 4usize],
    #[doc = "0x3c4 - PLL0 clock divider"]
    pub pll0clkdiv: crate::Reg<pll0clkdiv::PLL0CLKDIV_SPEC>,
    _reserved65: [u8; 52usize],
    #[doc = "0x3fc - Control clock configuration registers access (like xxxDIV, xxxSEL)"]
    pub clockgenupdatelockout: crate::Reg<clockgenupdatelockout::CLOCKGENUPDATELOCKOUT_SPEC>,
    #[doc = "0x400 - FMC configuration register"]
    pub fmccr: crate::Reg<fmccr::FMCCR_SPEC>,
    _reserved67: [u8; 8usize],
    #[doc = "0x40c - USB0 need clock control"]
    pub usb0needclkctrl: crate::Reg<usb0needclkctrl::USB0NEEDCLKCTRL_SPEC>,
    #[doc = "0x410 - USB0 need clock status"]
    pub usb0needclkstat: crate::Reg<usb0needclkstat::USB0NEEDCLKSTAT_SPEC>,
    _reserved69: [u8; 8usize],
    #[doc = "0x41c - FMCflush control"]
    pub fmcflush: crate::Reg<fmcflush::FMCFLUSH_SPEC>,
    #[doc = "0x420 - MCLK control"]
    pub mclkio: crate::Reg<mclkio::MCLKIO_SPEC>,
    #[doc = "0x424 - USB1 need clock control"]
    pub usb1needclkctrl: crate::Reg<usb1needclkctrl::USB1NEEDCLKCTRL_SPEC>,
    #[doc = "0x428 - USB1 need clock status"]
    pub usb1needclkstat: crate::Reg<usb1needclkstat::USB1NEEDCLKSTAT_SPEC>,
    _reserved73: [u8; 52usize],
    #[doc = "0x460 - SDIO CCLKIN phase and delay control"]
    pub sdioclkctrl: crate::Reg<sdioclkctrl::SDIOCLKCTRL_SPEC>,
    _reserved74: [u8; 252usize],
    #[doc = "0x560 - PLL1 550m control"]
    pub pll1ctrl: crate::Reg<pll1ctrl::PLL1CTRL_SPEC>,
    #[doc = "0x564 - PLL1 550m status"]
    pub pll1stat: crate::Reg<pll1stat::PLL1STAT_SPEC>,
    #[doc = "0x568 - PLL1 550m N divider"]
    pub pll1ndec: crate::Reg<pll1ndec::PLL1NDEC_SPEC>,
    #[doc = "0x56c - PLL1 550m M divider"]
    pub pll1mdec: crate::Reg<pll1mdec::PLL1MDEC_SPEC>,
    #[doc = "0x570 - PLL1 550m P divider"]
    pub pll1pdec: crate::Reg<pll1pdec::PLL1PDEC_SPEC>,
    _reserved79: [u8; 12usize],
    #[doc = "0x580 - PLL0 550m control"]
    pub pll0ctrl: crate::Reg<pll0ctrl::PLL0CTRL_SPEC>,
    #[doc = "0x584 - PLL0 550m status"]
    pub pll0stat: crate::Reg<pll0stat::PLL0STAT_SPEC>,
    #[doc = "0x588 - PLL0 550m N divider"]
    pub pll0ndec: crate::Reg<pll0ndec::PLL0NDEC_SPEC>,
    #[doc = "0x58c - PLL0 550m P divider"]
    pub pll0pdec: crate::Reg<pll0pdec::PLL0PDEC_SPEC>,
    #[doc = "0x590 - PLL0 Spread Spectrum Wrapper control register 0"]
    pub pll0sscg0: crate::Reg<pll0sscg0::PLL0SSCG0_SPEC>,
    #[doc = "0x594 - PLL0 Spread Spectrum Wrapper control register 1"]
    pub pll0sscg1: crate::Reg<pll0sscg1::PLL0SSCG1_SPEC>,
    _reserved85: [u8; 616usize],
    #[doc = "0x800 - CPU Control for multiple processors"]
    pub cpuctrl: crate::Reg<cpuctrl::CPUCTRL_SPEC>,
    #[doc = "0x804 - Coprocessor Boot Address"]
    pub cpboot: crate::Reg<cpboot::CPBOOT_SPEC>,
    _reserved87: [u8; 4usize],
    #[doc = "0x80c - CPU Status"]
    pub cpstat: crate::Reg<cpstat::CPSTAT_SPEC>,
    _reserved88: [u8; 520usize],
    #[doc = "0xa18 - Various system clock controls : Flash clock (48 MHz) control, clocks to Frequency Measures"]
    pub clock_ctrl: crate::Reg<clock_ctrl::CLOCK_CTRL_SPEC>,
    _reserved89: [u8; 244usize],
    #[doc = "0xb10 - Comparator Interrupt control"]
    pub comp_int_ctrl: crate::Reg<comp_int_ctrl::COMP_INT_CTRL_SPEC>,
    #[doc = "0xb14 - Comparator Interrupt status"]
    pub comp_int_status: crate::Reg<comp_int_status::COMP_INT_STATUS_SPEC>,
    _reserved91: [u8; 748usize],
    #[doc = "0xe04 - Control automatic clock gating"]
    pub autoclkgateoverride: crate::Reg<autoclkgateoverride::AUTOCLKGATEOVERRIDE_SPEC>,
    #[doc = "0xe08 - Enable bypass of the first stage of synchonization inside GPIO_INT module"]
    pub gpiopsync: crate::Reg<gpiopsync::GPIOPSYNC_SPEC>,
    _reserved93: [u8; 404usize],
    #[doc = "0xfa0 - Control write access to security registers."]
    pub debug_lock_en: crate::Reg<debug_lock_en::DEBUG_LOCK_EN_SPEC>,
    #[doc = "0xfa4 - Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control."]
    pub debug_features: crate::Reg<debug_features::DEBUG_FEATURES_SPEC>,
    #[doc = "0xfa8 - Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control DUPLICATE register."]
    pub debug_features_dp: crate::Reg<debug_features_dp::DEBUG_FEATURES_DP_SPEC>,
    _reserved96: [u8; 16usize],
    #[doc = "0xfbc - block quiddikey/PUF all index."]
    pub key_block: crate::Reg<key_block::KEY_BLOCK_SPEC>,
    #[doc = "0xfc0 - Debug authentication BEACON register"]
    pub debug_auth_beacon: crate::Reg<debug_auth_beacon::DEBUG_AUTH_BEACON_SPEC>,
    _reserved98: [u8; 16usize],
    #[doc = "0xfd4 - CPUs configuration register"]
    pub cpucfg: crate::Reg<cpucfg::CPUCFG_SPEC>,
    _reserved99: [u8; 32usize],
    #[doc = "0xff8 - Device ID"]
    pub device_id0: crate::Reg<device_id0::DEVICE_ID0_SPEC>,
    #[doc = "0xffc - Chip revision ID and Number"]
    pub dieid: crate::Reg<dieid::DIEID_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x260 - Peripheral reset control register"]
    #[inline(always)]
    pub fn systickclkselx0(&self) -> &crate::Reg<systickclkselx0::SYSTICKCLKSELX0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(608usize)
                as *const crate::Reg<systickclkselx0::SYSTICKCLKSELX0_SPEC>)
        }
    }
    #[doc = "0x260 - System Tick Timer for CPU0 source select"]
    #[inline(always)]
    pub fn systickclksel0(&self) -> &crate::Reg<systickclksel0::SYSTICKCLKSEL0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(608usize)
                as *const crate::Reg<systickclksel0::SYSTICKCLKSEL0_SPEC>)
        }
    }
    #[doc = "0x264 - Peripheral reset control register"]
    #[inline(always)]
    pub fn systickclkselx1(&self) -> &crate::Reg<systickclkselx1::SYSTICKCLKSELX1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(612usize)
                as *const crate::Reg<systickclkselx1::SYSTICKCLKSELX1_SPEC>)
        }
    }
    #[doc = "0x264 - System Tick Timer for CPU1 source select"]
    #[inline(always)]
    pub fn systickclksel1(&self) -> &crate::Reg<systickclksel1::SYSTICKCLKSEL1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(612usize)
                as *const crate::Reg<systickclksel1::SYSTICKCLKSEL1_SPEC>)
        }
    }
    #[doc = "0x26c - Peripheral reset control register"]
    #[inline(always)]
    pub fn ctimerclkselx0(&self) -> &crate::Reg<ctimerclkselx0::CTIMERCLKSELX0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(620usize)
                as *const crate::Reg<ctimerclkselx0::CTIMERCLKSELX0_SPEC>)
        }
    }
    #[doc = "0x26c - CTimer 0 clock source select"]
    #[inline(always)]
    pub fn ctimerclksel0(&self) -> &crate::Reg<ctimerclksel0::CTIMERCLKSEL0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(620usize)
                as *const crate::Reg<ctimerclksel0::CTIMERCLKSEL0_SPEC>)
        }
    }
    #[doc = "0x270 - Peripheral reset control register"]
    #[inline(always)]
    pub fn ctimerclkselx1(&self) -> &crate::Reg<ctimerclkselx1::CTIMERCLKSELX1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(624usize)
                as *const crate::Reg<ctimerclkselx1::CTIMERCLKSELX1_SPEC>)
        }
    }
    #[doc = "0x270 - CTimer 1 clock source select"]
    #[inline(always)]
    pub fn ctimerclksel1(&self) -> &crate::Reg<ctimerclksel1::CTIMERCLKSEL1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(624usize)
                as *const crate::Reg<ctimerclksel1::CTIMERCLKSEL1_SPEC>)
        }
    }
    #[doc = "0x274 - Peripheral reset control register"]
    #[inline(always)]
    pub fn ctimerclkselx2(&self) -> &crate::Reg<ctimerclkselx2::CTIMERCLKSELX2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(628usize)
                as *const crate::Reg<ctimerclkselx2::CTIMERCLKSELX2_SPEC>)
        }
    }
    #[doc = "0x274 - CTimer 2 clock source select"]
    #[inline(always)]
    pub fn ctimerclksel2(&self) -> &crate::Reg<ctimerclksel2::CTIMERCLKSEL2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(628usize)
                as *const crate::Reg<ctimerclksel2::CTIMERCLKSEL2_SPEC>)
        }
    }
    #[doc = "0x278 - Peripheral reset control register"]
    #[inline(always)]
    pub fn ctimerclkselx3(&self) -> &crate::Reg<ctimerclkselx3::CTIMERCLKSELX3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(632usize)
                as *const crate::Reg<ctimerclkselx3::CTIMERCLKSELX3_SPEC>)
        }
    }
    #[doc = "0x278 - CTimer 3 clock source select"]
    #[inline(always)]
    pub fn ctimerclksel3(&self) -> &crate::Reg<ctimerclksel3::CTIMERCLKSEL3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(632usize)
                as *const crate::Reg<ctimerclksel3::CTIMERCLKSEL3_SPEC>)
        }
    }
    #[doc = "0x27c - Peripheral reset control register"]
    #[inline(always)]
    pub fn ctimerclkselx4(&self) -> &crate::Reg<ctimerclkselx4::CTIMERCLKSELX4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(636usize)
                as *const crate::Reg<ctimerclkselx4::CTIMERCLKSELX4_SPEC>)
        }
    }
    #[doc = "0x27c - CTimer 4 clock source select"]
    #[inline(always)]
    pub fn ctimerclksel4(&self) -> &crate::Reg<ctimerclksel4::CTIMERCLKSEL4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(636usize)
                as *const crate::Reg<ctimerclksel4::CTIMERCLKSEL4_SPEC>)
        }
    }
    #[doc = "0x2b0 - Peripheral reset control register"]
    #[inline(always)]
    pub fn fcclkselx0(&self) -> &crate::Reg<fcclkselx0::FCCLKSELX0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(688usize)
                as *const crate::Reg<fcclkselx0::FCCLKSELX0_SPEC>)
        }
    }
    #[doc = "0x2b0 - Flexcomm Interface 0 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub fn fcclksel0(&self) -> &crate::Reg<fcclksel0::FCCLKSEL0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(688usize)
                as *const crate::Reg<fcclksel0::FCCLKSEL0_SPEC>)
        }
    }
    #[doc = "0x2b4 - Peripheral reset control register"]
    #[inline(always)]
    pub fn fcclkselx1(&self) -> &crate::Reg<fcclkselx1::FCCLKSELX1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(692usize)
                as *const crate::Reg<fcclkselx1::FCCLKSELX1_SPEC>)
        }
    }
    #[doc = "0x2b4 - Flexcomm Interface 1 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub fn fcclksel1(&self) -> &crate::Reg<fcclksel1::FCCLKSEL1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(692usize)
                as *const crate::Reg<fcclksel1::FCCLKSEL1_SPEC>)
        }
    }
    #[doc = "0x2b8 - Peripheral reset control register"]
    #[inline(always)]
    pub fn fcclkselx2(&self) -> &crate::Reg<fcclkselx2::FCCLKSELX2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(696usize)
                as *const crate::Reg<fcclkselx2::FCCLKSELX2_SPEC>)
        }
    }
    #[doc = "0x2b8 - Flexcomm Interface 2 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub fn fcclksel2(&self) -> &crate::Reg<fcclksel2::FCCLKSEL2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(696usize)
                as *const crate::Reg<fcclksel2::FCCLKSEL2_SPEC>)
        }
    }
    #[doc = "0x2bc - Peripheral reset control register"]
    #[inline(always)]
    pub fn fcclkselx3(&self) -> &crate::Reg<fcclkselx3::FCCLKSELX3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(700usize)
                as *const crate::Reg<fcclkselx3::FCCLKSELX3_SPEC>)
        }
    }
    #[doc = "0x2bc - Flexcomm Interface 3 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub fn fcclksel3(&self) -> &crate::Reg<fcclksel3::FCCLKSEL3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(700usize)
                as *const crate::Reg<fcclksel3::FCCLKSEL3_SPEC>)
        }
    }
    #[doc = "0x2c0 - Peripheral reset control register"]
    #[inline(always)]
    pub fn fcclkselx4(&self) -> &crate::Reg<fcclkselx4::FCCLKSELX4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(704usize)
                as *const crate::Reg<fcclkselx4::FCCLKSELX4_SPEC>)
        }
    }
    #[doc = "0x2c0 - Flexcomm Interface 4 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub fn fcclksel4(&self) -> &crate::Reg<fcclksel4::FCCLKSEL4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(704usize)
                as *const crate::Reg<fcclksel4::FCCLKSEL4_SPEC>)
        }
    }
    #[doc = "0x2c4 - Peripheral reset control register"]
    #[inline(always)]
    pub fn fcclkselx5(&self) -> &crate::Reg<fcclkselx5::FCCLKSELX5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(708usize)
                as *const crate::Reg<fcclkselx5::FCCLKSELX5_SPEC>)
        }
    }
    #[doc = "0x2c4 - Flexcomm Interface 5 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub fn fcclksel5(&self) -> &crate::Reg<fcclksel5::FCCLKSEL5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(708usize)
                as *const crate::Reg<fcclksel5::FCCLKSEL5_SPEC>)
        }
    }
    #[doc = "0x2c8 - Peripheral reset control register"]
    #[inline(always)]
    pub fn fcclkselx6(&self) -> &crate::Reg<fcclkselx6::FCCLKSELX6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(712usize)
                as *const crate::Reg<fcclkselx6::FCCLKSELX6_SPEC>)
        }
    }
    #[doc = "0x2c8 - Flexcomm Interface 6 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub fn fcclksel6(&self) -> &crate::Reg<fcclksel6::FCCLKSEL6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(712usize)
                as *const crate::Reg<fcclksel6::FCCLKSEL6_SPEC>)
        }
    }
    #[doc = "0x2cc - Peripheral reset control register"]
    #[inline(always)]
    pub fn fcclkselx7(&self) -> &crate::Reg<fcclkselx7::FCCLKSELX7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(716usize)
                as *const crate::Reg<fcclkselx7::FCCLKSELX7_SPEC>)
        }
    }
    #[doc = "0x2cc - Flexcomm Interface 7 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub fn fcclksel7(&self) -> &crate::Reg<fcclksel7::FCCLKSEL7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(716usize)
                as *const crate::Reg<fcclksel7::FCCLKSEL7_SPEC>)
        }
    }
    #[doc = "0x320 - Peripheral reset control register"]
    #[inline(always)]
    pub fn flexfrgxctrl0(&self) -> &crate::Reg<flexfrgxctrl0::FLEXFRGXCTRL0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(800usize)
                as *const crate::Reg<flexfrgxctrl0::FLEXFRGXCTRL0_SPEC>)
        }
    }
    #[doc = "0x320 - Fractional rate divider for flexcomm 0"]
    #[inline(always)]
    pub fn flexfrg0ctrl(&self) -> &crate::Reg<flexfrg0ctrl::FLEXFRG0CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(800usize)
                as *const crate::Reg<flexfrg0ctrl::FLEXFRG0CTRL_SPEC>)
        }
    }
    #[doc = "0x324 - Peripheral reset control register"]
    #[inline(always)]
    pub fn flexfrgxctrl1(&self) -> &crate::Reg<flexfrgxctrl1::FLEXFRGXCTRL1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(804usize)
                as *const crate::Reg<flexfrgxctrl1::FLEXFRGXCTRL1_SPEC>)
        }
    }
    #[doc = "0x324 - Fractional rate divider for flexcomm 1"]
    #[inline(always)]
    pub fn flexfrg1ctrl(&self) -> &crate::Reg<flexfrg1ctrl::FLEXFRG1CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(804usize)
                as *const crate::Reg<flexfrg1ctrl::FLEXFRG1CTRL_SPEC>)
        }
    }
    #[doc = "0x328 - Peripheral reset control register"]
    #[inline(always)]
    pub fn flexfrgxctrl2(&self) -> &crate::Reg<flexfrgxctrl2::FLEXFRGXCTRL2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(808usize)
                as *const crate::Reg<flexfrgxctrl2::FLEXFRGXCTRL2_SPEC>)
        }
    }
    #[doc = "0x328 - Fractional rate divider for flexcomm 2"]
    #[inline(always)]
    pub fn flexfrg2ctrl(&self) -> &crate::Reg<flexfrg2ctrl::FLEXFRG2CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(808usize)
                as *const crate::Reg<flexfrg2ctrl::FLEXFRG2CTRL_SPEC>)
        }
    }
    #[doc = "0x32c - Peripheral reset control register"]
    #[inline(always)]
    pub fn flexfrgxctrl3(&self) -> &crate::Reg<flexfrgxctrl3::FLEXFRGXCTRL3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(812usize)
                as *const crate::Reg<flexfrgxctrl3::FLEXFRGXCTRL3_SPEC>)
        }
    }
    #[doc = "0x32c - Fractional rate divider for flexcomm 3"]
    #[inline(always)]
    pub fn flexfrg3ctrl(&self) -> &crate::Reg<flexfrg3ctrl::FLEXFRG3CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(812usize)
                as *const crate::Reg<flexfrg3ctrl::FLEXFRG3CTRL_SPEC>)
        }
    }
    #[doc = "0x330 - Peripheral reset control register"]
    #[inline(always)]
    pub fn flexfrgxctrl4(&self) -> &crate::Reg<flexfrgxctrl4::FLEXFRGXCTRL4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(816usize)
                as *const crate::Reg<flexfrgxctrl4::FLEXFRGXCTRL4_SPEC>)
        }
    }
    #[doc = "0x330 - Fractional rate divider for flexcomm 4"]
    #[inline(always)]
    pub fn flexfrg4ctrl(&self) -> &crate::Reg<flexfrg4ctrl::FLEXFRG4CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(816usize)
                as *const crate::Reg<flexfrg4ctrl::FLEXFRG4CTRL_SPEC>)
        }
    }
    #[doc = "0x334 - Peripheral reset control register"]
    #[inline(always)]
    pub fn flexfrgxctrl5(&self) -> &crate::Reg<flexfrgxctrl5::FLEXFRGXCTRL5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(820usize)
                as *const crate::Reg<flexfrgxctrl5::FLEXFRGXCTRL5_SPEC>)
        }
    }
    #[doc = "0x334 - Fractional rate divider for flexcomm 5"]
    #[inline(always)]
    pub fn flexfrg5ctrl(&self) -> &crate::Reg<flexfrg5ctrl::FLEXFRG5CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(820usize)
                as *const crate::Reg<flexfrg5ctrl::FLEXFRG5CTRL_SPEC>)
        }
    }
    #[doc = "0x338 - Peripheral reset control register"]
    #[inline(always)]
    pub fn flexfrgxctrl6(&self) -> &crate::Reg<flexfrgxctrl6::FLEXFRGXCTRL6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(824usize)
                as *const crate::Reg<flexfrgxctrl6::FLEXFRGXCTRL6_SPEC>)
        }
    }
    #[doc = "0x338 - Fractional rate divider for flexcomm 6"]
    #[inline(always)]
    pub fn flexfrg6ctrl(&self) -> &crate::Reg<flexfrg6ctrl::FLEXFRG6CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(824usize)
                as *const crate::Reg<flexfrg6ctrl::FLEXFRG6CTRL_SPEC>)
        }
    }
    #[doc = "0x33c - Peripheral reset control register"]
    #[inline(always)]
    pub fn flexfrgxctrl7(&self) -> &crate::Reg<flexfrgxctrl7::FLEXFRGXCTRL7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(828usize)
                as *const crate::Reg<flexfrgxctrl7::FLEXFRGXCTRL7_SPEC>)
        }
    }
    #[doc = "0x33c - Fractional rate divider for flexcomm 7"]
    #[inline(always)]
    pub fn flexfrg7ctrl(&self) -> &crate::Reg<flexfrg7ctrl::FLEXFRG7CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(828usize)
                as *const crate::Reg<flexfrg7ctrl::FLEXFRG7CTRL_SPEC>)
        }
    }
}
#[doc = "MEMORYREMAP register accessor: an alias for `Reg<MEMORYREMAP_SPEC>`"]
pub type MEMORYREMAP = crate::Reg<memoryremap::MEMORYREMAP_SPEC>;
#[doc = "Memory Remap control register"]
pub mod memoryremap;
#[doc = "AHBMATPRIO register accessor: an alias for `Reg<AHBMATPRIO_SPEC>`"]
pub type AHBMATPRIO = crate::Reg<ahbmatprio::AHBMATPRIO_SPEC>;
#[doc = "AHB Matrix priority control register Priority values are 3 = highest, 0 = lowest"]
pub mod ahbmatprio;
#[doc = "CPU0STCKCAL register accessor: an alias for `Reg<CPU0STCKCAL_SPEC>`"]
pub type CPU0STCKCAL = crate::Reg<cpu0stckcal::CPU0STCKCAL_SPEC>;
#[doc = "System tick calibration for secure part of CPU0"]
pub mod cpu0stckcal;
#[doc = "CPU0NSTCKCAL register accessor: an alias for `Reg<CPU0NSTCKCAL_SPEC>`"]
pub type CPU0NSTCKCAL = crate::Reg<cpu0nstckcal::CPU0NSTCKCAL_SPEC>;
#[doc = "System tick calibration for non-secure part of CPU0"]
pub mod cpu0nstckcal;
#[doc = "CPU1STCKCAL register accessor: an alias for `Reg<CPU1STCKCAL_SPEC>`"]
pub type CPU1STCKCAL = crate::Reg<cpu1stckcal::CPU1STCKCAL_SPEC>;
#[doc = "System tick calibration for CPU1"]
pub mod cpu1stckcal;
#[doc = "NMISRC register accessor: an alias for `Reg<NMISRC_SPEC>`"]
pub type NMISRC = crate::Reg<nmisrc::NMISRC_SPEC>;
#[doc = "NMI Source Select"]
pub mod nmisrc;
#[doc = "PRESETCTRL0 register accessor: an alias for `Reg<PRESETCTRL0_SPEC>`"]
pub type PRESETCTRL0 = crate::Reg<presetctrl0::PRESETCTRL0_SPEC>;
#[doc = "Peripheral reset control 0"]
pub mod presetctrl0;
#[doc = "PRESETCTRL1 register accessor: an alias for `Reg<PRESETCTRL1_SPEC>`"]
pub type PRESETCTRL1 = crate::Reg<presetctrl1::PRESETCTRL1_SPEC>;
#[doc = "Peripheral reset control 1"]
pub mod presetctrl1;
#[doc = "PRESETCTRL2 register accessor: an alias for `Reg<PRESETCTRL2_SPEC>`"]
pub type PRESETCTRL2 = crate::Reg<presetctrl2::PRESETCTRL2_SPEC>;
#[doc = "Peripheral reset control 2"]
pub mod presetctrl2;
#[doc = "PRESETCTRLSET register accessor: an alias for `Reg<PRESETCTRLSET_SPEC>`"]
pub type PRESETCTRLSET = crate::Reg<presetctrlset::PRESETCTRLSET_SPEC>;
#[doc = "Peripheral reset control set register"]
pub mod presetctrlset;
#[doc = "PRESETCTRLCLR register accessor: an alias for `Reg<PRESETCTRLCLR_SPEC>`"]
pub type PRESETCTRLCLR = crate::Reg<presetctrlclr::PRESETCTRLCLR_SPEC>;
#[doc = "Peripheral reset control clear register"]
pub mod presetctrlclr;
#[doc = "SWR_RESET register accessor: an alias for `Reg<SWR_RESET_SPEC>`"]
pub type SWR_RESET = crate::Reg<swr_reset::SWR_RESET_SPEC>;
#[doc = "generate a software_reset"]
pub mod swr_reset;
#[doc = "AHBCLKCTRL0 register accessor: an alias for `Reg<AHBCLKCTRL0_SPEC>`"]
pub type AHBCLKCTRL0 = crate::Reg<ahbclkctrl0::AHBCLKCTRL0_SPEC>;
#[doc = "AHB Clock control 0"]
pub mod ahbclkctrl0;
#[doc = "AHBCLKCTRL1 register accessor: an alias for `Reg<AHBCLKCTRL1_SPEC>`"]
pub type AHBCLKCTRL1 = crate::Reg<ahbclkctrl1::AHBCLKCTRL1_SPEC>;
#[doc = "AHB Clock control 1"]
pub mod ahbclkctrl1;
#[doc = "AHBCLKCTRL2 register accessor: an alias for `Reg<AHBCLKCTRL2_SPEC>`"]
pub type AHBCLKCTRL2 = crate::Reg<ahbclkctrl2::AHBCLKCTRL2_SPEC>;
#[doc = "AHB Clock control 2"]
pub mod ahbclkctrl2;
#[doc = "AHBCLKCTRLSET register accessor: an alias for `Reg<AHBCLKCTRLSET_SPEC>`"]
pub type AHBCLKCTRLSET = crate::Reg<ahbclkctrlset::AHBCLKCTRLSET_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod ahbclkctrlset;
#[doc = "AHBCLKCTRLCLR register accessor: an alias for `Reg<AHBCLKCTRLCLR_SPEC>`"]
pub type AHBCLKCTRLCLR = crate::Reg<ahbclkctrlclr::AHBCLKCTRLCLR_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod ahbclkctrlclr;
#[doc = "SYSTICKCLKSEL0 register accessor: an alias for `Reg<SYSTICKCLKSEL0_SPEC>`"]
pub type SYSTICKCLKSEL0 = crate::Reg<systickclksel0::SYSTICKCLKSEL0_SPEC>;
#[doc = "System Tick Timer for CPU0 source select"]
pub mod systickclksel0;
#[doc = "SYSTICKCLKSELX0 register accessor: an alias for `Reg<SYSTICKCLKSELX0_SPEC>`"]
pub type SYSTICKCLKSELX0 = crate::Reg<systickclkselx0::SYSTICKCLKSELX0_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod systickclkselx0;
#[doc = "SYSTICKCLKSEL1 register accessor: an alias for `Reg<SYSTICKCLKSEL1_SPEC>`"]
pub type SYSTICKCLKSEL1 = crate::Reg<systickclksel1::SYSTICKCLKSEL1_SPEC>;
#[doc = "System Tick Timer for CPU1 source select"]
pub mod systickclksel1;
#[doc = "SYSTICKCLKSELX1 register accessor: an alias for `Reg<SYSTICKCLKSELX1_SPEC>`"]
pub type SYSTICKCLKSELX1 = crate::Reg<systickclkselx1::SYSTICKCLKSELX1_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod systickclkselx1;
#[doc = "TRACECLKSEL register accessor: an alias for `Reg<TRACECLKSEL_SPEC>`"]
pub type TRACECLKSEL = crate::Reg<traceclksel::TRACECLKSEL_SPEC>;
#[doc = "Trace clock source select"]
pub mod traceclksel;
#[doc = "CTIMERCLKSEL0 register accessor: an alias for `Reg<CTIMERCLKSEL0_SPEC>`"]
pub type CTIMERCLKSEL0 = crate::Reg<ctimerclksel0::CTIMERCLKSEL0_SPEC>;
#[doc = "CTimer 0 clock source select"]
pub mod ctimerclksel0;
#[doc = "CTIMERCLKSELX0 register accessor: an alias for `Reg<CTIMERCLKSELX0_SPEC>`"]
pub type CTIMERCLKSELX0 = crate::Reg<ctimerclkselx0::CTIMERCLKSELX0_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod ctimerclkselx0;
#[doc = "CTIMERCLKSEL1 register accessor: an alias for `Reg<CTIMERCLKSEL1_SPEC>`"]
pub type CTIMERCLKSEL1 = crate::Reg<ctimerclksel1::CTIMERCLKSEL1_SPEC>;
#[doc = "CTimer 1 clock source select"]
pub mod ctimerclksel1;
#[doc = "CTIMERCLKSELX1 register accessor: an alias for `Reg<CTIMERCLKSELX1_SPEC>`"]
pub type CTIMERCLKSELX1 = crate::Reg<ctimerclkselx1::CTIMERCLKSELX1_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod ctimerclkselx1;
#[doc = "CTIMERCLKSEL2 register accessor: an alias for `Reg<CTIMERCLKSEL2_SPEC>`"]
pub type CTIMERCLKSEL2 = crate::Reg<ctimerclksel2::CTIMERCLKSEL2_SPEC>;
#[doc = "CTimer 2 clock source select"]
pub mod ctimerclksel2;
#[doc = "CTIMERCLKSELX2 register accessor: an alias for `Reg<CTIMERCLKSELX2_SPEC>`"]
pub type CTIMERCLKSELX2 = crate::Reg<ctimerclkselx2::CTIMERCLKSELX2_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod ctimerclkselx2;
#[doc = "CTIMERCLKSEL3 register accessor: an alias for `Reg<CTIMERCLKSEL3_SPEC>`"]
pub type CTIMERCLKSEL3 = crate::Reg<ctimerclksel3::CTIMERCLKSEL3_SPEC>;
#[doc = "CTimer 3 clock source select"]
pub mod ctimerclksel3;
#[doc = "CTIMERCLKSELX3 register accessor: an alias for `Reg<CTIMERCLKSELX3_SPEC>`"]
pub type CTIMERCLKSELX3 = crate::Reg<ctimerclkselx3::CTIMERCLKSELX3_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod ctimerclkselx3;
#[doc = "CTIMERCLKSEL4 register accessor: an alias for `Reg<CTIMERCLKSEL4_SPEC>`"]
pub type CTIMERCLKSEL4 = crate::Reg<ctimerclksel4::CTIMERCLKSEL4_SPEC>;
#[doc = "CTimer 4 clock source select"]
pub mod ctimerclksel4;
#[doc = "CTIMERCLKSELX4 register accessor: an alias for `Reg<CTIMERCLKSELX4_SPEC>`"]
pub type CTIMERCLKSELX4 = crate::Reg<ctimerclkselx4::CTIMERCLKSELX4_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod ctimerclkselx4;
#[doc = "MAINCLKSELA register accessor: an alias for `Reg<MAINCLKSELA_SPEC>`"]
pub type MAINCLKSELA = crate::Reg<mainclksela::MAINCLKSELA_SPEC>;
#[doc = "Main clock A source select"]
pub mod mainclksela;
#[doc = "MAINCLKSELB register accessor: an alias for `Reg<MAINCLKSELB_SPEC>`"]
pub type MAINCLKSELB = crate::Reg<mainclkselb::MAINCLKSELB_SPEC>;
#[doc = "Main clock source select"]
pub mod mainclkselb;
#[doc = "CLKOUTSEL register accessor: an alias for `Reg<CLKOUTSEL_SPEC>`"]
pub type CLKOUTSEL = crate::Reg<clkoutsel::CLKOUTSEL_SPEC>;
#[doc = "CLKOUT clock source select"]
pub mod clkoutsel;
#[doc = "PLL0CLKSEL register accessor: an alias for `Reg<PLL0CLKSEL_SPEC>`"]
pub type PLL0CLKSEL = crate::Reg<pll0clksel::PLL0CLKSEL_SPEC>;
#[doc = "PLL0 clock source select"]
pub mod pll0clksel;
#[doc = "PLL1CLKSEL register accessor: an alias for `Reg<PLL1CLKSEL_SPEC>`"]
pub type PLL1CLKSEL = crate::Reg<pll1clksel::PLL1CLKSEL_SPEC>;
#[doc = "PLL1 clock source select"]
pub mod pll1clksel;
#[doc = "ADCCLKSEL register accessor: an alias for `Reg<ADCCLKSEL_SPEC>`"]
pub type ADCCLKSEL = crate::Reg<adcclksel::ADCCLKSEL_SPEC>;
#[doc = "ADC clock source select"]
pub mod adcclksel;
#[doc = "USB0CLKSEL register accessor: an alias for `Reg<USB0CLKSEL_SPEC>`"]
pub type USB0CLKSEL = crate::Reg<usb0clksel::USB0CLKSEL_SPEC>;
#[doc = "FS USB clock source select"]
pub mod usb0clksel;
#[doc = "FCCLKSEL0 register accessor: an alias for `Reg<FCCLKSEL0_SPEC>`"]
pub type FCCLKSEL0 = crate::Reg<fcclksel0::FCCLKSEL0_SPEC>;
#[doc = "Flexcomm Interface 0 clock source select for Fractional Rate Divider"]
pub mod fcclksel0;
#[doc = "FCCLKSELX0 register accessor: an alias for `Reg<FCCLKSELX0_SPEC>`"]
pub type FCCLKSELX0 = crate::Reg<fcclkselx0::FCCLKSELX0_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod fcclkselx0;
#[doc = "FCCLKSEL1 register accessor: an alias for `Reg<FCCLKSEL1_SPEC>`"]
pub type FCCLKSEL1 = crate::Reg<fcclksel1::FCCLKSEL1_SPEC>;
#[doc = "Flexcomm Interface 1 clock source select for Fractional Rate Divider"]
pub mod fcclksel1;
#[doc = "FCCLKSELX1 register accessor: an alias for `Reg<FCCLKSELX1_SPEC>`"]
pub type FCCLKSELX1 = crate::Reg<fcclkselx1::FCCLKSELX1_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod fcclkselx1;
#[doc = "FCCLKSEL2 register accessor: an alias for `Reg<FCCLKSEL2_SPEC>`"]
pub type FCCLKSEL2 = crate::Reg<fcclksel2::FCCLKSEL2_SPEC>;
#[doc = "Flexcomm Interface 2 clock source select for Fractional Rate Divider"]
pub mod fcclksel2;
#[doc = "FCCLKSELX2 register accessor: an alias for `Reg<FCCLKSELX2_SPEC>`"]
pub type FCCLKSELX2 = crate::Reg<fcclkselx2::FCCLKSELX2_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod fcclkselx2;
#[doc = "FCCLKSEL3 register accessor: an alias for `Reg<FCCLKSEL3_SPEC>`"]
pub type FCCLKSEL3 = crate::Reg<fcclksel3::FCCLKSEL3_SPEC>;
#[doc = "Flexcomm Interface 3 clock source select for Fractional Rate Divider"]
pub mod fcclksel3;
#[doc = "FCCLKSELX3 register accessor: an alias for `Reg<FCCLKSELX3_SPEC>`"]
pub type FCCLKSELX3 = crate::Reg<fcclkselx3::FCCLKSELX3_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod fcclkselx3;
#[doc = "FCCLKSEL4 register accessor: an alias for `Reg<FCCLKSEL4_SPEC>`"]
pub type FCCLKSEL4 = crate::Reg<fcclksel4::FCCLKSEL4_SPEC>;
#[doc = "Flexcomm Interface 4 clock source select for Fractional Rate Divider"]
pub mod fcclksel4;
#[doc = "FCCLKSELX4 register accessor: an alias for `Reg<FCCLKSELX4_SPEC>`"]
pub type FCCLKSELX4 = crate::Reg<fcclkselx4::FCCLKSELX4_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod fcclkselx4;
#[doc = "FCCLKSEL5 register accessor: an alias for `Reg<FCCLKSEL5_SPEC>`"]
pub type FCCLKSEL5 = crate::Reg<fcclksel5::FCCLKSEL5_SPEC>;
#[doc = "Flexcomm Interface 5 clock source select for Fractional Rate Divider"]
pub mod fcclksel5;
#[doc = "FCCLKSELX5 register accessor: an alias for `Reg<FCCLKSELX5_SPEC>`"]
pub type FCCLKSELX5 = crate::Reg<fcclkselx5::FCCLKSELX5_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod fcclkselx5;
#[doc = "FCCLKSEL6 register accessor: an alias for `Reg<FCCLKSEL6_SPEC>`"]
pub type FCCLKSEL6 = crate::Reg<fcclksel6::FCCLKSEL6_SPEC>;
#[doc = "Flexcomm Interface 6 clock source select for Fractional Rate Divider"]
pub mod fcclksel6;
#[doc = "FCCLKSELX6 register accessor: an alias for `Reg<FCCLKSELX6_SPEC>`"]
pub type FCCLKSELX6 = crate::Reg<fcclkselx6::FCCLKSELX6_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod fcclkselx6;
#[doc = "FCCLKSEL7 register accessor: an alias for `Reg<FCCLKSEL7_SPEC>`"]
pub type FCCLKSEL7 = crate::Reg<fcclksel7::FCCLKSEL7_SPEC>;
#[doc = "Flexcomm Interface 7 clock source select for Fractional Rate Divider"]
pub mod fcclksel7;
#[doc = "FCCLKSELX7 register accessor: an alias for `Reg<FCCLKSELX7_SPEC>`"]
pub type FCCLKSELX7 = crate::Reg<fcclkselx7::FCCLKSELX7_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod fcclkselx7;
#[doc = "HSLSPICLKSEL register accessor: an alias for `Reg<HSLSPICLKSEL_SPEC>`"]
pub type HSLSPICLKSEL = crate::Reg<hslspiclksel::HSLSPICLKSEL_SPEC>;
#[doc = "HS LSPI clock source select"]
pub mod hslspiclksel;
#[doc = "MCLKCLKSEL register accessor: an alias for `Reg<MCLKCLKSEL_SPEC>`"]
pub type MCLKCLKSEL = crate::Reg<mclkclksel::MCLKCLKSEL_SPEC>;
#[doc = "MCLK clock source select"]
pub mod mclkclksel;
#[doc = "SCTCLKSEL register accessor: an alias for `Reg<SCTCLKSEL_SPEC>`"]
pub type SCTCLKSEL = crate::Reg<sctclksel::SCTCLKSEL_SPEC>;
#[doc = "SCTimer/PWM clock source select"]
pub mod sctclksel;
#[doc = "SDIOCLKSEL register accessor: an alias for `Reg<SDIOCLKSEL_SPEC>`"]
pub type SDIOCLKSEL = crate::Reg<sdioclksel::SDIOCLKSEL_SPEC>;
#[doc = "SDIO clock source select"]
pub mod sdioclksel;
#[doc = "SYSTICKCLKDIV0 register accessor: an alias for `Reg<SYSTICKCLKDIV0_SPEC>`"]
pub type SYSTICKCLKDIV0 = crate::Reg<systickclkdiv0::SYSTICKCLKDIV0_SPEC>;
#[doc = "System Tick Timer divider for CPU0"]
pub mod systickclkdiv0;
#[doc = "SYSTICKCLKDIV1 register accessor: an alias for `Reg<SYSTICKCLKDIV1_SPEC>`"]
pub type SYSTICKCLKDIV1 = crate::Reg<systickclkdiv1::SYSTICKCLKDIV1_SPEC>;
#[doc = "System Tick Timer divider for CPU1"]
pub mod systickclkdiv1;
#[doc = "TRACECLKDIV register accessor: an alias for `Reg<TRACECLKDIV_SPEC>`"]
pub type TRACECLKDIV = crate::Reg<traceclkdiv::TRACECLKDIV_SPEC>;
#[doc = "TRACE clock divider"]
pub mod traceclkdiv;
#[doc = "FLEXFRG0CTRL register accessor: an alias for `Reg<FLEXFRG0CTRL_SPEC>`"]
pub type FLEXFRG0CTRL = crate::Reg<flexfrg0ctrl::FLEXFRG0CTRL_SPEC>;
#[doc = "Fractional rate divider for flexcomm 0"]
pub mod flexfrg0ctrl;
#[doc = "FLEXFRGXCTRL0 register accessor: an alias for `Reg<FLEXFRGXCTRL0_SPEC>`"]
pub type FLEXFRGXCTRL0 = crate::Reg<flexfrgxctrl0::FLEXFRGXCTRL0_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod flexfrgxctrl0;
#[doc = "FLEXFRG1CTRL register accessor: an alias for `Reg<FLEXFRG1CTRL_SPEC>`"]
pub type FLEXFRG1CTRL = crate::Reg<flexfrg1ctrl::FLEXFRG1CTRL_SPEC>;
#[doc = "Fractional rate divider for flexcomm 1"]
pub mod flexfrg1ctrl;
#[doc = "FLEXFRGXCTRL1 register accessor: an alias for `Reg<FLEXFRGXCTRL1_SPEC>`"]
pub type FLEXFRGXCTRL1 = crate::Reg<flexfrgxctrl1::FLEXFRGXCTRL1_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod flexfrgxctrl1;
#[doc = "FLEXFRG2CTRL register accessor: an alias for `Reg<FLEXFRG2CTRL_SPEC>`"]
pub type FLEXFRG2CTRL = crate::Reg<flexfrg2ctrl::FLEXFRG2CTRL_SPEC>;
#[doc = "Fractional rate divider for flexcomm 2"]
pub mod flexfrg2ctrl;
#[doc = "FLEXFRGXCTRL2 register accessor: an alias for `Reg<FLEXFRGXCTRL2_SPEC>`"]
pub type FLEXFRGXCTRL2 = crate::Reg<flexfrgxctrl2::FLEXFRGXCTRL2_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod flexfrgxctrl2;
#[doc = "FLEXFRG3CTRL register accessor: an alias for `Reg<FLEXFRG3CTRL_SPEC>`"]
pub type FLEXFRG3CTRL = crate::Reg<flexfrg3ctrl::FLEXFRG3CTRL_SPEC>;
#[doc = "Fractional rate divider for flexcomm 3"]
pub mod flexfrg3ctrl;
#[doc = "FLEXFRGXCTRL3 register accessor: an alias for `Reg<FLEXFRGXCTRL3_SPEC>`"]
pub type FLEXFRGXCTRL3 = crate::Reg<flexfrgxctrl3::FLEXFRGXCTRL3_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod flexfrgxctrl3;
#[doc = "FLEXFRG4CTRL register accessor: an alias for `Reg<FLEXFRG4CTRL_SPEC>`"]
pub type FLEXFRG4CTRL = crate::Reg<flexfrg4ctrl::FLEXFRG4CTRL_SPEC>;
#[doc = "Fractional rate divider for flexcomm 4"]
pub mod flexfrg4ctrl;
#[doc = "FLEXFRGXCTRL4 register accessor: an alias for `Reg<FLEXFRGXCTRL4_SPEC>`"]
pub type FLEXFRGXCTRL4 = crate::Reg<flexfrgxctrl4::FLEXFRGXCTRL4_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod flexfrgxctrl4;
#[doc = "FLEXFRG5CTRL register accessor: an alias for `Reg<FLEXFRG5CTRL_SPEC>`"]
pub type FLEXFRG5CTRL = crate::Reg<flexfrg5ctrl::FLEXFRG5CTRL_SPEC>;
#[doc = "Fractional rate divider for flexcomm 5"]
pub mod flexfrg5ctrl;
#[doc = "FLEXFRGXCTRL5 register accessor: an alias for `Reg<FLEXFRGXCTRL5_SPEC>`"]
pub type FLEXFRGXCTRL5 = crate::Reg<flexfrgxctrl5::FLEXFRGXCTRL5_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod flexfrgxctrl5;
#[doc = "FLEXFRG6CTRL register accessor: an alias for `Reg<FLEXFRG6CTRL_SPEC>`"]
pub type FLEXFRG6CTRL = crate::Reg<flexfrg6ctrl::FLEXFRG6CTRL_SPEC>;
#[doc = "Fractional rate divider for flexcomm 6"]
pub mod flexfrg6ctrl;
#[doc = "FLEXFRGXCTRL6 register accessor: an alias for `Reg<FLEXFRGXCTRL6_SPEC>`"]
pub type FLEXFRGXCTRL6 = crate::Reg<flexfrgxctrl6::FLEXFRGXCTRL6_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod flexfrgxctrl6;
#[doc = "FLEXFRG7CTRL register accessor: an alias for `Reg<FLEXFRG7CTRL_SPEC>`"]
pub type FLEXFRG7CTRL = crate::Reg<flexfrg7ctrl::FLEXFRG7CTRL_SPEC>;
#[doc = "Fractional rate divider for flexcomm 7"]
pub mod flexfrg7ctrl;
#[doc = "FLEXFRGXCTRL7 register accessor: an alias for `Reg<FLEXFRGXCTRL7_SPEC>`"]
pub type FLEXFRGXCTRL7 = crate::Reg<flexfrgxctrl7::FLEXFRGXCTRL7_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod flexfrgxctrl7;
#[doc = "AHBCLKDIV register accessor: an alias for `Reg<AHBCLKDIV_SPEC>`"]
pub type AHBCLKDIV = crate::Reg<ahbclkdiv::AHBCLKDIV_SPEC>;
#[doc = "System clock divider"]
pub mod ahbclkdiv;
#[doc = "CLKOUTDIV register accessor: an alias for `Reg<CLKOUTDIV_SPEC>`"]
pub type CLKOUTDIV = crate::Reg<clkoutdiv::CLKOUTDIV_SPEC>;
#[doc = "CLKOUT clock divider"]
pub mod clkoutdiv;
#[doc = "FROHFDIV register accessor: an alias for `Reg<FROHFDIV_SPEC>`"]
pub type FROHFDIV = crate::Reg<frohfdiv::FROHFDIV_SPEC>;
#[doc = "FRO_HF (96MHz) clock divider"]
pub mod frohfdiv;
#[doc = "WDTCLKDIV register accessor: an alias for `Reg<WDTCLKDIV_SPEC>`"]
pub type WDTCLKDIV = crate::Reg<wdtclkdiv::WDTCLKDIV_SPEC>;
#[doc = "WDT clock divider"]
pub mod wdtclkdiv;
#[doc = "ADCCLKDIV register accessor: an alias for `Reg<ADCCLKDIV_SPEC>`"]
pub type ADCCLKDIV = crate::Reg<adcclkdiv::ADCCLKDIV_SPEC>;
#[doc = "ADC clock divider"]
pub mod adcclkdiv;
#[doc = "USB0CLKDIV register accessor: an alias for `Reg<USB0CLKDIV_SPEC>`"]
pub type USB0CLKDIV = crate::Reg<usb0clkdiv::USB0CLKDIV_SPEC>;
#[doc = "USB0 Clock divider"]
pub mod usb0clkdiv;
#[doc = "MCLKDIV register accessor: an alias for `Reg<MCLKDIV_SPEC>`"]
pub type MCLKDIV = crate::Reg<mclkdiv::MCLKDIV_SPEC>;
#[doc = "I2S MCLK clock divider"]
pub mod mclkdiv;
#[doc = "SCTCLKDIV register accessor: an alias for `Reg<SCTCLKDIV_SPEC>`"]
pub type SCTCLKDIV = crate::Reg<sctclkdiv::SCTCLKDIV_SPEC>;
#[doc = "SCT/PWM clock divider"]
pub mod sctclkdiv;
#[doc = "SDIOCLKDIV register accessor: an alias for `Reg<SDIOCLKDIV_SPEC>`"]
pub type SDIOCLKDIV = crate::Reg<sdioclkdiv::SDIOCLKDIV_SPEC>;
#[doc = "SDIO clock divider"]
pub mod sdioclkdiv;
#[doc = "PLL0CLKDIV register accessor: an alias for `Reg<PLL0CLKDIV_SPEC>`"]
pub type PLL0CLKDIV = crate::Reg<pll0clkdiv::PLL0CLKDIV_SPEC>;
#[doc = "PLL0 clock divider"]
pub mod pll0clkdiv;
#[doc = "CLOCKGENUPDATELOCKOUT register accessor: an alias for `Reg<CLOCKGENUPDATELOCKOUT_SPEC>`"]
pub type CLOCKGENUPDATELOCKOUT = crate::Reg<clockgenupdatelockout::CLOCKGENUPDATELOCKOUT_SPEC>;
#[doc = "Control clock configuration registers access (like xxxDIV, xxxSEL)"]
pub mod clockgenupdatelockout;
#[doc = "FMCCR register accessor: an alias for `Reg<FMCCR_SPEC>`"]
pub type FMCCR = crate::Reg<fmccr::FMCCR_SPEC>;
#[doc = "FMC configuration register"]
pub mod fmccr;
#[doc = "USB0NEEDCLKCTRL register accessor: an alias for `Reg<USB0NEEDCLKCTRL_SPEC>`"]
pub type USB0NEEDCLKCTRL = crate::Reg<usb0needclkctrl::USB0NEEDCLKCTRL_SPEC>;
#[doc = "USB0 need clock control"]
pub mod usb0needclkctrl;
#[doc = "USB0NEEDCLKSTAT register accessor: an alias for `Reg<USB0NEEDCLKSTAT_SPEC>`"]
pub type USB0NEEDCLKSTAT = crate::Reg<usb0needclkstat::USB0NEEDCLKSTAT_SPEC>;
#[doc = "USB0 need clock status"]
pub mod usb0needclkstat;
#[doc = "FMCFLUSH register accessor: an alias for `Reg<FMCFLUSH_SPEC>`"]
pub type FMCFLUSH = crate::Reg<fmcflush::FMCFLUSH_SPEC>;
#[doc = "FMCflush control"]
pub mod fmcflush;
#[doc = "MCLKIO register accessor: an alias for `Reg<MCLKIO_SPEC>`"]
pub type MCLKIO = crate::Reg<mclkio::MCLKIO_SPEC>;
#[doc = "MCLK control"]
pub mod mclkio;
#[doc = "USB1NEEDCLKCTRL register accessor: an alias for `Reg<USB1NEEDCLKCTRL_SPEC>`"]
pub type USB1NEEDCLKCTRL = crate::Reg<usb1needclkctrl::USB1NEEDCLKCTRL_SPEC>;
#[doc = "USB1 need clock control"]
pub mod usb1needclkctrl;
#[doc = "USB1NEEDCLKSTAT register accessor: an alias for `Reg<USB1NEEDCLKSTAT_SPEC>`"]
pub type USB1NEEDCLKSTAT = crate::Reg<usb1needclkstat::USB1NEEDCLKSTAT_SPEC>;
#[doc = "USB1 need clock status"]
pub mod usb1needclkstat;
#[doc = "SDIOCLKCTRL register accessor: an alias for `Reg<SDIOCLKCTRL_SPEC>`"]
pub type SDIOCLKCTRL = crate::Reg<sdioclkctrl::SDIOCLKCTRL_SPEC>;
#[doc = "SDIO CCLKIN phase and delay control"]
pub mod sdioclkctrl;
#[doc = "PLL1CTRL register accessor: an alias for `Reg<PLL1CTRL_SPEC>`"]
pub type PLL1CTRL = crate::Reg<pll1ctrl::PLL1CTRL_SPEC>;
#[doc = "PLL1 550m control"]
pub mod pll1ctrl;
#[doc = "PLL1STAT register accessor: an alias for `Reg<PLL1STAT_SPEC>`"]
pub type PLL1STAT = crate::Reg<pll1stat::PLL1STAT_SPEC>;
#[doc = "PLL1 550m status"]
pub mod pll1stat;
#[doc = "PLL1NDEC register accessor: an alias for `Reg<PLL1NDEC_SPEC>`"]
pub type PLL1NDEC = crate::Reg<pll1ndec::PLL1NDEC_SPEC>;
#[doc = "PLL1 550m N divider"]
pub mod pll1ndec;
#[doc = "PLL1MDEC register accessor: an alias for `Reg<PLL1MDEC_SPEC>`"]
pub type PLL1MDEC = crate::Reg<pll1mdec::PLL1MDEC_SPEC>;
#[doc = "PLL1 550m M divider"]
pub mod pll1mdec;
#[doc = "PLL1PDEC register accessor: an alias for `Reg<PLL1PDEC_SPEC>`"]
pub type PLL1PDEC = crate::Reg<pll1pdec::PLL1PDEC_SPEC>;
#[doc = "PLL1 550m P divider"]
pub mod pll1pdec;
#[doc = "PLL0CTRL register accessor: an alias for `Reg<PLL0CTRL_SPEC>`"]
pub type PLL0CTRL = crate::Reg<pll0ctrl::PLL0CTRL_SPEC>;
#[doc = "PLL0 550m control"]
pub mod pll0ctrl;
#[doc = "PLL0STAT register accessor: an alias for `Reg<PLL0STAT_SPEC>`"]
pub type PLL0STAT = crate::Reg<pll0stat::PLL0STAT_SPEC>;
#[doc = "PLL0 550m status"]
pub mod pll0stat;
#[doc = "PLL0NDEC register accessor: an alias for `Reg<PLL0NDEC_SPEC>`"]
pub type PLL0NDEC = crate::Reg<pll0ndec::PLL0NDEC_SPEC>;
#[doc = "PLL0 550m N divider"]
pub mod pll0ndec;
#[doc = "PLL0PDEC register accessor: an alias for `Reg<PLL0PDEC_SPEC>`"]
pub type PLL0PDEC = crate::Reg<pll0pdec::PLL0PDEC_SPEC>;
#[doc = "PLL0 550m P divider"]
pub mod pll0pdec;
#[doc = "PLL0SSCG0 register accessor: an alias for `Reg<PLL0SSCG0_SPEC>`"]
pub type PLL0SSCG0 = crate::Reg<pll0sscg0::PLL0SSCG0_SPEC>;
#[doc = "PLL0 Spread Spectrum Wrapper control register 0"]
pub mod pll0sscg0;
#[doc = "PLL0SSCG1 register accessor: an alias for `Reg<PLL0SSCG1_SPEC>`"]
pub type PLL0SSCG1 = crate::Reg<pll0sscg1::PLL0SSCG1_SPEC>;
#[doc = "PLL0 Spread Spectrum Wrapper control register 1"]
pub mod pll0sscg1;
#[doc = "CPUCTRL register accessor: an alias for `Reg<CPUCTRL_SPEC>`"]
pub type CPUCTRL = crate::Reg<cpuctrl::CPUCTRL_SPEC>;
#[doc = "CPU Control for multiple processors"]
pub mod cpuctrl;
#[doc = "CPBOOT register accessor: an alias for `Reg<CPBOOT_SPEC>`"]
pub type CPBOOT = crate::Reg<cpboot::CPBOOT_SPEC>;
#[doc = "Coprocessor Boot Address"]
pub mod cpboot;
#[doc = "CPSTAT register accessor: an alias for `Reg<CPSTAT_SPEC>`"]
pub type CPSTAT = crate::Reg<cpstat::CPSTAT_SPEC>;
#[doc = "CPU Status"]
pub mod cpstat;
#[doc = "CLOCK_CTRL register accessor: an alias for `Reg<CLOCK_CTRL_SPEC>`"]
pub type CLOCK_CTRL = crate::Reg<clock_ctrl::CLOCK_CTRL_SPEC>;
#[doc = "Various system clock controls : Flash clock (48 MHz) control, clocks to Frequency Measures"]
pub mod clock_ctrl;
#[doc = "COMP_INT_CTRL register accessor: an alias for `Reg<COMP_INT_CTRL_SPEC>`"]
pub type COMP_INT_CTRL = crate::Reg<comp_int_ctrl::COMP_INT_CTRL_SPEC>;
#[doc = "Comparator Interrupt control"]
pub mod comp_int_ctrl;
#[doc = "COMP_INT_STATUS register accessor: an alias for `Reg<COMP_INT_STATUS_SPEC>`"]
pub type COMP_INT_STATUS = crate::Reg<comp_int_status::COMP_INT_STATUS_SPEC>;
#[doc = "Comparator Interrupt status"]
pub mod comp_int_status;
#[doc = "AUTOCLKGATEOVERRIDE register accessor: an alias for `Reg<AUTOCLKGATEOVERRIDE_SPEC>`"]
pub type AUTOCLKGATEOVERRIDE = crate::Reg<autoclkgateoverride::AUTOCLKGATEOVERRIDE_SPEC>;
#[doc = "Control automatic clock gating"]
pub mod autoclkgateoverride;
#[doc = "GPIOPSYNC register accessor: an alias for `Reg<GPIOPSYNC_SPEC>`"]
pub type GPIOPSYNC = crate::Reg<gpiopsync::GPIOPSYNC_SPEC>;
#[doc = "Enable bypass of the first stage of synchonization inside GPIO_INT module"]
pub mod gpiopsync;
#[doc = "DEBUG_LOCK_EN register accessor: an alias for `Reg<DEBUG_LOCK_EN_SPEC>`"]
pub type DEBUG_LOCK_EN = crate::Reg<debug_lock_en::DEBUG_LOCK_EN_SPEC>;
#[doc = "Control write access to security registers."]
pub mod debug_lock_en;
#[doc = "DEBUG_FEATURES register accessor: an alias for `Reg<DEBUG_FEATURES_SPEC>`"]
pub type DEBUG_FEATURES = crate::Reg<debug_features::DEBUG_FEATURES_SPEC>;
#[doc = "Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control."]
pub mod debug_features;
#[doc = "DEBUG_FEATURES_DP register accessor: an alias for `Reg<DEBUG_FEATURES_DP_SPEC>`"]
pub type DEBUG_FEATURES_DP = crate::Reg<debug_features_dp::DEBUG_FEATURES_DP_SPEC>;
#[doc = "Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control DUPLICATE register."]
pub mod debug_features_dp;
#[doc = "KEY_BLOCK register accessor: an alias for `Reg<KEY_BLOCK_SPEC>`"]
pub type KEY_BLOCK = crate::Reg<key_block::KEY_BLOCK_SPEC>;
#[doc = "block quiddikey/PUF all index."]
pub mod key_block;
#[doc = "DEBUG_AUTH_BEACON register accessor: an alias for `Reg<DEBUG_AUTH_BEACON_SPEC>`"]
pub type DEBUG_AUTH_BEACON = crate::Reg<debug_auth_beacon::DEBUG_AUTH_BEACON_SPEC>;
#[doc = "Debug authentication BEACON register"]
pub mod debug_auth_beacon;
#[doc = "CPUCFG register accessor: an alias for `Reg<CPUCFG_SPEC>`"]
pub type CPUCFG = crate::Reg<cpucfg::CPUCFG_SPEC>;
#[doc = "CPUs configuration register"]
pub mod cpucfg;
#[doc = "DEVICE_ID0 register accessor: an alias for `Reg<DEVICE_ID0_SPEC>`"]
pub type DEVICE_ID0 = crate::Reg<device_id0::DEVICE_ID0_SPEC>;
#[doc = "Device ID"]
pub mod device_id0;
#[doc = "DIEID register accessor: an alias for `Reg<DIEID_SPEC>`"]
pub type DIEID = crate::Reg<dieid::DIEID_SPEC>;
#[doc = "Chip revision ID and Number"]
pub mod dieid;
