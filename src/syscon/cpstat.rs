#[doc = "Reader of register CPSTAT"]
pub type R = crate::R<u32, super::CPSTAT>;
#[doc = "Writer for register CPSTAT"]
pub type W = crate::W<u32, super::CPSTAT>;
#[doc = "Register CPSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::CPSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "The CPU0 sleeping state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU0SLEEPING_A {
    #[doc = "0: the CPU is not sleeping."]
    AWAKE = 0,
    #[doc = "1: the CPU is sleeping."]
    SLEEPING = 1,
}
impl From<CPU0SLEEPING_A> for bool {
    #[inline(always)]
    fn from(variant: CPU0SLEEPING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPU0SLEEPING`"]
pub type CPU0SLEEPING_R = crate::R<bool, CPU0SLEEPING_A>;
impl CPU0SLEEPING_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPU0SLEEPING_A {
        match self.bits {
            false => CPU0SLEEPING_A::AWAKE,
            true => CPU0SLEEPING_A::SLEEPING,
        }
    }
    #[doc = "Checks if the value of the field is `AWAKE`"]
    #[inline(always)]
    pub fn is_awake(&self) -> bool {
        *self == CPU0SLEEPING_A::AWAKE
    }
    #[doc = "Checks if the value of the field is `SLEEPING`"]
    #[inline(always)]
    pub fn is_sleeping(&self) -> bool {
        *self == CPU0SLEEPING_A::SLEEPING
    }
}
#[doc = "The CPU1 sleeping state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU1SLEEPING_A {
    #[doc = "0: the CPU is not sleeping."]
    AWAKE = 0,
    #[doc = "1: the CPU is sleeping."]
    SLEEPING = 1,
}
impl From<CPU1SLEEPING_A> for bool {
    #[inline(always)]
    fn from(variant: CPU1SLEEPING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPU1SLEEPING`"]
pub type CPU1SLEEPING_R = crate::R<bool, CPU1SLEEPING_A>;
impl CPU1SLEEPING_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPU1SLEEPING_A {
        match self.bits {
            false => CPU1SLEEPING_A::AWAKE,
            true => CPU1SLEEPING_A::SLEEPING,
        }
    }
    #[doc = "Checks if the value of the field is `AWAKE`"]
    #[inline(always)]
    pub fn is_awake(&self) -> bool {
        *self == CPU1SLEEPING_A::AWAKE
    }
    #[doc = "Checks if the value of the field is `SLEEPING`"]
    #[inline(always)]
    pub fn is_sleeping(&self) -> bool {
        *self == CPU1SLEEPING_A::SLEEPING
    }
}
#[doc = "The CPU0 lockup state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU0LOCKUP_A {
    #[doc = "0: the CPU is not in lockup."]
    AWAKE = 0,
    #[doc = "1: the CPU is in lockup."]
    SLEEPING = 1,
}
impl From<CPU0LOCKUP_A> for bool {
    #[inline(always)]
    fn from(variant: CPU0LOCKUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPU0LOCKUP`"]
pub type CPU0LOCKUP_R = crate::R<bool, CPU0LOCKUP_A>;
impl CPU0LOCKUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPU0LOCKUP_A {
        match self.bits {
            false => CPU0LOCKUP_A::AWAKE,
            true => CPU0LOCKUP_A::SLEEPING,
        }
    }
    #[doc = "Checks if the value of the field is `AWAKE`"]
    #[inline(always)]
    pub fn is_awake(&self) -> bool {
        *self == CPU0LOCKUP_A::AWAKE
    }
    #[doc = "Checks if the value of the field is `SLEEPING`"]
    #[inline(always)]
    pub fn is_sleeping(&self) -> bool {
        *self == CPU0LOCKUP_A::SLEEPING
    }
}
#[doc = "The CPU1 lockup state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU1LOCKUP_A {
    #[doc = "0: the CPU is not in lockup."]
    AWAKE = 0,
    #[doc = "1: the CPU is in lockup."]
    SLEEPING = 1,
}
impl From<CPU1LOCKUP_A> for bool {
    #[inline(always)]
    fn from(variant: CPU1LOCKUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPU1LOCKUP`"]
pub type CPU1LOCKUP_R = crate::R<bool, CPU1LOCKUP_A>;
impl CPU1LOCKUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPU1LOCKUP_A {
        match self.bits {
            false => CPU1LOCKUP_A::AWAKE,
            true => CPU1LOCKUP_A::SLEEPING,
        }
    }
    #[doc = "Checks if the value of the field is `AWAKE`"]
    #[inline(always)]
    pub fn is_awake(&self) -> bool {
        *self == CPU1LOCKUP_A::AWAKE
    }
    #[doc = "Checks if the value of the field is `SLEEPING`"]
    #[inline(always)]
    pub fn is_sleeping(&self) -> bool {
        *self == CPU1LOCKUP_A::SLEEPING
    }
}
impl R {
    #[doc = "Bit 0 - The CPU0 sleeping state."]
    #[inline(always)]
    pub fn cpu0sleeping(&self) -> CPU0SLEEPING_R {
        CPU0SLEEPING_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - The CPU1 sleeping state."]
    #[inline(always)]
    pub fn cpu1sleeping(&self) -> CPU1SLEEPING_R {
        CPU1SLEEPING_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The CPU0 lockup state."]
    #[inline(always)]
    pub fn cpu0lockup(&self) -> CPU0LOCKUP_R {
        CPU0LOCKUP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - The CPU1 lockup state."]
    #[inline(always)]
    pub fn cpu1lockup(&self) -> CPU1LOCKUP_R {
        CPU1LOCKUP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {}
