#[doc = "Reader of register STATUSCLK"]
pub type R = crate::R<u32, super::STATUSCLK>;
#[doc = "Writer for register STATUSCLK"]
pub type W = crate::W<u32, super::STATUSCLK>;
#[doc = "Register STATUSCLK `reset()`'s with value 0x06"]
impl crate::ResetValue for super::STATUSCLK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x06
    }
}
#[doc = "Reader of field `XTAL32KOK`"]
pub type XTAL32KOK_R = crate::R<bool, bool>;
#[doc = "Reader of field `FRO1MCLKVALID`"]
pub type FRO1MCLKVALID_R = crate::R<bool, bool>;
#[doc = "Possible values of the field `XTAL32KOSCFAILURE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTAL32KOSCFAILURE_A {
    #[doc = "No oscillation failure has been detetced since the last time this bit has been cleared.."]
    NOFAIL,
    #[doc = "At least one oscillation failure has been detetced since the last time this bit has been cleared.."]
    FAILURE,
}
impl From<XTAL32KOSCFAILURE_A> for bool {
    #[inline(always)]
    fn from(variant: XTAL32KOSCFAILURE_A) -> Self {
        match variant {
            XTAL32KOSCFAILURE_A::NOFAIL => false,
            XTAL32KOSCFAILURE_A::FAILURE => true,
        }
    }
}
#[doc = "Reader of field `XTAL32KOSCFAILURE`"]
pub type XTAL32KOSCFAILURE_R = crate::R<bool, XTAL32KOSCFAILURE_A>;
impl XTAL32KOSCFAILURE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XTAL32KOSCFAILURE_A {
        match self.bits {
            false => XTAL32KOSCFAILURE_A::NOFAIL,
            true => XTAL32KOSCFAILURE_A::FAILURE,
        }
    }
    #[doc = "Checks if the value of the field is `NOFAIL`"]
    #[inline(always)]
    pub fn is_nofail(&self) -> bool {
        *self == XTAL32KOSCFAILURE_A::NOFAIL
    }
    #[doc = "Checks if the value of the field is `FAILURE`"]
    #[inline(always)]
    pub fn is_failure(&self) -> bool {
        *self == XTAL32KOSCFAILURE_A::FAILURE
    }
}
#[doc = "Write proxy for field `XTAL32KOSCFAILURE`"]
pub struct XTAL32KOSCFAILURE_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32KOSCFAILURE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTAL32KOSCFAILURE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No oscillation failure has been detetced since the last time this bit has been cleared.."]
    #[inline(always)]
    pub fn nofail(self) -> &'a mut W {
        self.variant(XTAL32KOSCFAILURE_A::NOFAIL)
    }
    #[doc = "At least one oscillation failure has been detetced since the last time this bit has been cleared.."]
    #[inline(always)]
    pub fn failure(self) -> &'a mut W {
        self.variant(XTAL32KOSCFAILURE_A::FAILURE)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - XTAL oscillator 32 K OK signal."]
    #[inline(always)]
    pub fn xtal32kok(&self) -> XTAL32KOK_R {
        XTAL32KOK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FRO 1 MHz CCO voltage detector output."]
    #[inline(always)]
    pub fn fro1mclkvalid(&self) -> FRO1MCLKVALID_R {
        FRO1MCLKVALID_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - XTAL32 KHZ oscillator oscillation failure detection indicator."]
    #[inline(always)]
    pub fn xtal32koscfailure(&self) -> XTAL32KOSCFAILURE_R {
        XTAL32KOSCFAILURE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - XTAL32 KHZ oscillator oscillation failure detection indicator."]
    #[inline(always)]
    pub fn xtal32koscfailure(&mut self) -> XTAL32KOSCFAILURE_W {
        XTAL32KOSCFAILURE_W { w: self }
    }
}
