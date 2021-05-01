#[doc = "Register `MCLKIO` reader"]
pub struct R(crate::R<MCLKIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCLKIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MCLKIO_SPEC>> for R {
    fn from(reader: crate::R<MCLKIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCLKIO` writer"]
pub struct W(crate::W<MCLKIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCLKIO_SPEC>;
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
impl core::convert::From<crate::W<MCLKIO_SPEC>> for W {
    fn from(writer: crate::W<MCLKIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "MCLK control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCLKIO_A {
    #[doc = "0: input mode."]
    INPUT = 0,
    #[doc = "1: output mode."]
    OUTPUT = 1,
}
impl From<MCLKIO_A> for bool {
    #[inline(always)]
    fn from(variant: MCLKIO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCLKIO` reader - MCLK control."]
pub struct MCLKIO_R(crate::FieldReader<bool, MCLKIO_A>);
impl MCLKIO_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCLKIO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCLKIO_A {
        match self.bits {
            false => MCLKIO_A::INPUT,
            true => MCLKIO_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        **self == MCLKIO_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        **self == MCLKIO_A::OUTPUT
    }
}
impl core::ops::Deref for MCLKIO_R {
    type Target = crate::FieldReader<bool, MCLKIO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCLKIO` writer - MCLK control."]
pub struct MCLKIO_W<'a> {
    w: &'a mut W,
}
impl<'a> MCLKIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCLKIO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "input mode."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MCLKIO_A::INPUT)
    }
    #[doc = "output mode."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MCLKIO_A::OUTPUT)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - MCLK control."]
    #[inline(always)]
    pub fn mclkio(&self) -> MCLKIO_R {
        MCLKIO_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MCLK control."]
    #[inline(always)]
    pub fn mclkio(&mut self) -> MCLKIO_W {
        MCLKIO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCLK control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mclkio](index.html) module"]
pub struct MCLKIO_SPEC;
impl crate::RegisterSpec for MCLKIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mclkio::R](R) reader structure"]
impl crate::Readable for MCLKIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mclkio::W](W) writer structure"]
impl crate::Writable for MCLKIO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCLKIO to value 0"]
impl crate::Resettable for MCLKIO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
