#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CPSTAT {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `CPU0SLEEPING`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU0SLEEPINGR {
    #[doc = "the CPU is not sleeping."]
    AWAKE,
    #[doc = "the CPU is sleeping."]
    SLEEPING,
}
impl CPU0SLEEPINGR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CPU0SLEEPINGR::AWAKE => false,
            CPU0SLEEPINGR::SLEEPING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPU0SLEEPINGR {
        match value {
            false => CPU0SLEEPINGR::AWAKE,
            true => CPU0SLEEPINGR::SLEEPING,
        }
    }
    #[doc = "Checks if the value of the field is `AWAKE`"]
    #[inline]
    pub fn is_awake(&self) -> bool {
        *self == CPU0SLEEPINGR::AWAKE
    }
    #[doc = "Checks if the value of the field is `SLEEPING`"]
    #[inline]
    pub fn is_sleeping(&self) -> bool {
        *self == CPU0SLEEPINGR::SLEEPING
    }
}
#[doc = "Possible values of the field `CPU1SLEEPING`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU1SLEEPINGR {
    #[doc = "the CPU is not sleeping."]
    AWAKE,
    #[doc = "the CPU is sleeping."]
    SLEEPING,
}
impl CPU1SLEEPINGR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CPU1SLEEPINGR::AWAKE => false,
            CPU1SLEEPINGR::SLEEPING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPU1SLEEPINGR {
        match value {
            false => CPU1SLEEPINGR::AWAKE,
            true => CPU1SLEEPINGR::SLEEPING,
        }
    }
    #[doc = "Checks if the value of the field is `AWAKE`"]
    #[inline]
    pub fn is_awake(&self) -> bool {
        *self == CPU1SLEEPINGR::AWAKE
    }
    #[doc = "Checks if the value of the field is `SLEEPING`"]
    #[inline]
    pub fn is_sleeping(&self) -> bool {
        *self == CPU1SLEEPINGR::SLEEPING
    }
}
#[doc = "Possible values of the field `CPU0LOCKUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU0LOCKUPR {
    #[doc = "the CPU is not in lockup."]
    AWAKE,
    #[doc = "the CPU is in lockup."]
    SLEEPING,
}
impl CPU0LOCKUPR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CPU0LOCKUPR::AWAKE => false,
            CPU0LOCKUPR::SLEEPING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPU0LOCKUPR {
        match value {
            false => CPU0LOCKUPR::AWAKE,
            true => CPU0LOCKUPR::SLEEPING,
        }
    }
    #[doc = "Checks if the value of the field is `AWAKE`"]
    #[inline]
    pub fn is_awake(&self) -> bool {
        *self == CPU0LOCKUPR::AWAKE
    }
    #[doc = "Checks if the value of the field is `SLEEPING`"]
    #[inline]
    pub fn is_sleeping(&self) -> bool {
        *self == CPU0LOCKUPR::SLEEPING
    }
}
#[doc = "Possible values of the field `CPU1LOCKUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU1LOCKUPR {
    #[doc = "the CPU is not in lockup."]
    AWAKE,
    #[doc = "the CPU is in lockup."]
    SLEEPING,
}
impl CPU1LOCKUPR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CPU1LOCKUPR::AWAKE => false,
            CPU1LOCKUPR::SLEEPING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPU1LOCKUPR {
        match value {
            false => CPU1LOCKUPR::AWAKE,
            true => CPU1LOCKUPR::SLEEPING,
        }
    }
    #[doc = "Checks if the value of the field is `AWAKE`"]
    #[inline]
    pub fn is_awake(&self) -> bool {
        *self == CPU1LOCKUPR::AWAKE
    }
    #[doc = "Checks if the value of the field is `SLEEPING`"]
    #[inline]
    pub fn is_sleeping(&self) -> bool {
        *self == CPU1LOCKUPR::SLEEPING
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - The CPU0 sleeping state."]
    #[inline]
    pub fn cpu0sleeping(&self) -> CPU0SLEEPINGR {
        CPU0SLEEPINGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - The CPU1 sleeping state."]
    #[inline]
    pub fn cpu1sleeping(&self) -> CPU1SLEEPINGR {
        CPU1SLEEPINGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - The CPU0 lockup state."]
    #[inline]
    pub fn cpu0lockup(&self) -> CPU0LOCKUPR {
        CPU0LOCKUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - The CPU1 lockup state."]
    #[inline]
    pub fn cpu1lockup(&self) -> CPU1LOCKUPR {
        CPU1LOCKUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
