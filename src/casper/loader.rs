#[doc = "Reader of register LOADER"]
pub type R = crate::R<u32, super::LOADER>;
#[doc = "Writer for register LOADER"]
pub type W = crate::W<u32, super::LOADER>;
#[doc = "Register LOADER `reset()`'s with value 0"]
impl crate::ResetValue for super::LOADER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COUNT`"]
pub type COUNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COUNT`"]
pub struct COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Possible values of the field `CTRLBPAIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTRLBPAIR_A {
    #[doc = "Bank-pair 0 (1st)"]
    PAIR0,
    #[doc = "Bank-pair 1 (2nd)"]
    PAIR1,
}
impl From<CTRLBPAIR_A> for bool {
    #[inline(always)]
    fn from(variant: CTRLBPAIR_A) -> Self {
        match variant {
            CTRLBPAIR_A::PAIR0 => false,
            CTRLBPAIR_A::PAIR1 => true,
        }
    }
}
#[doc = "Reader of field `CTRLBPAIR`"]
pub type CTRLBPAIR_R = crate::R<bool, CTRLBPAIR_A>;
impl CTRLBPAIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTRLBPAIR_A {
        match self.bits {
            false => CTRLBPAIR_A::PAIR0,
            true => CTRLBPAIR_A::PAIR1,
        }
    }
    #[doc = "Checks if the value of the field is `PAIR0`"]
    #[inline(always)]
    pub fn is_pair0(&self) -> bool {
        *self == CTRLBPAIR_A::PAIR0
    }
    #[doc = "Checks if the value of the field is `PAIR1`"]
    #[inline(always)]
    pub fn is_pair1(&self) -> bool {
        *self == CTRLBPAIR_A::PAIR1
    }
}
#[doc = "Write proxy for field `CTRLBPAIR`"]
pub struct CTRLBPAIR_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRLBPAIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRLBPAIR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bank-pair 0 (1st)"]
    #[inline(always)]
    pub fn pair0(self) -> &'a mut W {
        self.variant(CTRLBPAIR_A::PAIR0)
    }
    #[doc = "Bank-pair 1 (2nd)"]
    #[inline(always)]
    pub fn pair1(self) -> &'a mut W {
        self.variant(CTRLBPAIR_A::PAIR1)
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
#[doc = "Reader of field `CTRLOFF`"]
pub type CTRLOFF_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CTRLOFF`"]
pub struct CTRLOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRLOFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 18)) | (((value as u32) & 0x07ff) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Number of control pairs to load 0 relative (so 1 means load 1). write 1 means Does one op - does not iterate, write N means N control pairs to load"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - Which bank-pair the offset CTRLOFF is within. This must be 0 if only 2-up. Does not matter which bank is used as this is loaded when not performing an operation."]
    #[inline(always)]
    pub fn ctrlbpair(&self) -> CTRLBPAIR_R {
        CTRLBPAIR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 18:28 - DWord Offset of CTRL pair to load next."]
    #[inline(always)]
    pub fn ctrloff(&self) -> CTRLOFF_R {
        CTRLOFF_R::new(((self.bits >> 18) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Number of control pairs to load 0 relative (so 1 means load 1). write 1 means Does one op - does not iterate, write N means N control pairs to load"]
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W {
        COUNT_W { w: self }
    }
    #[doc = "Bit 16 - Which bank-pair the offset CTRLOFF is within. This must be 0 if only 2-up. Does not matter which bank is used as this is loaded when not performing an operation."]
    #[inline(always)]
    pub fn ctrlbpair(&mut self) -> CTRLBPAIR_W {
        CTRLBPAIR_W { w: self }
    }
    #[doc = "Bits 18:28 - DWord Offset of CTRL pair to load next."]
    #[inline(always)]
    pub fn ctrloff(&mut self) -> CTRLOFF_W {
        CTRLOFF_W { w: self }
    }
}
