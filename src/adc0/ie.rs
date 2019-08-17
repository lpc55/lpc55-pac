#[doc = "Reader of register IE"]
pub type R = crate::R<u32, super::IE>;
#[doc = "Writer for register IE"]
pub type W = crate::W<u32, super::IE>;
#[doc = "Register IE `reset()`'s with value 0"]
impl crate::ResetValue for super::IE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `FWMIE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWMIE0_A {
    #[doc = "FIFO 0 watermark interrupts are not enabled."]
    FWMIE0_0,
    #[doc = "FIFO 0 watermark interrupts are enabled."]
    FWMIE0_1,
}
impl From<FWMIE0_A> for bool {
    #[inline(always)]
    fn from(variant: FWMIE0_A) -> Self {
        match variant {
            FWMIE0_A::FWMIE0_0 => false,
            FWMIE0_A::FWMIE0_1 => true,
        }
    }
}
#[doc = "Reader of field `FWMIE0`"]
pub type FWMIE0_R = crate::R<bool, FWMIE0_A>;
impl FWMIE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWMIE0_A {
        match self.bits {
            false => FWMIE0_A::FWMIE0_0,
            true => FWMIE0_A::FWMIE0_1,
        }
    }
    #[doc = "Checks if the value of the field is `FWMIE0_0`"]
    #[inline(always)]
    pub fn is_fwmie0_0(&self) -> bool {
        *self == FWMIE0_A::FWMIE0_0
    }
    #[doc = "Checks if the value of the field is `FWMIE0_1`"]
    #[inline(always)]
    pub fn is_fwmie0_1(&self) -> bool {
        *self == FWMIE0_A::FWMIE0_1
    }
}
#[doc = "Write proxy for field `FWMIE0`"]
pub struct FWMIE0_W<'a> {
    w: &'a mut W,
}
impl<'a> FWMIE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FWMIE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FIFO 0 watermark interrupts are not enabled."]
    #[inline(always)]
    pub fn fwmie0_0(self) -> &'a mut W {
        self.variant(FWMIE0_A::FWMIE0_0)
    }
    #[doc = "FIFO 0 watermark interrupts are enabled."]
    #[inline(always)]
    pub fn fwmie0_1(self) -> &'a mut W {
        self.variant(FWMIE0_A::FWMIE0_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Possible values of the field `FOFIE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOFIE0_A {
    #[doc = "FIFO 0 overflow interrupts are not enabled."]
    FOFIE0_0,
    #[doc = "FIFO 0 overflow interrupts are enabled."]
    FOFIE0_1,
}
impl From<FOFIE0_A> for bool {
    #[inline(always)]
    fn from(variant: FOFIE0_A) -> Self {
        match variant {
            FOFIE0_A::FOFIE0_0 => false,
            FOFIE0_A::FOFIE0_1 => true,
        }
    }
}
#[doc = "Reader of field `FOFIE0`"]
pub type FOFIE0_R = crate::R<bool, FOFIE0_A>;
impl FOFIE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FOFIE0_A {
        match self.bits {
            false => FOFIE0_A::FOFIE0_0,
            true => FOFIE0_A::FOFIE0_1,
        }
    }
    #[doc = "Checks if the value of the field is `FOFIE0_0`"]
    #[inline(always)]
    pub fn is_fofie0_0(&self) -> bool {
        *self == FOFIE0_A::FOFIE0_0
    }
    #[doc = "Checks if the value of the field is `FOFIE0_1`"]
    #[inline(always)]
    pub fn is_fofie0_1(&self) -> bool {
        *self == FOFIE0_A::FOFIE0_1
    }
}
#[doc = "Write proxy for field `FOFIE0`"]
pub struct FOFIE0_W<'a> {
    w: &'a mut W,
}
impl<'a> FOFIE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FOFIE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FIFO 0 overflow interrupts are not enabled."]
    #[inline(always)]
    pub fn fofie0_0(self) -> &'a mut W {
        self.variant(FOFIE0_A::FOFIE0_0)
    }
    #[doc = "FIFO 0 overflow interrupts are enabled."]
    #[inline(always)]
    pub fn fofie0_1(self) -> &'a mut W {
        self.variant(FOFIE0_A::FOFIE0_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Possible values of the field `FWMIE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWMIE1_A {
    #[doc = "FIFO1 watermark interrupts are not enabled."]
    FWMIE1_0,
    #[doc = "FIFO1 watermark interrupts are enabled."]
    FWMIE1_1,
}
impl From<FWMIE1_A> for bool {
    #[inline(always)]
    fn from(variant: FWMIE1_A) -> Self {
        match variant {
            FWMIE1_A::FWMIE1_0 => false,
            FWMIE1_A::FWMIE1_1 => true,
        }
    }
}
#[doc = "Reader of field `FWMIE1`"]
pub type FWMIE1_R = crate::R<bool, FWMIE1_A>;
impl FWMIE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWMIE1_A {
        match self.bits {
            false => FWMIE1_A::FWMIE1_0,
            true => FWMIE1_A::FWMIE1_1,
        }
    }
    #[doc = "Checks if the value of the field is `FWMIE1_0`"]
    #[inline(always)]
    pub fn is_fwmie1_0(&self) -> bool {
        *self == FWMIE1_A::FWMIE1_0
    }
    #[doc = "Checks if the value of the field is `FWMIE1_1`"]
    #[inline(always)]
    pub fn is_fwmie1_1(&self) -> bool {
        *self == FWMIE1_A::FWMIE1_1
    }
}
#[doc = "Write proxy for field `FWMIE1`"]
pub struct FWMIE1_W<'a> {
    w: &'a mut W,
}
impl<'a> FWMIE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FWMIE1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FIFO1 watermark interrupts are not enabled."]
    #[inline(always)]
    pub fn fwmie1_0(self) -> &'a mut W {
        self.variant(FWMIE1_A::FWMIE1_0)
    }
    #[doc = "FIFO1 watermark interrupts are enabled."]
    #[inline(always)]
    pub fn fwmie1_1(self) -> &'a mut W {
        self.variant(FWMIE1_A::FWMIE1_1)
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
#[doc = "Possible values of the field `FOFIE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOFIE1_A {
    #[doc = "No result FIFO1 overflow has occurred since the last time the flag was cleared."]
    FOFIE1_0,
    #[doc = "At least one result FIFO1 overflow has occurred since the last time the flag was cleared."]
    FOFIE1_1,
}
impl From<FOFIE1_A> for bool {
    #[inline(always)]
    fn from(variant: FOFIE1_A) -> Self {
        match variant {
            FOFIE1_A::FOFIE1_0 => false,
            FOFIE1_A::FOFIE1_1 => true,
        }
    }
}
#[doc = "Reader of field `FOFIE1`"]
pub type FOFIE1_R = crate::R<bool, FOFIE1_A>;
impl FOFIE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FOFIE1_A {
        match self.bits {
            false => FOFIE1_A::FOFIE1_0,
            true => FOFIE1_A::FOFIE1_1,
        }
    }
    #[doc = "Checks if the value of the field is `FOFIE1_0`"]
    #[inline(always)]
    pub fn is_fofie1_0(&self) -> bool {
        *self == FOFIE1_A::FOFIE1_0
    }
    #[doc = "Checks if the value of the field is `FOFIE1_1`"]
    #[inline(always)]
    pub fn is_fofie1_1(&self) -> bool {
        *self == FOFIE1_A::FOFIE1_1
    }
}
#[doc = "Write proxy for field `FOFIE1`"]
pub struct FOFIE1_W<'a> {
    w: &'a mut W,
}
impl<'a> FOFIE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FOFIE1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No result FIFO1 overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn fofie1_0(self) -> &'a mut W {
        self.variant(FOFIE1_A::FOFIE1_0)
    }
    #[doc = "At least one result FIFO1 overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn fofie1_1(self) -> &'a mut W {
        self.variant(FOFIE1_A::FOFIE1_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Possible values of the field `TEXC_IE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEXC_IE_A {
    #[doc = "Trigger exception interrupts are disabled."]
    TEXC_IE_0,
    #[doc = "Trigger exception interrupts are enabled."]
    TEXC_IE_1,
}
impl From<TEXC_IE_A> for bool {
    #[inline(always)]
    fn from(variant: TEXC_IE_A) -> Self {
        match variant {
            TEXC_IE_A::TEXC_IE_0 => false,
            TEXC_IE_A::TEXC_IE_1 => true,
        }
    }
}
#[doc = "Reader of field `TEXC_IE`"]
pub type TEXC_IE_R = crate::R<bool, TEXC_IE_A>;
impl TEXC_IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEXC_IE_A {
        match self.bits {
            false => TEXC_IE_A::TEXC_IE_0,
            true => TEXC_IE_A::TEXC_IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TEXC_IE_0`"]
    #[inline(always)]
    pub fn is_texc_ie_0(&self) -> bool {
        *self == TEXC_IE_A::TEXC_IE_0
    }
    #[doc = "Checks if the value of the field is `TEXC_IE_1`"]
    #[inline(always)]
    pub fn is_texc_ie_1(&self) -> bool {
        *self == TEXC_IE_A::TEXC_IE_1
    }
}
#[doc = "Write proxy for field `TEXC_IE`"]
pub struct TEXC_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEXC_IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEXC_IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger exception interrupts are disabled."]
    #[inline(always)]
    pub fn texc_ie_0(self) -> &'a mut W {
        self.variant(TEXC_IE_A::TEXC_IE_0)
    }
    #[doc = "Trigger exception interrupts are enabled."]
    #[inline(always)]
    pub fn texc_ie_1(self) -> &'a mut W {
        self.variant(TEXC_IE_A::TEXC_IE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `TCOMP_IE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCOMP_IE_A {
    #[doc = "Trigger completion interrupts are disabled."]
    TCOMP_IE_0,
    #[doc = "Trigger completion interrupts are enabled for trigger source 0 only."]
    TCOMP_IE_1,
    #[doc = "Trigger completion interrupts are enabled for trigger source 1 only."]
    TCOMP_IE_2,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TCOMP_IE_3,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TCOMP_IE_4,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TCOMP_IE_5,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TCOMP_IE_6,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TCOMP_IE_7,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TCOMP_IE_8,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TCOMP_IE_9,
    #[doc = "Trigger completion interrupts are enabled for every trigger source."]
    TCOMP_IE_65535,
}
impl From<TCOMP_IE_A> for u16 {
    #[inline(always)]
    fn from(variant: TCOMP_IE_A) -> Self {
        match variant {
            TCOMP_IE_A::TCOMP_IE_0 => 0,
            TCOMP_IE_A::TCOMP_IE_1 => 1,
            TCOMP_IE_A::TCOMP_IE_2 => 2,
            TCOMP_IE_A::TCOMP_IE_3 => 3,
            TCOMP_IE_A::TCOMP_IE_4 => 4,
            TCOMP_IE_A::TCOMP_IE_5 => 5,
            TCOMP_IE_A::TCOMP_IE_6 => 6,
            TCOMP_IE_A::TCOMP_IE_7 => 7,
            TCOMP_IE_A::TCOMP_IE_8 => 8,
            TCOMP_IE_A::TCOMP_IE_9 => 9,
            TCOMP_IE_A::TCOMP_IE_65535 => 65535,
        }
    }
}
#[doc = "Reader of field `TCOMP_IE`"]
pub type TCOMP_IE_R = crate::R<u16, TCOMP_IE_A>;
impl TCOMP_IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, TCOMP_IE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TCOMP_IE_A::TCOMP_IE_0),
            1 => Val(TCOMP_IE_A::TCOMP_IE_1),
            2 => Val(TCOMP_IE_A::TCOMP_IE_2),
            3 => Val(TCOMP_IE_A::TCOMP_IE_3),
            4 => Val(TCOMP_IE_A::TCOMP_IE_4),
            5 => Val(TCOMP_IE_A::TCOMP_IE_5),
            6 => Val(TCOMP_IE_A::TCOMP_IE_6),
            7 => Val(TCOMP_IE_A::TCOMP_IE_7),
            8 => Val(TCOMP_IE_A::TCOMP_IE_8),
            9 => Val(TCOMP_IE_A::TCOMP_IE_9),
            65535 => Val(TCOMP_IE_A::TCOMP_IE_65535),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_0`"]
    #[inline(always)]
    pub fn is_tcomp_ie_0(&self) -> bool {
        *self == TCOMP_IE_A::TCOMP_IE_0
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_1`"]
    #[inline(always)]
    pub fn is_tcomp_ie_1(&self) -> bool {
        *self == TCOMP_IE_A::TCOMP_IE_1
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_2`"]
    #[inline(always)]
    pub fn is_tcomp_ie_2(&self) -> bool {
        *self == TCOMP_IE_A::TCOMP_IE_2
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_3`"]
    #[inline(always)]
    pub fn is_tcomp_ie_3(&self) -> bool {
        *self == TCOMP_IE_A::TCOMP_IE_3
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_4`"]
    #[inline(always)]
    pub fn is_tcomp_ie_4(&self) -> bool {
        *self == TCOMP_IE_A::TCOMP_IE_4
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_5`"]
    #[inline(always)]
    pub fn is_tcomp_ie_5(&self) -> bool {
        *self == TCOMP_IE_A::TCOMP_IE_5
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_6`"]
    #[inline(always)]
    pub fn is_tcomp_ie_6(&self) -> bool {
        *self == TCOMP_IE_A::TCOMP_IE_6
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_7`"]
    #[inline(always)]
    pub fn is_tcomp_ie_7(&self) -> bool {
        *self == TCOMP_IE_A::TCOMP_IE_7
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_8`"]
    #[inline(always)]
    pub fn is_tcomp_ie_8(&self) -> bool {
        *self == TCOMP_IE_A::TCOMP_IE_8
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_9`"]
    #[inline(always)]
    pub fn is_tcomp_ie_9(&self) -> bool {
        *self == TCOMP_IE_A::TCOMP_IE_9
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_65535`"]
    #[inline(always)]
    pub fn is_tcomp_ie_65535(&self) -> bool {
        *self == TCOMP_IE_A::TCOMP_IE_65535
    }
}
#[doc = "Write proxy for field `TCOMP_IE`"]
pub struct TCOMP_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCOMP_IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCOMP_IE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Trigger completion interrupts are disabled."]
    #[inline(always)]
    pub fn tcomp_ie_0(self) -> &'a mut W {
        self.variant(TCOMP_IE_A::TCOMP_IE_0)
    }
    #[doc = "Trigger completion interrupts are enabled for trigger source 0 only."]
    #[inline(always)]
    pub fn tcomp_ie_1(self) -> &'a mut W {
        self.variant(TCOMP_IE_A::TCOMP_IE_1)
    }
    #[doc = "Trigger completion interrupts are enabled for trigger source 1 only."]
    #[inline(always)]
    pub fn tcomp_ie_2(self) -> &'a mut W {
        self.variant(TCOMP_IE_A::TCOMP_IE_2)
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline(always)]
    pub fn tcomp_ie_3(self) -> &'a mut W {
        self.variant(TCOMP_IE_A::TCOMP_IE_3)
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline(always)]
    pub fn tcomp_ie_4(self) -> &'a mut W {
        self.variant(TCOMP_IE_A::TCOMP_IE_4)
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline(always)]
    pub fn tcomp_ie_5(self) -> &'a mut W {
        self.variant(TCOMP_IE_A::TCOMP_IE_5)
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline(always)]
    pub fn tcomp_ie_6(self) -> &'a mut W {
        self.variant(TCOMP_IE_A::TCOMP_IE_6)
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline(always)]
    pub fn tcomp_ie_7(self) -> &'a mut W {
        self.variant(TCOMP_IE_A::TCOMP_IE_7)
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline(always)]
    pub fn tcomp_ie_8(self) -> &'a mut W {
        self.variant(TCOMP_IE_A::TCOMP_IE_8)
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline(always)]
    pub fn tcomp_ie_9(self) -> &'a mut W {
        self.variant(TCOMP_IE_A::TCOMP_IE_9)
    }
    #[doc = "Trigger completion interrupts are enabled for every trigger source."]
    #[inline(always)]
    pub fn tcomp_ie_65535(self) -> &'a mut W {
        self.variant(TCOMP_IE_A::TCOMP_IE_65535)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - FIFO 0 Watermark Interrupt Enable"]
    #[inline(always)]
    pub fn fwmie0(&self) -> FWMIE0_R {
        FWMIE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Result FIFO 0 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn fofie0(&self) -> FOFIE0_R {
        FOFIE0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FIFO1 Watermark Interrupt Enable"]
    #[inline(always)]
    pub fn fwmie1(&self) -> FWMIE1_R {
        FWMIE1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Result FIFO1 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn fofie1(&self) -> FOFIE1_R {
        FOFIE1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Trigger Exception Interrupt Enable"]
    #[inline(always)]
    pub fn texc_ie(&self) -> TEXC_IE_R {
        TEXC_IE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - Trigger Completion Interrupt Enable"]
    #[inline(always)]
    pub fn tcomp_ie(&self) -> TCOMP_IE_R {
        TCOMP_IE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO 0 Watermark Interrupt Enable"]
    #[inline(always)]
    pub fn fwmie0(&mut self) -> FWMIE0_W {
        FWMIE0_W { w: self }
    }
    #[doc = "Bit 1 - Result FIFO 0 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn fofie0(&mut self) -> FOFIE0_W {
        FOFIE0_W { w: self }
    }
    #[doc = "Bit 2 - FIFO1 Watermark Interrupt Enable"]
    #[inline(always)]
    pub fn fwmie1(&mut self) -> FWMIE1_W {
        FWMIE1_W { w: self }
    }
    #[doc = "Bit 3 - Result FIFO1 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn fofie1(&mut self) -> FOFIE1_W {
        FOFIE1_W { w: self }
    }
    #[doc = "Bit 8 - Trigger Exception Interrupt Enable"]
    #[inline(always)]
    pub fn texc_ie(&mut self) -> TEXC_IE_W {
        TEXC_IE_W { w: self }
    }
    #[doc = "Bits 16:31 - Trigger Completion Interrupt Enable"]
    #[inline(always)]
    pub fn tcomp_ie(&mut self) -> TCOMP_IE_W {
        TCOMP_IE_W { w: self }
    }
}
