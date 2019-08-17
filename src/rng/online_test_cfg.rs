#[doc = "Reader of register ONLINE_TEST_CFG"]
pub type R = crate::R<u32, super::ONLINE_TEST_CFG>;
#[doc = "Writer for register ONLINE_TEST_CFG"]
pub type W = crate::W<u32, super::ONLINE_TEST_CFG>;
#[doc = "Register ONLINE_TEST_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::ONLINE_TEST_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ACTIVATE`"]
pub type ACTIVATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACTIVATE`"]
pub struct ACTIVATE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVATE_W<'a> {
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
#[doc = "Reader of field `DATA_SEL`"]
pub type DATA_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_SEL`"]
pub struct DATA_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 0: disabled 1: activated Update rythm for VAL depends on COUNTER_CFG if data_sel is set to COUNTER."]
    #[inline(always)]
    pub fn activate(&self) -> ACTIVATE_R {
        ACTIVATE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Selects source on which to apply online test: 00: LSB of COUNTER: raw data from one or all sources of entropy 01: MSB of COUNTER: raw data from one or all sources of entropy 10: RANDOM_NUMBER 11: ENCRYPTED_NUMBER 'activate' should be set to 'disabled' before changing this field."]
    #[inline(always)]
    pub fn data_sel(&self) -> DATA_SEL_R {
        DATA_SEL_R::new(((self.bits >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0: disabled 1: activated Update rythm for VAL depends on COUNTER_CFG if data_sel is set to COUNTER."]
    #[inline(always)]
    pub fn activate(&mut self) -> ACTIVATE_W {
        ACTIVATE_W { w: self }
    }
    #[doc = "Bits 1:2 - Selects source on which to apply online test: 00: LSB of COUNTER: raw data from one or all sources of entropy 01: MSB of COUNTER: raw data from one or all sources of entropy 10: RANDOM_NUMBER 11: ENCRYPTED_NUMBER 'activate' should be set to 'disabled' before changing this field."]
    #[inline(always)]
    pub fn data_sel(&mut self) -> DATA_SEL_W {
        DATA_SEL_W { w: self }
    }
}
