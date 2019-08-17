#[doc = "Reader of register DEBUG1_CLR"]
pub type R = crate::R<u32, super::DEBUG1_CLR>;
#[doc = "Writer for register DEBUG1_CLR"]
pub type W = crate::W<u32, super::DEBUG1_CLR>;
#[doc = "Register DEBUG1_CLR `reset()`'s with value 0x1000"]
impl crate::ResetValue for super::DEBUG1_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1000
    }
}
#[doc = "Possible values of the field `ENTAILADJVD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENTAILADJVD_A {
    #[doc = "Delay is nominal"]
    VALUE0,
    #[doc = "Delay is +20%"]
    VALUE1,
    #[doc = "Delay is -20%"]
    VALUE2,
    #[doc = "Delay is -40%"]
    VALUE3,
}
impl From<ENTAILADJVD_A> for u8 {
    #[inline(always)]
    fn from(variant: ENTAILADJVD_A) -> Self {
        match variant {
            ENTAILADJVD_A::VALUE0 => 0,
            ENTAILADJVD_A::VALUE1 => 1,
            ENTAILADJVD_A::VALUE2 => 2,
            ENTAILADJVD_A::VALUE3 => 3,
        }
    }
}
#[doc = "Reader of field `ENTAILADJVD`"]
pub type ENTAILADJVD_R = crate::R<u8, ENTAILADJVD_A>;
impl ENTAILADJVD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENTAILADJVD_A {
        match self.bits {
            0 => ENTAILADJVD_A::VALUE0,
            1 => ENTAILADJVD_A::VALUE1,
            2 => ENTAILADJVD_A::VALUE2,
            3 => ENTAILADJVD_A::VALUE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == ENTAILADJVD_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ENTAILADJVD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ENTAILADJVD_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ENTAILADJVD_A::VALUE3
    }
}
#[doc = "Write proxy for field `ENTAILADJVD`"]
pub struct ENTAILADJVD_W<'a> {
    w: &'a mut W,
}
impl<'a> ENTAILADJVD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENTAILADJVD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Delay is nominal"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(ENTAILADJVD_A::VALUE0)
    }
    #[doc = "Delay is +20%"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENTAILADJVD_A::VALUE1)
    }
    #[doc = "Delay is -20%"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENTAILADJVD_A::VALUE2)
    }
    #[doc = "Delay is -40%"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ENTAILADJVD_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `USB2_REFBIAS_VBGADJ`"]
pub type USB2_REFBIAS_VBGADJ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USB2_REFBIAS_VBGADJ`"]
pub struct USB2_REFBIAS_VBGADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> USB2_REFBIAS_VBGADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Reader of field `USB2_REFBIAS_TST`"]
pub type USB2_REFBIAS_TST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USB2_REFBIAS_TST`"]
pub struct USB2_REFBIAS_TST_W<'a> {
    w: &'a mut W,
}
impl<'a> USB2_REFBIAS_TST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bits 13:14 - Delay increment of the rise of squelch:"]
    #[inline(always)]
    pub fn entailadjvd(&self) -> ENTAILADJVD_R {
        ENTAILADJVD_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bits 18:20 - Adjustment bits on bandgap"]
    #[inline(always)]
    pub fn usb2_refbias_vbgadj(&self) -> USB2_REFBIAS_VBGADJ_R {
        USB2_REFBIAS_VBGADJ_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 21:22 - Bias current control for usb2_phy"]
    #[inline(always)]
    pub fn usb2_refbias_tst(&self) -> USB2_REFBIAS_TST_R {
        USB2_REFBIAS_TST_R::new(((self.bits >> 21) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 13:14 - Delay increment of the rise of squelch:"]
    #[inline(always)]
    pub fn entailadjvd(&mut self) -> ENTAILADJVD_W {
        ENTAILADJVD_W { w: self }
    }
    #[doc = "Bits 18:20 - Adjustment bits on bandgap"]
    #[inline(always)]
    pub fn usb2_refbias_vbgadj(&mut self) -> USB2_REFBIAS_VBGADJ_W {
        USB2_REFBIAS_VBGADJ_W { w: self }
    }
    #[doc = "Bits 21:22 - Bias current control for usb2_phy"]
    #[inline(always)]
    pub fn usb2_refbias_tst(&mut self) -> USB2_REFBIAS_TST_W {
        USB2_REFBIAS_TST_W { w: self }
    }
}
