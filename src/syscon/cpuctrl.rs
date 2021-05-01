#[doc = "Register `CPUCTRL` reader"]
pub struct R(crate::R<CPUCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPUCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CPUCTRL_SPEC>> for R {
    fn from(reader: crate::R<CPUCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPUCTRL` writer"]
pub struct W(crate::W<CPUCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPUCTRL_SPEC>;
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
impl core::convert::From<crate::W<CPUCTRL_SPEC>> for W {
    fn from(writer: crate::W<CPUCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CPU1 clock enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU1CLKEN_A {
    #[doc = "0: The CPU1 clock is not enabled."]
    DISABLE = 0,
    #[doc = "1: The CPU1 clock is enabled."]
    ENABLE = 1,
}
impl From<CPU1CLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: CPU1CLKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU1CLKEN` reader - CPU1 clock enable."]
pub struct CPU1CLKEN_R(crate::FieldReader<bool, CPU1CLKEN_A>);
impl CPU1CLKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPU1CLKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPU1CLKEN_A {
        match self.bits {
            false => CPU1CLKEN_A::DISABLE,
            true => CPU1CLKEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CPU1CLKEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CPU1CLKEN_A::ENABLE
    }
}
impl core::ops::Deref for CPU1CLKEN_R {
    type Target = crate::FieldReader<bool, CPU1CLKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU1CLKEN` writer - CPU1 clock enable."]
pub struct CPU1CLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU1CLKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPU1CLKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The CPU1 clock is not enabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CPU1CLKEN_A::DISABLE)
    }
    #[doc = "The CPU1 clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CPU1CLKEN_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "CPU1 reset.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU1RSTEN_A {
    #[doc = "0: The CPU1 is not being reset."]
    RELEASED = 0,
    #[doc = "1: The CPU1 is being reset."]
    ASSERTED = 1,
}
impl From<CPU1RSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: CPU1RSTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU1RSTEN` reader - CPU1 reset."]
pub struct CPU1RSTEN_R(crate::FieldReader<bool, CPU1RSTEN_A>);
impl CPU1RSTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPU1RSTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPU1RSTEN_A {
        match self.bits {
            false => CPU1RSTEN_A::RELEASED,
            true => CPU1RSTEN_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        **self == CPU1RSTEN_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == CPU1RSTEN_A::ASSERTED
    }
}
impl core::ops::Deref for CPU1RSTEN_R {
    type Target = crate::FieldReader<bool, CPU1RSTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU1RSTEN` writer - CPU1 reset."]
pub struct CPU1RSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU1RSTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPU1RSTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The CPU1 is not being reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(CPU1RSTEN_A::RELEASED)
    }
    #[doc = "The CPU1 is being reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(CPU1RSTEN_A::ASSERTED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - CPU1 clock enable."]
    #[inline(always)]
    pub fn cpu1clken(&self) -> CPU1CLKEN_R {
        CPU1CLKEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CPU1 reset."]
    #[inline(always)]
    pub fn cpu1rsten(&self) -> CPU1RSTEN_R {
        CPU1RSTEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - CPU1 clock enable."]
    #[inline(always)]
    pub fn cpu1clken(&mut self) -> CPU1CLKEN_W {
        CPU1CLKEN_W { w: self }
    }
    #[doc = "Bit 5 - CPU1 reset."]
    #[inline(always)]
    pub fn cpu1rsten(&mut self) -> CPU1RSTEN_W {
        CPU1RSTEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU Control for multiple processors\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuctrl](index.html) module"]
pub struct CPUCTRL_SPEC;
impl crate::RegisterSpec for CPUCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpuctrl::R](R) reader structure"]
impl crate::Readable for CPUCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpuctrl::W](W) writer structure"]
impl crate::Writable for CPUCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPUCTRL to value 0x2c"]
impl crate::Resettable for CPUCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2c
    }
}
