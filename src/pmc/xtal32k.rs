#[doc = "Reader of register XTAL32K"]
pub type R = crate::R<u32, super::XTAL32K>;
#[doc = "Writer for register XTAL32K"]
pub type W = crate::W<u32, super::XTAL32K>;
#[doc = "Register XTAL32K `reset()`'s with value 0x0020_4052"]
impl crate::ResetValue for super::XTAL32K {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0020_4052
    }
}
#[doc = "Reader of field `IREF`"]
pub type IREF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IREF`"]
pub struct IREF_W<'a> {
    w: &'a mut W,
}
impl<'a> IREF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `TEST`"]
pub type TEST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEST`"]
pub struct TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_W<'a> {
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
#[doc = "Reader of field `IBIAS`"]
pub type IBIAS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IBIAS`"]
pub struct IBIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> IBIAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `AMPL`"]
pub type AMPL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AMPL`"]
pub struct AMPL_W<'a> {
    w: &'a mut W,
}
impl<'a> AMPL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `CAPBANKIN`"]
pub type CAPBANKIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CAPBANKIN`"]
pub struct CAPBANKIN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPBANKIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `CAPBANKOUT`"]
pub type CAPBANKOUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CAPBANKOUT`"]
pub struct CAPBANKOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPBANKOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 15)) | (((value as u32) & 0x7f) << 15);
        self.w
    }
}
#[doc = "Possible values of the field `CAPTESTSTARTSRCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTESTSTARTSRCSEL_A {
    #[doc = "Sourced from CAPTESTSTART."]
    CAPSTART,
    #[doc = "Sourced from calibration."]
    CALIB,
}
impl From<CAPTESTSTARTSRCSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CAPTESTSTARTSRCSEL_A) -> Self {
        match variant {
            CAPTESTSTARTSRCSEL_A::CAPSTART => false,
            CAPTESTSTARTSRCSEL_A::CALIB => true,
        }
    }
}
#[doc = "Reader of field `CAPTESTSTARTSRCSEL`"]
pub type CAPTESTSTARTSRCSEL_R = crate::R<bool, CAPTESTSTARTSRCSEL_A>;
impl CAPTESTSTARTSRCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTESTSTARTSRCSEL_A {
        match self.bits {
            false => CAPTESTSTARTSRCSEL_A::CAPSTART,
            true => CAPTESTSTARTSRCSEL_A::CALIB,
        }
    }
    #[doc = "Checks if the value of the field is `CAPSTART`"]
    #[inline(always)]
    pub fn is_capstart(&self) -> bool {
        *self == CAPTESTSTARTSRCSEL_A::CAPSTART
    }
    #[doc = "Checks if the value of the field is `CALIB`"]
    #[inline(always)]
    pub fn is_calib(&self) -> bool {
        *self == CAPTESTSTARTSRCSEL_A::CALIB
    }
}
#[doc = "Write proxy for field `CAPTESTSTARTSRCSEL`"]
pub struct CAPTESTSTARTSRCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTESTSTARTSRCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPTESTSTARTSRCSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sourced from CAPTESTSTART."]
    #[inline(always)]
    pub fn capstart(self) -> &'a mut W {
        self.variant(CAPTESTSTARTSRCSEL_A::CAPSTART)
    }
    #[doc = "Sourced from calibration."]
    #[inline(always)]
    pub fn calib(self) -> &'a mut W {
        self.variant(CAPTESTSTARTSRCSEL_A::CALIB)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `CAPTESTSTART`"]
