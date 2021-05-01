#[doc = "Register `CLOCKGENUPDATELOCKOUT` reader"]
pub struct R(crate::R<CLOCKGENUPDATELOCKOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCKGENUPDATELOCKOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CLOCKGENUPDATELOCKOUT_SPEC>> for R {
    fn from(reader: crate::R<CLOCKGENUPDATELOCKOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLOCKGENUPDATELOCKOUT` writer"]
pub struct W(crate::W<CLOCKGENUPDATELOCKOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLOCKGENUPDATELOCKOUT_SPEC>;
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
impl core::convert::From<crate::W<CLOCKGENUPDATELOCKOUT_SPEC>> for W {
    fn from(writer: crate::W<CLOCKGENUPDATELOCKOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Control clock configuration registers access (like xxxDIV, xxxSEL).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum CLOCKGENUPDATELOCKOUT_A {
    #[doc = "0: all hardware clock configruration are freeze."]
    FREEZE = 0,
    #[doc = "1: update all clock configuration."]
    ENABLE = 1,
}
impl From<CLOCKGENUPDATELOCKOUT_A> for u32 {
    #[inline(always)]
    fn from(variant: CLOCKGENUPDATELOCKOUT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLOCKGENUPDATELOCKOUT` reader - Control clock configuration registers access (like xxxDIV, xxxSEL)."]
pub struct CLOCKGENUPDATELOCKOUT_R(crate::FieldReader<u32, CLOCKGENUPDATELOCKOUT_A>);
impl CLOCKGENUPDATELOCKOUT_R {
    pub(crate) fn new(bits: u32) -> Self {
        CLOCKGENUPDATELOCKOUT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLOCKGENUPDATELOCKOUT_A> {
        match self.bits {
            0 => Some(CLOCKGENUPDATELOCKOUT_A::FREEZE),
            1 => Some(CLOCKGENUPDATELOCKOUT_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FREEZE`"]
    #[inline(always)]
    pub fn is_freeze(&self) -> bool {
        **self == CLOCKGENUPDATELOCKOUT_A::FREEZE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CLOCKGENUPDATELOCKOUT_A::ENABLE
    }
}
impl core::ops::Deref for CLOCKGENUPDATELOCKOUT_R {
    type Target = crate::FieldReader<u32, CLOCKGENUPDATELOCKOUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLOCKGENUPDATELOCKOUT` writer - Control clock configuration registers access (like xxxDIV, xxxSEL)."]
pub struct CLOCKGENUPDATELOCKOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCKGENUPDATELOCKOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLOCKGENUPDATELOCKOUT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "all hardware clock configruration are freeze."]
    #[inline(always)]
    pub fn freeze(self) -> &'a mut W {
        self.variant(CLOCKGENUPDATELOCKOUT_A::FREEZE)
    }
    #[doc = "update all clock configuration."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CLOCKGENUPDATELOCKOUT_A::ENABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Control clock configuration registers access (like xxxDIV, xxxSEL)."]
    #[inline(always)]
    pub fn clockgenupdatelockout(&self) -> CLOCKGENUPDATELOCKOUT_R {
        CLOCKGENUPDATELOCKOUT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Control clock configuration registers access (like xxxDIV, xxxSEL)."]
    #[inline(always)]
    pub fn clockgenupdatelockout(&mut self) -> CLOCKGENUPDATELOCKOUT_W {
        CLOCKGENUPDATELOCKOUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control clock configuration registers access (like xxxDIV, xxxSEL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clockgenupdatelockout](index.html) module"]
pub struct CLOCKGENUPDATELOCKOUT_SPEC;
impl crate::RegisterSpec for CLOCKGENUPDATELOCKOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clockgenupdatelockout::R](R) reader structure"]
impl crate::Readable for CLOCKGENUPDATELOCKOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clockgenupdatelockout::W](W) writer structure"]
impl crate::Writable for CLOCKGENUPDATELOCKOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLOCKGENUPDATELOCKOUT to value 0"]
impl crate::Resettable for CLOCKGENUPDATELOCKOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
