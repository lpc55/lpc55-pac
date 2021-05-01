#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CFG_SPEC>> for R {
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl core::convert::From<crate::W<CFG_SPEC>> for W {
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLOCKENROLL_SETKEY` reader - Block enroll operation. Write 1 to set, cleared on reset."]
pub struct BLOCKENROLL_SETKEY_R(crate::FieldReader<bool, bool>);
impl BLOCKENROLL_SETKEY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLOCKENROLL_SETKEY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLOCKENROLL_SETKEY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLOCKENROLL_SETKEY` writer - Block enroll operation. Write 1 to set, cleared on reset."]
pub struct BLOCKENROLL_SETKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCKENROLL_SETKEY_W<'a> {
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
#[doc = "Field `BLOCKKEYOUTPUT` reader - Block set key operation. Write 1 to set, cleared on reset."]
pub struct BLOCKKEYOUTPUT_R(crate::FieldReader<bool, bool>);
impl BLOCKKEYOUTPUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLOCKKEYOUTPUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLOCKKEYOUTPUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLOCKKEYOUTPUT` writer - Block set key operation. Write 1 to set, cleared on reset."]
pub struct BLOCKKEYOUTPUT_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCKKEYOUTPUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Block enroll operation. Write 1 to set, cleared on reset."]
    #[inline(always)]
    pub fn blockenroll_setkey(&self) -> BLOCKENROLL_SETKEY_R {
        BLOCKENROLL_SETKEY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Block set key operation. Write 1 to set, cleared on reset."]
    #[inline(always)]
    pub fn blockkeyoutput(&self) -> BLOCKKEYOUTPUT_R {
        BLOCKKEYOUTPUT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Block enroll operation. Write 1 to set, cleared on reset."]
    #[inline(always)]
    pub fn blockenroll_setkey(&mut self) -> BLOCKENROLL_SETKEY_W {
        BLOCKENROLL_SETKEY_W { w: self }
    }
    #[doc = "Bit 1 - Block set key operation. Write 1 to set, cleared on reset."]
    #[inline(always)]
    pub fn blockkeyoutput(&mut self) -> BLOCKKEYOUTPUT_W {
        BLOCKKEYOUTPUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PUF config register for block bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
