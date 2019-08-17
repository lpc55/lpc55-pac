#[doc = "Reader of register CMDL15"]
pub type R = crate::R<u32, super::CMDL15>;
#[doc = "Writer for register CMDL15"]
pub type W = crate::W<u32, super::CMDL15>;
#[doc = "Register CMDL15 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMDL15 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `ADCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCH_A {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    ADCH_0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    ADCH_1,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    ADCH_2,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    ADCH_3,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_4,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_5,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_6,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_7,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_8,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_9,
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    ADCH_30,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    ADCH_31,
}
impl From<ADCH_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCH_A) -> Self {
        match variant {
            ADCH_A::ADCH_0 => 0,
            ADCH_A::ADCH_1 => 1,
            ADCH_A::ADCH_2 => 2,
            ADCH_A::ADCH_3 => 3,
            ADCH_A::ADCH_4 => 4,
            ADCH_A::ADCH_5 => 5,
            ADCH_A::ADCH_6 => 6,
            ADCH_A::ADCH_7 => 7,
            ADCH_A::ADCH_8 => 8,
            ADCH_A::ADCH_9 => 9,
            ADCH_A::ADCH_30 => 30,
            ADCH_A::ADCH_31 => 31,
        }
    }
}
#[doc = "Reader of field `ADCH`"]
pub type ADCH_R = crate::R<u8, ADCH_A>;
impl ADCH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ADCH_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ADCH_A::ADCH_0),
            1 => Val(ADCH_A::ADCH_1),
            2 => Val(ADCH_A::ADCH_2),
            3 => Val(ADCH_A::ADCH_3),
            4 => Val(ADCH_A::ADCH_4),
            5 => Val(ADCH_A::ADCH_5),
            6 => Val(ADCH_A::ADCH_6),
            7 => Val(ADCH_A::ADCH_7),
            8 => Val(ADCH_A::ADCH_8),
            9 => Val(ADCH_A::ADCH_9),
            30 => Val(ADCH_A::ADCH_30),
            31 => Val(ADCH_A::ADCH_31),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADCH_0`"]
    #[inline(always)]
    pub fn is_adch_0(&self) -> bool {
        *self == ADCH_A::ADCH_0
    }
    #[doc = "Checks if the value of the field is `ADCH_1`"]
    #[inline(always)]
    pub fn is_adch_1(&self) -> bool {
        *self == ADCH_A::ADCH_1
    }
    #[doc = "Checks if the value of the field is `ADCH_2`"]
    #[inline(always)]
    pub fn is_adch_2(&self) -> bool {
        *self == ADCH_A::ADCH_2
    }
    #[doc = "Checks if the value of the field is `ADCH_3`"]
    #[inline(always)]
    pub fn is_adch_3(&self) -> bool {
        *self == ADCH_A::ADCH_3
    }
    #[doc = "Checks if the value of the field is `ADCH_4`"]
    #[inline(always)]
    pub fn is_adch_4(&self) -> bool {
        *self == ADCH_A::ADCH_4
    }
    #[doc = "Checks if the value of the field is `ADCH_5`"]
    #[inline(always)]
    pub fn is_adch_5(&self) -> bool {
        *self == ADCH_A::ADCH_5
    }
    #[doc = "Checks if the value of the field is `ADCH_6`"]
    #[inline(always)]
    pub fn is_adch_6(&self) -> bool {
        *self == ADCH_A::ADCH_6
    }
    #[doc = "Checks if the value of the field is `ADCH_7`"]
    #[inline(always)]
    pub fn is_adch_7(&self) -> bool {
        *self == ADCH_A::ADCH_7
    }
    #[doc = "Checks if the value of the field is `ADCH_8`"]
    #[inline(always)]
    pub fn is_adch_8(&self) -> bool {
        *self == ADCH_A::ADCH_8
    }
    #[doc = "Checks if the value of the field is `ADCH_9`"]
    #[inline(always)]
    pub fn is_adch_9(&self) -> bool {
        *self == ADCH_A::ADCH_9
    }
    #[doc = "Checks if the value of the field is `ADCH_30`"]
    #[inline(always)]
    pub fn is_adch_30(&self) -> bool {
        *self == ADCH_A::ADCH_30
    }
    #[doc = "Checks if the value of the field is `ADCH_31`"]
    #[inline(always)]
    pub fn is_adch_31(&self) -> bool {
        *self == ADCH_A::ADCH_31
    }
}
#[doc = "Write proxy for field `ADCH`"]
pub struct ADCH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    #[inline(always)]
    pub fn adch_0(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_0)
    }
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    #[inline(always)]
    pub fn adch_1(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_1)
    }
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    #[inline(always)]
    pub fn adch_2(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_2)
    }
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    #[inline(always)]
    pub fn adch_3(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_3)
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn adch_4(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_4)
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn adch_5(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_5)
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn adch_6(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_6)
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn adch_7(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_7)
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn adch_8(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_8)
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn adch_9(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_9)
    }
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    #[inline(always)]
    pub fn adch_30(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_30)
    }
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    #[inline(always)]
    pub fn adch_31(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Possible values of the field `CTYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTYPE_A {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    CTYPE_0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    CTYPE_1,
    #[doc = "Differential Mode. A-B."]
    CTYPE_2,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    CTYPE_3,
}
impl From<CTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: CTYPE_A) -> Self {
        match variant {
            CTYPE_A::CTYPE_0 => 0,
            CTYPE_A::CTYPE_1 => 1,
            CTYPE_A::CTYPE_2 => 2,
            CTYPE_A::CTYPE_3 => 3,
        }
    }
}
#[doc = "Reader of field `CTYPE`"]
pub type CTYPE_R = crate::R<u8, CTYPE_A>;
impl CTYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTYPE_A {
        match self.bits {
            0 => CTYPE_A::CTYPE_0,
            1 => CTYPE_A::CTYPE_1,
            2 => CTYPE_A::CTYPE_2,
            3 => CTYPE_A::CTYPE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CTYPE_0`"]
    #[inline(always)]
    pub fn is_ctype_0(&self) -> bool {
        *self == CTYPE_A::CTYPE_0
    }
    #[doc = "Checks if the value of the field is `CTYPE_1`"]
    #[inline(always)]
    pub fn is_ctype_1(&self) -> bool {
        *self == CTYPE_A::CTYPE_1
    }
    #[doc = "Checks if the value of the field is `CTYPE_2`"]
    #[inline(always)]
    pub fn is_ctype_2(&self) -> bool {
        *self == CTYPE_A::CTYPE_2
    }
    #[doc = "Checks if the value of the field is `CTYPE_3`"]
    #[inline(always)]
    pub fn is_ctype_3(&self) -> bool {
        *self == CTYPE_A::CTYPE_3
    }
}
#[doc = "Write proxy for field `CTYPE`"]
pub struct CTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTYPE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    #[inline(always)]
    pub fn ctype_0(self) -> &'a mut W {
        self.variant(CTYPE_A::CTYPE_0)
    }
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    #[inline(always)]
    pub fn ctype_1(self) -> &'a mut W {
        self.variant(CTYPE_A::CTYPE_1)
    }
    #[doc = "Differential Mode. A-B."]
    #[inline(always)]
    pub fn ctype_2(self) -> &'a mut W {
        self.variant(CTYPE_A::CTYPE_2)
    }
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    #[inline(always)]
    pub fn ctype_3(self) -> &'a mut W {
        self.variant(CTYPE_A::CTYPE_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    MODE_0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    MODE_1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        match variant {
            MODE_A::MODE_0 => false,
            MODE_A::MODE_1 => true,
        }
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<bool, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::MODE_0,
            true => MODE_A::MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `MODE_0`"]
    #[inline(always)]
    pub fn is_mode_0(&self) -> bool {
        *self == MODE_A::MODE_0
    }
    #[doc = "Checks if the value of the field is `MODE_1`"]
    #[inline(always)]
    pub fn is_mode_1(&self) -> bool {
        *self == MODE_A::MODE_1
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    #[inline(always)]
    pub fn mode_0(self) -> &'a mut W {
        self.variant(MODE_A::MODE_0)
    }
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    #[inline(always)]
    pub fn mode_1(self) -> &'a mut W {
        self.variant(MODE_A::MODE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Input channel select"]
    #[inline(always)]
    pub fn adch(&self) -> ADCH_R {
        ADCH_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - Conversion Type"]
    #[inline(always)]
    pub fn ctype(&self) -> CTYPE_R {
        CTYPE_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Select resolution of conversions"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input channel select"]
    #[inline(always)]
    pub fn adch(&mut self) -> ADCH_W {
        ADCH_W { w: self }
    }
    #[doc = "Bits 5:6 - Conversion Type"]
    #[inline(always)]
    pub fn ctype(&mut self) -> CTYPE_W {
        CTYPE_W { w: self }
    }
    #[doc = "Bit 7 - Select resolution of conversions"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
}
