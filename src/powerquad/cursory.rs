#[doc = "Register `CURSORY` reader"]
pub struct R(crate::R<CURSORY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CURSORY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CURSORY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CURSORY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CURSORY` writer"]
pub struct W(crate::W<CURSORY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CURSORY_SPEC>;
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
impl From<crate::W<CURSORY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CURSORY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cursory` reader - 1 : Enable cursory mode"]
pub struct CURSORY_R(crate::FieldReader<bool, bool>);
impl CURSORY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CURSORY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CURSORY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cursory` writer - 1 : Enable cursory mode"]
pub struct CURSORY_W<'a> {
    w: &'a mut W,
}
impl<'a> CURSORY_W<'a> {
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
    #[doc = "Bit 0 - 1 : Enable cursory mode"]
    #[inline(always)]
    pub fn cursory(&self) -> CURSORY_R {
        CURSORY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1 : Enable cursory mode"]
    #[inline(always)]
    pub fn cursory(&mut self) -> CURSORY_W {
        CURSORY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cursory register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cursory](index.html) module"]
pub struct CURSORY_SPEC;
impl crate::RegisterSpec for CURSORY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cursory::R](R) reader structure"]
impl crate::Readable for CURSORY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cursory::W](W) writer structure"]
impl crate::Writable for CURSORY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CURSORY to value 0"]
impl crate::Resettable for CURSORY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
