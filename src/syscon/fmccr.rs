#[doc = "Reader of register FMCCR"]
pub type R = crate::R<u32, super::FMCCR>;
#[doc = "Writer for register FMCCR"]
pub type W = crate::W<u32, super::FMCCR>;
#[doc = "Register FMCCR `reset()`'s with value 0x3000"]
impl crate::ResetValue for super::FMCCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3000
    }
}
#[doc = "Possible values of the field `FETCHCTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FETCHCTL_A {
    #[doc = "No buffering (bypass always used) for Fetch cycles"]
    NOBUF,
    #[doc = "One buffer is used for all Fetch cycles"]
    ONEBUF,
    #[doc = "All buffers can be used for Fetch cycles"]
    ALLBUF,
}
impl From<FETCHCTL_A> for u8 {
    #[inline(always)]
    fn from(variant: FETCHCTL_A) -> Self {
        match variant {
            FETCHCTL_A::NOBUF => 0,
            FETCHCTL_A::ONEBUF => 1,
            FETCHCTL_A::ALLBUF => 2,
        }
    }
}
#[doc = "Reader of field `FETCHCTL`"]
pub type FETCHCTL_R = crate::R<u8, FETCHCTL_A>;
impl FETCHCTL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FETCHCTL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FETCHCTL_A::NOBUF),
            1 => Val(FETCHCTL_A::ONEBUF),
            2 => Val(FETCHCTL_A::ALLBUF),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOBUF`"]
    #[inline(always)]
    pub fn is_nobuf(&self) -> bool {
        *self == FETCHCTL_A::NOBUF
    }
    #[doc = "Checks if the value of the field is `ONEBUF`"]
    #[inline(always)]
    pub fn is_onebuf(&self) -> bool {
        *self == FETCHCTL_A::ONEBUF
    }
    #[doc = "Checks if the value of the field is `ALLBUF`"]
    #[inline(always)]
    pub fn is_allbuf(&self) -> bool {
        *self == FETCHCTL_A::ALLBUF
    }
}
#[doc = "Write proxy for field `FETCHCTL`"]
pub struct FETCHCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> FETCHCTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FETCHCTL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No buffering (bypass always used) for Fetch cycles"]
    #[inline(always)]
    pub fn nobuf(self) -> &'a mut W {
        self.variant(FETCHCTL_A::NOBUF)
    }
    #[doc = "One buffer is used for all Fetch cycles"]
    #[inline(always)]
    pub fn onebuf(self) -> &'a mut W {
        self.variant(FETCHCTL_A::ONEBUF)
    }
    #[doc = "All buffers can be used for Fetch cycles"]
    #[inline(always)]
    pub fn allbuf(self) -> &'a mut W {
        self.variant(FETCHCTL_A::ALLBUF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `DATACTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATACTL_A {
    #[doc = "No buffering (bypass always used) for Data cycles"]
    NOBUF,
    #[doc = "One buffer is used for all Data cycles"]
    ONEBUF,
    #[doc = "All buffers can be used for Data cycles"]
    ALLBUF,
}
impl From<DATACTL_A> for u8 {
    #[inline(always)]
    fn from(variant: DATACTL_A) -> Self {
        match variant {
            DATACTL_A::NOBUF => 0,
            DATACTL_A::ONEBUF => 1,
            DATACTL_A::ALLBUF => 2,
        }
    }
}
#[doc = "Reader of field `DATACTL`"]
pub type DATACTL_R = crate::R<u8, DATACTL_A>;
impl DATACTL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DATACTL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DATACTL_A::NOBUF),
            1 => Val(DATACTL_A::ONEBUF),
            2 => Val(DATACTL_A::ALLBUF),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOBUF`"]
    #[inline(always)]
    pub fn is_nobuf(&self) -> bool {
        *self == DATACTL_A::NOBUF
    }
    #[doc = "Checks if the value of the field is `ONEBUF`"]
    #[inline(always)]
    pub fn is_onebuf(&self) -> bool {
        *self == DATACTL_A::ONEBUF
    }
    #[doc = "Checks if the value of the field is `ALLBUF`"]
    #[inline(always)]
    pub fn is_allbuf(&self) -> bool {
        *self == DATACTL_A::ALLBUF
    }
}
#[doc = "Write proxy for field `DATACTL`"]
pub struct DATACTL_W<'a> {
    w: &'a mut W,
}
impl<'a> DATACTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATACTL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No buffering (bypass always used) for Data cycles"]
    #[inline(always)]
    pub fn nobuf(self) -> &'a mut W {
        self.variant(DATACTL_A::NOBUF)
    }
    #[doc = "One buffer is used for all Data cycles"]
    #[inline(always)]
    pub fn onebuf(self) -> &'a mut W {
        self.variant(DATACTL_A::ONEBUF)
    }
    #[doc = "All buffers can be used for Data cycles"]
    #[inline(always)]
    pub fn allbuf(self) -> &'a mut W {
        self.variant(DATACTL_A::ALLBUF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `ACCEL`"]
pub type ACCEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACCEL`"]
pub struct ACCEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `PREFEN`"]
pub type PREFEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PREFEN`"]
pub struct PREFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PREFEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `PREFOVR`"]
pub type PREFOVR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PREFOVR`"]
pub struct PREFOVR_W<'a> {
    w: &'a mut W,
}
impl<'a> PREFOVR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `PREFCRI`"]
pub type PREFCRI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PREFCRI`"]
pub struct PREFCRI_W<'a> {
    w: &'a mut W,
}
impl<'a> PREFCRI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `FMCTIM`"]
pub type FMCTIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FMCTIM`"]
pub struct FMCTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FMCTIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 12)) | (((value as u32) & 0x1f) << 12);
        self.w
    }
}
#[doc = "Reader of field `PFISLRU`"]
pub type PFISLRU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PFISLRU`"]
pub struct PFISLRU_W<'a> {
    w: &'a mut W,
}
impl<'a> PFISLRU_W<'a> {
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
#[doc = "Reader of field `PFADAP`"]
pub type PFADAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PFADAP`"]
pub struct PFADAP_W<'a> {
    w: &'a mut W,
}
impl<'a> PFADAP_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - Fetch control"]
    #[inline(always)]
    pub fn fetchctl(&self) -> FETCHCTL_R {
        FETCHCTL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Data control"]
    #[inline(always)]
    pub fn datactl(&self) -> DATACTL_R {
        DATACTL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - ACCEL"]
    #[inline(always)]
    pub fn accel(&self) -> ACCEL_R {
        ACCEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Pref enable"]
    #[inline(always)]
    pub fn prefen(&self) -> PREFEN_R {
        PREFEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Pref ovr"]
    #[inline(always)]
    pub fn prefovr(&self) -> PREFOVR_R {
        PREFOVR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Pref CRI"]
    #[inline(always)]
    pub fn prefcri(&self) -> PREFCRI_R {
        PREFCRI_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:16 - TMC time"]
    #[inline(always)]
    pub fn fmctim(&self) -> FMCTIM_R {
        FMCTIM_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bit 17 - When set, prefetch uses LRU buffer replacement policy"]
    #[inline(always)]
    pub fn pfislru(&self) -> PFISLRU_R {
        PFISLRU_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - When set, prefetch will adaptively select between parent and LRU buffer replacement policies."]
    #[inline(always)]
    pub fn pfadap(&self) -> PFADAP_R {
        PFADAP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Fetch control"]
    #[inline(always)]
    pub fn fetchctl(&mut self) -> FETCHCTL_W {
        FETCHCTL_W { w: self }
    }
    #[doc = "Bits 2:3 - Data control"]
    #[inline(always)]
    pub fn datactl(&mut self) -> DATACTL_W {
        DATACTL_W { w: self }
    }
    #[doc = "Bit 4 - ACCEL"]
    #[inline(always)]
    pub fn accel(&mut self) -> ACCEL_W {
        ACCEL_W { w: self }
    }
    #[doc = "Bit 5 - Pref enable"]
    #[inline(always)]
    pub fn prefen(&mut self) -> PREFEN_W {
        PREFEN_W { w: self }
    }
    #[doc = "Bit 6 - Pref ovr"]
    #[inline(always)]
    pub fn prefovr(&mut self) -> PREFOVR_W {
        PREFOVR_W { w: self }
    }
    #[doc = "Bits 8:10 - Pref CRI"]
    #[inline(always)]
    pub fn prefcri(&mut self) -> PREFCRI_W {
        PREFCRI_W { w: self }
    }
    #[doc = "Bits 12:16 - TMC time"]
    #[inline(always)]
    pub fn fmctim(&mut self) -> FMCTIM_W {
        FMCTIM_W { w: self }
    }
    #[doc = "Bit 17 - When set, prefetch uses LRU buffer replacement policy"]
    #[inline(always)]
    pub fn pfislru(&mut self) -> PFISLRU_W {
        PFISLRU_W { w: self }
    }
    #[doc = "Bit 18 - When set, prefetch will adaptively select between parent and LRU buffer replacement policies."]
    #[inline(always)]
    pub fn pfadap(&mut self) -> PFADAP_W {
        PFADAP_W { w: self }
    }
}
