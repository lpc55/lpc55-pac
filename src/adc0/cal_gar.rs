#[doc = "Reader of register CAL_GAR[%s]"]
pub type R = crate::R<u32, super::CAL_GAR>;
#[doc = "Writer for register CAL_GAR[%s]"]
pub type W = crate::W<u32, super::CAL_GAR>;
#[doc = "Register CAL_GAR[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::CAL_GAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CAL_GAR_VAL`"]
pub type CAL_GAR_VAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CAL_GAR_VAL`"]
pub struct CAL_GAR_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAL_GAR_VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Calibration General A Side Register Element"]
    #[inline(always)]
    pub fn cal_gar_val(&self) -> CAL_GAR_VAL_R {
        CAL_GAR_VAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Calibration General A Side Register Element"]
    #[inline(always)]
    pub fn cal_gar_val(&mut self) -> CAL_GAR_VAL_W {
        CAL_GAR_VAL_W { w: self }
    }
}
