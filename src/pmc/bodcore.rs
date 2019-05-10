#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BODCORE {
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
    #[doc = "0.60 V."]
    V_0P60,
    #[doc = "0.65 V."]
    V_0P65,
    #[doc = "0.70 V."]
    V_0P70,
    #[doc = "0.75 V."]
    V_0P75,
    #[doc = "0.80 V."]
    V_0P80,
    #[doc = "0.85 V."]
    V_0P85,
    #[doc = "0.90 V."]
    V_0P90,
    #[doc = "0.95 V."]
    V_0P95,
}
impl TRIGLVLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRIGLVLR::V_0P60 => 0,
            TRIGLVLR::V_0P65 => 1,
            TRIGLVLR::V_0P70 => 2,
            TRIGLVLR::V_0P75 => 3,
            TRIGLVLR::V_0P80 => 4,
            TRIGLVLR::V_0P85 => 5,
            TRIGLVLR::V_0P90 => 6,
            TRIGLVLR::V_0P95 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRIGLVLR {
        match value {
            0 => TRIGLVLR::V_0P60,
            1 => TRIGLVLR::V_0P65,
            2 => TRIGLVLR::V_0P70,
            3 => TRIGLVLR::V_0P75,
            4 => TRIGLVLR::V_0P80,
            5 => TRIGLVLR::V_0P85,
            6 => TRIGLVLR::V_0P90,
            7 => TRIGLVLR::V_0P95,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `V_0P60`"]
    #[inline]
    pub fn is_v_0p60(&self) -> bool {
        *self == TRIGLVLR::V_0P60
    }
    #[doc = "Checks if the value of the field is `V_0P65`"]
    #[inline]
    pub fn is_v_0p65(&self) -> bool {
        *self == TRIGLVLR::V_0P65
    }
    #[doc = "Checks if the value of the field is `V_0P70`"]
    #[inline]
    pub fn is_v_0p70(&self) -> bool {
        *self == TRIGLVLR::V_0P70
    }
    #[doc = "Checks if the value of the field is `V_0P75`"]
    #[inline]
    pub fn is_v_0p75(&self) -> bool {
        *self == TRIGLVLR::V_0P75
    }
    #[doc = "Checks if the value of the field is `V_0P80`"]
    #[inline]
    pub fn is_v_0p80(&self) -> bool {
        *self == TRIGLVLR::V_0P80
    }
    #[doc = "Checks if the value of the field is `V_0P85`"]
    #[inline]
    pub fn is_v_0p85(&self) -> bool {
        *self == TRIGLVLR::V_0P85
    }
    #[doc = "Checks if the value of the field is `V_0P90`"]
    #[inline]
    pub fn is_v_0p90(&self) -> bool {
        *self == TRIGLVLR::V_0P90
    }
    #[doc = "Checks if the value of the field is `V_0P95`"]
    #[inline]
    pub fn is_v_0p95(&self) -> bool {
        *self == TRIGLVLR::V_0P95
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
    #[doc = "0.60 V."]
    V_0P60,
    #[doc = "0.65 V."]
    V_0P65,
    #[doc = "0.70 V."]
    V_0P70,
    #[doc = "0.75 V."]
    V_0P75,
    #[doc = "0.80 V."]
    V_0P80,
    #[doc = "0.85 V."]
    V_0P85,
    #[doc = "0.90 V."]
    V_0P90,
    #[doc = "0.95 V."]
    V_0P95,
}
impl TRIGLVLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRIGLVLW::V_0P60 => 0,
            TRIGLVLW::V_0P65 => 1,
            TRIGLVLW::V_0P70 => 2,
            TRIGLVLW::V_0P75 => 3,
            TRIGLVLW::V_0P80 => 4,
            TRIGLVLW::V_0P85 => 5,
            TRIGLVLW::V_0P90 => 6,
            TRIGLVLW::V_0P95 => 7,
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
    #[doc = "0.60 V."]
    #[inline]
    pub fn v_0p60(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_0P60)
    }
    #[doc = "0.65 V."]
    #[inline]
    pub fn v_0p65(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_0P65)
    }
    #[doc = "0.70 V."]
    #[inline]
    pub fn v_0p70(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_0P70)
    }
    #[doc = "0.75 V."]
    #[inline]
    pub fn v_0p75(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_0P75)
    }
    #[doc = "0.80 V."]
    #[inline]
    pub fn v_0p80(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_0P80)
    }
    #[doc = "0.85 V."]
    #[inline]
    pub fn v_0p85(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_0P85)
    }
    #[doc = "0.90 V."]
    #[inline]
    pub fn v_0p90(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_0P90)
    }
    #[doc = "0.95 V."]
    #[inline]
    pub fn v_0p95(self) -> &'a mut W {
        self.variant(TRIGLVLW::V_0P95)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
        const OFFSET: u8 = 4;
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
    #[doc = "Bits 0:2 - BoD trigger level."]
    #[inline]
    pub fn triglvl(&self) -> TRIGLVLR {
        TRIGLVLR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - BoD Core Hysteresis control."]
    #[inline]
    pub fn hyst(&self) -> HYSTR {
        HYSTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 23 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - BoD trigger level."]
    #[inline]
    pub fn triglvl(&mut self) -> _TRIGLVLW {
        _TRIGLVLW { w: self }
    }
    #[doc = "Bits 4:5 - BoD Core Hysteresis control."]
    #[inline]
    pub fn hyst(&mut self) -> _HYSTW {
        _HYSTW { w: self }
    }
}
