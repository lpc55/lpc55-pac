#[doc = "Reader of register GCC[%s]"]
pub type R = crate::R<u32, super::GCC>;
#[doc = "Reader of field `GAIN_CAL`"]
pub type GAIN_CAL_R = crate::R<u16, u16>;
#[doc = "Gain Calibration Value Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDY_A {
    #[doc = "0: The gain calibration value is invalid. Run the auto-calibration routine for this value to be written."]
    RDY_0 = 0,
    #[doc = "1: The gain calibration value is valid. It should be used to update the GCRa\\[GCALR\\]
register field."]
    RDY_1 = 1,
}
impl From<RDY_A> for bool {
    #[inline(always)]
    fn from(variant: RDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RDY`"]
pub type RDY_R = crate::R<bool, RDY_A>;
impl RDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDY_A {
        match self.bits {
            false => RDY_A::RDY_0,
            true => RDY_A::RDY_1,
        }
    }
    #[doc = "Checks if the value of the field is `RDY_0`"]
    #[inline(always)]
    pub fn is_rdy_0(&self) -> bool {
        *self == RDY_A::RDY_0
    }
    #[doc = "Checks if the value of the field is `RDY_1`"]
    #[inline(always)]
    pub fn is_rdy_1(&self) -> bool {
        *self == RDY_A::RDY_1
    }
}
impl R {
    #[doc = "Bits 0:15 - Gain Calibration Value"]
    #[inline(always)]
    pub fn gain_cal(&self) -> GAIN_CAL_R {
        GAIN_CAL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 24 - Gain Calibration Value Valid"]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
