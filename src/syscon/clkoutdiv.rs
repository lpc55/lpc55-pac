#[doc = "Register `CLKOUTDIV` reader"]
pub struct R(crate::R<CLKOUTDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKOUTDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKOUTDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKOUTDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKOUTDIV` writer"]
pub struct W(crate::W<CLKOUTDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKOUTDIV_SPEC>;
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
impl From<crate::W<CLKOUTDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKOUTDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV` reader - Clock divider value."]
pub struct DIV_R(crate::FieldReader<u8, u8>);
impl DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV` writer - Clock divider value."]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Resets the divider counter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESET_AW {
    #[doc = "0: Divider is not reset."]
    RELEASED = 0,
    #[doc = "1: Divider is reset."]
    ASSERTED = 1,
}
impl From<RESET_AW> for bool {
    #[inline(always)]
    fn from(variant: RESET_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESET` writer - Resets the divider counter."]
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESET_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Divider is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(RESET_AW::RELEASED)
    }
    #[doc = "Divider is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(RESET_AW::ASSERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Halts the divider counter.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALT_A {
    #[doc = "0: Divider clock is running."]
    RUN = 0,
    #[doc = "1: Divider clock is stoped."]
    HALT = 1,
}
impl From<HALT_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT` reader - Halts the divider counter."]
pub struct HALT_R(crate::FieldReader<bool, HALT_A>);
impl HALT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HALT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALT_A {
        match self.bits {
            false => HALT_A::RUN,
            true => HALT_A::HALT,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        **self == HALT_A::RUN
    }
    #[doc = "Checks if the value of the field is `HALT`"]
    #[inline(always)]
    pub fn is_halt(&self) -> bool {
        **self == HALT_A::HALT
    }
}
impl core::ops::Deref for HALT_R {
    type Target = crate::FieldReader<bool, HALT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALT` writer - Halts the divider counter."]
pub struct HALT_W<'a> {
    w: &'a mut W,
}
impl<'a> HALT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HALT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Divider clock is running."]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(HALT_A::RUN)
    }
    #[doc = "Divider clock is stoped."]
    #[inline(always)]
    pub fn halt(self) -> &'a mut W {
        self.variant(HALT_A::HALT)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Divider status flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REQFLAG_A {
    #[doc = "0: Divider clock is stable."]
    STABLE = 0,
    #[doc = "1: Clock frequency is not stable."]
    ONGOING = 1,
}
impl From<REQFLAG_A> for bool {
    #[inline(always)]
    fn from(variant: REQFLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REQFLAG` reader - Divider status flag."]
pub struct REQFLAG_R(crate::FieldReader<bool, REQFLAG_A>);
impl REQFLAG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REQFLAG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REQFLAG_A {
        match self.bits {
            false => REQFLAG_A::STABLE,
            true => REQFLAG_A::ONGOING,
        }
    }
    #[doc = "Checks if the value of the field is `STABLE`"]
    #[inline(always)]
    pub fn is_stable(&self) -> bool {
        **self == REQFLAG_A::STABLE
    }
    #[doc = "Checks if the value of the field is `ONGOING`"]
    #[inline(always)]
    pub fn is_ongoing(&self) -> bool {
        **self == REQFLAG_A::ONGOING
    }
}
impl core::ops::Deref for REQFLAG_R {
    type Target = crate::FieldReader<bool, REQFLAG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Clock divider value."]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 30 - Halts the divider counter."]
    #[inline(always)]
    pub fn halt(&self) -> HALT_R {
        HALT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Divider status flag."]
    #[inline(always)]
    pub fn reqflag(&self) -> REQFLAG_R {
        REQFLAG_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock divider value."]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
    #[doc = "Bit 29 - Resets the divider counter."]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    #[doc = "Bit 30 - Halts the divider counter."]
    #[inline(always)]
    pub fn halt(&mut self) -> HALT_W {
        HALT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CLKOUT clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkoutdiv](index.html) module"]
pub struct CLKOUTDIV_SPEC;
impl crate::RegisterSpec for CLKOUTDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkoutdiv::R](R) reader structure"]
impl crate::Readable for CLKOUTDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkoutdiv::W](W) writer structure"]
impl crate::Writable for CLKOUTDIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKOUTDIV to value 0x4000_0000"]
impl crate::Resettable for CLKOUTDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000_0000
    }
}
