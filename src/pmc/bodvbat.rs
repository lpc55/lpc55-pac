#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BODVBAT {
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
#[doc = "Possible values of the field `TRIGLVL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGLVLR {
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
impl TRIGLVLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRIGLVLR::V_1P00 => 0,
            TRIGLVLR::V_1P10 => 1,
            TRIGLVLR::V_1P20 => 2,
            TRIGLVLR::V_1P30 => 3,
            TRIGLVLR::V_1P40 => 4,
            TRIGLVLR::V_1P50 => 5,
            TRIGLVLR::V_1P60 => 6,
            TRIGLVLR::V_1P65 => 7,
            TRIGLVLR::V_1P70 => 8,
            TRIGLVLR::V_1P75 => 9,
            TRIGLVLR::V_1P80 => 10,
            TRIGLVLR::V_1P90 => 11,
            TRIGLVLR::V_2P00 => 12,
            TRIGLVLR::V_2P10 => 13,
            TRIGLVLR::V_2P20 => 14,
            TRIGLVLR::V_2P30 => 15,
            TRIGLVLR::V_2P40 => 16,
            TRIGLVLR::V_2P50 => 17,
            TRIGLVLR::V_2P60 => 18,
            TRIGLVLR::V_2P70 => 19,
            TRIGLVLR::V_2P80 => 20,
            TRIGLVLR::V_2P90 => 21,
            TRIGLVLR::V_3P00 => 22,
            TRIGLVLR::V_3P10 => 23,
            TRIGLVLR::V_3P20 => 24,
            TRIGLVLR::V_3P30_2 => 25,
            TRIGLVLR::V_3P30_3 => 26,
            TRIGLVLR::V_3P30_4 => 27,
            TRIGLVLR::V_3P30_5 => 28,
            TRIGLVLR::V_3P30_6 => 29,
            TRIGLVLR::V_3P30_7 => 30,
            TRIGLVLR::V_3P30_8 => 31,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRIGLVLR {
        match value {
            0 => TRIGLVLR::V_1P00,
            1 => TRIGLVLR::V_1P10,
            2 => TRIGLVLR::V_1P20,
            3 => TRIGLVLR::V_1P30,
            4 => TRIGLVLR::V_1P40,
            5 => TRIGLVLR::V_1P50,
            6 => TRIGLVLR::V_1P60,
            7 => TRIGLVLR::V_1P65,
            8 => TRIGLVLR::V_1P70,
            9 => TRIGLVLR::V_1P75,
            10 => TRIGLVLR::V_1P80,
            11 => TRIGLVLR::V_1P90,
            12 => TRIGLVLR::V_2P00,
            13 => TRIGLVLR::V_2P10,
            14 => TRIGLVLR::V_2P20,
            15 => TRIGLVLR::V_2P30,
            16 => TRIGLVLR::V_2P40,
            17 => TRIGLVLR::V_2P50,
            18 => TRIGLVLR::V_2P60,
            19 => TRIGLVLR::V_2P70,
            20 => TRIGLVLR::V_2P80,
            21 => TRIGLVLR::V_2P90,
            22 => TRIGLVLR::V_3P00,
            23 => TRIGLVLR::V_3P10,
            24 => TRIGLVLR::V_3P20,
            25 => TRIGLVLR::V_3P30_2,
            26 => TRIGLVLR::V_3P30_3,
            27 => TRIGLVLR::V_3P30_4,
            28 => TRIGLVLR::V_3P30_5,
            29 => TRIGLVLR::V_3P30_6,
            30 => TRIGLVLR::V_3P30_7,
            31 => TRIGLVLR::V_3P30_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `V_1P00`"]
    #[inline]
    pub fn is_v_1p00(&self) -> bool {
        *self == TRIGLVLR::V_1P00
    }
    #[doc = "Checks if the value of the field is `V_1P10`"]
    #[inline]
    pub fn is_v_1p10(&self) -> bool {
        *self == TRIGLVLR::V_1P10
    }
    #[doc = "Checks if the value of the field is `V_1P20`"]
    #[inline]
    pub fn is_v_1p20(&self) -> bool {
        *self == TRIGLVLR::V_1P20
    }
    #[doc = "Checks if the value of the field is `V_1P30`"]
    #[inline]
    pub fn is_v_1p30(&self) -> bool {
        *self == TRIGLVLR::V_1P30
    }
    #[doc = "Checks if the value of the field is `V_1P40`"]
    #[inline]
    pub fn is_v_1p40(&self) -> bool {
        *self == TRIGLVLR::V_1P40
    }
    #[doc = "Checks if the value of the field is `V_1P50`"]
    #[inline]
    pub fn is_v_1p50(&self) -> bool {
        *self == TRIGLVLR::V_1P50
    }
    #[doc = "Checks if the value of the field is `V_1P60`"]
    #[inline]
    pub fn is_v_1p60(&self) -> bool {
        *self == TRIGLVLR::V_1P60
    }
    #[doc = "Checks if the value of the field is `V_1P65`"]
    #[inline]
    pub fn is_v_1p65(&self) -> bool {
        *self == TRIGLVLR::V_1P65
    }
    #[doc = "Checks if the value of the field is `V_1P70`"]
    #[inline]
    pub fn is_v_1p70(&self) -> bool {
        *self == TRIGLVLR::V_1P70
    }
    #[doc = "Checks if the value of the field is `V_1P75`"]
    #[inline]
    pub fn is_v_1p75(&self) -> bool {
        *self == TRIGLVLR::V_1P75
    }
    #[doc = "Checks if the value of the field is `V_1P80`"]
    #[inline]
    pub fn is_v_1p80(&self) -> bool {
        *self == TRIGLVLR::V_1P80
    }
    #[doc = "Checks if the value of the field is `V_1P90`"]
    #[inline]
    pub fn is_v_1p90(&self) -> bool {
        *self == TRIGLVLR::V_1P90
    }
    #[doc = "Checks if the value of the field is `V_2P00`"]
    #[inline]
    pub fn is_v_2p00(&self) -> bool {
        *self == TRIGLVLR::V_2P00
    }
    #[doc = "Checks if the value of the field is `V_2P10`"]
    #[inline]
    pub fn is_v_2p10(&self) -> bool {
        *self == TRIGLVLR::V_2P10
    }
    #[doc = "Checks if the value of the field is `V_2P20`"]
    #[inline]
    pub fn is_v_2p20(&self) -> bool {
        *self == TRIGLVLR::V_2P20
    }
    #[doc = "Checks if the value of the field is `V_2P30`"]
    #[inline]
    pub fn is_v_2p30(&self) -> bool {
        *self == TRIGLVLR::V_2P30
    }
    #[doc = "Checks if the value of the field is `V_2P40`"]
    #[inline]
    pub fn is_v_2p40(&self) -> bool {
        *self == TRIGLVLR::V_2P40
    }
    #[doc = "Checks if the value of the field is `V_2P50`"]
    #[inline]
    pub fn is_v_2p50(&self) -> bool {
        *self == TRIGLVLR::V_2P50
    }
    #[doc = "Checks if the value of the field is `V_2P60`"]
    #[inline]
    pub fn is_v_2p60(&self) -> bool {
        *self == TRIGLVLR::V_2P60
    }
    #[doc = "Checks if the value of the field is `V_2P70`"]
    #[inline]
    pub fn is_v_2p70(&self) -> bool {
        *self == TRIGLVLR::V_2P70
    }
    #[doc = "Checks if the value of the field is `V_2P80`"]
    #[inline]
    pub fn is_v_2p80(&self) -> bool {
        *self == TRIGLVLR::V_2P80
    }
    #[doc = "Checks if the value of the field is `V_2P90`"]
    #[inline]
    pub fn is_v_2p90(&self) -> bool {
        *self == TRIGLVLR::V_2P90
    }
    #[doc = "Checks if the value of the field is `V_3P00`"]
    #[inline]
    pub fn is_v_3p00(&self) -> bool {
        *self == TRIGLVLR::V_3P00
    }
    #[doc = "Checks if the value of the field is `V_3P10`"]
    #[inline]
    pub fn is_v_3p10(&self) -> bool {
        *self == TRIGLVLR::V_3P10
    }
    #[doc = "Checks if the value of the field is `V_3P20`"]
    #[inline]
    pub fn is_v_3p20(&self) -> bool {
        *self == TRIGLVLR::V_3P20
    }
    #[doc = "Checks if the value of the field is `V_3P30_2`"]
    #[inline]
    pub fn is_v_3p30_2(&self) -> bool {
        *self == TRIGLVLR::V_3P30_2
    }
    #[doc = "Checks if the value of the field is `V_3P30_3`"]
    #[inline]
    pub fn is_v_3p30_3(&self) -> bool {
        *self == TRIGLVLR::V_3P30_3
    }
    #[doc = "Checks if the value of the field is `V_3P30_4`"]
    #[inline]
    pub fn is_v_3p30_4(&self) -> bool {
        *self == TRIGLVLR::V_3P30_4
    }
    #[doc = "Checks if the value of the field is `V_3P30_5`"]
    #[inline]
    pub fn is_v_3p30_5(&self) -> bool {
        *self == TRIGLVLR::V_3P30_5
    }
    #[doc = "Checks if the value of the field is `V_3P30_6`"]
    #[inline]
    pub fn is_v_3p30_6(&self) -> bool {
        *self == TRIGLVLR::V_3P30_6
    }
    #[doc = "Checks if the value of the field is `V_3P30_7`"]
    #[inline]
    pub fn is_v_3p30_7(&self) -> bool {
        *self == TRIGLVLR::V_3P30_7
    }
    #[doc = "Checks if the value of the field is `V_3P30_8`"]
    #[inline]
    pub fn is_v_3p30_8(&self) -> bool {
        *self == TRIGLVLR::V_3P30_8
    }
}
#[doc = "Possible values of the field `HYST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYSTR {
    #[doc = "25 mV."]
    HYST_25MV,
    #[doc = "50 mV."]
    HYST_50MV,
    #[doc = "75 mV."]
    HYST_75MV,
    #[doc = "100 mV."]
    HYST_100MV,
}
impl HYSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HYSTR::HYST_25MV => 0,
            HYSTR::HYST_50MV => 1,
            HYSTR::HYST_75MV => 2,
            HYSTR::HYST_100MV => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HYSTR {
        match value {
            0 => HYSTR::HYST_25MV,
            1 => HYSTR::HYST_50MV,
            2 => HYSTR::HYST_75MV,
            3 => HYSTR::HYST_100MV,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HYST_25MV`"]
    #[inline]
    pub fn is_hyst_25mv(&self) -> bool {
        *self == HYSTR::HYST_25MV
    }
    #[doc = "Checks if the value of the field is `HYST_50MV`"]
    #[inline]
    pub fn is_hyst_50mv(&self) -> bool {
        *self == HYSTR::HYST_50MV
    }
    #[doc = "Checks if the value of the field is `HYST_75MV`"]
    #[inline]
    pub fn is_hyst_75mv(&self) -> bool {
        *self == HYSTR::HYST_75MV
    }
    #[doc = "Checks if the value of the field is `HYST_100MV`"]
    #[inline]
    pub fn is_hyst_100mv(&self) -> bool {
        *self == HYSTR::HYST_100MV
    }
}
#[doc = "Values that can be written to the field `TRIGLVL`"]
pub enum TRIGLVLW {
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
impl TRIGLVLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRIGLVLW::V_1P00 => 0,
            TRIGLVLW::V_1P10 => 1,
            TRIGLVLW::V_1P20 => 2,
            TRIGLVLW::V_1P30 => 3,
            TRIGLVLW::V_1P40 => 4,
            TRIGLVLW::V_1P50 => 5,
            TRIGLVLW::V_1P60 => 6,
            TRIGLVLW::V_1P65 => 7,
            TRIGLVLW::V_1P70 => 8,
            TRIGLVLW::V_1P75 => 9,
            TRIGLVLW::V_1P80 => 10,
            TRIGLVLW::V_1P90 => 11,
            TRIGLVLW::V_2P00 => 12,
            TRIGLVLW::V_2P10 => 13,
            TRIGLVLW::V_2P20 => 14,
            TRIGLVLW::V_2P30 => 15,
            TRIGLVLW::V_2P40 => 16,
            TRIGLVLW::V_2P50 => 17,
            TRIGLVLW::V_2P60 => 18,
            TRIGLVLW::V_2P70 => 19,
            TRIGLVLW::V_2P80 => 20,
            TRIGLVLW::V_2P90 => 21,
            TRIGLVLW::V_3P00 => 22,
            TRIGLVLW::V_3P10 => 23,
            TRIGLVLW::V_3P20 => 24,
            TRIGLVLW::V_3P30_2 => 25,
            TRIGLVLW::V_3P30_3 => 26,
            TRIGLVLW::V_3P30_4 => 27,
            TRIGLVLW::V_3P30_5 => 28,
            TRIGLVLW::V_3P30_6 => 29,
            TRIGLVLW::V_3P30_7 => 30,
            TRIGLVLW::V_3P30_8 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGLVLW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGLVLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGLVLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1.00 V."]
    #[inline]
    pub fn v_1p00(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_1P00)
    }
    #[doc = "1.10 V."]
    #[inline]
    pub fn v_1p10(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_1P10)
    }
    #[doc = "1.20 V."]
    #[inline]
    pub fn v_1p20(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_1P20)
    }
    #[doc = "1.30 V."]
    #[inline]
    pub fn v_1p30(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_1P30)
    }
    #[doc = "1.40 V."]
    #[inline]
    pub fn v_1p40(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_1P40)
    }
    #[doc = "1.50 V."]
    #[inline]
    pub fn v_1p50(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_1P50)
    }
    #[doc = "1.60 V."]
    #[inline]
    pub fn v_1p60(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_1P60)
    }
    #[doc = "1.65 V."]
    #[inline]
    pub fn v_1p65(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_1P65)
    }
    #[doc = "1.70 V."]
    #[inline]
    pub fn v_1p70(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_1P70)
    }
    #[doc = "1.75 V."]
    #[inline]
    pub fn v_1p75(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_1P75)
    }
    #[doc = "1.80 V."]
    #[inline]
    pub fn v_1p80(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_1P80)
    }
    #[doc = "1.90 V."]
    #[inline]
    pub fn v_1p90(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_1P90)
    }
    #[doc = "2.00 V."]
    #[inline]
    pub fn v_2p00(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_2P00)
    }
    #[doc = "2.10 V."]
    #[inline]
    pub fn v_2p10(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_2P10)
    }
    #[doc = "2.20 V."]
    #[inline]
    pub fn v_2p20(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_2P20)
    }
    #[doc = "2.30 V."]
    #[inline]
    pub fn v_2p30(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_2P30)
    }
    #[doc = "2.40 V."]
    #[inline]
    pub fn v_2p40(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_2P40)
    }
    #[doc = "2.50 V."]
    #[inline]
    pub fn v_2p50(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_2P50)
    }
    #[doc = "2.60 V."]
    #[inline]
    pub fn v_2p60(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_2P60)
    }
    #[doc = "2.70 V."]
    #[inline]
    pub fn v_2p70(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_2P70)
    }
    #[doc = "2.806 V."]
    #[inline]
    pub fn v_2p80(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_2P80)
    }
    #[doc = "2.90 V."]
    #[inline]
    pub fn v_2p90(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_2P90)
    }
    #[doc = "3.00 V."]
    #[inline]
    pub fn v_3p00(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_3P00)
    }
    #[doc = "3.10 V."]
    #[inline]
    pub fn v_3p10(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_3P10)
    }
    #[doc = "3.20 V."]
    #[inline]
    pub fn v_3p20(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_3P20)
    }
    #[doc = "3.30 V."]
    #[inline]
    pub fn v_3p30_2(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_3P30_2)
    }
    #[doc = "3.30 V."]
    #[inline]
    pub fn v_3p30_3(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_3P30_3)
    }
    #[doc = "3.30 V."]
    #[inline]
    pub fn v_3p30_4(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_3P30_4)
    }
    #[doc = "3.30 V."]
    #[inline]
    pub fn v_3p30_5(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_3P30_5)
    }
    #[doc = "3.30 V."]
    #[inline]
    pub fn v_3p30_6(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_3P30_6)
    }
    #[doc = "3.30 V."]
    #[inline]
    pub fn v_3p30_7(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_3P30_7)
    }
    #[doc = "3.30 V."]
    #[inline]
    pub fn v_3p30_8(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_3P30_8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HYST`"]
pub enum HYSTW {
    #[doc = "25 mV."]
    HYST_25MV,
    #[doc = "50 mV."]
    HYST_50MV,
    #[doc = "75 mV."]
    HYST_75MV,
    #[doc = "100 mV."]
    HYST_100MV,
}
impl HYSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HYSTW::HYST_25MV => 0,
            HYSTW::HYST_50MV => 1,
            HYSTW::HYST_75MV => 2,
            HYSTW::HYST_100MV => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HYSTW<'a> {
    w: &'a mut W,
}
impl<'a> _HYSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HYSTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "25 mV."]
    #[inline]
    pub fn hyst_25mv(self) -> &'a mut W {
        self.variant(HYSTW::HYST_25MV)
    }
    #[doc = "50 mV."]
    #[inline]
    pub fn hyst_50mv(self) -> &'a mut W {
        self.variant(HYSTW::HYST_50MV)
    }
    #[doc = "75 mV."]
    #[inline]
    pub fn hyst_75mv(self) -> &'a mut W {
        self.variant(HYSTW::HYST_75MV)
    }
    #[doc = "100 mV."]
    #[inline]
    pub fn hyst_100mv(self) -> &'a mut W {
        self.variant(HYSTW::HYST_100MV)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
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
    #[doc = "Bits 0:4 - BoD trigger level."]
    #[inline]
    pub fn triglvl(&self) -> TRIGLVLR {
        TRIGLVLR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 5:6 - BoD Hysteresis control."]
    #[inline]
    pub fn hyst(&self) -> HYSTR {
        HYSTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 105 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - BoD trigger level."]
    #[inline]
    pub fn triglvl(&mut self) -> _TRIGLVLW {
        _TRIGLVLW { w: self }
    }
    #[doc = "Bits 5:6 - BoD Hysteresis control."]
    #[inline]
    pub fn hyst(&mut self) -> _HYSTW {
        _HYSTW { w: self }
    }
}
