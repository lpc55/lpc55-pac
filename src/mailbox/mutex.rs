#[doc = "Register `MUTEX` reader"]
pub struct R(crate::R<MUTEX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MUTEX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MUTEX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MUTEX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MUTEX` writer"]
pub struct W(crate::W<MUTEX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MUTEX_SPEC>;
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
impl From<crate::W<MUTEX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MUTEX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EX` reader - Cleared when read, set when written. See usage description above."]
pub struct EX_R(crate::FieldReader<bool, bool>);
impl EX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EX` writer - Cleared when read, set when written. See usage description above."]
pub struct EX_W<'a> {
    w: &'a mut W,
}
impl<'a> EX_W<'a> {
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
    #[doc = "Bit 0 - Cleared when read, set when written. See usage description above."]
    #[inline(always)]
    pub fn ex(&self) -> EX_R {
        EX_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cleared when read, set when written. See usage description above."]
    #[inline(always)]
    pub fn ex(&mut self) -> EX_W {
        EX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mutual exclusion register\\[1\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mutex](index.html) module"]
pub struct MUTEX_SPEC;
impl crate::RegisterSpec for MUTEX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mutex::R](R) reader structure"]
impl crate::Readable for MUTEX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mutex::W](W) writer structure"]
impl crate::Writable for MUTEX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MUTEX to value 0x01"]
impl crate::Resettable for MUTEX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