pub type CAPTESTSTART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPTESTSTART`"]
pub struct CAPTESTSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTESTSTART_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `CAPTESTENABLE`"]
pub type CAPTESTENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPTESTENABLE`"]
pub struct CAPTESTENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTESTENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Possible values of the field `CAPTESTOSCINSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTESTOSCINSEL_A {
    #[doc = "Oscillator output pin (osc_out)."]
    OSCOUT,
    #[doc = "Oscillator input pin (osc_in)."]
    OSCIN,
}
impl From<CAPTESTOSCINSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CAPTESTOSCINSEL_A) -> Self {
        match variant {
            CAPTESTOSCINSEL_A::OSCOUT => false,
            CAPTESTOSCINSEL_A::OSCIN => true,
        }
    }
}
#[doc = "Reader of field `CAPTESTOSCINSEL`"]
pub type CAPTESTOSCINSEL_R = crate::R<bool, CAPTESTOSCINSEL_A>;
impl CAPTESTOSCINSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTESTOSCINSEL_A {
        match self.bits {
            false => CAPTESTOSCINSEL_A::OSCOUT,
            true => CAPTESTOSCINSEL_A::OSCIN,
        }
    }
    #[doc = "Checks if the value of the field is `OSCOUT`"]
    #[inline(always)]
    pub fn is_oscout(&self) -> bool {
        *self == CAPTESTOSCINSEL_A::OSCOUT
    }
    #[doc = "Checks if the value of the field is `OSCIN`"]
    #[inline(always)]
    pub fn is_oscin(&self) -> bool {
        *self == CAPTESTOSCINSEL_A::OSCIN
    }
}
#[doc = "Write proxy for field `CAPTESTOSCINSEL`"]
pub struct CAPTESTOSCINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTESTOSCINSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPTESTOSCINSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Oscillator output pin (osc_out)."]
    #[inline(always)]
    pub fn oscout(self) -> &'a mut W {
        self.variant(CAPTESTOSCINSEL_A::OSCOUT)
    }
    #[doc = "Oscillator input pin (osc_in)."]
    #[inline(always)]
    pub fn oscin(self) -> &'a mut W {
        self.variant(CAPTESTOSCINSEL_A::OSCIN)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:2 - reference output current selection inputs."]
    #[inline(always)]
    pub fn iref(&self) -> IREF_R {
        IREF_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Oscillator Test Mode."]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - bias current selection inputs."]
    #[inline(always)]
    pub fn ibias(&self) -> IBIAS_R {
        IBIAS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - oscillator amplitude selection inputs."]
    #[inline(always)]
    pub fn ampl(&self) -> AMPL_R {
        AMPL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:14 - Capa bank setting input."]
    #[inline(always)]
    pub fn capbankin(&self) -> CAPBANKIN_R {
        CAPBANKIN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 15:21 - Capa bank setting output."]
    #[inline(always)]
    pub fn capbankout(&self) -> CAPBANKOUT_R {
        CAPBANKOUT_R::new(((self.bits >> 15) & 0x7f) as u8)
    }
    #[doc = "Bit 22 - Source selection for xo32k_captest_start_ao_set."]
    #[inline(always)]
    pub fn capteststartsrcsel(&self) -> CAPTESTSTARTSRCSEL_R {
        CAPTESTSTARTSRCSEL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Start test."]
    #[inline(always)]
    pub fn capteststart(&self) -> CAPTESTSTART_R {
        CAPTESTSTART_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enable signal for cap test."]
    #[inline(always)]
    pub fn captestenable(&self) -> CAPTESTENABLE_R {
        CAPTESTENABLE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Select the input for test."]
    #[inline(always)]
    pub fn captestoscinsel(&self) -> CAPTESTOSCINSEL_R {
        CAPTESTOSCINSEL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:2 - reference output current selection inputs."]
    #[inline(always)]
    pub fn iref(&mut self) -> IREF_W {
        IREF_W { w: self }
    }
    #[doc = "Bit 3 - Oscillator Test Mode."]
    #[inline(always)]
    pub fn test(&mut self) -> TEST_W {
        TEST_W { w: self }
    }
    #[doc = "Bits 4:5 - bias current selection inputs."]
    #[inline(always)]
    pub fn ibias(&mut self) -> IBIAS_W {
        IBIAS_W { w: self }
    }
    #[doc = "Bits 6:7 - oscillator amplitude selection inputs."]
    #[inline(always)]
    pub fn ampl(&mut self) -> AMPL_W {
        AMPL_W { w: self }
    }
    #[doc = "Bits 8:14 - Capa bank setting input."]
    #[inline(always)]
    pub fn capbankin(&mut self) -> CAPBANKIN_W {
        CAPBANKIN_W { w: self }
    }
    #[doc = "Bits 15:21 - Capa bank setting output."]
    #[inline(always)]
    pub fn capbankout(&mut self) -> CAPBANKOUT_W {
        CAPBANKOUT_W { w: self }
    }
    #[doc = "Bit 22 - Source selection for xo32k_captest_start_ao_set."]
    #[inline(always)]
    pub fn capteststartsrcsel(&mut self) -> CAPTESTSTARTSRCSEL_W {
        CAPTESTSTARTSRCSEL_W { w: self }
    }
    #[doc = "Bit 23 - Start test."]
    #[inline(always)]
    pub fn capteststart(&mut self) -> CAPTESTSTART_W {
        CAPTESTSTART_W { w: self }
    }
    #[doc = "Bit 24 - Enable signal for cap test."]
    #[inline(always)]
    pub fn captestenable(&mut self) -> CAPTESTENABLE_W {
        CAPTESTENABLE_W { w: self }
    }
    #[doc = "Bit 25 - Select the input for test."]
    #[inline(always)]
    pub fn captestoscinsel(&mut self) -> CAPTESTOSCINSEL_W {
        CAPTESTOSCINSEL_W { w: self }
    }
}
