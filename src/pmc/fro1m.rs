#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FRO1M {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct FREQSELR {
    bits: u8,
}
impl FREQSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ATBCTRLR {
    bits: u8,
}
impl ATBCTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DIVSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVSELR {
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
impl DIVSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DIVSELR::DIV_2 => 0,
            DIVSELR::DIV_4 => 1,
            DIVSELR::DIV_6 => 2,
            DIVSELR::DIV_8 => 3,
            DIVSELR::DIV_10 => 4,
            DIVSELR::DIV_12 => 5,
            DIVSELR::DIV_14 => 6,
            DIVSELR::DIV_16 => 7,
            DIVSELR::DIV_18 => 8,
            DIVSELR::DIV_20 => 9,
            DIVSELR::DIV_22 => 10,
            DIVSELR::DIV_24 => 11,
            DIVSELR::DIV_26 => 12,
            DIVSELR::DIV_28 => 13,
            DIVSELR::DIV_30 => 14,
            DIVSELR::DIV_32 => 15,
            DIVSELR::DIV_34 => 16,
            DIVSELR::DIV_36 => 17,
            DIVSELR::DIV_38 => 18,
            DIVSELR::DIV_40 => 19,
            DIVSELR::DIV_42 => 20,
            DIVSELR::DIV_44 => 21,
            DIVSELR::DIV_46 => 22,
            DIVSELR::DIV_48 => 23,
            DIVSELR::DIV_50 => 24,
            DIVSELR::DIV_52 => 25,
            DIVSELR::DIV_54 => 26,
            DIVSELR::DIV_56 => 27,
            DIVSELR::DIV_58 => 28,
            DIVSELR::DIV_60 => 29,
            DIVSELR::DIV_62 => 30,
            DIVSELR::DIV_1 => 31,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DIVSELR {
        match value {
            0 => DIVSELR::DIV_2,
            1 => DIVSELR::DIV_4,
            2 => DIVSELR::DIV_6,
            3 => DIVSELR::DIV_8,
            4 => DIVSELR::DIV_10,
            5 => DIVSELR::DIV_12,
            6 => DIVSELR::DIV_14,
            7 => DIVSELR::DIV_16,
            8 => DIVSELR::DIV_18,
            9 => DIVSELR::DIV_20,
            10 => DIVSELR::DIV_22,
            11 => DIVSELR::DIV_24,
            12 => DIVSELR::DIV_26,
            13 => DIVSELR::DIV_28,
            14 => DIVSELR::DIV_30,
            15 => DIVSELR::DIV_32,
            16 => DIVSELR::DIV_34,
            17 => DIVSELR::DIV_36,
            18 => DIVSELR::DIV_38,
            19 => DIVSELR::DIV_40,
            20 => DIVSELR::DIV_42,
            21 => DIVSELR::DIV_44,
            22 => DIVSELR::DIV_46,
            23 => DIVSELR::DIV_48,
            24 => DIVSELR::DIV_50,
            25 => DIVSELR::DIV_52,
            26 => DIVSELR::DIV_54,
            27 => DIVSELR::DIV_56,
            28 => DIVSELR::DIV_58,
            29 => DIVSELR::DIV_60,
            30 => DIVSELR::DIV_62,
            31 => DIVSELR::DIV_1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV_2`"]
    #[inline]
    pub fn is_div_2(&self) -> bool {
        *self == DIVSELR::DIV_2
    }
    #[doc = "Checks if the value of the field is `DIV_4`"]
    #[inline]
    pub fn is_div_4(&self) -> bool {
        *self == DIVSELR::DIV_4
    }
    #[doc = "Checks if the value of the field is `DIV_6`"]
    #[inline]
    pub fn is_div_6(&self) -> bool {
        *self == DIVSELR::DIV_6
    }
    #[doc = "Checks if the value of the field is `DIV_8`"]
    #[inline]
    pub fn is_div_8(&self) -> bool {
        *self == DIVSELR::DIV_8
    }
    #[doc = "Checks if the value of the field is `DIV_10`"]
    #[inline]
    pub fn is_div_10(&self) -> bool {
        *self == DIVSELR::DIV_10
    }
    #[doc = "Checks if the value of the field is `DIV_12`"]
    #[inline]
    pub fn is_div_12(&self) -> bool {
        *self == DIVSELR::DIV_12
    }
    #[doc = "Checks if the value of the field is `DIV_14`"]
    #[inline]
    pub fn is_div_14(&self) -> bool {
        *self == DIVSELR::DIV_14
    }
    #[doc = "Checks if the value of the field is `DIV_16`"]
    #[inline]
    pub fn is_div_16(&self) -> bool {
        *self == DIVSELR::DIV_16
    }
    #[doc = "Checks if the value of the field is `DIV_18`"]
    #[inline]
    pub fn is_div_18(&self) -> bool {
        *self == DIVSELR::DIV_18
    }
    #[doc = "Checks if the value of the field is `DIV_20`"]
    #[inline]
    pub fn is_div_20(&self) -> bool {
        *self == DIVSELR::DIV_20
    }
    #[doc = "Checks if the value of the field is `DIV_22`"]
    #[inline]
    pub fn is_div_22(&self) -> bool {
        *self == DIVSELR::DIV_22
    }
    #[doc = "Checks if the value of the field is `DIV_24`"]
    #[inline]
    pub fn is_div_24(&self) -> bool {
        *self == DIVSELR::DIV_24
    }
    #[doc = "Checks if the value of the field is `DIV_26`"]
    #[inline]
    pub fn is_div_26(&self) -> bool {
        *self == DIVSELR::DIV_26
    }
    #[doc = "Checks if the value of the field is `DIV_28`"]
    #[inline]
    pub fn is_div_28(&self) -> bool {
        *self == DIVSELR::DIV_28
    }
    #[doc = "Checks if the value of the field is `DIV_30`"]
    #[inline]
    pub fn is_div_30(&self) -> bool {
        *self == DIVSELR::DIV_30
    }
    #[doc = "Checks if the value of the field is `DIV_32`"]
    #[inline]
    pub fn is_div_32(&self) -> bool {
        *self == DIVSELR::DIV_32
    }
    #[doc = "Checks if the value of the field is `DIV_34`"]
    #[inline]
    pub fn is_div_34(&self) -> bool {
        *self == DIVSELR::DIV_34
    }
    #[doc = "Checks if the value of the field is `DIV_36`"]
    #[inline]
    pub fn is_div_36(&self) -> bool {
        *self == DIVSELR::DIV_36
    }
    #[doc = "Checks if the value of the field is `DIV_38`"]
    #[inline]
    pub fn is_div_38(&self) -> bool {
        *self == DIVSELR::DIV_38
    }
    #[doc = "Checks if the value of the field is `DIV_40`"]
    #[inline]
    pub fn is_div_40(&self) -> bool {
        *self == DIVSELR::DIV_40
    }
    #[doc = "Checks if the value of the field is `DIV_42`"]
    #[inline]
    pub fn is_div_42(&self) -> bool {
        *self == DIVSELR::DIV_42
    }
    #[doc = "Checks if the value of the field is `DIV_44`"]
    #[inline]
    pub fn is_div_44(&self) -> bool {
        *self == DIVSELR::DIV_44
    }
    #[doc = "Checks if the value of the field is `DIV_46`"]
    #[inline]
    pub fn is_div_46(&self) -> bool {
        *self == DIVSELR::DIV_46
    }
    #[doc = "Checks if the value of the field is `DIV_48`"]
    #[inline]
    pub fn is_div_48(&self) -> bool {
        *self == DIVSELR::DIV_48
    }
    #[doc = "Checks if the value of the field is `DIV_50`"]
    #[inline]
    pub fn is_div_50(&self) -> bool {
        *self == DIVSELR::DIV_50
    }
    #[doc = "Checks if the value of the field is `DIV_52`"]
    #[inline]
    pub fn is_div_52(&self) -> bool {
        *self == DIVSELR::DIV_52
    }
    #[doc = "Checks if the value of the field is `DIV_54`"]
    #[inline]
    pub fn is_div_54(&self) -> bool {
        *self == DIVSELR::DIV_54
    }
    #[doc = "Checks if the value of the field is `DIV_56`"]
    #[inline]
    pub fn is_div_56(&self) -> bool {
        *self == DIVSELR::DIV_56
    }
    #[doc = "Checks if the value of the field is `DIV_58`"]
    #[inline]
    pub fn is_div_58(&self) -> bool {
        *self == DIVSELR::DIV_58
    }
    #[doc = "Checks if the value of the field is `DIV_60`"]
    #[inline]
    pub fn is_div_60(&self) -> bool {
        *self == DIVSELR::DIV_60
    }
    #[doc = "Checks if the value of the field is `DIV_62`"]
    #[inline]
    pub fn is_div_62(&self) -> bool {
        *self == DIVSELR::DIV_62
    }
    #[doc = "Checks if the value of the field is `DIV_1`"]
    #[inline]
    pub fn is_div_1(&self) -> bool {
        *self == DIVSELR::DIV_1
    }
}
#[doc = r" Proxy"]
pub struct _FREQSELW<'a> {
    w: &'a mut W,
}
impl<'a> _FREQSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ATBCTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _ATBCTRLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DIVSEL`"]
pub enum DIVSELW {
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
impl DIVSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DIVSELW::DIV_2 => 0,
            DIVSELW::DIV_4 => 1,
            DIVSELW::DIV_6 => 2,
            DIVSELW::DIV_8 => 3,
            DIVSELW::DIV_10 => 4,
            DIVSELW::DIV_12 => 5,
            DIVSELW::DIV_14 => 6,
            DIVSELW::DIV_16 => 7,
            DIVSELW::DIV_18 => 8,
            DIVSELW::DIV_20 => 9,
            DIVSELW::DIV_22 => 10,
            DIVSELW::DIV_24 => 11,
            DIVSELW::DIV_26 => 12,
            DIVSELW::DIV_28 => 13,
            DIVSELW::DIV_30 => 14,
            DIVSELW::DIV_32 => 15,
            DIVSELW::DIV_34 => 16,
            DIVSELW::DIV_36 => 17,
            DIVSELW::DIV_38 => 18,
            DIVSELW::DIV_40 => 19,
            DIVSELW::DIV_42 => 20,
            DIVSELW::DIV_44 => 21,
            DIVSELW::DIV_46 => 22,
            DIVSELW::DIV_48 => 23,
            DIVSELW::DIV_50 => 24,
            DIVSELW::DIV_52 => 25,
            DIVSELW::DIV_54 => 26,
            DIVSELW::DIV_56 => 27,
            DIVSELW::DIV_58 => 28,
            DIVSELW::DIV_60 => 29,
            DIVSELW::DIV_62 => 30,
            DIVSELW::DIV_1 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIVSELW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIVSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "2.0."]
    #[inline]
    pub fn div_2(self) -> &'a mut W {
        self.variant(DIVSELW::DIV_2)
    }
    #[doc = "4.0."]
    #[inline]
    pub fn div_4(self) -> &'a mut W {
        self.variant(DIVSELW::DIV_4)
    }
    #[doc = "6.0."]
    #[inline]
    pub fn div_6(self) -> &'a mut W {
        self.variant(DIVSELW::DIV_6)
    }
    #[doc = "8.0."]
    #[inline]
    pub fn div_8(self) -> &'a mut W {
        self.variant(DIVSELW::DIV_8)
    }
    #[doc = "10.0."]
    #[inline]
    pub fn div_10(self) -> &'a mut W {
        self.variant(DIVSELW::DIV_10)
    }
    #[doc = "12.0."]
    #[inline]
    pub fn div_12(self) -> &'a mut W {
        self.variant(DIVSELW::DIV_12)
    }
    #[doc = "14.0."]
    #[inline]
    pub fn div_14(self) -> &'a mut W {
        self.variant(DIVSELW::DIV_14)
    }
    #[doc = "16.0."]
    #[inline]
    pub fn div_16(self) -> &'a mut W {
        self.variant(DIVSELW::DIV_16)
    }
    #[doc = "18.0."]
    #[inline]
    pub fn div_18(self) -> &'a mut W {
        self.variant(DIVSELW::DIV_18)
    }
    #[doc = "20.0."]
    #[inline]
    pub fn div_20(self) -> &'a mut W {
        self.variant(DIVSELW::DIV_20)
    }
    #[doc = "22.0."]
    #[inline]
    pub fn div_22(self) -> &'a mut W {
        self.variant(DIVSELW::DIV_22)
    }
    #[doc = "24.0."]
    #[inline]
    pub fn div_24(self) -> &'a mut W {
        self.variant(DIVSELW::DIV_24)
    }
    #[doc = "26.0."]
    #[inline]
    pub fn div_26(self) -> &'a mut W {
        self.variant(DIVSELW::DIV_26)
    }
    #[doc = "28.0."]
    #[inline]
    pub fn div_28(self) -> &'a mut W {
        self.variant(DIVSELW::DIV_28)
    }
    #[doc = "30.0."]
    #[inline]
    pub fn div_30(self) -> &'a mut W {
        self.variant(DIVSELW::DIV_30)
    }
    #[doc = "32.0."]
    #[inline]
    pub fn div_32(self) -> &'a mut W {
        self.variant(DIVSELW::DIV_32)
    }
    #[doc = "34.0."]
    #[inline]
    pub fn div_34(self) -> &'a mut W {
        self.variant(DIVSELW::DIV_34)
    }
    #[doc = "36.0."]
    #[inline]
    pub fn div_36(self) -> &'a mut W {
        self.variant(DIVSELW::DIV_36)
    }
    #[doc = "38.0."]
    #[inline]
    pub fn div_38(self) -> &'a mut W {
        self.variant(DIVSELW::DIV_38)
    }
    #[doc = "40.0."]
    #[inline]
    pub fn div_40(self) -> &'a mut W {
        self.variant(DIVSELW::DIV_40)
    }
    #[doc = "42.0."]
    #[inline]
    pub fn div_42(self) -> &'a mut W {
        self.variant(DIVSELW::DIV_42)
    }
    #[doc = "44.0."]
    #[inline]
    pub fn div_44(self) -> &'a mut W {
        self.variant(DIVSELW::DIV_44)
    }
    #[doc = "46.0."]
    #[inline]
    pub fn div_46(self) -> &'a mut W {
        self.variant(DIVSELW::DIV_46)
    }
    #[doc = "48.0."]
    #[inline]
    pub fn div_48(self) -> &'a mut W {
        self.variant(DIVSELW::DIV_48)
    }
    #[doc = "50.0."]
    #[inline]
    pub fn div_50(self) -> &'a mut W {
        self.variant(DIVSELW::DIV_50)
    }
    #[doc = "52.0."]
    #[inline]
    pub fn div_52(self) -> &'a mut W {
        self.variant(DIVSELW::DIV_52)
    }
    #[doc = "54.0."]
    #[inline]
    pub fn div_54(self) -> &'a mut W {
        self.variant(DIVSELW::DIV_54)
    }
    #[doc = "56.0."]
    #[inline]
    pub fn div_56(self) -> &'a mut W {
        self.variant(DIVSELW::DIV_56)
    }
    #[doc = "58.0."]
    #[inline]
    pub fn div_58(self) -> &'a mut W {
        self.variant(DIVSELW::DIV_58)
    }
    #[doc = "60.0."]
    #[inline]
    pub fn div_60(self) -> &'a mut W {
        self.variant(DIVSELW::DIV_60)
    }
    #[doc = "62.0."]
    #[inline]
    pub fn div_62(self) -> &'a mut W {
        self.variant(DIVSELW::DIV_62)
    }
    #[doc = "1.0."]
    #[inline]
    pub fn div_1(self) -> &'a mut W {
        self.variant(DIVSELW::DIV_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:6 - Frequency trimming bits."]
    #[inline]
    pub fn freqsel(&self) -> FREQSELR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FREQSELR { bits }
    }
    #[doc = "Bits 7:8 - Debug control bits to set the analog/digital test modes."]
    #[inline]
    pub fn atbctrl(&self) -> ATBCTRLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ATBCTRLR { bits }
    }
    #[doc = "Bits 9:13 - Divider selection bits."]
    #[inline]
    pub fn divsel(&self) -> DIVSELR {
        DIVSELR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 80 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:6 - Frequency trimming bits."]
    #[inline]
    pub fn freqsel(&mut self) -> _FREQSELW {
        _FREQSELW { w: self }
    }
    #[doc = "Bits 7:8 - Debug control bits to set the analog/digital test modes."]
    #[inline]
    pub fn atbctrl(&mut self) -> _ATBCTRLW {
        _ATBCTRLW { w: self }
    }
    #[doc = "Bits 9:13 - Divider selection bits."]
    #[inline]
    pub fn divsel(&mut self) -> _DIVSELW {
        _DIVSELW { w: self }
    }
}
