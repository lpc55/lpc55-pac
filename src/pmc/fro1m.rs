#[doc = "Reader of register FRO1M"]
pub type R = crate::R<u32, super::FRO1M>;
#[doc = "Writer for register FRO1M"]
pub type W = crate::W<u32, super::FRO1M>;
#[doc = "Register FRO1M `reset()`'s with value 0x50"]
impl crate::ResetValue for super::FRO1M {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x50
    }
}
#[doc = "Reader of field `FREQSEL`"]
pub type FREQSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FREQSEL`"]
pub struct FREQSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `ATBCTRL`"]
pub type ATBCTRL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ATBCTRL`"]
pub struct ATBCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> ATBCTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | (((value as u32) & 0x03) << 7);
        self.w
    }
}
#[doc = "Possible values of the field `DIVSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVSEL_A {
    #[doc = "2.0."]
    DIV_2,
    #[doc = "4.0."]
    DIV_4,
    #[doc = "6.0."]
    DIV_6,
    #[doc = "8.0."]
    DIV_8,
    #[doc = "10.0."]
    DIV_10,
    #[doc = "12.0."]
    DIV_12,
    #[doc = "14.0."]
    DIV_14,
    #[doc = "16.0."]
    DIV_16,
    #[doc = "18.0."]
    DIV_18,
    #[doc = "20.0."]
    DIV_20,
    #[doc = "22.0."]
    DIV_22,
    #[doc = "24.0."]
    DIV_24,
    #[doc = "26.0."]
    DIV_26,
    #[doc = "28.0."]
    DIV_28,
    #[doc = "30.0."]
    DIV_30,
    #[doc = "32.0."]
    DIV_32,
    #[doc = "34.0."]
    DIV_34,
    #[doc = "36.0."]
    DIV_36,
    #[doc = "38.0."]
    DIV_38,
    #[doc = "40.0."]
    DIV_40,
    #[doc = "42.0."]
    DIV_42,
    #[doc = "44.0."]
    DIV_44,
    #[doc = "46.0."]
    DIV_46,
    #[doc = "48.0."]
    DIV_48,
    #[doc = "50.0."]
    DIV_50,
    #[doc = "52.0."]
    DIV_52,
    #[doc = "54.0."]
    DIV_54,
    #[doc = "56.0."]
    DIV_56,
    #[doc = "58.0."]
    DIV_58,
    #[doc = "60.0."]
    DIV_60,
    #[doc = "62.0."]
    DIV_62,
    #[doc = "1.0."]
    DIV_1,
}
impl From<DIVSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVSEL_A) -> Self {
        match variant {
            DIVSEL_A::DIV_2 => 0,
            DIVSEL_A::DIV_4 => 1,
            DIVSEL_A::DIV_6 => 2,
            DIVSEL_A::DIV_8 => 3,
            DIVSEL_A::DIV_10 => 4,
            DIVSEL_A::DIV_12 => 5,
            DIVSEL_A::DIV_14 => 6,
            DIVSEL_A::DIV_16 => 7,
            DIVSEL_A::DIV_18 => 8,
            DIVSEL_A::DIV_20 => 9,
            DIVSEL_A::DIV_22 => 10,
            DIVSEL_A::DIV_24 => 11,
            DIVSEL_A::DIV_26 => 12,
            DIVSEL_A::DIV_28 => 13,
            DIVSEL_A::DIV_30 => 14,
            DIVSEL_A::DIV_32 => 15,
            DIVSEL_A::DIV_34 => 16,
            DIVSEL_A::DIV_36 => 17,
            DIVSEL_A::DIV_38 => 18,
            DIVSEL_A::DIV_40 => 19,
            DIVSEL_A::DIV_42 => 20,
            DIVSEL_A::DIV_44 => 21,
            DIVSEL_A::DIV_46 => 22,
            DIVSEL_A::DIV_48 => 23,
            DIVSEL_A::DIV_50 => 24,
            DIVSEL_A::DIV_52 => 25,
            DIVSEL_A::DIV_54 => 26,
            DIVSEL_A::DIV_56 => 27,
            DIVSEL_A::DIV_58 => 28,
            DIVSEL_A::DIV_60 => 29,
            DIVSEL_A::DIV_62 => 30,
            DIVSEL_A::DIV_1 => 31,
        }
    }
}
#[doc = "Reader of field `DIVSEL`"]
pub type DIVSEL_R = crate::R<u8, DIVSEL_A>;
impl DIVSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVSEL_A {
        match self.bits {
            0 => DIVSEL_A::DIV_2,
            1 => DIVSEL_A::DIV_4,
            2 => DIVSEL_A::DIV_6,
            3 => DIVSEL_A::DIV_8,
            4 => DIVSEL_A::DIV_10,
            5 => DIVSEL_A::DIV_12,
            6 => DIVSEL_A::DIV_14,
            7 => DIVSEL_A::DIV_16,
            8 => DIVSEL_A::DIV_18,
            9 => DIVSEL_A::DIV_20,
            10 => DIVSEL_A::DIV_22,
            11 => DIVSEL_A::DIV_24,
            12 => DIVSEL_A::DIV_26,
            13 => DIVSEL_A::DIV_28,
            14 => DIVSEL_A::DIV_30,
            15 => DIVSEL_A::DIV_32,
            16 => DIVSEL_A::DIV_34,
            17 => DIVSEL_A::DIV_36,
            18 => DIVSEL_A::DIV_38,
            19 => DIVSEL_A::DIV_40,
            20 => DIVSEL_A::DIV_42,
            21 => DIVSEL_A::DIV_44,
            22 => DIVSEL_A::DIV_46,
            23 => DIVSEL_A::DIV_48,
            24 => DIVSEL_A::DIV_50,
            25 => DIVSEL_A::DIV_52,
            26 => DIVSEL_A::DIV_54,
            27 => DIVSEL_A::DIV_56,
            28 => DIVSEL_A::DIV_58,
            29 => DIVSEL_A::DIV_60,
            30 => DIVSEL_A::DIV_62,
            31 => DIVSEL_A::DIV_1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV_2`"]
    #[inline(always)]
    pub fn is_div_2(&self) -> bool {
        *self == DIVSEL_A::DIV_2
    }
    #[doc = "Checks if the value of the field is `DIV_4`"]
    #[inline(always)]
    pub fn is_div_4(&self) -> bool {
        *self == DIVSEL_A::DIV_4
    }
    #[doc = "Checks if the value of the field is `DIV_6`"]
    #[inline(always)]
    pub fn is_div_6(&self) -> bool {
        *self == DIVSEL_A::DIV_6
    }
    #[doc = "Checks if the value of the field is `DIV_8`"]
    #[inline(always)]
    pub fn is_div_8(&self) -> bool {
        *self == DIVSEL_A::DIV_8
    }
    #[doc = "Checks if the value of the field is `DIV_10`"]
    #[inline(always)]
    pub fn is_div_10(&self) -> bool {
        *self == DIVSEL_A::DIV_10
    }
    #[doc = "Checks if the value of the field is `DIV_12`"]
    #[inline(always)]
    pub fn is_div_12(&self) -> bool {
        *self == DIVSEL_A::DIV_12
    }
    #[doc = "Checks if the value of the field is `DIV_14`"]
    #[inline(always)]
    pub fn is_div_14(&self) -> bool {
        *self == DIVSEL_A::DIV_14
    }
    #[doc = "Checks if the value of the field is `DIV_16`"]
    #[inline(always)]
    pub fn is_div_16(&self) -> bool {
        *self == DIVSEL_A::DIV_16
    }
    #[doc = "Checks if the value of the field is `DIV_18`"]
    #[inline(always)]
    pub fn is_div_18(&self) -> bool {
        *self == DIVSEL_A::DIV_18
    }
    #[doc = "Checks if the value of the field is `DIV_20`"]
    #[inline(always)]
    pub fn is_div_20(&self) -> bool {
        *self == DIVSEL_A::DIV_20
    }
    #[doc = "Checks if the value of the field is `DIV_22`"]
    #[inline(always)]
    pub fn is_div_22(&self) -> bool {
        *self == DIVSEL_A::DIV_22
    }
    #[doc = "Checks if the value of the field is `DIV_24`"]
    #[inline(always)]
    pub fn is_div_24(&self) -> bool {
        *self == DIVSEL_A::DIV_24
    }
    #[doc = "Checks if the value of the field is `DIV_26`"]
    #[inline(always)]
    pub fn is_div_26(&self) -> bool {
        *self == DIVSEL_A::DIV_26
    }
    #[doc = "Checks if the value of the field is `DIV_28`"]
    #[inline(always)]
    pub fn is_div_28(&self) -> bool {
        *self == DIVSEL_A::DIV_28
    }
    #[doc = "Checks if the value of the field is `DIV_30`"]
    #[inline(always)]
    pub fn is_div_30(&self) -> bool {
        *self == DIVSEL_A::DIV_30
    }
    #[doc = "Checks if the value of the field is `DIV_32`"]
    #[inline(always)]
    pub fn is_div_32(&self) -> bool {
        *self == DIVSEL_A::DIV_32
    }
    #[doc = "Checks if the value of the field is `DIV_34`"]
    #[inline(always)]
    pub fn is_div_34(&self) -> bool {
        *self == DIVSEL_A::DIV_34
    }
    #[doc = "Checks if the value of the field is `DIV_36`"]
    #[inline(always)]
    pub fn is_div_36(&self) -> bool {
        *self == DIVSEL_A::DIV_36
    }
    #[doc = "Checks if the value of the field is `DIV_38`"]
    #[inline(always)]
    pub fn is_div_38(&self) -> bool {
        *self == DIVSEL_A::DIV_38
    }
    #[doc = "Checks if the value of the field is `DIV_40`"]
    #[inline(always)]
    pub fn is_div_40(&self) -> bool {
        *self == DIVSEL_A::DIV_40
    }
    #[doc = "Checks if the value of the field is `DIV_42`"]
    #[inline(always)]
    pub fn is_div_42(&self) -> bool {
        *self == DIVSEL_A::DIV_42
    }
    #[doc = "Checks if the value of the field is `DIV_44`"]
    #[inline(always)]
    pub fn is_div_44(&self) -> bool {
        *self == DIVSEL_A::DIV_44
    }
    #[doc = "Checks if the value of the field is `DIV_46`"]
    #[inline(always)]
    pub fn is_div_46(&self) -> bool {
        *self == DIVSEL_A::DIV_46
    }
    #[doc = "Checks if the value of the field is `DIV_48`"]
    #[inline(always)]
    pub fn is_div_48(&self) -> bool {
        *self == DIVSEL_A::DIV_48
    }
    #[doc = "Checks if the value of the field is `DIV_50`"]
    #[inline(always)]
    pub fn is_div_50(&self) -> bool {
        *self == DIVSEL_A::DIV_50
    }
    #[doc = "Checks if the value of the field is `DIV_52`"]
    #[inline(always)]
    pub fn is_div_52(&self) -> bool {
        *self == DIVSEL_A::DIV_52
    }
    #[doc = "Checks if the value of the field is `DIV_54`"]
    #[inline(always)]
    pub fn is_div_54(&self) -> bool {
        *self == DIVSEL_A::DIV_54
    }
    #[doc = "Checks if the value of the field is `DIV_56`"]
    #[inline(always)]
    pub fn is_div_56(&self) -> bool {
        *self == DIVSEL_A::DIV_56
    }
    #[doc = "Checks if the value of the field is `DIV_58`"]
    #[inline(always)]
    pub fn is_div_58(&self) -> bool {
        *self == DIVSEL_A::DIV_58
    }
    #[doc = "Checks if the value of the field is `DIV_60`"]
    #[inline(always)]
    pub fn is_div_60(&self) -> bool {
        *self == DIVSEL_A::DIV_60
    }
    #[doc = "Checks if the value of the field is `DIV_62`"]
    #[inline(always)]
    pub fn is_div_62(&self) -> bool {
        *self == DIVSEL_A::DIV_62
    }
    #[doc = "Checks if the value of the field is `DIV_1`"]
    #[inline(always)]
    pub fn is_div_1(&self) -> bool {
        *self == DIVSEL_A::DIV_1
    }
}
#[doc = "Write proxy for field `DIVSEL`"]
pub struct DIVSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "2.0."]
    #[inline(always)]
    pub fn div_2(self) -> &'a mut W {
        self.variant(DIVSEL_A::DIV_2)
    }
    #[doc = "4.0."]
    #[inline(always)]
    pub fn div_4(self) -> &'a mut W {
        self.variant(DIVSEL_A::DIV_4)
    }
    #[doc = "6.0."]
    #[inline(always)]
    pub fn div_6(self) -> &'a mut W {
        self.variant(DIVSEL_A::DIV_6)
    }
    #[doc = "8.0."]
    #[inline(always)]
    pub fn div_8(self) -> &'a mut W {
        self.variant(DIVSEL_A::DIV_8)
    }
    #[doc = "10.0."]
    #[inline(always)]
    pub fn div_10(self) -> &'a mut W {
        self.variant(DIVSEL_A::DIV_10)
    }
    #[doc = "12.0."]
    #[inline(always)]
    pub fn div_12(self) -> &'a mut W {
        self.variant(DIVSEL_A::DIV_12)
    }
    #[doc = "14.0."]
    #[inline(always)]
    pub fn div_14(self) -> &'a mut W {
        self.variant(DIVSEL_A::DIV_14)
    }
    #[doc = "16.0."]
    #[inline(always)]
    pub fn div_16(self) -> &'a mut W {
        self.variant(DIVSEL_A::DIV_16)
    }
    #[doc = "18.0."]
    #[inline(always)]
    pub fn div_18(self) -> &'a mut W {
        self.variant(DIVSEL_A::DIV_18)
    }
    #[doc = "20.0."]
    #[inline(always)]
    pub fn div_20(self) -> &'a mut W {
        self.variant(DIVSEL_A::DIV_20)
    }
    #[doc = "22.0."]
    #[inline(always)]
    pub fn div_22(self) -> &'a mut W {
        self.variant(DIVSEL_A::DIV_22)
    }
    #[doc = "24.0."]
    #[inline(always)]
    pub fn div_24(self) -> &'a mut W {
        self.variant(DIVSEL_A::DIV_24)
    }
    #[doc = "26.0."]
    #[inline(always)]
    pub fn div_26(self) -> &'a mut W {
        self.variant(DIVSEL_A::DIV_26)
    }
    #[doc = "28.0."]
    #[inline(always)]
    pub fn div_28(self) -> &'a mut W {
        self.variant(DIVSEL_A::DIV_28)
    }
    #[doc = "30.0."]
    #[inline(always)]
    pub fn div_30(self) -> &'a mut W {
        self.variant(DIVSEL_A::DIV_30)
    }
    #[doc = "32.0."]
    #[inline(always)]
    pub fn div_32(self) -> &'a mut W {
        self.variant(DIVSEL_A::DIV_32)
    }
    #[doc = "34.0."]
    #[inline(always)]
    pub fn div_34(self) -> &'a mut W {
        self.variant(DIVSEL_A::DIV_34)
    }
    #[doc = "36.0."]
    #[inline(always)]
    pub fn div_36(self) -> &'a mut W {
        self.variant(DIVSEL_A::DIV_36)
    }
    #[doc = "38.0."]
    #[inline(always)]
    pub fn div_38(self) -> &'a mut W {
        self.variant(DIVSEL_A::DIV_38)
    }
    #[doc = "40.0."]
    #[inline(always)]
    pub fn div_40(self) -> &'a mut W {
        self.variant(DIVSEL_A::DIV_40)
    }
    #[doc = "42.0."]
    #[inline(always)]
    pub fn div_42(self) -> &'a mut W {
        self.variant(DIVSEL_A::DIV_42)
    }
    #[doc = "44.0."]
    #[inline(always)]
    pub fn div_44(self) -> &'a mut W {
        self.variant(DIVSEL_A::DIV_44)
    }
    #[doc = "46.0."]
    #[inline(always)]
    pub fn div_46(self) -> &'a mut W {
        self.variant(DIVSEL_A::DIV_46)
    }
    #[doc = "48.0."]
    #[inline(always)]
    pub fn div_48(self) -> &'a mut W {
        self.variant(DIVSEL_A::DIV_48)
    }
    #[doc = "50.0."]
    #[inline(always)]
    pub fn div_50(self) -> &'a mut W {
        self.variant(DIVSEL_A::DIV_50)
    }
    #[doc = "52.0."]
    #[inline(always)]
    pub fn div_52(self) -> &'a mut W {
        self.variant(DIVSEL_A::DIV_52)
    }
    #[doc = "54.0."]
    #[inline(always)]
    pub fn div_54(self) -> &'a mut W {
        self.variant(DIVSEL_A::DIV_54)
    }
    #[doc = "56.0."]
    #[inline(always)]
    pub fn div_56(self) -> &'a mut W {
        self.variant(DIVSEL_A::DIV_56)
    }
    #[doc = "58.0."]
    #[inline(always)]
    pub fn div_58(self) -> &'a mut W {
        self.variant(DIVSEL_A::DIV_58)
    }
    #[doc = "60.0."]
    #[inline(always)]
    pub fn div_60(self) -> &'a mut W {
        self.variant(DIVSEL_A::DIV_60)
    }
    #[doc = "62.0."]
    #[inline(always)]
    pub fn div_62(self) -> &'a mut W {
        self.variant(DIVSEL_A::DIV_62)
    }
    #[doc = "1.0."]
    #[inline(always)]
    pub fn div_1(self) -> &'a mut W {
        self.variant(DIVSEL_A::DIV_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 9)) | (((value as u32) & 0x1f) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Frequency trimming bits."]
    #[inline(always)]
    pub fn freqsel(&self) -> FREQSEL_R {
        FREQSEL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:8 - Debug control bits to set the analog/digital test modes."]
    #[inline(always)]
    pub fn atbctrl(&self) -> ATBCTRL_R {
        ATBCTRL_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bits 9:13 - Divider selection bits."]
    #[inline(always)]
    pub fn divsel(&self) -> DIVSEL_R {
        DIVSEL_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Frequency trimming bits."]
    #[inline(always)]
    pub fn freqsel(&mut self) -> FREQSEL_W {
        FREQSEL_W { w: self }
    }
    #[doc = "Bits 7:8 - Debug control bits to set the analog/digital test modes."]
    #[inline(always)]
    pub fn atbctrl(&mut self) -> ATBCTRL_W {
        ATBCTRL_W { w: self }
    }
    #[doc = "Bits 9:13 - Divider selection bits."]
    #[inline(always)]
    pub fn divsel(&mut self) -> DIVSEL_W {
        DIVSEL_W { w: self }
    }
}
