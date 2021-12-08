#[doc = "Register `FREQ_ME_CTRL` reader"]
pub struct R(crate::R<FREQ_ME_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FREQ_ME_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FREQ_ME_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FREQ_ME_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FREQ_ME_CTRL` writer"]
pub struct W(crate::W<FREQ_ME_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FREQ_ME_CTRL_SPEC>;
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
impl From<crate::W<FREQ_ME_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FREQ_ME_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPVAL_SCALE` reader - Frequency measure result /Frequency measur scale"]
pub struct CAPVAL_SCALE_R(crate::FieldReader<u32, u32>);
impl CAPVAL_SCALE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CAPVAL_SCALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPVAL_SCALE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPVAL_SCALE` writer - Frequency measure result /Frequency measur scale"]
pub struct CAPVAL_SCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPVAL_SCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff_ffff) | (value as u32 & 0x7fff_ffff);
        self.w
    }
}
#[doc = "Field `PROG` reader - Set this bit to one to initiate a frequency measurement cycle. Hardware clears this bit when the measurement cycle has completed and there is valid capture data in the CAPVAL field (bits 30:0)."]
pub struct PROG_R(crate::FieldReader<bool, bool>);
impl PROG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROG` writer - Set this bit to one to initiate a frequency measurement cycle. Hardware clears this bit when the measurement cycle has completed and there is valid capture data in the CAPVAL field (bits 30:0)."]
pub struct PROG_W<'a> {
    w: &'a mut W,
}
impl<'a> PROG_W<'a> {
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
    #[doc = "Bits 0:30 - Frequency measure result /Frequency measur scale"]
    #[inline(always)]
    pub fn capval_scale(&self) -> CAPVAL_SCALE_R {
        CAPVAL_SCALE_R::new((self.bits & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 31 - Set this bit to one to initiate a frequency measurement cycle. Hardware clears this bit when the measurement cycle has completed and there is valid capture data in the CAPVAL field (bits 30:0)."]
    #[inline(always)]
    pub fn prog(&self) -> PROG_R {
        PROG_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Frequency measure result /Frequency measur scale"]
    #[inline(always)]
    pub fn capval_scale(&mut self) -> CAPVAL_SCALE_W {
        CAPVAL_SCALE_W { w: self }
    }
    #[doc = "Bit 31 - Set this bit to one to initiate a frequency measurement cycle. Hardware clears this bit when the measurement cycle has completed and there is valid capture data in the CAPVAL field (bits 30:0)."]
    #[inline(always)]
    pub fn prog(&mut self) -> PROG_W {
        PROG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frequency Measure function control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [freq_me_ctrl](index.html) module"]
pub struct FREQ_ME_CTRL_SPEC;
impl crate::RegisterSpec for FREQ_ME_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [freq_me_ctrl::R](R) reader structure"]
impl crate::Readable for FREQ_ME_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [freq_me_ctrl::W](W) writer structure"]
impl crate::Writable for FREQ_ME_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FREQ_ME_CTRL to value 0"]
impl crate::Resettable for FREQ_ME_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
