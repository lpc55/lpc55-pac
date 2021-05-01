#[doc = "Register `DEBUG_LOCK_EN` reader"]
pub struct R(crate::R<DEBUG_LOCK_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUG_LOCK_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DEBUG_LOCK_EN_SPEC>> for R {
    fn from(reader: crate::R<DEBUG_LOCK_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEBUG_LOCK_EN` writer"]
pub struct W(crate::W<DEBUG_LOCK_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEBUG_LOCK_EN_SPEC>;
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
impl core::convert::From<crate::W<DEBUG_LOCK_EN_SPEC>> for W {
    fn from(writer: crate::W<DEBUG_LOCK_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Control write access to CODESECURITYPROTTEST, CODESECURITYPROTCPU0, CODESECURITYPROTCPU1, CPU0_DEBUG_FEATURES, CPU1_DEBUG_FEATURES and DBG_AUTH_SCRATCH registers.\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LOCK_ALL_A {
    #[doc = "0: Any other value than b1010: disable write access to all 6 registers."]
    DISABLE = 0,
    #[doc = "10: 1010: Enable write access to all 6 registers."]
    ENABLE = 10,
}
impl From<LOCK_ALL_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_ALL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LOCK_ALL` reader - Control write access to CODESECURITYPROTTEST, CODESECURITYPROTCPU0, CODESECURITYPROTCPU1, CPU0_DEBUG_FEATURES, CPU1_DEBUG_FEATURES and DBG_AUTH_SCRATCH registers."]
pub struct LOCK_ALL_R(crate::FieldReader<u8, LOCK_ALL_A>);
impl LOCK_ALL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LOCK_ALL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOCK_ALL_A> {
        match self.bits {
            0 => Some(LOCK_ALL_A::DISABLE),
            10 => Some(LOCK_ALL_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == LOCK_ALL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == LOCK_ALL_A::ENABLE
    }
}
impl core::ops::Deref for LOCK_ALL_R {
    type Target = crate::FieldReader<u8, LOCK_ALL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_ALL` writer - Control write access to CODESECURITYPROTTEST, CODESECURITYPROTCPU0, CODESECURITYPROTCPU1, CPU0_DEBUG_FEATURES, CPU1_DEBUG_FEATURES and DBG_AUTH_SCRATCH registers."]
pub struct LOCK_ALL_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_ALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_ALL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Any other value than b1010: disable write access to all 6 registers."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LOCK_ALL_A::DISABLE)
    }
    #[doc = "1010: Enable write access to all 6 registers."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LOCK_ALL_A::ENABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Control write access to CODESECURITYPROTTEST, CODESECURITYPROTCPU0, CODESECURITYPROTCPU1, CPU0_DEBUG_FEATURES, CPU1_DEBUG_FEATURES and DBG_AUTH_SCRATCH registers."]
    #[inline(always)]
    pub fn lock_all(&self) -> LOCK_ALL_R {
        LOCK_ALL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Control write access to CODESECURITYPROTTEST, CODESECURITYPROTCPU0, CODESECURITYPROTCPU1, CPU0_DEBUG_FEATURES, CPU1_DEBUG_FEATURES and DBG_AUTH_SCRATCH registers."]
    #[inline(always)]
    pub fn lock_all(&mut self) -> LOCK_ALL_W {
        LOCK_ALL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control write access to security registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug_lock_en](index.html) module"]
pub struct DEBUG_LOCK_EN_SPEC;
impl crate::RegisterSpec for DEBUG_LOCK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debug_lock_en::R](R) reader structure"]
impl crate::Readable for DEBUG_LOCK_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [debug_lock_en::W](W) writer structure"]
impl crate::Writable for DEBUG_LOCK_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEBUG_LOCK_EN to value 0x05"]
impl crate::Resettable for DEBUG_LOCK_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x05
    }
}
