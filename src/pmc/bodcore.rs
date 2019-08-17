#[doc = "Reader of register BODCORE"]
pub type R = crate::R<u32, super::BODCORE>;
#[doc = "Writer for register BODCORE"]
pub type W = crate::W<u32, super::BODCORE>;
#[doc = "Register BODCORE `reset()`'s with value 0x17"]
impl crate::ResetValue for super::BODCORE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x17
    }
}
#[doc = "Possible values of the field `TRIGLVL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGLVL_A {
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
impl From<TRIGLVL_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGLVL_A) -> Self {
        match variant {
            TRIGLVL_A::V_0P60 => 0,
            TRIGLVL_A::V_0P65 => 1,
            TRIGLVL_A::V_0P70 => 2,
            TRIGLVL_A::V_0P75 => 3,
            TRIGLVL_A::V_0P80 => 4,
            TRIGLVL_A::V_0P85 => 5,
            TRIGLVL_A::V_0P90 => 6,
            TRIGLVL_A::V_0P95 => 7,
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
            0 => TRIGLVL_A::V_0P60,
            1 => TRIGLVL_A::V_0P65,
            2 => TRIGLVL_A::V_0P70,
            3 => TRIGLVL_A::V_0P75,
            4 => TRIGLVL_A::V_0P80,
            5 => TRIGLVL_A::V_0P85,
            6 => TRIGLVL_A::V_0P90,
            7 => TRIGLVL_A::V_0P95,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `V_0P60`"]
    #[inline(always)]
    pub fn is_v_0p60(&self) -> bool {
        *self == TRIGLVL_A::V_0P60
    }
    #[doc = "Checks if the value of the field is `V_0P65`"]
    #[inline(always)]
    pub fn is_v_0p65(&self) -> bool {
        *self == TRIGLVL_A::V_0P65
    }
    #[doc = "Checks if the value of the field is `V_0P70`"]
    #[inline(always)]
    pub fn is_v_0p70(&self) -> bool {
        *self == TRIGLVL_A::V_0P70
    }
    #[doc = "Checks if the value of the field is `V_0P75`"]
    #[inline(always)]
    pub fn is_v_0p75(&self) -> bool {
        *self == TRIGLVL_A::V_0P75
    }
    #[doc = "Checks if the value of the field is `V_0P80`"]
    #[inline(always)]
    pub fn is_v_0p80(&self) -> bool {
        *self == TRIGLVL_A::V_0P80
    }
    #[doc = "Checks if the value of the field is `V_0P85`"]
    #[inline(always)]
    pub fn is_v_0p85(&self) -> bool {
        *self == TRIGLVL_A::V_0P85
    }
    #[doc = "Checks if the value of the field is `V_0P90`"]
    #[inline(always)]
    pub fn is_v_0p90(&self) -> bool {
        *self == TRIGLVL_A::V_0P90
    }
    #[doc = "Checks if the value of the field is `V_0P95`"]
    #[inline(always)]
    pub fn is_v_0p95(&self) -> bool {
        *self == TRIGLVL_A::V_0P95
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
    #[doc = "0.60 V."]
    #[inline(always)]
    pub fn v_0p60(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_0P60)
    }
    #[doc = "0.65 V."]
    #[inline(always)]
    pub fn v_0p65(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_0P65)
    }
    #[doc = "0.70 V."]
    #[inline(always)]
    pub fn v_0p70(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_0P70)
    }
    #[doc = "0.75 V."]
    #[inline(always)]
    pub fn v_0p75(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_0P75)
    }
    #[doc = "0.80 V."]
    #[inline(always)]
    pub fn v_0p80(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_0P80)
    }
    #[doc = "0.85 V."]
    #[inline(always)]
    pub fn v_0p85(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_0P85)
    }
    #[doc = "0.90 V."]
    #[inline(always)]
    pub fn v_0p90(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_0P90)
    }
    #[doc = "0.95 V."]
    #[inline(always)]
    pub fn v_0p95(self) -> &'a mut W {
        self.variant(TRIGLVL_A::V_0P95)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
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
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - BoD trigger level."]
    #[inline(always)]
    pub fn triglvl(&self) -> TRIGLVL_R {
        TRIGLVL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:5 - BoD Core Hysteresis control."]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - BoD trigger level."]
    #[inline(always)]
    pub fn triglvl(&mut self) -> TRIGLVL_W {
        TRIGLVL_W { w: self }
    }
    #[doc = "Bits 4:5 - BoD Core Hysteresis control."]
    #[inline(always)]
    pub fn hyst(&mut self) -> HYST_W {
        HYST_W { w: self }
    }
}
