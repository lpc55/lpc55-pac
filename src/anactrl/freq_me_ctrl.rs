#[doc = "Reader of register FREQ_ME_CTRL"]
pub type R = crate::R<u32, super::FREQ_ME_CTRL>;
#[doc = "Writer for register FREQ_ME_CTRL"]
pub type W = crate::W<u32, super::FREQ_ME_CTRL>;
#[doc = "Register FREQ_ME_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::FREQ_ME_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CAPVAL_SCALE`"]
pub type CAPVAL_SCALE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CAPVAL_SCALE`"]
pub struct CAPVAL_SCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPVAL_SCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff_ffff) | ((value as u32) & 0x7fff_ffff);
        self.w
    }
}
#[doc = "Reader of field `PROG`"]
pub type PROG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROG`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
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
}
