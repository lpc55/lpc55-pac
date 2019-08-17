#[doc = "Reader of register BODVBAT"]
pub type R = crate::R<u32, super::BODVBAT>;
#[doc = "Writer for register BODVBAT"]
pub type W = crate::W<u32, super::BODVBAT>;
#[doc = "Register BODVBAT `reset()`'s with value 0x69"]
impl crate::ResetValue for super::BODVBAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x69
    }
}
#[doc = "Possible values of the field `TRIGLVL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGLVL_A {
    #[doc = "1.00 V."]
    V_1P00,
    #[doc = "1.10 V."]
    V_1P10,
    #[doc = "1.20 V."]
    V_1P20,
    #[doc = "1.30 V."]
    V_1P30,
    #[doc = "1.40 V."]
    V_1P40,
    #[doc = "1.50 V."]
    V_1P50,
    #[doc = "1.60 V."]
    V_1P60,
    #[doc = "1.65 V."]
    V_1P65,
    #[doc = "1.70 V."]
    V_1P70,
    #[doc = "1.75 V."]
    V_1P75,
    #[doc = "1.80 V."]
    V_1P80,
    #[doc = "1.90 V."]
    V_1P90,
    #[doc = "2.00 V."]
    V_2P00,
    #[doc = "2.10 V."]
    V_2P10,
    #[doc = "2.20 V."]
    V_2P20,
    #[doc = "2.30 V."]
    V_2P30,
    #[doc = "2.40 V."]
    V_2P40,
    #[doc = "2.50 V."]
    V_2P50,
    #[doc = "2.60 V."]
    V_2P60,
    #[doc = "2.70 V."]
    V_2P70,
    #[doc = "2.806 V."]
    V_2P80,
    #[doc = "2.90 V."]
    V_2P90,
    #[doc = "3.00 V."]
    V_3P00,
    #[doc = "3.10 V."]
    V_3P10,
    #[doc = "3.20 V."]
    V_3P20,
    #[doc = "3.30 V."]
    V_3P30_2,
    #[doc = "3.30 V."]
    V_3P30_3,
    #[doc = "3.30 V."]
    V_3P30_4,
    #[doc = "3.30 V."]
    V_3P30_5,
    #[doc = "3.30 V."]
    V_3P30_6,
    #[doc = "3.30 V."]
    V_3P30_7,
    #[doc = "3.30 V."]
    V_3P30_8,
}
impl From<TRIGLVL_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGLVL_A) -> Self {
        match variant {
            TRIGLVL_A::V_1P00 => 0,
            TRIGLVL_A::V_1P10 => 1,
            TRIGLVL_A::V_1P20 => 2,
            TRIGLVL_A::V_1P30 => 3,
            TRIGLVL_A::V_1P40 => 4,
            TRIGLVL_A::V_1P50 => 5,
            TRIGLVL_A::V_1P60 => 6,
            TRIGLVL_A::V_1P65 => 7,
            TRIGLVL_A::V_1P70 => 8,
            TRIGLVL_A::V_1P75 => 9,
            TRIGLVL_A::V_1P80 => 10,
            TRIGLVL_A::V_1P90 => 11,
            TRIGLVL_A::V_2P00 => 12,
            TRIGLVL_A::V_2P10 => 13,
            TRIGLVL_A::V_2P20 => 14,
            TRIGLVL_A::V_2P30 => 15,
            TRIGLVL_A::V_2P40 => 16,
            TRIGLVL_A::V_2P50 => 17,
            TRIGLVL_A::V_2P60 => 18,
            TRIGLVL_A::V_2P70 => 19,
            TRIGLVL_A::V_2P80 => 20,
            TRIGLVL_A::V_2P90 => 21,
            TRIGLVL_A::V_3P00 => 22,
            TRIGLVL_A::V_3P10 => 23,
            TRIGLVL_A::V_3P20 => 24,
            TRIGLVL_A::V_3P30_2 => 25,
            TRIGLVL_A::V_3P30_3 => 26,
            TRIGLVL_A::V_3P30_4 => 27,
            TRIGLVL_A::V_3P30_5 => 28,
            TRIGLVL_A::V_3P30_6 => 29,
            TRIGLVL_A::V_3P30_7 => 30,
            TRIGLVL_A::V_3P30_8 => 31,
        }
    }
}
#[doc = "Reader of field `TRIGLVL`"]
pub type TRIGLVL_R = crate::R<u8, TRIGLVL_A>;
impl TRIGLVL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGLVL_A {
        match self.bits {
            0 => TRIGLVL_A::V_1P00,
            1 => TRIGLVL_A::V_1P10,
            2 => TRIGLVL_A::V_1P20,
            3 => TRIGLVL_A::V_1P30,
            4 => TRIGLVL_A::V_1P40,
            5 => TRIGLVL_A::V_1P50,
            6 => TRIGLVL_A::V_1P60,
            7 => TRIGLVL_A::V_1P65,
            8 => TRIGLVL_A::V_1P70,
            9 => TRIGLVL_A::V_1P75,
            10 => TRIGLVL_A::V_1P80,
            11 => TRIGLVL_A::V_1P90,
            12 => TRIGLVL_A::V_2P00,
            13 => TRIGLVL_A::V_2P10,
            14 => TRIGLVL_A::V_2P20,
            15 => TRIGLVL_A::V_2P30,
            16 => TRIGLVL_A::V_2P40,
            17 => TRIGLVL_A::V_2P50,
            18 => TRIGLVL_A::V_2P60,
            19 => TRIGLVL_A::V_2P70,
            20 => TRIGLVL_A::V_2P80,
            21 => TRIGLVL_A::V_2P90,
            22 => TRIGLVL_A::V_3P00,
            23 => TRIGLVL_A::V_3P10,
            24 => TRIGLVL_A::V_3P20,
            25 => TRIGLVL_A::V_3P30_2,
            26 => TRIGLVL_A::V_3P30_3,
            27 => TRIGLVL_A::V_3P30_4,
            28 => TRIGLVL_A::V_3P30_5,
            29 => TRIGLVL_A::V_3P30_6,
            30 => TRIGLVL_A::V_3P30_7,
            31 => TRIGLVL_A::V_3P30_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `V_1P00`"]
    #[inline(always)]
    pub fn is_v_1p00(&self) -> bool {
        *self == TRIGLVL_A::V_1P00
    }
    #[doc = "Checks if the value of the field is `V_1P10`"]
    #[inline(always)]
    pub fn is_v_1p10(&self) -> bool {
        *self == TRIGLVL_A::V_1P10
    }
    #[doc = "Checks if the value of the field is `V_1P20`"]
    #[inline(always)]
    pub fn is_v_1p20(&self) -> bool {
        *self == TRIGLVL_A::V_1P20
    }
    #[doc = "Checks if the value of the field is `V_1P30`"]
    #[inline(always)]
    pub fn is_v_1p30(&self) -> bool {
        *self == TRIGLVL_A::V_1P30
    }
    #[doc = "Checks if the value of the field is `V_1P40`"]
    #[inline(always)]
    pub fn is_v_1p40(&self) -> bool {
        *self == TRIGLVL_A::V_1P40
    }
    #[doc = "Checks if the value of the field is `V_1P50`"]
    #[inline(always)]
    pub fn is_v_1p50(&self) -> bool {
        *self == TRIGLVL_A::V_1P50
    }
    #[doc = "Checks if the value of the field is `V_1P60`"]
    #[inline(always)]
    pub fn is_v_1p60(&self) -> bool {
        *self == TRIGLVL_A::V_1P60
    }
    #[doc = "Checks if the value of the field is `V_1P65`"]
    #[inline(always)]
    pub fn is_v_1p65(&self) -> bool {
        *self == TRIGLVL_A::V_1P65
    }
    #[doc = "Checks if the value of the field is `V_1P70`"]
    #[inline(always)]
    pub fn is_v_1p70(&self) -> bool {
        *self == TRIGLVL_A::V_1P70
    }
    #[doc = "Checks if the value of the field is `V_1P75`"]
    #[inline(always)]
    pub fn is_v_1p75(&self) -> bool {
        *self == TRIGLVL_A::V_1P75
    }
    #[doc = "Checks if the value of the field is `V_1P80`"]
    #[inline(always)]
    pub fn is_v_1p80(&self) -> bool {
        *self == TRIGLVL_A::V_1P80
    }
    #[doc = "Checks if the value of the field is `V_1P90`"]
    #[inline(always)]
    pub fn is_v_1p90(&self) -> bool {
        *self == TRIGLVL_A::V_1P90
    }
    #[doc = "Checks if the value of the field is `V_2P00`"]
    #[inline(always)]
    pub fn is_v_2p00(&self) -> bool {
        *self == TRIGLVL_A::V_2P00
    }
    #[doc = "Checks if the value of the field is `V_2P10`"]
    #[inline(always)]
    pub fn is_v_2p10(&self) -> bool {
        *self == TRIGLVL_A::V_2P10
    }
    #[doc = "Checks if the value of the field is `V_2P20`"]
    #[inline(always)]
    pub fn is_v_2p20(&self) -> bool {
        *self == TRIGLVL_A::V_2P20
    }
    #[doc = "Checks if the value of the field is `V_2P30`"]
    #[inline(always)]
    pub fn is_v_2p30(&self) -> bool {
        *self == TRIGLVL_A::V_2P30
    }
    #[doc = "Checks if the value of the field is `V_2P40`"]
    #[inline(always)]
    pub fn is_v_2p40(&self) -> bool {
        *self == TRIGLVL_A::V_2P40
    }
    #[doc = "Checks if the value of the field is `V_2P50`"]
    #[inline(always)]
    pub fn is_v_2p50(&self) -> bool {
        *self == TRIGLVL_A::V_2P50
    }
    #[doc = "Checks if the value of the field is `V_2P60`"]
    #[inline(always)]
    pub fn is_v_2p60(&self) -> bool {
        *self == TRIGLVL_A::V_2P60
    }
    #[doc = "Checks if the value of the field is `V_2P70`"]
    #[inline(always)]
    pub fn is_v_2p70(&self) -> bool {
        *self == TRIGLVL_A::V_2P70
    }
    #[doc = "Checks if the value of the field is `V_2P80`"]
    #[inline(always)]
    pub fn is_v_2p80(&self) -> bool {
        *self == TRIGLVL_A::V_2P80
    }
    #[doc = "Checks if the value of the field is `V_2P90`"]
    #[inline(always)]
    pub fn is_v_2p90(&self) -> bool {
        *self == TRIGLVL_A::V_2P90
    }
    #[doc = "Checks if the value of the field is `V_3P00`"]
    #[inline(always)]
    pub fn is_v_3p00(&self) -> bool {
        *self == TRIGLVL_A::V_3P00
    }
    #[doc = "Checks if the value of the field is `V_3P10`"]
    #[inline(always)]
    pub fn is_v_3p10(&self) -> bool {
        *self == TRIGLVL_A::V_3P10
    }
    #[doc = "Checks if the value of the field is `V_3P20`"]
    #[inline(always)]
    pub fn is_v_3p20(&self) -> bool {
        *self == TRIGLVL_A::V_3P20
    }
    #[doc = "Checks if the value of the field is `V_3P30_2`"]
    #[inline(always)]
    pub fn is_v_3p30_2(&self) -> bool {
        *self == TRIGLVL_A::V_3P30_2
    }
    #[doc = "Checks if the value of the field is `V_3P30_3`"]
    #[inline(always)]
    pub fn is_v_3p30_3(&self) -> bool {
        *self == TRIGLVL_A::V_3P30_3
    }
    #[doc = "Checks if the value of the field is `V_3P30_4`"]
    #[inline(always)]
    pub fn is_v_3p30_4(&self) -> bool {
        *self == TRIGLVL_A::V_3P30_4
    }
    #[doc = "Checks if the value of the field is `V_3P30_5`"]
    #[inline(always)]
    pub fn is_v_3p30_5(&self) -> bool {
        *self == TRIGLVL_A::V_3P30_5
    }
    #[doc = "Checks if the value of the field is `V_3P30_6`"]
    #[inline(always)]
    pub fn is_v_3p30_6(&self) -> bool {
        *self == TRIGLVL_A::V_3P30_6
    }
    #[doc = "Checks if the value of the field is `V_3P30_7`"]
    #[inline(always)]
    pub fn is_v_3p30_7(&self) -> bool {
        *self == TRIGLVL_A::V_3P30_7
    }
    #[doc = "Checks if the value of the field is `V_3P30_8`"]
    #[inline(always)]
    pub fn is_v_3p30_8(&self) -> bool {
        *self == TRIGLVL_A::V_3P30_8
    }
}
#[doc = "Write proxy for field `TRIGLVL`"]
pub struct TRIGLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGLVL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGLVL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "1.00 V."]
    #[inline(always)]
    pub fn v_1p00(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_1P00)
    }
    #[doc = "1.10 V."]
    #[inline(always)]
    pub fn v_1p10(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_1P10)
    }
    #[doc = "1.20 V."]
    #[inline(always)]
    pub fn v_1p20(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_1P20)
    }
    #[doc = "1.30 V."]
    #[inline(always)]
    pub fn v_1p30(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_1P30)
    }
    #[doc = "1.40 V."]
    #[inline(always)]
    pub fn v_1p40(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_1P40)
    }
    #[doc = "1.50 V."]
    #[inline(always)]
    pub fn v_1p50(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_1P50)
    }
    #[doc = "1.60 V."]
    #[inline(always)]
    pub fn v_1p60(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_1P60)
    }
    #[doc = "1.65 V."]
    #[inline(always)]
    pub fn v_1p65(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_1P65)
    }
    #[doc = "1.70 V."]
    #[inline(always)]
    pub fn v_1p70(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_1P70)
    }
    #[doc = "1.75 V."]
    #[inline(always)]
    pub fn v_1p75(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_1P75)
    }
    #[doc = "1.80 V."]
    #[inline(always)]
    pub fn v_1p80(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_1P80)
    }
    #[doc = "1.90 V."]
    #[inline(always)]
    pub fn v_1p90(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_1P90)
    }
    #[doc = "2.00 V."]
    #[inline(always)]
    pub fn v_2p00(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_2P00)
    }
    #[doc = "2.10 V."]
    #[inline(always)]
    pub fn v_2p10(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_2P10)
    }
    #[doc = "2.20 V."]
    #[inline(always)]
    pub fn v_2p20(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_2P20)
    }
    #[doc = "2.30 V."]
    #[inline(always)]
    pub fn v_2p30(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_2P30)
    }
    #[doc = "2.40 V."]
    #[inline(always)]
    pub fn v_2p40(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_2P40)
    }
    #[doc = "2.50 V."]
    #[inline(always)]
    pub fn v_2p50(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_2P50)
    }
    #[doc = "2.60 V."]
    #[inline(always)]
    pub fn v_2p60(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_2P60)
    }
    #[doc = "2.70 V."]
    #[inline(always)]
    pub fn v_2p70(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_2P70)
    }
    #[doc = "2.806 V."]
    #[inline(always)]
    pub fn v_2p80(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_2P80)
    }
    #[doc = "2.90 V."]
    #[inline(always)]
    pub fn v_2p90(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_2P90)
    }
    #[doc = "3.00 V."]
    #[inline(always)]
    pub fn v_3p00(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_3P00)
    }
    #[doc = "3.10 V."]
    #[inline(always)]
    pub fn v_3p10(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_3P10)
    }
    #[doc = "3.20 V."]
    #[inline(always)]
    pub fn v_3p20(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_3P20)
    }
    #[doc = "3.30 V."]
    #[inline(always)]
    pub fn v_3p30_2(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_3P30_2)
    }
    #[doc = "3.30 V."]
    #[inline(always)]
    pub fn v_3p30_3(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_3P30_3)
    }
    #[doc = "3.30 V."]
    #[inline(always)]
    pub fn v_3p30_4(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_3P30_4)
    }
    #[doc = "3.30 V."]
    #[inline(always)]
    pub fn v_3p30_5(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_3P30_5)
    }
    #[doc = "3.30 V."]
    #[inline(always)]
    pub fn v_3p30_6(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_3P30_6)
    }
    #[doc = "3.30 V."]
    #[inline(always)]
    pub fn v_3p30_7(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_3P30_7)
    }
    #[doc = "3.30 V."]
    #[inline(always)]
    pub fn v_3p30_8(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_3P30_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Possible values of the field `HYST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYST_A {
    #[doc = "25 mV."]
    HYST_25MV,
    #[doc = "50 mV."]
    HYST_50MV,
    #[doc = "75 mV."]
    HYST_75MV,
    #[doc = "100 mV."]
    HYST_100MV,
}
impl From<HYST_A> for u8 {
    #[inline(always)]
    fn from(variant: HYST_A) -> Self {
        match variant {
            HYST_A::HYST_25MV => 0,
            HYST_A::HYST_50MV => 1,
            HYST_A::HYST_75MV => 2,
            HYST_A::HYST_100MV => 3,
        }
    }
}
#[doc = "Reader of field `HYST`"]
pub type HYST_R = crate::R<u8, HYST_A>;
impl HYST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYST_A {
        match self.bits {
            0 => HYST_A::HYST_25MV,
            1 => HYST_A::HYST_50MV,
            2 => HYST_A::HYST_75MV,
            3 => HYST_A::HYST_100MV,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HYST_25MV`"]
    #[inline(always)]
    pub fn is_hyst_25mv(&self) -> bool {
        *self == HYST_A::HYST_25MV
    }
    #[doc = "Checks if the value of the field is `HYST_50MV`"]
    #[inline(always)]
    pub fn is_hyst_50mv(&self) -> bool {
        *self == HYST_A::HYST_50MV
    }
    #[doc = "Checks if the value of the field is `HYST_75MV`"]
    #[inline(always)]
    pub fn is_hyst_75mv(&self) -> bool {
        *self == HYST_A::HYST_75MV
    }
    #[doc = "Checks if the value of the field is `HYST_100MV`"]
    #[inline(always)]
    pub fn is_hyst_100mv(&self) -> bool {
        *self == HYST_A::HYST_100MV
    }
}
#[doc = "Write proxy for field `HYST`"]
pub struct HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> HYST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HYST_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "25 mV."]
    #[inline(always)]
    pub fn hyst_25mv(self) -> &'a mut W {
        self.variant(HYST_A::HYST_25MV)
    }
    #[doc = "50 mV."]
    #[inline(always)]
    pub fn hyst_50mv(self) -> &'a mut W {
        self.variant(HYST_A::HYST_50MV)
    }
    #[doc = "75 mV."]
    #[inline(always)]
    pub fn hyst_75mv(self) -> &'a mut W {
        self.variant(HYST_A::HYST_75MV)
    }
    #[doc = "100 mV."]
    #[inline(always)]
    pub fn hyst_100mv(self) -> &'a mut W {
        self.variant(HYST_A::HYST_100MV)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - BoD trigger level."]
    #[inline(always)]
    pub fn triglvl(&self) -> TRIGLVL_R {
        TRIGLVL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - BoD Hysteresis control."]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 5) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - BoD trigger level."]
    #[inline(always)]
    pub fn triglvl(&mut self) -> TRIGLVL_W {
        TRIGLVL_W { w: self }
    }
    #[doc = "Bits 5:6 - BoD Hysteresis control."]
    #[inline(always)]
    pub fn hyst(&mut self) -> HYST_W {
        HYST_W { w: self }
    }
}
