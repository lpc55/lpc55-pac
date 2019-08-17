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
#[doc = "Possible values of the field `CPU0SLEEPING`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU0SLEEPING_A {
    #[doc = "the CPU is not sleeping."]
    AWAKE,
    #[doc = "the CPU is sleeping."]
    SLEEPING,
}
impl From<CPU0SLEEPING_A> for bool {
    #[inline(always)]
    fn from(variant: CPU0SLEEPING_A) -> Self {
        match variant {
            CPU0SLEEPING_A::AWAKE => false,
            CPU0SLEEPING_A::SLEEPING => true,
        }
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
#[doc = "Possible values of the field `CPU1SLEEPING`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU1SLEEPING_A {
    #[doc = "the CPU is not sleeping."]
    AWAKE,
    #[doc = "the CPU is sleeping."]
    SLEEPING,
}
impl From<CPU1SLEEPING_A> for bool {
    #[inline(always)]
    fn from(variant: CPU1SLEEPING_A) -> Self {
        match variant {
            CPU1SLEEPING_A::AWAKE => false,
            CPU1SLEEPING_A::SLEEPING => true,
        }
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
#[doc = "Possible values of the field `CPU0LOCKUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU0LOCKUP_A {
    #[doc = "the CPU is not in lockup."]
    AWAKE,
    #[doc = "the CPU is in lockup."]
    SLEEPING,
}
impl From<CPU0LOCKUP_A> for bool {
    #[inline(always)]
    fn from(variant: CPU0LOCKUP_A) -> Self {
        match variant {
            CPU0LOCKUP_A::AWAKE => false,
            CPU0LOCKUP_A::SLEEPING => true,
        }
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
#[doc = "Possible values of the field `CPU1LOCKUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU1LOCKUP_A {
    #[doc = "the CPU is not in lockup."]
    AWAKE,
    #[doc = "the CPU is in lockup."]
    SLEEPING,
}
impl From<CPU1LOCKUP_A> for bool {
    #[inline(always)]
    fn from(variant: CPU1LOCKUP_A) -> Self {
        match variant {
            CPU1LOCKUP_A::AWAKE => false,
            CPU1LOCKUP_A::SLEEPING => true,
        }
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
