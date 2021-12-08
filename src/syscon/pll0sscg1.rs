#[doc = "Register `PLL0SSCG1` reader"]
pub struct R(crate::R<PLL0SSCG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL0SSCG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL0SSCG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL0SSCG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL0SSCG1` writer"]
pub struct W(crate::W<PLL0SSCG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL0SSCG1_SPEC>;
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
impl From<crate::W<PLL0SSCG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL0SSCG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MD_MBS` reader - input word of the wrapper bit 32."]
pub struct MD_MBS_R(crate::FieldReader<bool, bool>);
impl MD_MBS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MD_MBS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MD_MBS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MD_MBS` writer - input word of the wrapper bit 32."]
pub struct MD_MBS_W<'a> {
    w: &'a mut W,
}
impl<'a> MD_MBS_W<'a> {
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
#[doc = "Field `MD_REQ` reader - md change request."]
pub struct MD_REQ_R(crate::FieldReader<bool, bool>);
impl MD_REQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MD_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MD_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MD_REQ` writer - md change request."]
pub struct MD_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> MD_REQ_W<'a> {
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
#[doc = "Field `MF` reader - programmable modulation frequency fm = Fref/Nss mf\\[2:0\\]
= 000 => Nss=512 (fm ~ 3."]
pub struct MF_R(crate::FieldReader<u8, u8>);
impl MF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MF` writer - programmable modulation frequency fm = Fref/Nss mf\\[2:0\\]
= 000 => Nss=512 (fm ~ 3."]
pub struct MF_W<'a> {
    w: &'a mut W,
}
impl<'a> MF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | ((value as u32 & 0x07) << 2);
        self.w
    }
}
#[doc = "Field `MR` reader - programmable frequency modulation depth Dfmodpk-pk = Fref*kss/Fcco = kss/(2*md\\[32:25\\]dec) mr\\[2:0\\]
= 000 => kss = 0 (no spread spectrum) mr\\[2:0\\]
= 001 => kss ~ 1 mr\\[2:0\\]
= 010 => kss ~ 1."]
pub struct MR_R(crate::FieldReader<u8, u8>);
impl MR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MR` writer - programmable frequency modulation depth Dfmodpk-pk = Fref*kss/Fcco = kss/(2*md\\[32:25\\]dec) mr\\[2:0\\]
= 000 => kss = 0 (no spread spectrum) mr\\[2:0\\]
= 001 => kss ~ 1 mr\\[2:0\\]
= 010 => kss ~ 1."]
pub struct MR_W<'a> {
    w: &'a mut W,
}
impl<'a> MR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "Field `MC` reader - modulation waveform control Compensation for low pass filtering of the PLL to get a triangular modulation at the output of the PLL, giving a flat frequency spectrum."]
pub struct MC_R(crate::FieldReader<u8, u8>);
impl MC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MC` writer - modulation waveform control Compensation for low pass filtering of the PLL to get a triangular modulation at the output of the PLL, giving a flat frequency spectrum."]
pub struct MC_W<'a> {
    w: &'a mut W,
}
impl<'a> MC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `MDIV_EXT` reader - to select an external mdiv value."]
pub struct MDIV_EXT_R(crate::FieldReader<u16, u16>);
impl MDIV_EXT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MDIV_EXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MDIV_EXT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDIV_EXT` writer - to select an external mdiv value."]
pub struct MDIV_EXT_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIV_EXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 10)) | ((value as u32 & 0xffff) << 10);
        self.w
    }
}
#[doc = "Field `MREQ` reader - to select an external mreq value."]
pub struct MREQ_R(crate::FieldReader<bool, bool>);
impl MREQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MREQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MREQ` writer - to select an external mreq value."]
pub struct MREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> MREQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `DITHER` reader - dithering between two modulation frequencies in a random way or in a pseudo random way (white noise), in order to decrease the probability that the modulated waveform will occur with the same phase on a particular point on the screen."]
pub struct DITHER_R(crate::FieldReader<bool, bool>);
impl DITHER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DITHER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DITHER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DITHER` writer - dithering between two modulation frequencies in a random way or in a pseudo random way (white noise), in order to decrease the probability that the modulated waveform will occur with the same phase on a particular point on the screen."]
pub struct DITHER_W<'a> {
    w: &'a mut W,
}
impl<'a> DITHER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `SEL_EXT` reader - to select mdiv_ext and mreq_ext sel_ext = 0: mdiv ~ md\\[32:0\\], mreq = 1 sel_ext = 1 : mdiv = mdiv_ext, mreq = mreq_ext."]
pub struct SEL_EXT_R(crate::FieldReader<bool, bool>);
impl SEL_EXT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEL_EXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL_EXT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL_EXT` writer - to select mdiv_ext and mreq_ext sel_ext = 0: mdiv ~ md\\[32:0\\], mreq = 1 sel_ext = 1 : mdiv = mdiv_ext, mreq = mreq_ext."]
pub struct SEL_EXT_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_EXT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - input word of the wrapper bit 32."]
    #[inline(always)]
    pub fn md_mbs(&self) -> MD_MBS_R {
        MD_MBS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - md change request."]
    #[inline(always)]
    pub fn md_req(&self) -> MD_REQ_R {
        MD_REQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:4 - programmable modulation frequency fm = Fref/Nss mf\\[2:0\\]
= 000 => Nss=512 (fm ~ 3."]
    #[inline(always)]
    pub fn mf(&self) -> MF_R {
        MF_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 5:7 - programmable frequency modulation depth Dfmodpk-pk = Fref*kss/Fcco = kss/(2*md\\[32:25\\]dec) mr\\[2:0\\]
= 000 => kss = 0 (no spread spectrum) mr\\[2:0\\]
= 001 => kss ~ 1 mr\\[2:0\\]
= 010 => kss ~ 1."]
    #[inline(always)]
    pub fn mr(&self) -> MR_R {
        MR_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:9 - modulation waveform control Compensation for low pass filtering of the PLL to get a triangular modulation at the output of the PLL, giving a flat frequency spectrum."]
    #[inline(always)]
    pub fn mc(&self) -> MC_R {
        MC_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:25 - to select an external mdiv value."]
    #[inline(always)]
    pub fn mdiv_ext(&self) -> MDIV_EXT_R {
        MDIV_EXT_R::new(((self.bits >> 10) & 0xffff) as u16)
    }
    #[doc = "Bit 26 - to select an external mreq value."]
    #[inline(always)]
    pub fn mreq(&self) -> MREQ_R {
        MREQ_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - dithering between two modulation frequencies in a random way or in a pseudo random way (white noise), in order to decrease the probability that the modulated waveform will occur with the same phase on a particular point on the screen."]
    #[inline(always)]
    pub fn dither(&self) -> DITHER_R {
        DITHER_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - to select mdiv_ext and mreq_ext sel_ext = 0: mdiv ~ md\\[32:0\\], mreq = 1 sel_ext = 1 : mdiv = mdiv_ext, mreq = mreq_ext."]
    #[inline(always)]
    pub fn sel_ext(&self) -> SEL_EXT_R {
        SEL_EXT_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - input word of the wrapper bit 32."]
    #[inline(always)]
    pub fn md_mbs(&mut self) -> MD_MBS_W {
        MD_MBS_W { w: self }
    }
    #[doc = "Bit 1 - md change request."]
    #[inline(always)]
    pub fn md_req(&mut self) -> MD_REQ_W {
        MD_REQ_W { w: self }
    }
    #[doc = "Bits 2:4 - programmable modulation frequency fm = Fref/Nss mf\\[2:0\\]
= 000 => Nss=512 (fm ~ 3."]
    #[inline(always)]
    pub fn mf(&mut self) -> MF_W {
        MF_W { w: self }
    }
    #[doc = "Bits 5:7 - programmable frequency modulation depth Dfmodpk-pk = Fref*kss/Fcco = kss/(2*md\\[32:25\\]dec) mr\\[2:0\\]
= 000 => kss = 0 (no spread spectrum) mr\\[2:0\\]
= 001 => kss ~ 1 mr\\[2:0\\]
= 010 => kss ~ 1."]
    #[inline(always)]
    pub fn mr(&mut self) -> MR_W {
        MR_W { w: self }
    }
    #[doc = "Bits 8:9 - modulation waveform control Compensation for low pass filtering of the PLL to get a triangular modulation at the output of the PLL, giving a flat frequency spectrum."]
    #[inline(always)]
    pub fn mc(&mut self) -> MC_W {
        MC_W { w: self }
    }
    #[doc = "Bits 10:25 - to select an external mdiv value."]
    #[inline(always)]
    pub fn mdiv_ext(&mut self) -> MDIV_EXT_W {
        MDIV_EXT_W { w: self }
    }
    #[doc = "Bit 26 - to select an external mreq value."]
    #[inline(always)]
    pub fn mreq(&mut self) -> MREQ_W {
        MREQ_W { w: self }
    }
    #[doc = "Bit 27 - dithering between two modulation frequencies in a random way or in a pseudo random way (white noise), in order to decrease the probability that the modulated waveform will occur with the same phase on a particular point on the screen."]
    #[inline(always)]
    pub fn dither(&mut self) -> DITHER_W {
        DITHER_W { w: self }
    }
    #[doc = "Bit 28 - to select mdiv_ext and mreq_ext sel_ext = 0: mdiv ~ md\\[32:0\\], mreq = 1 sel_ext = 1 : mdiv = mdiv_ext, mreq = mreq_ext."]
    #[inline(always)]
    pub fn sel_ext(&mut self) -> SEL_EXT_W {
        SEL_EXT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL0 Spread Spectrum Wrapper control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll0sscg1](index.html) module"]
pub struct PLL0SSCG1_SPEC;
impl crate::RegisterSpec for PLL0SSCG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll0sscg1::R](R) reader structure"]
impl crate::Readable for PLL0SSCG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll0sscg1::W](W) writer structure"]
impl crate::Writable for PLL0SSCG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL0SSCG1 to value 0"]
impl crate::Resettable for PLL0SSCG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
