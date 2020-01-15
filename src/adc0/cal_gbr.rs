#[doc = "Reader of register CAL_GBR[%s]"]
pub type R = crate::R<u32, super::CAL_GBR>;
#[doc = "Writer for register CAL_GBR[%s]"]
pub type W = crate::W<u32, super::CAL_GBR>;
#[doc = "Register CAL_GBR[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::CAL_GBR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CAL_GBR_VAL`"]
pub type CAL_GBR_VAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CAL_GBR_VAL`"]
pub struct CAL_GBR_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAL_GBR_VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Calibration General B Side Register Element"]
    #[inline(always)]
    pub fn cal_gbr_val(&self) -> CAL_GBR_VAL_R {
        CAL_GBR_VAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Calibration General B Side Register Element"]
    #[inline(always)]
    pub fn cal_gbr_val(&mut self) -> CAL_GBR_VAL_W {
        CAL_GBR_VAL_W { w: self }
    }
}
