#[doc = "Register `CPSTAT` reader"]
pub struct R(crate::R<CPSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CPSTAT_SPEC>> for R {
    fn from(reader: crate::R<CPSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPSTAT` writer"]
pub struct W(crate::W<CPSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<CPSTAT_SPEC>> for W {
    fn from(writer: crate::W<CPSTAT_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `CPU0SLEEPING` reader - The CPU0 sleeping state."]
pub struct CPU0SLEEPING_R(crate::FieldReader<bool, CPU0SLEEPING_A>);
impl CPU0SLEEPING_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPU0SLEEPING_R(crate::FieldReader::new(bits))
    }
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
        **self == CPU0SLEEPING_A::AWAKE
    }
    #[doc = "Checks if the value of the field is `SLEEPING`"]
    #[inline(always)]
    pub fn is_sleeping(&self) -> bool {
        **self == CPU0SLEEPING_A::SLEEPING
    }
}
impl core::ops::Deref for CPU0SLEEPING_R {
    type Target = crate::FieldReader<bool, CPU0SLEEPING_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `CPU1SLEEPING` reader - The CPU1 sleeping state."]
pub struct CPU1SLEEPING_R(crate::FieldReader<bool, CPU1SLEEPING_A>);
impl CPU1SLEEPING_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPU1SLEEPING_R(crate::FieldReader::new(bits))
    }
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
        **self == CPU1SLEEPING_A::AWAKE
    }
    #[doc = "Checks if the value of the field is `SLEEPING`"]
    #[inline(always)]
    pub fn is_sleeping(&self) -> bool {
        **self == CPU1SLEEPING_A::SLEEPING
    }
}
impl core::ops::Deref for CPU1SLEEPING_R {
    type Target = crate::FieldReader<bool, CPU1SLEEPING_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `CPU0LOCKUP` reader - The CPU0 lockup state."]
pub struct CPU0LOCKUP_R(crate::FieldReader<bool, CPU0LOCKUP_A>);
impl CPU0LOCKUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPU0LOCKUP_R(crate::FieldReader::new(bits))
    }
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
        **self == CPU0LOCKUP_A::AWAKE
    }
    #[doc = "Checks if the value of the field is `SLEEPING`"]
    #[inline(always)]
    pub fn is_sleeping(&self) -> bool {
        **self == CPU0LOCKUP_A::SLEEPING
    }
}
impl core::ops::Deref for CPU0LOCKUP_R {
    type Target = crate::FieldReader<bool, CPU0LOCKUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `CPU1LOCKUP` reader - The CPU1 lockup state."]
pub struct CPU1LOCKUP_R(crate::FieldReader<bool, CPU1LOCKUP_A>);
impl CPU1LOCKUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPU1LOCKUP_R(crate::FieldReader::new(bits))
    }
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
        **self == CPU1LOCKUP_A::AWAKE
    }
    #[doc = "Checks if the value of the field is `SLEEPING`"]
    #[inline(always)]
    pub fn is_sleeping(&self) -> bool {
        **self == CPU1LOCKUP_A::SLEEPING
    }
}
impl core::ops::Deref for CPU1LOCKUP_R {
    type Target = crate::FieldReader<bool, CPU1LOCKUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
impl W {
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpstat](index.html) module"]
pub struct CPSTAT_SPEC;
impl crate::RegisterSpec for CPSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpstat::R](R) reader structure"]
impl crate::Readable for CPSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpstat::W](W) writer structure"]
impl crate::Writable for CPSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPSTAT to value 0"]
impl crate::Resettable for CPSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
