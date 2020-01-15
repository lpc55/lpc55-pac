#[doc = "Reader of register SHAREDCTRLSET%s"]
pub type R = crate::R<u32, super::SHAREDCTRLSET>;
#[doc = "Writer for register SHAREDCTRLSET%s"]
pub type W = crate::W<u32, super::SHAREDCTRLSET>;
#[doc = "Register SHAREDCTRLSET%s `reset()`'s with value 0"]
impl crate::ResetValue for super::SHAREDCTRLSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selects the source for SCK of this shared signal set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SHAREDSCKSEL_A {
    #[doc = "0: SCK for this shared signal set comes from Flexcomm 0."]
    FLEXCOMM0 = 0,
    #[doc = "1: SCK for this shared signal set comes from Flexcomm 1."]
    FLEXCOMM1 = 1,
    #[doc = "2: SCK for this shared signal set comes from Flexcomm 2."]
    FLEXCOMM2 = 2,
    #[doc = "3: SCK for this shared signal set comes from Flexcomm 3."]
    FLEXCOMM3 = 3,
    #[doc = "4: SCK for this shared signal set comes from Flexcomm 4."]
    FLEXCOMM4 = 4,
    #[doc = "5: SCK for this shared signal set comes from Flexcomm 5."]
    FLEXCOMM5 = 5,
    #[doc = "6: SCK for this shared signal set comes from Flexcomm 6."]
    FLEXCOMM6 = 6,
    #[doc = "7: SCK for this shared signal set comes from Flexcomm 7."]
    FLEXCOMM7 = 7,
}
impl From<SHAREDSCKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SHAREDSCKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SHAREDSCKSEL`"]
pub type SHAREDSCKSEL_R = crate::R<u8, SHAREDSCKSEL_A>;
impl SHAREDSCKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHAREDSCKSEL_A {
        match self.bits {
            0 => SHAREDSCKSEL_A::FLEXCOMM0,
            1 => SHAREDSCKSEL_A::FLEXCOMM1,
            2 => SHAREDSCKSEL_A::FLEXCOMM2,
            3 => SHAREDSCKSEL_A::FLEXCOMM3,
            4 => SHAREDSCKSEL_A::FLEXCOMM4,
            5 => SHAREDSCKSEL_A::FLEXCOMM5,
            6 => SHAREDSCKSEL_A::FLEXCOMM6,
            7 => SHAREDSCKSEL_A::FLEXCOMM7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM0`"]
    #[inline(always)]
    pub fn is_flexcomm0(&self) -> bool {
        *self == SHAREDSCKSEL_A::FLEXCOMM0
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM1`"]
    #[inline(always)]
    pub fn is_flexcomm1(&self) -> bool {
        *self == SHAREDSCKSEL_A::FLEXCOMM1
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM2`"]
    #[inline(always)]
    pub fn is_flexcomm2(&self) -> bool {
        *self == SHAREDSCKSEL_A::FLEXCOMM2
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM3`"]
    #[inline(always)]
    pub fn is_flexcomm3(&self) -> bool {
        *self == SHAREDSCKSEL_A::FLEXCOMM3
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM4`"]
    #[inline(always)]
    pub fn is_flexcomm4(&self) -> bool {
        *self == SHAREDSCKSEL_A::FLEXCOMM4
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM5`"]
    #[inline(always)]
    pub fn is_flexcomm5(&self) -> bool {
        *self == SHAREDSCKSEL_A::FLEXCOMM5
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM6`"]
    #[inline(always)]
    pub fn is_flexcomm6(&self) -> bool {
        *self == SHAREDSCKSEL_A::FLEXCOMM6
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM7`"]
    #[inline(always)]
    pub fn is_flexcomm7(&self) -> bool {
        *self == SHAREDSCKSEL_A::FLEXCOMM7
    }
}
#[doc = "Write proxy for field `SHAREDSCKSEL`"]
pub struct SHAREDSCKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SHAREDSCKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SHAREDSCKSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 0."]
    #[inline(always)]
    pub fn flexcomm0(self) -> &'a mut W {
        self.variant(SHAREDSCKSEL_A::FLEXCOMM0)
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 1."]
    #[inline(always)]
    pub fn flexcomm1(self) -> &'a mut W {
        self.variant(SHAREDSCKSEL_A::FLEXCOMM1)
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 2."]
    #[inline(always)]
    pub fn flexcomm2(self) -> &'a mut W {
        self.variant(SHAREDSCKSEL_A::FLEXCOMM2)
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 3."]
    #[inline(always)]
    pub fn flexcomm3(self) -> &'a mut W {
        self.variant(SHAREDSCKSEL_A::FLEXCOMM3)
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 4."]
    #[inline(always)]
    pub fn flexcomm4(self) -> &'a mut W {
        self.variant(SHAREDSCKSEL_A::FLEXCOMM4)
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 5."]
    #[inline(always)]
    pub fn flexcomm5(self) -> &'a mut W {
        self.variant(SHAREDSCKSEL_A::FLEXCOMM5)
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 6."]
    #[inline(always)]
    pub fn flexcomm6(self) -> &'a mut W {
        self.variant(SHAREDSCKSEL_A::FLEXCOMM6)
    }
    #[doc = "SCK for this shared signal set comes from Flexcomm 7."]
    #[inline(always)]
    pub fn flexcomm7(self) -> &'a mut W {
        self.variant(SHAREDSCKSEL_A::FLEXCOMM7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Selects the source for WS of this shared signal set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SHAREDWSSEL_A {
    #[doc = "0: WS for this shared signal set comes from Flexcomm 0."]
    FLEXCOMM0 = 0,
    #[doc = "1: WS for this shared signal set comes from Flexcomm 1."]
    FLEXCOMM1 = 1,
    #[doc = "2: WS for this shared signal set comes from Flexcomm 2."]
    FLEXCOMM2 = 2,
    #[doc = "3: WS for this shared signal set comes from Flexcomm 3."]
    FLEXCOMM3 = 3,
    #[doc = "4: WS for this shared signal set comes from Flexcomm 4."]
    FLEXCOMM4 = 4,
    #[doc = "5: WS for this shared signal set comes from Flexcomm 5."]
    FLEXCOMM5 = 5,
    #[doc = "6: WS for this shared signal set comes from Flexcomm 6."]
    FLEXCOMM6 = 6,
    #[doc = "7: WS for this shared signal set comes from Flexcomm 7."]
    FLEXCOMM7 = 7,
}
impl From<SHAREDWSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SHAREDWSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SHAREDWSSEL`"]
pub type SHAREDWSSEL_R = crate::R<u8, SHAREDWSSEL_A>;
impl SHAREDWSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHAREDWSSEL_A {
        match self.bits {
            0 => SHAREDWSSEL_A::FLEXCOMM0,
            1 => SHAREDWSSEL_A::FLEXCOMM1,
            2 => SHAREDWSSEL_A::FLEXCOMM2,
            3 => SHAREDWSSEL_A::FLEXCOMM3,
            4 => SHAREDWSSEL_A::FLEXCOMM4,
            5 => SHAREDWSSEL_A::FLEXCOMM5,
            6 => SHAREDWSSEL_A::FLEXCOMM6,
            7 => SHAREDWSSEL_A::FLEXCOMM7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM0`"]
    #[inline(always)]
    pub fn is_flexcomm0(&self) -> bool {
        *self == SHAREDWSSEL_A::FLEXCOMM0
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM1`"]
    #[inline(always)]
    pub fn is_flexcomm1(&self) -> bool {
        *self == SHAREDWSSEL_A::FLEXCOMM1
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM2`"]
    #[inline(always)]
    pub fn is_flexcomm2(&self) -> bool {
        *self == SHAREDWSSEL_A::FLEXCOMM2
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM3`"]
    #[inline(always)]
    pub fn is_flexcomm3(&self) -> bool {
        *self == SHAREDWSSEL_A::FLEXCOMM3
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM4`"]
    #[inline(always)]
    pub fn is_flexcomm4(&self) -> bool {
        *self == SHAREDWSSEL_A::FLEXCOMM4
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM5`"]
    #[inline(always)]
    pub fn is_flexcomm5(&self) -> bool {
        *self == SHAREDWSSEL_A::FLEXCOMM5
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM6`"]
    #[inline(always)]
    pub fn is_flexcomm6(&self) -> bool {
        *self == SHAREDWSSEL_A::FLEXCOMM6
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM7`"]
    #[inline(always)]
    pub fn is_flexcomm7(&self) -> bool {
        *self == SHAREDWSSEL_A::FLEXCOMM7
    }
}
#[doc = "Write proxy for field `SHAREDWSSEL`"]
pub struct SHAREDWSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SHAREDWSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SHAREDWSSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 0."]
    #[inline(always)]
    pub fn flexcomm0(self) -> &'a mut W {
        self.variant(SHAREDWSSEL_A::FLEXCOMM0)
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 1."]
    #[inline(always)]
    pub fn flexcomm1(self) -> &'a mut W {
        self.variant(SHAREDWSSEL_A::FLEXCOMM1)
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 2."]
    #[inline(always)]
    pub fn flexcomm2(self) -> &'a mut W {
        self.variant(SHAREDWSSEL_A::FLEXCOMM2)
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 3."]
    #[inline(always)]
    pub fn flexcomm3(self) -> &'a mut W {
        self.variant(SHAREDWSSEL_A::FLEXCOMM3)
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 4."]
    #[inline(always)]
    pub fn flexcomm4(self) -> &'a mut W {
        self.variant(SHAREDWSSEL_A::FLEXCOMM4)
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 5."]
    #[inline(always)]
    pub fn flexcomm5(self) -> &'a mut W {
        self.variant(SHAREDWSSEL_A::FLEXCOMM5)
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 6."]
    #[inline(always)]
    pub fn flexcomm6(self) -> &'a mut W {
        self.variant(SHAREDWSSEL_A::FLEXCOMM6)
    }
    #[doc = "WS for this shared signal set comes from Flexcomm 7."]
    #[inline(always)]
    pub fn flexcomm7(self) -> &'a mut W {
        self.variant(SHAREDWSSEL_A::FLEXCOMM7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Selects the source for DATA input for this shared signal set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SHAREDDATASEL_A {
    #[doc = "0: DATA input for this shared signal set comes from Flexcomm 0."]
    FLEXCOMM0 = 0,
    #[doc = "1: DATA input for this shared signal set comes from Flexcomm 1."]
    FLEXCOMM1 = 1,
    #[doc = "2: DATA input for this shared signal set comes from Flexcomm 2."]
    FLEXCOMM2 = 2,
    #[doc = "3: DATA input for this shared signal set comes from Flexcomm 3."]
    FLEXCOMM3 = 3,
    #[doc = "4: DATA input for this shared signal set comes from Flexcomm 4."]
    FLEXCOMM4 = 4,
    #[doc = "5: DATA input for this shared signal set comes from Flexcomm 5."]
    FLEXCOMM5 = 5,
    #[doc = "6: DATA input for this shared signal set comes from Flexcomm 6."]
    FLEXCOMM6 = 6,
    #[doc = "7: DATA input for this shared signal set comes from Flexcomm 7."]
    FLEXCOMM7 = 7,
}
impl From<SHAREDDATASEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SHAREDDATASEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SHAREDDATASEL`"]
pub type SHAREDDATASEL_R = crate::R<u8, SHAREDDATASEL_A>;
impl SHAREDDATASEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHAREDDATASEL_A {
        match self.bits {
            0 => SHAREDDATASEL_A::FLEXCOMM0,
            1 => SHAREDDATASEL_A::FLEXCOMM1,
            2 => SHAREDDATASEL_A::FLEXCOMM2,
            3 => SHAREDDATASEL_A::FLEXCOMM3,
            4 => SHAREDDATASEL_A::FLEXCOMM4,
            5 => SHAREDDATASEL_A::FLEXCOMM5,
            6 => SHAREDDATASEL_A::FLEXCOMM6,
            7 => SHAREDDATASEL_A::FLEXCOMM7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM0`"]
    #[inline(always)]
    pub fn is_flexcomm0(&self) -> bool {
        *self == SHAREDDATASEL_A::FLEXCOMM0
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM1`"]
    #[inline(always)]
    pub fn is_flexcomm1(&self) -> bool {
        *self == SHAREDDATASEL_A::FLEXCOMM1
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM2`"]
    #[inline(always)]
    pub fn is_flexcomm2(&self) -> bool {
        *self == SHAREDDATASEL_A::FLEXCOMM2
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM3`"]
    #[inline(always)]
    pub fn is_flexcomm3(&self) -> bool {
        *self == SHAREDDATASEL_A::FLEXCOMM3
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM4`"]
    #[inline(always)]
    pub fn is_flexcomm4(&self) -> bool {
        *self == SHAREDDATASEL_A::FLEXCOMM4
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM5`"]
    #[inline(always)]
    pub fn is_flexcomm5(&self) -> bool {
        *self == SHAREDDATASEL_A::FLEXCOMM5
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM6`"]
    #[inline(always)]
    pub fn is_flexcomm6(&self) -> bool {
        *self == SHAREDDATASEL_A::FLEXCOMM6
    }
    #[doc = "Checks if the value of the field is `FLEXCOMM7`"]
    #[inline(always)]
    pub fn is_flexcomm7(&self) -> bool {
        *self == SHAREDDATASEL_A::FLEXCOMM7
    }
}
#[doc = "Write proxy for field `SHAREDDATASEL`"]
pub struct SHAREDDATASEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SHAREDDATASEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SHAREDDATASEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 0."]
    #[inline(always)]
    pub fn flexcomm0(self) -> &'a mut W {
        self.variant(SHAREDDATASEL_A::FLEXCOMM0)
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 1."]
    #[inline(always)]
    pub fn flexcomm1(self) -> &'a mut W {
        self.variant(SHAREDDATASEL_A::FLEXCOMM1)
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 2."]
    #[inline(always)]
    pub fn flexcomm2(self) -> &'a mut W {
        self.variant(SHAREDDATASEL_A::FLEXCOMM2)
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 3."]
    #[inline(always)]
    pub fn flexcomm3(self) -> &'a mut W {
        self.variant(SHAREDDATASEL_A::FLEXCOMM3)
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 4."]
    #[inline(always)]
    pub fn flexcomm4(self) -> &'a mut W {
        self.variant(SHAREDDATASEL_A::FLEXCOMM4)
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 5."]
    #[inline(always)]
    pub fn flexcomm5(self) -> &'a mut W {
        self.variant(SHAREDDATASEL_A::FLEXCOMM5)
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 6."]
    #[inline(always)]
    pub fn flexcomm6(self) -> &'a mut W {
        self.variant(SHAREDDATASEL_A::FLEXCOMM6)
    }
    #[doc = "DATA input for this shared signal set comes from Flexcomm 7."]
    #[inline(always)]
    pub fn flexcomm7(self) -> &'a mut W {
        self.variant(SHAREDDATASEL_A::FLEXCOMM7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Controls FC0 contribution to SHAREDDATAOUT for this shared set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC0DATAOUTEN_A {
    #[doc = "0: Data output from FC0 does not contribute to this shared set."]
    INPUT = 0,
    #[doc = "1: Data output from FC0 does contribute to this shared set."]
    OUTPUT = 1,
}
impl From<FC0DATAOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: FC0DATAOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FC0DATAOUTEN`"]
pub type FC0DATAOUTEN_R = crate::R<bool, FC0DATAOUTEN_A>;
impl FC0DATAOUTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC0DATAOUTEN_A {
        match self.bits {
            false => FC0DATAOUTEN_A::INPUT,
            true => FC0DATAOUTEN_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FC0DATAOUTEN_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FC0DATAOUTEN_A::OUTPUT
    }
}
#[doc = "Write proxy for field `FC0DATAOUTEN`"]
pub struct FC0DATAOUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FC0DATAOUTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC0DATAOUTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data output from FC0 does not contribute to this shared set."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FC0DATAOUTEN_A::INPUT)
    }
    #[doc = "Data output from FC0 does contribute to this shared set."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(FC0DATAOUTEN_A::OUTPUT)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Controls FC1 contribution to SHAREDDATAOUT for this shared set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC1DATAOUTEN_A {
    #[doc = "0: Data output from FC1 does not contribute to this shared set."]
    INPUT = 0,
    #[doc = "1: Data output from FC1 does contribute to this shared set."]
    OUTPUT = 1,
}
impl From<FC1DATAOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: FC1DATAOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FC1DATAOUTEN`"]
pub type FC1DATAOUTEN_R = crate::R<bool, FC1DATAOUTEN_A>;
impl FC1DATAOUTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC1DATAOUTEN_A {
        match self.bits {
            false => FC1DATAOUTEN_A::INPUT,
            true => FC1DATAOUTEN_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FC1DATAOUTEN_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FC1DATAOUTEN_A::OUTPUT
    }
}
#[doc = "Write proxy for field `FC1DATAOUTEN`"]
pub struct FC1DATAOUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FC1DATAOUTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC1DATAOUTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data output from FC1 does not contribute to this shared set."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FC1DATAOUTEN_A::INPUT)
    }
    #[doc = "Data output from FC1 does contribute to this shared set."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(FC1DATAOUTEN_A::OUTPUT)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Controls FC2 contribution to SHAREDDATAOUT for this shared set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC2DATAOUTEN_A {
    #[doc = "0: Data output from FC2 does not contribute to this shared set."]
    INPUT = 0,
    #[doc = "1: Data output from FC2 does contribute to this shared set."]
    OUTPUT = 1,
}
impl From<FC2DATAOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: FC2DATAOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FC2DATAOUTEN`"]
pub type FC2DATAOUTEN_R = crate::R<bool, FC2DATAOUTEN_A>;
impl FC2DATAOUTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC2DATAOUTEN_A {
        match self.bits {
            false => FC2DATAOUTEN_A::INPUT,
            true => FC2DATAOUTEN_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FC2DATAOUTEN_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FC2DATAOUTEN_A::OUTPUT
    }
}
#[doc = "Write proxy for field `FC2DATAOUTEN`"]
pub struct FC2DATAOUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FC2DATAOUTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC2DATAOUTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data output from FC2 does not contribute to this shared set."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FC2DATAOUTEN_A::INPUT)
    }
    #[doc = "Data output from FC2 does contribute to this shared set."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(FC2DATAOUTEN_A::OUTPUT)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Controls FC4 contribution to SHAREDDATAOUT for this shared set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC4DATAOUTEN_A {
    #[doc = "0: Data output from FC4 does not contribute to this shared set."]
    INPUT = 0,
    #[doc = "1: Data output from FC4 does contribute to this shared set."]
    OUTPUT = 1,
}
impl From<FC4DATAOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: FC4DATAOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FC4DATAOUTEN`"]
pub type FC4DATAOUTEN_R = crate::R<bool, FC4DATAOUTEN_A>;
impl FC4DATAOUTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC4DATAOUTEN_A {
        match self.bits {
            false => FC4DATAOUTEN_A::INPUT,
            true => FC4DATAOUTEN_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FC4DATAOUTEN_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FC4DATAOUTEN_A::OUTPUT
    }
}
#[doc = "Write proxy for field `FC4DATAOUTEN`"]
pub struct FC4DATAOUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FC4DATAOUTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC4DATAOUTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data output from FC4 does not contribute to this shared set."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FC4DATAOUTEN_A::INPUT)
    }
    #[doc = "Data output from FC4 does contribute to this shared set."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(FC4DATAOUTEN_A::OUTPUT)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Controls FC5 contribution to SHAREDDATAOUT for this shared set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC5DATAOUTEN_A {
    #[doc = "0: Data output from FC5 does not contribute to this shared set."]
    INPUT = 0,
    #[doc = "1: Data output from FC5 does contribute to this shared set."]
    OUTPUT = 1,
}
impl From<FC5DATAOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: FC5DATAOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FC5DATAOUTEN`"]
pub type FC5DATAOUTEN_R = crate::R<bool, FC5DATAOUTEN_A>;
impl FC5DATAOUTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC5DATAOUTEN_A {
        match self.bits {
            false => FC5DATAOUTEN_A::INPUT,
            true => FC5DATAOUTEN_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FC5DATAOUTEN_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FC5DATAOUTEN_A::OUTPUT
    }
}
#[doc = "Write proxy for field `FC5DATAOUTEN`"]
pub struct FC5DATAOUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FC5DATAOUTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC5DATAOUTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data output from FC5 does not contribute to this shared set."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FC5DATAOUTEN_A::INPUT)
    }
    #[doc = "Data output from FC5 does contribute to this shared set."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(FC5DATAOUTEN_A::OUTPUT)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Controls FC6 contribution to SHAREDDATAOUT for this shared set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC6DATAOUTEN_A {
    #[doc = "0: Data output from FC6 does not contribute to this shared set."]
    INPUT = 0,
    #[doc = "1: Data output from FC6 does contribute to this shared set."]
    OUTPUT = 1,
}
impl From<FC6DATAOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: FC6DATAOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FC6DATAOUTEN`"]
pub type FC6DATAOUTEN_R = crate::R<bool, FC6DATAOUTEN_A>;
impl FC6DATAOUTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC6DATAOUTEN_A {
        match self.bits {
            false => FC6DATAOUTEN_A::INPUT,
            true => FC6DATAOUTEN_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FC6DATAOUTEN_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FC6DATAOUTEN_A::OUTPUT
    }
}
#[doc = "Write proxy for field `FC6DATAOUTEN`"]
pub struct FC6DATAOUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FC6DATAOUTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC6DATAOUTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data output from FC6 does not contribute to this shared set."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FC6DATAOUTEN_A::INPUT)
    }
    #[doc = "Data output from FC6 does contribute to this shared set."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(FC6DATAOUTEN_A::OUTPUT)
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
#[doc = "Controls FC7 contribution to SHAREDDATAOUT for this shared set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC7DATAOUTEN_A {
    #[doc = "0: Data output from FC7 does not contribute to this shared set."]
    INPUT = 0,
    #[doc = "1: Data output from FC7 does contribute to this shared set."]
    OUTPUT = 1,
}
impl From<FC7DATAOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: FC7DATAOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FC7DATAOUTEN`"]
pub type FC7DATAOUTEN_R = crate::R<bool, FC7DATAOUTEN_A>;
impl FC7DATAOUTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC7DATAOUTEN_A {
        match self.bits {
            false => FC7DATAOUTEN_A::INPUT,
            true => FC7DATAOUTEN_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FC7DATAOUTEN_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == FC7DATAOUTEN_A::OUTPUT
    }
}
#[doc = "Write proxy for field `FC7DATAOUTEN`"]
pub struct FC7DATAOUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FC7DATAOUTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC7DATAOUTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data output from FC7 does not contribute to this shared set."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FC7DATAOUTEN_A::INPUT)
    }
    #[doc = "Data output from FC7 does contribute to this shared set."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(FC7DATAOUTEN_A::OUTPUT)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects the source for SCK of this shared signal set."]
    #[inline(always)]
    pub fn sharedscksel(&self) -> SHAREDSCKSEL_R {
        SHAREDSCKSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Selects the source for WS of this shared signal set."]
    #[inline(always)]
    pub fn sharedwssel(&self) -> SHAREDWSSEL_R {
        SHAREDWSSEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Selects the source for DATA input for this shared signal set."]
    #[inline(always)]
    pub fn shareddatasel(&self) -> SHAREDDATASEL_R {
        SHAREDDATASEL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 16 - Controls FC0 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub fn fc0dataouten(&self) -> FC0DATAOUTEN_R {
        FC0DATAOUTEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Controls FC1 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub fn fc1dataouten(&self) -> FC1DATAOUTEN_R {
        FC1DATAOUTEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Controls FC2 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub fn fc2dataouten(&self) -> FC2DATAOUTEN_R {
        FC2DATAOUTEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Controls FC4 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub fn fc4dataouten(&self) -> FC4DATAOUTEN_R {
        FC4DATAOUTEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Controls FC5 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub fn fc5dataouten(&self) -> FC5DATAOUTEN_R {
        FC5DATAOUTEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Controls FC6 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub fn fc6dataouten(&self) -> FC6DATAOUTEN_R {
        FC6DATAOUTEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Controls FC7 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub fn fc7dataouten(&self) -> FC7DATAOUTEN_R {
        FC7DATAOUTEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects the source for SCK of this shared signal set."]
    #[inline(always)]
    pub fn sharedscksel(&mut self) -> SHAREDSCKSEL_W {
        SHAREDSCKSEL_W { w: self }
    }
    #[doc = "Bits 4:6 - Selects the source for WS of this shared signal set."]
    #[inline(always)]
    pub fn sharedwssel(&mut self) -> SHAREDWSSEL_W {
        SHAREDWSSEL_W { w: self }
    }
    #[doc = "Bits 8:10 - Selects the source for DATA input for this shared signal set."]
    #[inline(always)]
    pub fn shareddatasel(&mut self) -> SHAREDDATASEL_W {
        SHAREDDATASEL_W { w: self }
    }
    #[doc = "Bit 16 - Controls FC0 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub fn fc0dataouten(&mut self) -> FC0DATAOUTEN_W {
        FC0DATAOUTEN_W { w: self }
    }
    #[doc = "Bit 17 - Controls FC1 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub fn fc1dataouten(&mut self) -> FC1DATAOUTEN_W {
        FC1DATAOUTEN_W { w: self }
    }
    #[doc = "Bit 18 - Controls FC2 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub fn fc2dataouten(&mut self) -> FC2DATAOUTEN_W {
        FC2DATAOUTEN_W { w: self }
    }
    #[doc = "Bit 20 - Controls FC4 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub fn fc4dataouten(&mut self) -> FC4DATAOUTEN_W {
        FC4DATAOUTEN_W { w: self }
    }
    #[doc = "Bit 21 - Controls FC5 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub fn fc5dataouten(&mut self) -> FC5DATAOUTEN_W {
        FC5DATAOUTEN_W { w: self }
    }
    #[doc = "Bit 22 - Controls FC6 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub fn fc6dataouten(&mut self) -> FC6DATAOUTEN_W {
        FC6DATAOUTEN_W { w: self }
    }
    #[doc = "Bit 23 - Controls FC7 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub fn fc7dataouten(&mut self) -> FC7DATAOUTEN_W {
        FC7DATAOUTEN_W { w: self }
    }
}
