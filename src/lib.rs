#![doc = "Peripheral access API for LPC55S69_CM33_CORE0 microcontrollers (generated using svd2rust v0.16.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.16.0/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
extern "C" {
    fn WDT_BOD();
    fn DMA0();
    fn GINT0();
    fn GINT1();
    fn PIN_INT0();
    fn PIN_INT1();
    fn PIN_INT2();
    fn PIN_INT3();
    fn UTICK0();
    fn MRT0();
    fn CTIMER0();
    fn CTIMER1();
    fn SCT0();
    fn CTIMER3();
    fn FLEXCOMM0();
    fn FLEXCOMM1();
    fn FLEXCOMM2();
    fn FLEXCOMM3();
    fn FLEXCOMM4();
    fn FLEXCOMM5();
    fn FLEXCOMM6();
    fn FLEXCOMM7();
    fn ADC0();
    fn USB0_NEEDCLK();
    fn USB0();
    fn RTC();
    fn MAILBOX();
    fn PIN_INT4();
    fn PIN_INT5();
    fn PIN_INT6();
    fn PIN_INT7();
    fn CTIMER2();
    fn CTIMER4();
    fn OS_EVENT();
    fn SDIO();
    fn USB1_UTMI();
    fn USB1();
    fn USB1_NEEDCLK();
    fn SEC_GPIO_INT0_IRQ0();
    fn SEC_GPIO_INT0_IRQ1();
    fn PLU();
    fn HASHCRYPT();
    fn PUF();
    fn DMA1();
    fn FLEXCOMM8();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 60] = [
    Vector { _handler: WDT_BOD },
    Vector { _handler: DMA0 },
    Vector { _handler: GINT0 },
    Vector { _handler: GINT1 },
    Vector { _handler: PIN_INT0 },
    Vector { _handler: PIN_INT1 },
    Vector { _handler: PIN_INT2 },
    Vector { _handler: PIN_INT3 },
    Vector { _handler: UTICK0 },
    Vector { _handler: MRT0 },
    Vector { _handler: CTIMER0 },
    Vector { _handler: CTIMER1 },
    Vector { _handler: SCT0 },
    Vector { _handler: CTIMER3 },
    Vector {
        _handler: FLEXCOMM0,
    },
    Vector {
        _handler: FLEXCOMM1,
    },
    Vector {
        _handler: FLEXCOMM2,
    },
    Vector {
        _handler: FLEXCOMM3,
    },
    Vector {
        _handler: FLEXCOMM4,
    },
    Vector {
        _handler: FLEXCOMM5,
    },
    Vector {
        _handler: FLEXCOMM6,
    },
    Vector {
        _handler: FLEXCOMM7,
    },
    Vector { _handler: ADC0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: USB0_NEEDCLK,
    },
    Vector { _handler: USB0 },
    Vector { _handler: RTC },
    Vector { _reserved: 0 },
    Vector { _handler: MAILBOX },
    Vector { _handler: PIN_INT4 },
    Vector { _handler: PIN_INT5 },
    Vector { _handler: PIN_INT6 },
    Vector { _handler: PIN_INT7 },
    Vector { _handler: CTIMER2 },
    Vector { _handler: CTIMER4 },
    Vector { _handler: OS_EVENT },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: SDIO },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: USB1_UTMI,
    },
    Vector { _handler: USB1 },
    Vector {
        _handler: USB1_NEEDCLK,
    },
    Vector { _reserved: 0 },
    Vector {
        _handler: SEC_GPIO_INT0_IRQ0,
    },
    Vector {
        _handler: SEC_GPIO_INT0_IRQ1,
    },
    Vector { _handler: PLU },
    Vector { _reserved: 0 },
    Vector {
        _handler: HASHCRYPT,
    },
    Vector { _reserved: 0 },
    Vector { _handler: PUF },
    Vector { _reserved: 0 },
    Vector { _handler: DMA1 },
    Vector {
        _handler: FLEXCOMM8,
    },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
    #[doc = "0 - WDT_BOD"]
    WDT_BOD,
    #[doc = "1 - DMA0"]
    DMA0,
    #[doc = "2 - GINT0"]
    GINT0,
    #[doc = "3 - GINT1"]
    GINT1,
    #[doc = "4 - PIN_INT0"]
    PIN_INT0,
    #[doc = "5 - PIN_INT1"]
    PIN_INT1,
    #[doc = "6 - PIN_INT2"]
    PIN_INT2,
    #[doc = "7 - PIN_INT3"]
    PIN_INT3,
    #[doc = "8 - UTICK0"]
    UTICK0,
    #[doc = "9 - MRT0"]
    MRT0,
    #[doc = "10 - CTIMER0"]
    CTIMER0,
    #[doc = "11 - CTIMER1"]
    CTIMER1,
    #[doc = "12 - SCT0"]
    SCT0,
    #[doc = "13 - CTIMER3"]
    CTIMER3,
    #[doc = "14 - FLEXCOMM0"]
    FLEXCOMM0,
    #[doc = "15 - FLEXCOMM1"]
    FLEXCOMM1,
    #[doc = "16 - FLEXCOMM2"]
    FLEXCOMM2,
    #[doc = "17 - FLEXCOMM3"]
    FLEXCOMM3,
    #[doc = "18 - FLEXCOMM4"]
    FLEXCOMM4,
    #[doc = "19 - FLEXCOMM5"]
    FLEXCOMM5,
    #[doc = "20 - FLEXCOMM6"]
    FLEXCOMM6,
    #[doc = "21 - FLEXCOMM7"]
    FLEXCOMM7,
    #[doc = "22 - ADC0"]
    ADC0,
    #[doc = "27 - USB0_NEEDCLK"]
    USB0_NEEDCLK,
    #[doc = "28 - USB0"]
    USB0,
    #[doc = "29 - RTC"]
    RTC,
    #[doc = "31 - MAILBOX"]
    MAILBOX,
    #[doc = "32 - PIN_INT4"]
    PIN_INT4,
    #[doc = "33 - PIN_INT5"]
    PIN_INT5,
    #[doc = "34 - PIN_INT6"]
    PIN_INT6,
    #[doc = "35 - PIN_INT7"]
    PIN_INT7,
    #[doc = "36 - CTIMER2"]
    CTIMER2,
    #[doc = "37 - CTIMER4"]
    CTIMER4,
    #[doc = "38 - OS_EVENT"]
    OS_EVENT,
    #[doc = "42 - SDIO"]
    SDIO,
    #[doc = "46 - USB1_UTMI"]
    USB1_UTMI,
    #[doc = "47 - USB1"]
    USB1,
    #[doc = "48 - USB1_NEEDCLK"]
    USB1_NEEDCLK,
    #[doc = "50 - SEC_GPIO_INT0_IRQ0"]
    SEC_GPIO_INT0_IRQ0,
    #[doc = "51 - SEC_GPIO_INT0_IRQ1"]
    SEC_GPIO_INT0_IRQ1,
    #[doc = "52 - PLU"]
    PLU,
    #[doc = "54 - HASHCRYPT"]
    HASHCRYPT,
    #[doc = "56 - PUF"]
    PUF,
    #[doc = "58 - DMA1"]
    DMA1,
    #[doc = "59 - FLEXCOMM8"]
    FLEXCOMM8,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::WDT_BOD => 0,
            Interrupt::DMA0 => 1,
            Interrupt::GINT0 => 2,
            Interrupt::GINT1 => 3,
            Interrupt::PIN_INT0 => 4,
            Interrupt::PIN_INT1 => 5,
            Interrupt::PIN_INT2 => 6,
            Interrupt::PIN_INT3 => 7,
            Interrupt::UTICK0 => 8,
            Interrupt::MRT0 => 9,
            Interrupt::CTIMER0 => 10,
            Interrupt::CTIMER1 => 11,
            Interrupt::SCT0 => 12,
            Interrupt::CTIMER3 => 13,
            Interrupt::FLEXCOMM0 => 14,
            Interrupt::FLEXCOMM1 => 15,
            Interrupt::FLEXCOMM2 => 16,
            Interrupt::FLEXCOMM3 => 17,
            Interrupt::FLEXCOMM4 => 18,
            Interrupt::FLEXCOMM5 => 19,
            Interrupt::FLEXCOMM6 => 20,
            Interrupt::FLEXCOMM7 => 21,
            Interrupt::ADC0 => 22,
            Interrupt::USB0_NEEDCLK => 27,
            Interrupt::USB0 => 28,
            Interrupt::RTC => 29,
            Interrupt::MAILBOX => 31,
            Interrupt::PIN_INT4 => 32,
            Interrupt::PIN_INT5 => 33,
            Interrupt::PIN_INT6 => 34,
            Interrupt::PIN_INT7 => 35,
            Interrupt::CTIMER2 => 36,
            Interrupt::CTIMER4 => 37,
            Interrupt::OS_EVENT => 38,
            Interrupt::SDIO => 42,
            Interrupt::USB1_UTMI => 46,
            Interrupt::USB1 => 47,
            Interrupt::USB1_NEEDCLK => 48,
            Interrupt::SEC_GPIO_INT0_IRQ0 => 50,
            Interrupt::SEC_GPIO_INT0_IRQ1 => 51,
            Interrupt::PLU => 52,
            Interrupt::HASHCRYPT => 54,
            Interrupt::PUF => 56,
            Interrupt::DMA1 => 58,
            Interrupt::FLEXCOMM8 => 59,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "FLASH_CFPA"]
