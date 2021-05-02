#[doc = "Register `IFSTAT` reader"]
pub struct R(crate::R<IFSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IFSTAT_SPEC>> for R {
    fn from(reader: crate::R<IFSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IFSTAT` writer"]
pub struct W(crate::W<IFSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFSTAT_SPEC>;
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
impl core::convert::From<crate::W<IFSTAT_SPEC>> for W {
    fn from(writer: crate::W<IFSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERROR` reader - Indicates that an APB error has occurred,Writing logic1 clears the if_error bit"]
pub struct ERROR_R(crate::FieldReader<bool, bool>);
impl ERROR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROR` writer - Indicates that an APB error has occurred,Writing logic1 clears the if_error bit"]
pub struct ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_W<'a> {
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
    #[doc = "Bit 0 - Indicates that an APB error has occurred,Writing logic1 clears the if_error bit"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates that an APB error has occurred,Writing logic1 clears the if_error bit"]
    #[inline(always)]
    pub fn error(&mut self) -> ERROR_W {
        ERROR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PUF Interface Status and clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifstat](index.html) module"]
pub struct IFSTAT_SPEC;
impl crate::RegisterSpec for IFSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ifstat::R](R) reader structure"]
impl crate::Readable for IFSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ifstat::W](W) writer structure"]
impl crate::Writable for IFSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFSTAT to value 0"]
impl crate::Resettable for IFSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
