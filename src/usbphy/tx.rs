#[doc = "Reader of register TX"]
pub type R = crate::R<u32, super::TX>;
#[doc = "Writer for register TX"]
pub type W = crate::W<u32, super::TX>;
#[doc = "Register TX `reset()`'s with value 0x0a00_0402"]
impl crate::ResetValue for super::TX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0a00_0402
    }
}
#[doc = "Possible values of the field `D_CAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D_CAL_A {
    #[doc = "Maximum current, approximately 19% above nominal."]
    VALUE0,
    #[doc = "Nominal"]
    VALUE7,
    #[doc = "Minimum current, approximately 19% below nominal."]
    VALUE15,
}
impl From<D_CAL_A> for u8 {
    #[inline(always)]
    fn from(variant: D_CAL_A) -> Self {
        match variant {
            D_CAL_A::VALUE0 => 0,
            D_CAL_A::VALUE7 => 7,
            D_CAL_A::VALUE15 => 15,
        }
    }
}
#[doc = "Reader of field `D_CAL`"]
pub type D_CAL_R = crate::R<u8, D_CAL_A>;
impl D_CAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, D_CAL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(D_CAL_A::VALUE0),
            7 => Val(D_CAL_A::VALUE7),
            15 => Val(D_CAL_A::VALUE15),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == D_CAL_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == D_CAL_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == D_CAL_A::VALUE15
    }
}
#[doc = "Write proxy for field `D_CAL`"]
pub struct D_CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> D_CAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: D_CAL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Maximum current, approximately 19% above nominal."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(D_CAL_A::VALUE0)
    }
    #[doc = "Nominal"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(D_CAL_A::VALUE7)
    }
    #[doc = "Minimum current, approximately 19% below nominal."]
    #[inline(always)]
    pub fn value15(self) -> &'a mut W {
        self.variant(D_CAL_A::VALUE15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `TXCAL45DM`"]
pub type TXCAL45DM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXCAL45DM`"]
pub struct TXCAL45DM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCAL45DM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `TXENCAL45DN`"]
pub type TXENCAL45DN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXENCAL45DN`"]
pub struct TXENCAL45DN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXENCAL45DN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `TXCAL45DP`"]
pub type TXCAL45DP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXCAL45DP`"]
pub struct TXCAL45DP_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCAL45DP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `TXENCAL45DP`"]
pub type TXENCAL45DP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXENCAL45DP`"]
pub struct TXENCAL45DP_W<'a> {
    w: &'a mut W,
}
impl<'a> TXENCAL45DP_W<'a> {
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
impl R {
    #[doc = "Bits 0:3 - Decode to trim the nominal 17"]
    #[inline(always)]
    pub fn d_cal(&self) -> D_CAL_R {
        D_CAL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Decode to trim the nominal 45ohm series termination resistance to the USB_DM output pin"]
    #[inline(always)]
    pub fn txcal45dm(&self) -> TXCAL45DM_R {
        TXCAL45DM_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Enable resistance calibration on DN."]
    #[inline(always)]
    pub fn txencal45dn(&self) -> TXENCAL45DN_R {
        TXENCAL45DN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Decode to trim the nominal 45ohm series termination resistance to the USB_DP output pin"]
    #[inline(always)]
    pub fn txcal45dp(&self) -> TXCAL45DP_R {
        TXCAL45DP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 21 - Enable resistance calibration on DP."]
    #[inline(always)]
    pub fn txencal45dp(&self) -> TXENCAL45DP_R {
        TXENCAL45DP_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Decode to trim the nominal 17"]
    #[inline(always)]
    pub fn d_cal(&mut self) -> D_CAL_W {
        D_CAL_W { w: self }
    }
    #[doc = "Bits 8:11 - Decode to trim the nominal 45ohm series termination resistance to the USB_DM output pin"]
    #[inline(always)]
    pub fn txcal45dm(&mut self) -> TXCAL45DM_W {
        TXCAL45DM_W { w: self }
    }
    #[doc = "Bit 13 - Enable resistance calibration on DN."]
    #[inline(always)]
    pub fn txencal45dn(&mut self) -> TXENCAL45DN_W {
        TXENCAL45DN_W { w: self }
    }
    #[doc = "Bits 16:19 - Decode to trim the nominal 45ohm series termination resistance to the USB_DP output pin"]
    #[inline(always)]
    pub fn txcal45dp(&mut self) -> TXCAL45DP_W {
        TXCAL45DP_W { w: self }
    }
    #[doc = "Bit 21 - Enable resistance calibration on DP."]
    #[inline(always)]
    pub fn txencal45dp(&mut self) -> TXENCAL45DP_W {
        TXENCAL45DP_W { w: self }
    }
}