pub struct FLASH_CFPA0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH_CFPA0 {}
impl FLASH_CFPA0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flash_cfpa0::RegisterBlock {
        0x0009_e000 as *const _
    }
}
impl Deref for FLASH_CFPA0 {
    type Target = flash_cfpa0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLASH_CFPA0::ptr() }
    }
}
#[doc = "FLASH_CFPA"]
pub mod flash_cfpa0;
#[doc = "FLASH_CFPA"]
pub struct FLASH_CFPA_SCRATCH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH_CFPA_SCRATCH {}
impl FLASH_CFPA_SCRATCH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flash_cfpa0::RegisterBlock {
        0x0009_de00 as *const _
    }
}
impl Deref for FLASH_CFPA_SCRATCH {
    type Target = flash_cfpa0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLASH_CFPA_SCRATCH::ptr() }
    }
}
#[doc = "FLASH_CFPA"]
pub struct FLASH_CFPA1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH_CFPA1 {}
impl FLASH_CFPA1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flash_cfpa0::RegisterBlock {
        0x0009_e200 as *const _
    }
}
impl Deref for FLASH_CFPA1 {
    type Target = flash_cfpa0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLASH_CFPA1::ptr() }
    }
}
#[doc = "FLASH_CMPA"]
pub struct FLASH_CMPA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH_CMPA {}
impl FLASH_CMPA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flash_cmpa::RegisterBlock {
        0x0009_e400 as *const _
    }
}
impl Deref for FLASH_CMPA {
    type Target = flash_cmpa::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLASH_CMPA::ptr() }
    }
}
#[doc = "FLASH_CMPA"]
pub mod flash_cmpa;
#[doc = "FLASH_KEY_STORE"]
pub struct FLASH_KEY_STORE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH_KEY_STORE {}
impl FLASH_KEY_STORE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flash_key_store::RegisterBlock {
        0x0009_e600 as *const _
    }
}
impl Deref for FLASH_KEY_STORE {
    type Target = flash_key_store::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLASH_KEY_STORE::ptr() }
    }
}
#[doc = "FLASH_KEY_STORE"]
pub mod flash_key_store;
#[doc = "SYSCON"]
pub struct SYSCON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCON {}
impl SYSCON {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const syscon::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for SYSCON {
    type Target = syscon::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSCON::ptr() }
    }
}
#[doc = "SYSCON"]
pub mod syscon;
#[doc = "I/O pin configuration (IOCON)"]
pub struct IOCON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IOCON {}
impl IOCON {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const iocon::RegisterBlock {
        0x4000_1000 as *const _
    }
}
impl Deref for IOCON {
    type Target = iocon::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*IOCON::ptr() }
    }
}
#[doc = "I/O pin configuration (IOCON)"]
pub mod iocon;
#[doc = "Group GPIO input interrupt (GINT0/1)"]
pub struct GINT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GINT0 {}
impl GINT0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gint0::RegisterBlock {
        0x4000_2000 as *const _
    }
}
impl Deref for GINT0 {
    type Target = gint0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GINT0::ptr() }
    }
}
#[doc = "Group GPIO input interrupt (GINT0/1)"]
pub mod gint0;
#[doc = "Group GPIO input interrupt (GINT0/1)"]
pub struct GINT1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GINT1 {}
impl GINT1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gint0::RegisterBlock {
        0x4000_3000 as *const _
    }
}
impl Deref for GINT1 {
    type Target = gint0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GINT1::ptr() }
    }
}
#[doc = "Pin interrupt and pattern match (PINT)"]
pub struct PINT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PINT {}
impl PINT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pint::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for PINT {
    type Target = pint::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PINT::ptr() }
    }
}
#[doc = "Pin interrupt and pattern match (PINT)"]
pub mod pint;
#[doc = "Pin interrupt and pattern match (PINT)"]
pub struct SECPINT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SECPINT {}
impl SECPINT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pint::RegisterBlock {
        0x4000_5000 as *const _
    }
}
impl Deref for SECPINT {
    type Target = pint::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SECPINT::ptr() }
    }
}
#[doc = "Standard counter/timers (CTIMER0 to 4)"]
pub struct CTIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CTIMER0 {}
impl CTIMER0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ctimer0::RegisterBlock {
        0x4000_8000 as *const _
    }
}
impl Deref for CTIMER0 {
    type Target = ctimer0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CTIMER0::ptr() }
    }
}
#[doc = "Standard counter/timers (CTIMER0 to 4)"]
pub mod ctimer0;
#[doc = "Standard counter/timers (CTIMER0 to 4)"]
pub struct CTIMER1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CTIMER1 {}
impl CTIMER1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ctimer0::RegisterBlock {
        0x4000_9000 as *const _
    }
}
impl Deref for CTIMER1 {
    type Target = ctimer0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CTIMER1::ptr() }
    }
}
#[doc = "Standard counter/timers (CTIMER0 to 4)"]
pub struct CTIMER2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CTIMER2 {}
impl CTIMER2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ctimer0::RegisterBlock {
        0x4002_8000 as *const _
    }
}
impl Deref for CTIMER2 {
    type Target = ctimer0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CTIMER2::ptr() }
    }
}
#[doc = "Standard counter/timers (CTIMER0 to 4)"]
pub struct CTIMER3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CTIMER3 {}
impl CTIMER3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ctimer0::RegisterBlock {
        0x4002_9000 as *const _
    }
}
impl Deref for CTIMER3 {
    type Target = ctimer0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CTIMER3::ptr() }
    }
}
#[doc = "Standard counter/timers (CTIMER0 to 4)"]
pub struct CTIMER4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CTIMER4 {}
impl CTIMER4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ctimer0::RegisterBlock {
        0x4002_a000 as *const _
    }
}
impl Deref for CTIMER4 {
    type Target = ctimer0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CTIMER4::ptr() }
    }
}
#[doc = "Windowed Watchdog Timer (WWDT)"]
pub struct WWDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WWDT {}
impl WWDT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wwdt::RegisterBlock {
        0x4000_c000 as *const _
    }
}
impl Deref for WWDT {
    type Target = wwdt::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*WWDT::ptr() }
    }
}
#[doc = "Windowed Watchdog Timer (WWDT)"]
pub mod wwdt;
#[doc = "Multi-Rate Timer (MRT)"]
pub struct MRT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MRT0 {}
impl MRT0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mrt0::RegisterBlock {
        0x4000_d000 as *const _
    }
}
impl Deref for MRT0 {
    type Target = mrt0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*MRT0::ptr() }
    }
}
#[doc = "Multi-Rate Timer (MRT)"]
pub mod mrt0;
#[doc = "Micro-tick Timer (UTICK)"]
pub struct UTICK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UTICK {}
impl UTICK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const utick::RegisterBlock {
        0x4000_e000 as *const _
    }
}
impl Deref for UTICK {
    type Target = utick::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UTICK::ptr() }
    }
}
#[doc = "Micro-tick Timer (UTICK)"]
pub mod utick;
#[doc = "ANALOGCTRL"]
pub struct ANACTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ANACTRL {}
impl ANACTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const anactrl::RegisterBlock {
        0x4001_3000 as *const _
    }
}
impl Deref for ANACTRL {
    type Target = anactrl::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ANACTRL::ptr() }
    }
}
#[doc = "ANALOGCTRL"]
pub mod anactrl;
#[doc = "PMC"]
pub struct PMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMC {}
impl PMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pmc::RegisterBlock {
        0x4002_0000 as *const _
    }
}
impl Deref for PMC {
    type Target = pmc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PMC::ptr() }
    }
}
#[doc = "PMC"]
pub mod pmc;
#[doc = "system controller"]
pub struct SYSCTL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCTL {}
impl SYSCTL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sysctl::RegisterBlock {
        0x4002_3000 as *const _
    }
}
impl Deref for SYSCTL {
    type Target = sysctl::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSCTL::ptr() }
    }
}
#[doc = "system controller"]
pub mod sysctl;
#[doc = "Real-Time Clock (RTC)"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x4002_c000 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real-Time Clock (RTC)"]
pub mod rtc;
#[doc = "Synchronous OS/Event timer with Wakeup Timer"]
pub struct OSTIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OSTIMER {}
impl OSTIMER {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ostimer::RegisterBlock {
        0x4002_d000 as *const _
    }
}
impl Deref for OSTIMER {
    type Target = ostimer::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*OSTIMER::ptr() }
    }
}
#[doc = "Synchronous OS/Event timer with Wakeup Timer"]
pub mod ostimer;
#[doc = "FLASH"]
pub struct FLASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH {}
impl FLASH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flash::RegisterBlock {
        0x4003_4000 as *const _
    }
}
impl Deref for FLASH {
    type Target = flash::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLASH::ptr() }
    }
}
#[doc = "FLASH"]
pub mod flash;
#[doc = "PRINCE"]
pub struct PRINCE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PRINCE {}
impl PRINCE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const prince::RegisterBlock {
        0x4003_5000 as *const _
    }
}
impl Deref for PRINCE {
    type Target = prince::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PRINCE::ptr() }
    }
}
#[doc = "PRINCE"]
pub mod prince;
#[doc = "Universal System Bus Physical Layer"]
pub struct USBPHY {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USBPHY {}
impl USBPHY {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usbphy::RegisterBlock {
        0x4003_8000 as *const _
    }
}
impl Deref for USBPHY {
    type Target = usbphy::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USBPHY::ptr() }
    }
}
#[doc = "Universal System Bus Physical Layer"]
pub mod usbphy;
#[doc = "RNG"]
pub struct RNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RNG {}
impl RNG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rng::RegisterBlock {
        0x4003_a000 as *const _
    }
}
impl Deref for RNG {
    type Target = rng::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RNG::ptr() }
    }
}
#[doc = "RNG"]
pub mod rng;
#[doc = "PUFCTRL"]
pub struct PUF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PUF {}
impl PUF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const puf::RegisterBlock {
        0x4003_b000 as *const _
    }
}
impl Deref for PUF {
    type Target = puf::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PUF::ptr() }
    }
}
#[doc = "PUFCTRL"]
pub mod puf;
#[doc = "LPC80X Programmable Logic Unit (PLU)"]
pub struct PLU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PLU {}
impl PLU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const plu::RegisterBlock {
        0x4003_d000 as *const _
    }
}
impl Deref for PLU {
    type Target = plu::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PLU::ptr() }
    }
}
#[doc = "LPC80X Programmable Logic Unit (PLU)"]
pub mod plu;
#[doc = "DMA controller"]
pub struct DMA0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA0 {}
impl DMA0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma0::RegisterBlock {
        0x4008_2000 as *const _
    }
}
impl Deref for DMA0 {
    type Target = dma0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA0::ptr() }
    }
}
#[doc = "DMA controller"]
pub mod dma0;
#[doc = "DMA controller"]
pub struct DMA1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA1 {}
impl DMA1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma0::RegisterBlock {
        0x400a_7000 as *const _
    }
}
impl Deref for DMA1 {
    type Target = dma0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA1::ptr() }
    }
}
#[doc = "USB 2.0 Device Controller"]
pub struct USB0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0 {}
impl USB0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb0::RegisterBlock {
        0x4008_4000 as *const _
    }
}
impl Deref for USB0 {
    type Target = usb0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB0::ptr() }
    }
}
#[doc = "USB 2.0 Device Controller"]
pub mod usb0;
#[doc = "SCTimer/PWM (SCT)"]
pub struct SCT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCT0 {}
impl SCT0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sct0::RegisterBlock {
        0x4008_5000 as *const _
    }
}
impl Deref for SCT0 {
    type Target = sct0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCT0::ptr() }
    }
}
#[doc = "SCTimer/PWM (SCT)"]
pub mod sct0;
#[doc = "Flexcomm serial communication"]
pub struct FLEXCOMM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLEXCOMM0 {}
impl FLEXCOMM0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flexcomm0::RegisterBlock {
        0x4008_6000 as *const _
    }
}
impl Deref for FLEXCOMM0 {
    type Target = flexcomm0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLEXCOMM0::ptr() }
    }
}
#[doc = "Flexcomm serial communication"]
pub mod flexcomm0;
#[doc = "Flexcomm serial communication"]
pub struct FLEXCOMM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLEXCOMM1 {}
impl FLEXCOMM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flexcomm0::RegisterBlock {
        0x4008_7000 as *const _
    }
}
impl Deref for FLEXCOMM1 {
    type Target = flexcomm0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLEXCOMM1::ptr() }
    }
}
#[doc = "Flexcomm serial communication"]
pub struct FLEXCOMM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLEXCOMM2 {}
impl FLEXCOMM2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flexcomm0::RegisterBlock {
        0x4008_8000 as *const _
    }
}
impl Deref for FLEXCOMM2 {
    type Target = flexcomm0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLEXCOMM2::ptr() }
    }
}
#[doc = "Flexcomm serial communication"]
pub struct FLEXCOMM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLEXCOMM3 {}
impl FLEXCOMM3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flexcomm0::RegisterBlock {
        0x4008_9000 as *const _
    }
}
impl Deref for FLEXCOMM3 {
    type Target = flexcomm0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLEXCOMM3::ptr() }
    }
}
#[doc = "Flexcomm serial communication"]
pub struct FLEXCOMM4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLEXCOMM4 {}
impl FLEXCOMM4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flexcomm0::RegisterBlock {
        0x4008_a000 as *const _
    }
}
impl Deref for FLEXCOMM4 {
    type Target = flexcomm0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLEXCOMM4::ptr() }
    }
}
#[doc = "Flexcomm serial communication"]
pub struct FLEXCOMM5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLEXCOMM5 {}
impl FLEXCOMM5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flexcomm0::RegisterBlock {
        0x4009_6000 as *const _
    }
}
impl Deref for FLEXCOMM5 {
    type Target = flexcomm0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLEXCOMM5::ptr() }
    }
}
#[doc = "Flexcomm serial communication"]
pub struct FLEXCOMM6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLEXCOMM6 {}
impl FLEXCOMM6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flexcomm0::RegisterBlock {
        0x4009_7000 as *const _
    }
}
impl Deref for FLEXCOMM6 {
    type Target = flexcomm0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLEXCOMM6::ptr() }
    }
}
#[doc = "Flexcomm serial communication"]
pub struct FLEXCOMM7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLEXCOMM7 {}
impl FLEXCOMM7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flexcomm0::RegisterBlock {
        0x4009_8000 as *const _
    }
}
impl Deref for FLEXCOMM7 {
    type Target = flexcomm0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLEXCOMM7::ptr() }
    }
}
#[doc = "Flexcomm serial communication"]
pub struct FLEXCOMM8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLEXCOMM8 {}
impl FLEXCOMM8 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flexcomm0::RegisterBlock {
        0x4009_f000 as *const _
    }
}
impl Deref for FLEXCOMM8 {
    type Target = flexcomm0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLEXCOMM8::ptr() }
    }
}
#[doc = "I2C-bus interfaces"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4008_6000 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "I2C-bus interfaces"]
pub mod i2c0;
#[doc = "I2C-bus interfaces"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4008_7000 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "I2C-bus interfaces"]
pub struct I2C2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C2 {}
impl I2C2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4008_8000 as *const _
    }
}
impl Deref for I2C2 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C2::ptr() }
    }
}
#[doc = "I2C-bus interfaces"]
pub struct I2C3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C3 {}
impl I2C3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4008_9000 as *const _
    }
}
impl Deref for I2C3 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C3::ptr() }
    }
}
#[doc = "I2C-bus interfaces"]
pub struct I2C4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C4 {}
impl I2C4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4008_a000 as *const _
    }
}
impl Deref for I2C4 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C4::ptr() }
    }
}
#[doc = "I2C-bus interfaces"]
pub struct I2C5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C5 {}
impl I2C5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4009_6000 as *const _
    }
}
impl Deref for I2C5 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C5::ptr() }
    }
}
#[doc = "I2C-bus interfaces"]
pub struct I2C6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C6 {}
impl I2C6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4009_7000 as *const _
    }
}
impl Deref for I2C6 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C6::ptr() }
    }
}
#[doc = "I2C-bus interfaces"]
pub struct I2C7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C7 {}
impl I2C7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4009_8000 as *const _
    }
}
impl Deref for I2C7 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C7::ptr() }
    }
}
#[doc = "I2S interface"]
pub struct I2S0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S0 {}
impl I2S0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2s0::RegisterBlock {
        0x4008_6000 as *const _
    }
}
impl Deref for I2S0 {
    type Target = i2s0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2S0::ptr() }
    }
}
#[doc = "I2S interface"]
pub mod i2s0;
#[doc = "I2S interface"]
pub struct I2S1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S1 {}
impl I2S1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2s0::RegisterBlock {
        0x4008_7000 as *const _
    }
}
impl Deref for I2S1 {
    type Target = i2s0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2S1::ptr() }
    }
}
#[doc = "I2S interface"]
pub struct I2S2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S2 {}
impl I2S2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2s0::RegisterBlock {
        0x4008_8000 as *const _
    }
}
impl Deref for I2S2 {
    type Target = i2s0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2S2::ptr() }
    }
}
#[doc = "I2S interface"]
pub struct I2S3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S3 {}
impl I2S3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2s0::RegisterBlock {
        0x4008_9000 as *const _
    }
}
impl Deref for I2S3 {
    type Target = i2s0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2S3::ptr() }
    }
}
#[doc = "I2S interface"]
pub struct I2S4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S4 {}
impl I2S4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2s0::RegisterBlock {
        0x4008_a000 as *const _
    }
}
impl Deref for I2S4 {
    type Target = i2s0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2S4::ptr() }
    }
}
#[doc = "I2S interface"]
pub struct I2S5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S5 {}
impl I2S5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2s0::RegisterBlock {
        0x4009_6000 as *const _
    }
}
impl Deref for I2S5 {
    type Target = i2s0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2S5::ptr() }
    }
}
#[doc = "I2S interface"]
pub struct I2S6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S6 {}
impl I2S6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2s0::RegisterBlock {
        0x4009_7000 as *const _
    }
}
impl Deref for I2S6 {
    type Target = i2s0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2S6::ptr() }
    }
}
#[doc = "I2S interface"]
pub struct I2S7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S7 {}
impl I2S7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2s0::RegisterBlock {
        0x4009_8000 as *const _
    }
}
impl Deref for I2S7 {
    type Target = i2s0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2S7::ptr() }
    }
}
#[doc = "Serial Peripheral Interfaces (SPI)"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4008_6000 as *const _
    }
}
impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI0::ptr() }
    }
}
#[doc = "Serial Peripheral Interfaces (SPI)"]
pub mod spi0;
#[doc = "Serial Peripheral Interfaces (SPI)"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4008_7000 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "Serial Peripheral Interfaces (SPI)"]
pub struct SPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI2 {}
impl SPI2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4008_8000 as *const _
    }
}
impl Deref for SPI2 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI2::ptr() }
    }
}
#[doc = "Serial Peripheral Interfaces (SPI)"]
pub struct SPI3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI3 {}
impl SPI3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4008_9000 as *const _
    }
}
impl Deref for SPI3 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI3::ptr() }
    }
}
#[doc = "Serial Peripheral Interfaces (SPI)"]
pub struct SPI4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI4 {}
impl SPI4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4008_a000 as *const _
    }
}
impl Deref for SPI4 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI4::ptr() }
    }
}
#[doc = "Serial Peripheral Interfaces (SPI)"]
pub struct SPI5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI5 {}
impl SPI5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4009_6000 as *const _
    }
}
impl Deref for SPI5 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI5::ptr() }
    }
}
#[doc = "Serial Peripheral Interfaces (SPI)"]
pub struct SPI6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI6 {}
impl SPI6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4009_7000 as *const _
    }
}
impl Deref for SPI6 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI6::ptr() }
    }
}
#[doc = "Serial Peripheral Interfaces (SPI)"]
pub struct SPI7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI7 {}
impl SPI7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4009_8000 as *const _
    }
}
impl Deref for SPI7 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI7::ptr() }
    }
}
#[doc = "Serial Peripheral Interfaces (SPI)"]
pub struct SPI8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI8 {}
impl SPI8 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4009_f000 as *const _
    }
}
impl Deref for SPI8 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI8::ptr() }
    }
}
#[doc = "USARTs"]
pub struct USART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART0 {}
impl USART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4008_6000 as *const _
    }
}
impl Deref for USART0 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART0::ptr() }
    }
}
#[doc = "USARTs"]
pub mod usart0;
#[doc = "USARTs"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4008_7000 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART1::ptr() }
    }
}
#[doc = "USARTs"]
pub struct USART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART2 {}
impl USART2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4008_8000 as *const _
    }
}
impl Deref for USART2 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART2::ptr() }
    }
}
#[doc = "USARTs"]
pub struct USART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART3 {}
impl USART3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4008_9000 as *const _
    }
}
impl Deref for USART3 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART3::ptr() }
    }
}
#[doc = "USARTs"]
pub struct USART4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART4 {}
impl USART4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4008_a000 as *const _
    }
}
impl Deref for USART4 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART4::ptr() }
    }
}
#[doc = "USARTs"]
pub struct USART5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART5 {}
impl USART5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4009_6000 as *const _
    }
}
impl Deref for USART5 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART5::ptr() }
    }
}
#[doc = "USARTs"]
pub struct USART6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART6 {}
impl USART6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4009_7000 as *const _
    }
}
impl Deref for USART6 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART6::ptr() }
    }
}
#[doc = "USARTs"]
pub struct USART7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART7 {}
impl USART7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4009_8000 as *const _
    }
}
impl Deref for USART7 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART7::ptr() }
    }
}
#[doc = "Mailbox"]
pub struct MAILBOX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MAILBOX {}
impl MAILBOX {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mailbox::RegisterBlock {
        0x4008_b000 as *const _
    }
}
impl Deref for MAILBOX {
    type Target = mailbox::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*MAILBOX::ptr() }
    }
}
#[doc = "Mailbox"]
pub mod mailbox;
#[doc = "General Purpose I/O (GPIO)"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio::RegisterBlock {
        0x4008_c000 as *const _
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO::ptr() }
    }
}
#[doc = "General Purpose I/O (GPIO)"]
pub mod gpio;
#[doc = "General Purpose I/O (GPIO)"]
pub struct SECGPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SECGPIO {}
impl SECGPIO {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio::RegisterBlock {
        0x400a_8000 as *const _
    }
}
impl Deref for SECGPIO {
    type Target = gpio::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SECGPIO::ptr() }
    }
}
#[doc = "USB1 High-speed Device Controller"]
pub struct USBHSD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USBHSD {}
impl USBHSD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usbhsd::RegisterBlock {
        0x4009_4000 as *const _
    }
}
impl Deref for USBHSD {
    type Target = usbhsd::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USBHSD::ptr() }
    }
}
#[doc = "USB1 High-speed Device Controller"]
pub mod usbhsd;
#[doc = "CRC engine"]
pub struct CRC_ENGINE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC_ENGINE {}
impl CRC_ENGINE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crc_engine::RegisterBlock {
        0x4009_5000 as *const _
    }
}
impl Deref for CRC_ENGINE {
    type Target = crc_engine::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRC_ENGINE::ptr() }
    }
}
#[doc = "CRC engine"]
pub mod crc_engine;
#[doc = "SDMMC"]
pub struct SDIF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDIF {}
impl SDIF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sdif::RegisterBlock {
        0x4009_b000 as *const _
    }
}
impl Deref for SDIF {
    type Target = sdif::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SDIF::ptr() }
    }
}
#[doc = "SDMMC"]
pub mod sdif;
#[doc = "MCU Debugger Mailbox"]
pub struct DGBMAILBOX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DGBMAILBOX {}
impl DGBMAILBOX {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dgbmailbox::RegisterBlock {
        0x4009_c000 as *const _
    }
}
impl Deref for DGBMAILBOX {
    type Target = dgbmailbox::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DGBMAILBOX::ptr() }
    }
}
#[doc = "MCU Debugger Mailbox"]
pub mod dgbmailbox;
#[doc = "ADC"]
pub struct ADC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC0 {}
impl ADC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc0::RegisterBlock {
        0x400a_0000 as *const _
    }
}
impl Deref for ADC0 {
    type Target = adc0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC0::ptr() }
    }
}
#[doc = "ADC"]
pub mod adc0;
#[doc = "USB0 Full-speed Host controller"]
pub struct USBFSH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USBFSH {}
impl USBFSH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usbfsh::RegisterBlock {
        0x400a_2000 as *const _
    }
}
impl Deref for USBFSH {
    type Target = usbfsh::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USBFSH::ptr() }
    }
}
#[doc = "USB0 Full-speed Host controller"]
pub mod usbfsh;
#[doc = "USB1 High-speed Host Controller"]
pub struct USBHSH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USBHSH {}
impl USBHSH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usbhsh::RegisterBlock {
        0x400a_3000 as *const _
    }
}
impl Deref for USBHSH {
    type Target = usbhsh::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USBHSH::ptr() }
    }
}
#[doc = "USB1 High-speed Host Controller"]
pub mod usbhsh;
#[doc = "Hash-Crypt peripheral"]
pub struct HASHCRYPT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HASHCRYPT {}
impl HASHCRYPT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hashcrypt::RegisterBlock {
        0x400a_4000 as *const _
    }
}
impl Deref for HASHCRYPT {
    type Target = hashcrypt::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*HASHCRYPT::ptr() }
    }
}
#[doc = "Hash-Crypt peripheral"]
pub mod hashcrypt;
#[doc = "CASPER"]
pub struct CASPER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CASPER {}
impl CASPER {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const casper::RegisterBlock {
        0x400a_5000 as *const _
    }
}
impl Deref for CASPER {
    type Target = casper::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CASPER::ptr() }
    }
}
#[doc = "CASPER"]
pub mod casper;
#[doc = "Digital Signal Co-Processing companion to a Cortex-M v8M CPU core"]
pub struct POWERQUAD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for POWERQUAD {}
impl POWERQUAD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const powerquad::RegisterBlock {
        0x400a_6000 as *const _
    }
}
impl Deref for POWERQUAD {
    type Target = powerquad::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*POWERQUAD::ptr() }
    }
}
#[doc = "Digital Signal Co-Processing companion to a Cortex-M v8M CPU core"]
pub mod powerquad;
#[doc = "AHB secure controller"]
pub struct AHB_SECURE_CTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AHB_SECURE_CTRL {}
impl AHB_SECURE_CTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ahb_secure_ctrl::RegisterBlock {
        0x400a_c000 as *const _
    }
}
impl Deref for AHB_SECURE_CTRL {
    type Target = ahb_secure_ctrl::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*AHB_SECURE_CTRL::ptr() }
    }
}
#[doc = "AHB secure controller"]
pub mod ahb_secure_ctrl;
#[doc = "no description available"]
pub struct SCNSCB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCNSCB {}
impl SCNSCB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scn_scb::RegisterBlock {
        0xe000_e000 as *const _
    }
}
impl Deref for SCNSCB {
    type Target = scn_scb::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCNSCB::ptr() }
    }
}
#[doc = "no description available"]
pub mod scn_scb;
#[doc = "no description available"]
pub struct SAU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAU {}
impl SAU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sau::RegisterBlock {
        0xe000_edd0 as *const _
    }
}
impl Deref for SAU {
    type Target = sau::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SAU::ptr() }
    }
}
#[doc = "no description available"]
pub mod sau;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "FLASH_CFPA0"]
    pub FLASH_CFPA0: FLASH_CFPA0,
    #[doc = "FLASH_CFPA_SCRATCH"]
    pub FLASH_CFPA_SCRATCH: FLASH_CFPA_SCRATCH,
    #[doc = "FLASH_CFPA1"]
    pub FLASH_CFPA1: FLASH_CFPA1,
    #[doc = "FLASH_CMPA"]
    pub FLASH_CMPA: FLASH_CMPA,
    #[doc = "FLASH_KEY_STORE"]
    pub FLASH_KEY_STORE: FLASH_KEY_STORE,
    #[doc = "SYSCON"]
    pub SYSCON: SYSCON,
    #[doc = "IOCON"]
    pub IOCON: IOCON,
    #[doc = "GINT0"]
    pub GINT0: GINT0,
    #[doc = "GINT1"]
    pub GINT1: GINT1,
    #[doc = "PINT"]
    pub PINT: PINT,
    #[doc = "SECPINT"]
    pub SECPINT: SECPINT,
    #[doc = "CTIMER0"]
    pub CTIMER0: CTIMER0,
    #[doc = "CTIMER1"]
    pub CTIMER1: CTIMER1,
    #[doc = "CTIMER2"]
    pub CTIMER2: CTIMER2,
    #[doc = "CTIMER3"]
    pub CTIMER3: CTIMER3,
    #[doc = "CTIMER4"]
    pub CTIMER4: CTIMER4,
    #[doc = "WWDT"]
    pub WWDT: WWDT,
    #[doc = "MRT0"]
    pub MRT0: MRT0,
    #[doc = "UTICK"]
    pub UTICK: UTICK,
    #[doc = "ANACTRL"]
    pub ANACTRL: ANACTRL,
    #[doc = "PMC"]
    pub PMC: PMC,
    #[doc = "SYSCTL"]
    pub SYSCTL: SYSCTL,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "OSTIMER"]
    pub OSTIMER: OSTIMER,
    #[doc = "FLASH"]
    pub FLASH: FLASH,
    #[doc = "PRINCE"]
    pub PRINCE: PRINCE,
    #[doc = "USBPHY"]
    pub USBPHY: USBPHY,
    #[doc = "RNG"]
    pub RNG: RNG,
    #[doc = "PUF"]
    pub PUF: PUF,
    #[doc = "PLU"]
    pub PLU: PLU,
    #[doc = "DMA0"]
    pub DMA0: DMA0,
    #[doc = "DMA1"]
    pub DMA1: DMA1,
    #[doc = "USB0"]
    pub USB0: USB0,
    #[doc = "SCT0"]
    pub SCT0: SCT0,
    #[doc = "FLEXCOMM0"]
    pub FLEXCOMM0: FLEXCOMM0,
    #[doc = "FLEXCOMM1"]
    pub FLEXCOMM1: FLEXCOMM1,
    #[doc = "FLEXCOMM2"]
    pub FLEXCOMM2: FLEXCOMM2,
    #[doc = "FLEXCOMM3"]
    pub FLEXCOMM3: FLEXCOMM3,
    #[doc = "FLEXCOMM4"]
    pub FLEXCOMM4: FLEXCOMM4,
    #[doc = "FLEXCOMM5"]
    pub FLEXCOMM5: FLEXCOMM5,
    #[doc = "FLEXCOMM6"]
    pub FLEXCOMM6: FLEXCOMM6,
    #[doc = "FLEXCOMM7"]
    pub FLEXCOMM7: FLEXCOMM7,
    #[doc = "FLEXCOMM8"]
    pub FLEXCOMM8: FLEXCOMM8,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "I2C2"]
    pub I2C2: I2C2,
    #[doc = "I2C3"]
    pub I2C3: I2C3,
    #[doc = "I2C4"]
    pub I2C4: I2C4,
    #[doc = "I2C5"]
    pub I2C5: I2C5,
    #[doc = "I2C6"]
    pub I2C6: I2C6,
    #[doc = "I2C7"]
    pub I2C7: I2C7,
    #[doc = "I2S0"]
    pub I2S0: I2S0,
    #[doc = "I2S1"]
    pub I2S1: I2S1,
    #[doc = "I2S2"]
    pub I2S2: I2S2,
    #[doc = "I2S3"]
    pub I2S3: I2S3,
    #[doc = "I2S4"]
    pub I2S4: I2S4,
    #[doc = "I2S5"]
    pub I2S5: I2S5,
    #[doc = "I2S6"]
    pub I2S6: I2S6,
    #[doc = "I2S7"]
    pub I2S7: I2S7,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "SPI2"]
    pub SPI2: SPI2,
    #[doc = "SPI3"]
    pub SPI3: SPI3,
    #[doc = "SPI4"]
    pub SPI4: SPI4,
    #[doc = "SPI5"]
    pub SPI5: SPI5,
    #[doc = "SPI6"]
    pub SPI6: SPI6,
    #[doc = "SPI7"]
    pub SPI7: SPI7,
    #[doc = "SPI8"]
    pub SPI8: SPI8,
    #[doc = "USART0"]
    pub USART0: USART0,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "USART2"]
    pub USART2: USART2,
    #[doc = "USART3"]
    pub USART3: USART3,
    #[doc = "USART4"]
    pub USART4: USART4,
    #[doc = "USART5"]
    pub USART5: USART5,
    #[doc = "USART6"]
    pub USART6: USART6,
    #[doc = "USART7"]
    pub USART7: USART7,
    #[doc = "MAILBOX"]
    pub MAILBOX: MAILBOX,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "SECGPIO"]
    pub SECGPIO: SECGPIO,
    #[doc = "USBHSD"]
    pub USBHSD: USBHSD,
    #[doc = "CRC_ENGINE"]
    pub CRC_ENGINE: CRC_ENGINE,
    #[doc = "SDIF"]
    pub SDIF: SDIF,
    #[doc = "DGBMAILBOX"]
    pub DGBMAILBOX: DGBMAILBOX,
    #[doc = "ADC0"]
    pub ADC0: ADC0,
    #[doc = "USBFSH"]
    pub USBFSH: USBFSH,
    #[doc = "USBHSH"]
    pub USBHSH: USBHSH,
    #[doc = "HASHCRYPT"]
    pub HASHCRYPT: HASHCRYPT,
    #[doc = "CASPER"]
    pub CASPER: CASPER,
    #[doc = "POWERQUAD"]
    pub POWERQUAD: POWERQUAD,
    #[doc = "AHB_SECURE_CTRL"]
    pub AHB_SECURE_CTRL: AHB_SECURE_CTRL,
    #[doc = "SCNSCB"]
    pub SCNSCB: SCNSCB,
    #[doc = "SAU"]
    pub SAU: SAU,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            FLASH_CFPA0: FLASH_CFPA0 {
                _marker: PhantomData,
            },
            FLASH_CFPA_SCRATCH: FLASH_CFPA_SCRATCH {
                _marker: PhantomData,
            },
            FLASH_CFPA1: FLASH_CFPA1 {
                _marker: PhantomData,
            },
            FLASH_CMPA: FLASH_CMPA {
                _marker: PhantomData,
            },
            FLASH_KEY_STORE: FLASH_KEY_STORE {
                _marker: PhantomData,
            },
            SYSCON: SYSCON {
                _marker: PhantomData,
            },
            IOCON: IOCON {
                _marker: PhantomData,
            },
            GINT0: GINT0 {
                _marker: PhantomData,
            },
            GINT1: GINT1 {
                _marker: PhantomData,
            },
            PINT: PINT {
                _marker: PhantomData,
            },
            SECPINT: SECPINT {
                _marker: PhantomData,
            },
            CTIMER0: CTIMER0 {
                _marker: PhantomData,
            },
            CTIMER1: CTIMER1 {
                _marker: PhantomData,
            },
            CTIMER2: CTIMER2 {
                _marker: PhantomData,
            },
            CTIMER3: CTIMER3 {
                _marker: PhantomData,
            },
            CTIMER4: CTIMER4 {
                _marker: PhantomData,
            },
            WWDT: WWDT {
                _marker: PhantomData,
            },
            MRT0: MRT0 {
                _marker: PhantomData,
            },
            UTICK: UTICK {
                _marker: PhantomData,
            },
            ANACTRL: ANACTRL {
                _marker: PhantomData,
            },
            PMC: PMC {
                _marker: PhantomData,
            },
            SYSCTL: SYSCTL {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            OSTIMER: OSTIMER {
                _marker: PhantomData,
            },
            FLASH: FLASH {
                _marker: PhantomData,
            },
            PRINCE: PRINCE {
                _marker: PhantomData,
            },
            USBPHY: USBPHY {
                _marker: PhantomData,
            },
            RNG: RNG {
                _marker: PhantomData,
            },
            PUF: PUF {
                _marker: PhantomData,
            },
            PLU: PLU {
                _marker: PhantomData,
            },
            DMA0: DMA0 {
                _marker: PhantomData,
            },
            DMA1: DMA1 {
                _marker: PhantomData,
            },
            USB0: USB0 {
                _marker: PhantomData,
            },
            SCT0: SCT0 {
                _marker: PhantomData,
            },
            FLEXCOMM0: FLEXCOMM0 {
                _marker: PhantomData,
            },
            FLEXCOMM1: FLEXCOMM1 {
                _marker: PhantomData,
            },
            FLEXCOMM2: FLEXCOMM2 {
                _marker: PhantomData,
            },
            FLEXCOMM3: FLEXCOMM3 {
                _marker: PhantomData,
            },
            FLEXCOMM4: FLEXCOMM4 {
                _marker: PhantomData,
            },
            FLEXCOMM5: FLEXCOMM5 {
                _marker: PhantomData,
            },
            FLEXCOMM6: FLEXCOMM6 {
                _marker: PhantomData,
            },
            FLEXCOMM7: FLEXCOMM7 {
                _marker: PhantomData,
            },
            FLEXCOMM8: FLEXCOMM8 {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            I2C2: I2C2 {
                _marker: PhantomData,
            },
            I2C3: I2C3 {
                _marker: PhantomData,
            },
            I2C4: I2C4 {
                _marker: PhantomData,
            },
            I2C5: I2C5 {
                _marker: PhantomData,
            },
            I2C6: I2C6 {
                _marker: PhantomData,
            },
            I2C7: I2C7 {
                _marker: PhantomData,
            },
            I2S0: I2S0 {
                _marker: PhantomData,
            },
            I2S1: I2S1 {
                _marker: PhantomData,
            },
            I2S2: I2S2 {
                _marker: PhantomData,
            },
            I2S3: I2S3 {
                _marker: PhantomData,
            },
            I2S4: I2S4 {
                _marker: PhantomData,
            },
            I2S5: I2S5 {
                _marker: PhantomData,
            },
            I2S6: I2S6 {
                _marker: PhantomData,
            },
            I2S7: I2S7 {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            SPI2: SPI2 {
                _marker: PhantomData,
            },
            SPI3: SPI3 {
                _marker: PhantomData,
            },
            SPI4: SPI4 {
                _marker: PhantomData,
            },
            SPI5: SPI5 {
                _marker: PhantomData,
            },
            SPI6: SPI6 {
                _marker: PhantomData,
            },
            SPI7: SPI7 {
                _marker: PhantomData,
            },
            SPI8: SPI8 {
                _marker: PhantomData,
            },
            USART0: USART0 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            USART2: USART2 {
                _marker: PhantomData,
            },
            USART3: USART3 {
                _marker: PhantomData,
            },
            USART4: USART4 {
                _marker: PhantomData,
            },
            USART5: USART5 {
                _marker: PhantomData,
            },
            USART6: USART6 {
                _marker: PhantomData,
            },
            USART7: USART7 {
                _marker: PhantomData,
            },
            MAILBOX: MAILBOX {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
            SECGPIO: SECGPIO {
                _marker: PhantomData,
            },
            USBHSD: USBHSD {
                _marker: PhantomData,
            },
            CRC_ENGINE: CRC_ENGINE {
                _marker: PhantomData,
            },
            SDIF: SDIF {
                _marker: PhantomData,
            },
            DGBMAILBOX: DGBMAILBOX {
                _marker: PhantomData,
            },
            ADC0: ADC0 {
                _marker: PhantomData,
            },
            USBFSH: USBFSH {
                _marker: PhantomData,
            },
            USBHSH: USBHSH {
                _marker: PhantomData,
            },
            HASHCRYPT: HASHCRYPT {
                _marker: PhantomData,
            },
            CASPER: CASPER {
                _marker: PhantomData,
            },
            POWERQUAD: POWERQUAD {
                _marker: PhantomData,
            },
            AHB_SECURE_CTRL: AHB_SECURE_CTRL {
                _marker: PhantomData,
            },
            SCNSCB: SCNSCB {
                _marker: PhantomData,
            },
            SAU: SAU {
                _marker: PhantomData,
            },
        }
    }
}
