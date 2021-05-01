#[doc = "Register `CPUCFG` reader"]
pub struct R(crate::R<CPUCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPUCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CPUCFG_SPEC>> for R {
    fn from(reader: crate::R<CPUCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPUCFG` writer"]
pub struct W(crate::W<CPUCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPUCFG_SPEC>;
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
impl core::convert::From<crate::W<CPUCFG_SPEC>> for W {
    fn from(writer: crate::W<CPUCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable CPU1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU1ENABLE_A {
    #[doc = "0: CPU1 is disable (Processor in reset)."]
    DISABLE = 0,
    #[doc = "1: CPU1 is enable."]
    ENABLE = 1,
}
impl From<CPU1ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: CPU1ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU1ENABLE` reader - Enable CPU1."]
pub struct CPU1ENABLE_R(crate::FieldReader<bool, CPU1ENABLE_A>);
impl CPU1ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPU1ENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPU1ENABLE_A {
        match self.bits {
            false => CPU1ENABLE_A::DISABLE,
            true => CPU1ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CPU1ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CPU1ENABLE_A::ENABLE
    }
}
impl core::ops::Deref for CPU1ENABLE_R {
    type Target = crate::FieldReader<bool, CPU1ENABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU1ENABLE` writer - Enable CPU1."]
pub struct CPU1ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU1ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPU1ENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CPU1 is disable (Processor in reset)."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CPU1ENABLE_A::DISABLE)
    }
    #[doc = "CPU1 is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CPU1ENABLE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Enable CPU1."]
    #[inline(always)]
    pub fn cpu1enable(&self) -> CPU1ENABLE_R {
        CPU1ENABLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Enable CPU1."]
    #[inline(always)]
    pub fn cpu1enable(&mut self) -> CPU1ENABLE_W {
        CPU1ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPUs configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpucfg](index.html) module"]
pub struct CPUCFG_SPEC;
impl crate::RegisterSpec for CPUCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpucfg::R](R) reader structure"]
impl crate::Readable for CPUCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpucfg::W](W) writer structure"]
impl crate::Writable for CPUCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPUCFG to value 0x02"]
impl crate::Resettable for CPUCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
