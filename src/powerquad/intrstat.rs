#[doc = "Register `INTRSTAT` reader"]
pub struct R(crate::R<INTRSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTRSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<INTRSTAT_SPEC>> for R {
    fn from(reader: crate::R<INTRSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTRSTAT` writer"]
pub struct W(crate::W<INTRSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTRSTAT_SPEC>;
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
impl core::convert::From<crate::W<INTRSTAT_SPEC>> for W {
    fn from(writer: crate::W<INTRSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `intr_stat` reader - Intr status ( 1 bit to indicate interrupt captured, 0 means no new interrupt), write any value will clear this bit"]
pub struct INTR_STAT_R(crate::FieldReader<bool, bool>);
impl INTR_STAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTR_STAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTR_STAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `intr_stat` writer - Intr status ( 1 bit to indicate interrupt captured, 0 means no new interrupt), write any value will clear this bit"]
pub struct INTR_STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> INTR_STAT_W<'a> {
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
    #[doc = "Bit 0 - Intr status ( 1 bit to indicate interrupt captured, 0 means no new interrupt), write any value will clear this bit"]
    #[inline(always)]
    pub fn intr_stat(&self) -> INTR_STAT_R {
        INTR_STAT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Intr status ( 1 bit to indicate interrupt captured, 0 means no new interrupt), write any value will clear this bit"]
    #[inline(always)]
    pub fn intr_stat(&mut self) -> INTR_STAT_W {
        INTR_STAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "INTERRUPT STATUS register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intrstat](index.html) module"]
pub struct INTRSTAT_SPEC;
impl crate::RegisterSpec for INTRSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intrstat::R](R) reader structure"]
impl crate::Readable for INTRSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intrstat::W](W) writer structure"]
impl crate::Writable for INTRSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTRSTAT to value 0"]
impl crate::Resettable for INTRSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
