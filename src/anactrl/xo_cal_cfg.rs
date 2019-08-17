#[doc = "Reader of register XO_CAL_CFG"]
pub type R = crate::R<u32, super::XO_CAL_CFG>;
#[doc = "Writer for register XO_CAL_CFG"]
pub type W = crate::W<u32, super::XO_CAL_CFG>;
#[doc = "Register XO_CAL_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::XO_CAL_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `START_INV`"]
pub type START_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `START_INV`"]
pub struct START_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> START_INV_W<'a> {
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
#[doc = "Reader of field `START_OVR`"]
pub type START_OVR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `START_OVR`"]
pub struct START_OVR_W<'a> {
    w: &'a mut W,
}
impl<'a> START_OVR_W<'a> {
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
#[doc = "Reader of field `START`"]
pub type START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `START`"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
#[doc = "Reader of field `STOP_INV`"]
pub type STOP_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STOP_INV`"]
pub struct STOP_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `STOP_CNTR_END`"]
pub type STOP_CNTR_END_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STOP_CNTR_END`"]
pub struct STOP_CNTR_END_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_CNTR_END_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `XO32K_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XO32K_MODE_A {
    #[doc = "High speed crystal oscillator (12 MHz- 32 MHz) is used"]
    XO32MHZ,
    #[doc = "32 kHz crystal oscillator calibration is used."]
    XO32KHZ,
}
impl From<XO32K_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: XO32K_MODE_A) -> Self {
        match variant {
            XO32K_MODE_A::XO32MHZ => false,
            XO32K_MODE_A::XO32KHZ => true,
        }
    }
}
#[doc = "Reader of field `XO32K_MODE`"]
pub type XO32K_MODE_R = crate::R<bool, XO32K_MODE_A>;
impl XO32K_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XO32K_MODE_A {
        match self.bits {
            false => XO32K_MODE_A::XO32MHZ,
            true => XO32K_MODE_A::XO32KHZ,
        }
    }
    #[doc = "Checks if the value of the field is `XO32MHZ`"]
    #[inline(always)]
    pub fn is_xo32mhz(&self) -> bool {
        *self == XO32K_MODE_A::XO32MHZ
    }
    #[doc = "Checks if the value of the field is `XO32KHZ`"]
    #[inline(always)]
    pub fn is_xo32khz(&self) -> bool {
        *self == XO32K_MODE_A::XO32KHZ
    }
}
#[doc = "Write proxy for field `XO32K_MODE`"]
pub struct XO32K_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> XO32K_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XO32K_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "High speed crystal oscillator (12 MHz- 32 MHz) is used"]
    #[inline(always)]
    pub fn xo32mhz(self) -> &'a mut W {
        self.variant(XO32K_MODE_A::XO32MHZ)
    }
    #[doc = "32 kHz crystal oscillator calibration is used."]
    #[inline(always)]
    pub fn xo32khz(self) -> &'a mut W {
        self.variant(XO32K_MODE_A::XO32KHZ)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Polarity of the externally applied START signal"]
    #[inline(always)]
    pub fn start_inv(&self) -> START_INV_R {
        START_INV_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Override of the START signal."]
    #[inline(always)]
    pub fn start_ovr(&self) -> START_OVR_R {
        START_OVR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Override value of the START signal."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Polarity of the STOP signal."]
    #[inline(always)]
    pub fn stop_inv(&self) -> STOP_INV_R {
        STOP_INV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Generate the external DONE signal when the counter reaches its end."]
    #[inline(always)]
    pub fn stop_cntr_end(&self) -> STOP_CNTR_END_R {
        STOP_CNTR_END_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - When 0 : High speed crystal oscillator calibration is used. When 1 : 32 kHz crystal oscillator calibration is used."]
    #[inline(always)]
    pub fn xo32k_mode(&self) -> XO32K_MODE_R {
        XO32K_MODE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Polarity of the externally applied START signal"]
    #[inline(always)]
    pub fn start_inv(&mut self) -> START_INV_W {
        START_INV_W { w: self }
    }
    #[doc = "Bit 1 - Override of the START signal."]
    #[inline(always)]
    pub fn start_ovr(&mut self) -> START_OVR_W {
        START_OVR_W { w: self }
    }
    #[doc = "Bit 2 - Override value of the START signal."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 3 - Polarity of the STOP signal."]
    #[inline(always)]
    pub fn stop_inv(&mut self) -> STOP_INV_W {
        STOP_INV_W { w: self }
    }
    #[doc = "Bit 4 - Generate the external DONE signal when the counter reaches its end."]
    #[inline(always)]
    pub fn stop_cntr_end(&mut self) -> STOP_CNTR_END_W {
        STOP_CNTR_END_W { w: self }
    }
    #[doc = "Bit 5 - When 0 : High speed crystal oscillator calibration is used. When 1 : 32 kHz crystal oscillator calibration is used."]
    #[inline(always)]
    pub fn xo32k_mode(&mut self) -> XO32K_MODE_W {
        XO32K_MODE_W { w: self }
    }
}
