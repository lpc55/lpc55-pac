#[doc = "Register `PAUSE` reader"]
pub struct R(crate::R<PAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAUSE` writer"]
pub struct W(crate::W<PAUSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAUSE_SPEC>;
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
impl From<crate::W<PAUSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAUSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAUSEDLY` reader - Pause Delay"]
pub struct PAUSEDLY_R(crate::FieldReader<u16, u16>);
impl PAUSEDLY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PAUSEDLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAUSEDLY_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAUSEDLY` writer - Pause Delay"]
pub struct PAUSEDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> PAUSEDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
#[doc = "PAUSE Option Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAUSEEN_A {
    #[doc = "0: Pause operation disabled"]
    PAUSEEN_0 = 0,
    #[doc = "1: Pause operation enabled"]
    PAUSEEN_1 = 1,
}
impl From<PAUSEEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAUSEEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAUSEEN` reader - PAUSE Option Enable"]
pub struct PAUSEEN_R(crate::FieldReader<bool, PAUSEEN_A>);
impl PAUSEEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAUSEEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAUSEEN_A {
        match self.bits {
            false => PAUSEEN_A::PAUSEEN_0,
            true => PAUSEEN_A::PAUSEEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `PAUSEEN_0`"]
    #[inline(always)]
    pub fn is_pauseen_0(&self) -> bool {
        **self == PAUSEEN_A::PAUSEEN_0
    }
    #[doc = "Checks if the value of the field is `PAUSEEN_1`"]
    #[inline(always)]
    pub fn is_pauseen_1(&self) -> bool {
        **self == PAUSEEN_A::PAUSEEN_1
    }
}
impl core::ops::Deref for PAUSEEN_R {
    type Target = crate::FieldReader<bool, PAUSEEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAUSEEN` writer - PAUSE Option Enable"]
pub struct PAUSEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAUSEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAUSEEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pause operation disabled"]
    #[inline(always)]
    pub fn pauseen_0(self) -> &'a mut W {
        self.variant(PAUSEEN_A::PAUSEEN_0)
    }
    #[doc = "Pause operation enabled"]
    #[inline(always)]
    pub fn pauseen_1(self) -> &'a mut W {
        self.variant(PAUSEEN_A::PAUSEEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Pause Delay"]
    #[inline(always)]
    pub fn pausedly(&self) -> PAUSEDLY_R {
        PAUSEDLY_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 31 - PAUSE Option Enable"]
    #[inline(always)]
    pub fn pauseen(&self) -> PAUSEEN_R {
        PAUSEEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Pause Delay"]
    #[inline(always)]
    pub fn pausedly(&mut self) -> PAUSEDLY_W {
        PAUSEDLY_W { w: self }
    }
    #[doc = "Bit 31 - PAUSE Option Enable"]
    #[inline(always)]
    pub fn pauseen(&mut self) -> PAUSEEN_W {
        PAUSEEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Pause Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pause](index.html) module"]
pub struct PAUSE_SPEC;
impl crate::RegisterSpec for PAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pause::R](R) reader structure"]
impl crate::Readable for PAUSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pause::W](W) writer structure"]
impl crate::Writable for PAUSE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAUSE to value 0"]
impl crate::Resettable for PAUSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
