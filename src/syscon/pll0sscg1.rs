#[doc = "Reader of register PLL0SSCG1"]
pub type R = crate::R<u32, super::PLL0SSCG1>;
#[doc = "Writer for register PLL0SSCG1"]
pub type W = crate::W<u32, super::PLL0SSCG1>;
#[doc = "Register PLL0SSCG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PLL0SSCG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MD_MBS`"]
pub type MD_MBS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MD_MBS`"]
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `MD_REQ`"]
pub type MD_REQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MD_REQ`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `MF`"]
pub type MF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MF`"]
pub struct MF_W<'a> {
    w: &'a mut W,
}
impl<'a> MF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Reader of field `MR`"]
pub type MR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MR`"]
pub struct MR_W<'a> {
    w: &'a mut W,
}
impl<'a> MR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `MC`"]
pub type MC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MC`"]
pub struct MC_W<'a> {
    w: &'a mut W,
}
impl<'a> MC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `MDIV_EXT`"]
pub type MDIV_EXT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MDIV_EXT`"]
pub struct MDIV_EXT_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIV_EXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 10)) | (((value as u32) & 0xffff) << 10);
        self.w
    }
}
#[doc = "Reader of field `MREQ`"]
pub type MREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MREQ`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `DITHER`"]
pub type DITHER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DITHER`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `SEL_EXT`"]
pub type SEL_EXT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEL_EXT`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
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
    #[doc = "Bits 2:4 - programmable modulation frequency fm = Fref/Nss mf\\[2:0\\] = 000 => Nss=512 (fm ~ 3."]
    #[inline(always)]
    pub fn mf(&self) -> MF_R {
        MF_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 5:7 - programmable frequency modulation depth Dfmodpk-pk = Fref*kss/Fcco = kss/(2*md\\[32:25\\]dec) mr\\[2:0\\] = 000 => kss = 0 (no spread spectrum) mr\\[2:0\\] = 001 => kss ~ 1 mr\\[2:0\\] = 010 => kss ~ 1."]
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
    #[doc = "Bits 2:4 - programmable modulation frequency fm = Fref/Nss mf\\[2:0\\] = 000 => Nss=512 (fm ~ 3."]
    #[inline(always)]
    pub fn mf(&mut self) -> MF_W {
        MF_W { w: self }
    }
    #[doc = "Bits 5:7 - programmable frequency modulation depth Dfmodpk-pk = Fref*kss/Fcco = kss/(2*md\\[32:25\\]dec) mr\\[2:0\\] = 000 => kss = 0 (no spread spectrum) mr\\[2:0\\] = 001 => kss ~ 1 mr\\[2:0\\] = 010 => kss ~ 1."]
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
}
