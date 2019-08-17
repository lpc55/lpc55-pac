#[doc = "Reader of register RX"]
pub type R = crate::R<u32, super::RX>;
#[doc = "Writer for register RX"]
pub type W = crate::W<u32, super::RX>;
#[doc = "Register RX `reset()`'s with value 0"]
impl crate::ResetValue for super::RX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `ENVADJ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENVADJ_A {
    #[doc = "Trip-Level Voltage is 0.1000 V"]
    VALUE0,
    #[doc = "Trip-Level Voltage is 0.1125 V"]
    VALUE1,
    #[doc = "Trip-Level Voltage is 0.1250 V"]
    VALUE2,
    #[doc = "Trip-Level Voltage is 0.0875 V"]
    VALUE3,
}
impl From<ENVADJ_A> for u8 {
    #[inline(always)]
    fn from(variant: ENVADJ_A) -> Self {
        match variant {
            ENVADJ_A::VALUE0 => 0,
            ENVADJ_A::VALUE1 => 1,
            ENVADJ_A::VALUE2 => 2,
            ENVADJ_A::VALUE3 => 3,
        }
    }
}
#[doc = "Reader of field `ENVADJ`"]
pub type ENVADJ_R = crate::R<u8, ENVADJ_A>;
impl ENVADJ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ENVADJ_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ENVADJ_A::VALUE0),
            1 => Val(ENVADJ_A::VALUE1),
            2 => Val(ENVADJ_A::VALUE2),
            3 => Val(ENVADJ_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == ENVADJ_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ENVADJ_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ENVADJ_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ENVADJ_A::VALUE3
    }
}
#[doc = "Write proxy for field `ENVADJ`"]
pub struct ENVADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> ENVADJ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENVADJ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Trip-Level Voltage is 0.1000 V"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(ENVADJ_A::VALUE0)
    }
    #[doc = "Trip-Level Voltage is 0.1125 V"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENVADJ_A::VALUE1)
    }
    #[doc = "Trip-Level Voltage is 0.1250 V"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENVADJ_A::VALUE2)
    }
    #[doc = "Trip-Level Voltage is 0.0875 V"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ENVADJ_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Possible values of the field `DISCONADJ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISCONADJ_A {
    #[doc = "Trip-Level Voltage is 0.56875 V"]
    VALUE0,
    #[doc = "Trip-Level Voltage is 0.55000 V"]
    VALUE1,
    #[doc = "Trip-Level Voltage is 0.58125 V"]
    VALUE2,
    #[doc = "Trip-Level Voltage is 0.60000 V"]
    VALUE3,
}
impl From<DISCONADJ_A> for u8 {
    #[inline(always)]
    fn from(variant: DISCONADJ_A) -> Self {
        match variant {
            DISCONADJ_A::VALUE0 => 0,
            DISCONADJ_A::VALUE1 => 1,
            DISCONADJ_A::VALUE2 => 2,
            DISCONADJ_A::VALUE3 => 3,
        }
    }
}
#[doc = "Reader of field `DISCONADJ`"]
pub type DISCONADJ_R = crate::R<u8, DISCONADJ_A>;
impl DISCONADJ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DISCONADJ_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DISCONADJ_A::VALUE0),
            1 => Val(DISCONADJ_A::VALUE1),
            2 => Val(DISCONADJ_A::VALUE2),
            3 => Val(DISCONADJ_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == DISCONADJ_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DISCONADJ_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DISCONADJ_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == DISCONADJ_A::VALUE3
    }
}
#[doc = "Write proxy for field `DISCONADJ`"]
pub struct DISCONADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCONADJ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISCONADJ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Trip-Level Voltage is 0.56875 V"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(DISCONADJ_A::VALUE0)
    }
    #[doc = "Trip-Level Voltage is 0.55000 V"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DISCONADJ_A::VALUE1)
    }
    #[doc = "Trip-Level Voltage is 0.58125 V"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DISCONADJ_A::VALUE2)
    }
    #[doc = "Trip-Level Voltage is 0.60000 V"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(DISCONADJ_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `RXDBYPASS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDBYPASS_A {
    #[doc = "Normal operation."]
    VALUE0,
    #[doc = "Use the output of the USB_DP single-ended receiver in place of the full-speed differential receiver"]
    VALUE1,
}
impl From<RXDBYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: RXDBYPASS_A) -> Self {
        match variant {
            RXDBYPASS_A::VALUE0 => false,
            RXDBYPASS_A::VALUE1 => true,
        }
    }
}
#[doc = "Reader of field `RXDBYPASS`"]
pub type RXDBYPASS_R = crate::R<bool, RXDBYPASS_A>;
impl RXDBYPASS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDBYPASS_A {
        match self.bits {
            false => RXDBYPASS_A::VALUE0,
            true => RXDBYPASS_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == RXDBYPASS_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RXDBYPASS_A::VALUE1
    }
}
#[doc = "Write proxy for field `RXDBYPASS`"]
pub struct RXDBYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDBYPASS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXDBYPASS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(RXDBYPASS_A::VALUE0)
    }
    #[doc = "Use the output of the USB_DP single-ended receiver in place of the full-speed differential receiver"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXDBYPASS_A::VALUE1)
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
impl R {
    #[doc = "Bits 0:2 - The ENVADJ field adjusts the trip point for the envelope detector"]
    #[inline(always)]
    pub fn envadj(&self) -> ENVADJ_R {
        ENVADJ_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - The DISCONADJ field adjusts the trip point for the disconnect detector."]
    #[inline(always)]
    pub fn disconadj(&self) -> DISCONADJ_R {
        DISCONADJ_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 22 - This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver"]
    #[inline(always)]
    pub fn rxdbypass(&self) -> RXDBYPASS_R {
        RXDBYPASS_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - The ENVADJ field adjusts the trip point for the envelope detector"]
    #[inline(always)]
    pub fn envadj(&mut self) -> ENVADJ_W {
        ENVADJ_W { w: self }
    }
    #[doc = "Bits 4:6 - The DISCONADJ field adjusts the trip point for the disconnect detector."]
    #[inline(always)]
    pub fn disconadj(&mut self) -> DISCONADJ_W {
        DISCONADJ_W { w: self }
    }
    #[doc = "Bit 22 - This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver"]
    #[inline(always)]
    pub fn rxdbypass(&mut self) -> RXDBYPASS_W {
        RXDBYPASS_W { w: self }
    }
}
