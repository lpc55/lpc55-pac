#[doc = "Reader of register XO_CAL_STATUS"]
pub type R = crate::R<u32, super::XO_CAL_STATUS>;
#[doc = "Reader of field `CAL_CNTR`"]
pub type CAL_CNTR_R = crate::R<u16, u16>;
#[doc = "Reader of field `DONE`"]
pub type DONE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:15 - Value of the calibration counter (result of the calibration operation)."]
    #[inline(always)]
    pub fn cal_cntr(&self) -> CAL_CNTR_R {
        CAL_CNTR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Status of the calibration run. 1: Calibration is completed."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
