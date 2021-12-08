#[doc = "Register `BODVBAT` reader"]
pub struct R(crate::R<BODVBAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BODVBAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BODVBAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BODVBAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BODVBAT` writer"]
pub struct W(crate::W<BODVBAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BODVBAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<BODVBAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BODVBAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "BoD trigger level.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRIGLVL_A {
    #[doc = "0: 1.00 V."]
    V_1P00 = 0,
    #[doc = "1: 1.10 V."]
    V_1P10 = 1,
    #[doc = "2: 1.20 V."]
    V_1P20 = 2,
    #[doc = "3: 1.30 V."]
    V_1P30 = 3,
    #[doc = "4: 1.40 V."]
    V_1P40 = 4,
    #[doc = "5: 1.50 V."]
    V_1P50 = 5,
    #[doc = "6: 1.60 V."]
    V_1P60 = 6,
    #[doc = "7: 1.65 V."]
    V_1P65 = 7,
    #[doc = "8: 1.70 V."]
    V_1P70 = 8,
    #[doc = "9: 1.75 V."]
    V_1P75 = 9,
    #[doc = "10: 1.80 V."]
    V_1P80 = 10,
    #[doc = "11: 1.90 V."]
    V_1P90 = 11,
    #[doc = "12: 2.00 V."]
    V_2P00 = 12,
    #[doc = "13: 2.10 V."]
    V_2P10 = 13,
    #[doc = "14: 2.20 V."]
    V_2P20 = 14,
    #[doc = "15: 2.30 V."]
    V_2P30 = 15,
    #[doc = "16: 2.40 V."]
    V_2P40 = 16,
    #[doc = "17: 2.50 V."]
    V_2P50 = 17,
    #[doc = "18: 2.60 V."]
    V_2P60 = 18,
    #[doc = "19: 2.70 V."]
    V_2P70 = 19,
    #[doc = "20: 2.806 V."]
    V_2P80 = 20,
    #[doc = "21: 2.90 V."]
    V_2P90 = 21,
    #[doc = "22: 3.00 V."]
    V_3P00 = 22,
    #[doc = "23: 3.10 V."]
    V_3P10 = 23,
    #[doc = "24: 3.20 V."]
    V_3P20 = 24,
    #[doc = "25: 3.30 V."]
    V_3P30_2 = 25,
    #[doc = "26: 3.30 V."]
    V_3P30_3 = 26,
    #[doc = "27: 3.30 V."]
    V_3P30_4 = 27,
    #[doc = "28: 3.30 V."]
    V_3P30_5 = 28,
    #[doc = "29: 3.30 V."]
    V_3P30_6 = 29,
    #[doc = "30: 3.30 V."]
    V_3P30_7 = 30,
    #[doc = "31: 3.30 V."]
    V_3P30_8 = 31,
}
impl From<TRIGLVL_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGLVL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRIGLVL` reader - BoD trigger level."]
pub struct TRIGLVL_R(crate::FieldReader<u8, TRIGLVL_A>);
impl TRIGLVL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TRIGLVL_R(crate::FieldReader::new(bits))
    }
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
        **self == TRIGLVL_A::V_1P00
    }
    #[doc = "Checks if the value of the field is `V_1P10`"]
    #[inline(always)]
    pub fn is_v_1p10(&self) -> bool {
        **self == TRIGLVL_A::V_1P10
    }
    #[doc = "Checks if the value of the field is `V_1P20`"]
    #[inline(always)]
    pub fn is_v_1p20(&self) -> bool {
        **self == TRIGLVL_A::V_1P20
    }
    #[doc = "Checks if the value of the field is `V_1P30`"]
    #[inline(always)]
    pub fn is_v_1p30(&self) -> bool {
        **self == TRIGLVL_A::V_1P30
    }
    #[doc = "Checks if the value of the field is `V_1P40`"]
    #[inline(always)]
    pub fn is_v_1p40(&self) -> bool {
        **self == TRIGLVL_A::V_1P40
    }
    #[doc = "Checks if the value of the field is `V_1P50`"]
    #[inline(always)]
    pub fn is_v_1p50(&self) -> bool {
        **self == TRIGLVL_A::V_1P50
    }
    #[doc = "Checks if the value of the field is `V_1P60`"]
    #[inline(always)]
    pub fn is_v_1p60(&self) -> bool {
        **self == TRIGLVL_A::V_1P60
    }
    #[doc = "Checks if the value of the field is `V_1P65`"]
    #[inline(always)]
    pub fn is_v_1p65(&self) -> bool {
        **self == TRIGLVL_A::V_1P65
    }
    #[doc = "Checks if the value of the field is `V_1P70`"]
    #[inline(always)]
    pub fn is_v_1p70(&self) -> bool {
        **self == TRIGLVL_A::V_1P70
    }
    #[doc = "Checks if the value of the field is `V_1P75`"]
    #[inline(always)]
    pub fn is_v_1p75(&self) -> bool {
        **self == TRIGLVL_A::V_1P75
    }
    #[doc = "Checks if the value of the field is `V_1P80`"]
    #[inline(always)]
    pub fn is_v_1p80(&self) -> bool {
        **self == TRIGLVL_A::V_1P80
    }
    #[doc = "Checks if the value of the field is `V_1P90`"]
    #[inline(always)]
    pub fn is_v_1p90(&self) -> bool {
        **self == TRIGLVL_A::V_1P90
    }
    #[doc = "Checks if the value of the field is `V_2P00`"]
    #[inline(always)]
    pub fn is_v_2p00(&self) -> bool {
        **self == TRIGLVL_A::V_2P00
    }
    #[doc = "Checks if the value of the field is `V_2P10`"]
    #[inline(always)]
    pub fn is_v_2p10(&self) -> bool {
        **self == TRIGLVL_A::V_2P10
    }
    #[doc = "Checks if the value of the field is `V_2P20`"]
    #[inline(always)]
    pub fn is_v_2p20(&self) -> bool {
        **self == TRIGLVL_A::V_2P20
    }
    #[doc = "Checks if the value of the field is `V_2P30`"]
    #[inline(always)]
    pub fn is_v_2p30(&self) -> bool {
        **self == TRIGLVL_A::V_2P30
    }
    #[doc = "Checks if the value of the field is `V_2P40`"]
    #[inline(always)]
    pub fn is_v_2p40(&self) -> bool {
        **self == TRIGLVL_A::V_2P40
    }
    #[doc = "Checks if the value of the field is `V_2P50`"]
    #[inline(always)]
    pub fn is_v_2p50(&self) -> bool {
        **self == TRIGLVL_A::V_2P50
    }
    #[doc = "Checks if the value of the field is `V_2P60`"]
    #[inline(always)]
    pub fn is_v_2p60(&self) -> bool {
        **self == TRIGLVL_A::V_2P60
    }
    #[doc = "Checks if the value of the field is `V_2P70`"]
    #[inline(always)]
    pub fn is_v_2p70(&self) -> bool {
        **self == TRIGLVL_A::V_2P70
    }
    #[doc = "Checks if the value of the field is `V_2P80`"]
    #[inline(always)]
    pub fn is_v_2p80(&self) -> bool {
        **self == TRIGLVL_A::V_2P80
    }
    #[doc = "Checks if the value of the field is `V_2P90`"]
    #[inline(always)]
    pub fn is_v_2p90(&self) -> bool {
        **self == TRIGLVL_A::V_2P90
    }
    #[doc = "Checks if the value of the field is `V_3P00`"]
    #[inline(always)]
    pub fn is_v_3p00(&self) -> bool {
        **self == TRIGLVL_A::V_3P00
    }
    #[doc = "Checks if the value of the field is `V_3P10`"]
    #[inline(always)]
    pub fn is_v_3p10(&self) -> bool {
        **self == TRIGLVL_A::V_3P10
    }
    #[doc = "Checks if the value of the field is `V_3P20`"]
    #[inline(always)]
    pub fn is_v_3p20(&self) -> bool {
        **self == TRIGLVL_A::V_3P20
    }
    #[doc = "Checks if the value of the field is `V_3P30_2`"]
    #[inline(always)]
    pub fn is_v_3p30_2(&self) -> bool {
        **self == TRIGLVL_A::V_3P30_2
    }
    #[doc = "Checks if the value of the field is `V_3P30_3`"]
    #[inline(always)]
    pub fn is_v_3p30_3(&self) -> bool {
        **self == TRIGLVL_A::V_3P30_3
    }
    #[doc = "Checks if the value of the field is `V_3P30_4`"]
    #[inline(always)]
    pub fn is_v_3p30_4(&self) -> bool {
        **self == TRIGLVL_A::V_3P30_4
    }
    #[doc = "Checks if the value of the field is `V_3P30_5`"]
    #[inline(always)]
    pub fn is_v_3p30_5(&self) -> bool {
        **self == TRIGLVL_A::V_3P30_5
    }
    #[doc = "Checks if the value of the field is `V_3P30_6`"]
    #[inline(always)]
    pub fn is_v_3p30_6(&self) -> bool {
        **self == TRIGLVL_A::V_3P30_6
    }
    #[doc = "Checks if the value of the field is `V_3P30_7`"]
    #[inline(always)]
    pub fn is_v_3p30_7(&self) -> bool {
        **self == TRIGLVL_A::V_3P30_7
    }
    #[doc = "Checks if the value of the field is `V_3P30_8`"]
    #[inline(always)]
    pub fn is_v_3p30_8(&self) -> bool {
        **self == TRIGLVL_A::V_3P30_8
    }
}
impl core::ops::Deref for TRIGLVL_R {
    type Target = crate::FieldReader<u8, TRIGLVL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGLVL` writer - BoD trigger level."]
pub struct TRIGLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGLVL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGLVL_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "BoD Hysteresis control.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HYST_A {
    #[doc = "0: 25 mV."]
    HYST_25MV = 0,
    #[doc = "1: 50 mV."]
    HYST_50MV = 1,
    #[doc = "2: 75 mV."]
    HYST_75MV = 2,
    #[doc = "3: 100 mV."]
    HYST_100MV = 3,
}
impl From<HYST_A> for u8 {
    #[inline(always)]
    fn from(variant: HYST_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HYST` reader - BoD Hysteresis control."]
pub struct HYST_R(crate::FieldReader<u8, HYST_A>);
impl HYST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HYST_R(crate::FieldReader::new(bits))
    }
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
        **self == HYST_A::HYST_25MV
    }
    #[doc = "Checks if the value of the field is `HYST_50MV`"]
    #[inline(always)]
    pub fn is_hyst_50mv(&self) -> bool {
        **self == HYST_A::HYST_50MV
    }
    #[doc = "Checks if the value of the field is `HYST_75MV`"]
    #[inline(always)]
    pub fn is_hyst_75mv(&self) -> bool {
        **self == HYST_A::HYST_75MV
    }
    #[doc = "Checks if the value of the field is `HYST_100MV`"]
    #[inline(always)]
    pub fn is_hyst_100mv(&self) -> bool {
        **self == HYST_A::HYST_100MV
    }
}
impl core::ops::Deref for HYST_R {
    type Target = crate::FieldReader<u8, HYST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HYST` writer - BoD Hysteresis control."]
pub struct HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> HYST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HYST_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VBAT Brown Out Dectector (BoD) control register \\[Reset by: PoR, Pin Reset, Software Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bodvbat](index.html) module"]
pub struct BODVBAT_SPEC;
impl crate::RegisterSpec for BODVBAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bodvbat::R](R) reader structure"]
impl crate::Readable for BODVBAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bodvbat::W](W) writer structure"]
impl crate::Writable for BODVBAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BODVBAT to value 0x47"]
impl crate::Resettable for BODVBAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x47
    }
}
