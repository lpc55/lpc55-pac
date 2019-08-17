#[doc = "Reader of register CTRL1"]
pub type R = crate::R<u32, super::CTRL1>;
#[doc = "Writer for register CTRL1"]
pub type W = crate::W<u32, super::CTRL1>;
#[doc = "Register CTRL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ITER`"]
pub type ITER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ITER`"]
pub struct ITER_W<'a> {
    w: &'a mut W,
}
impl<'a> ITER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `RESBPAIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESBPAIR_A {
    #[doc = "Bank-pair 0 (1st)"]
    PAIR0,
    #[doc = "Bank-pair 1 (2nd)"]
    PAIR1,
}
impl From<RESBPAIR_A> for bool {
    #[inline(always)]
    fn from(variant: RESBPAIR_A) -> Self {
        match variant {
            RESBPAIR_A::PAIR0 => false,
            RESBPAIR_A::PAIR1 => true,
        }
    }
}
#[doc = "Reader of field `RESBPAIR`"]
pub type RESBPAIR_R = crate::R<bool, RESBPAIR_A>;
impl RESBPAIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESBPAIR_A {
        match self.bits {
            false => RESBPAIR_A::PAIR0,
            true => RESBPAIR_A::PAIR1,
        }
    }
    #[doc = "Checks if the value of the field is `PAIR0`"]
    #[inline(always)]
    pub fn is_pair0(&self) -> bool {
        *self == RESBPAIR_A::PAIR0
    }
    #[doc = "Checks if the value of the field is `PAIR1`"]
    #[inline(always)]
    pub fn is_pair1(&self) -> bool {
        *self == RESBPAIR_A::PAIR1
    }
}
#[doc = "Write proxy for field `RESBPAIR`"]
pub struct RESBPAIR_W<'a> {
    w: &'a mut W,
}
impl<'a> RESBPAIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESBPAIR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bank-pair 0 (1st)"]
    #[inline(always)]
    pub fn pair0(self) -> &'a mut W {
        self.variant(RESBPAIR_A::PAIR0)
    }
    #[doc = "Bank-pair 1 (2nd)"]
    #[inline(always)]
    pub fn pair1(self) -> &'a mut W {
        self.variant(RESBPAIR_A::PAIR1)
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
#[doc = "Reader of field `RESOFF`"]
pub type RESOFF_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESOFF`"]
pub struct RESOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RESOFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 18)) | (((value as u32) & 0x07ff) << 18);
        self.w
    }
}
#[doc = "Possible values of the field `CSKIP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSKIP_A {
    #[doc = "No Skip"]
    NO_SKIP,
    #[doc = "Skip if Carry is 1"]
    SKIP_IF_1,
    #[doc = "Skip if Carry is 0"]
    SKIP_IF_0,
    #[doc = "Set CTRLOFF to CDOFF and Skip"]
    SET_AND_SKIP,
}
impl From<CSKIP_A> for u8 {
    #[inline(always)]
    fn from(variant: CSKIP_A) -> Self {
        match variant {
            CSKIP_A::NO_SKIP => 0,
            CSKIP_A::SKIP_IF_1 => 1,
            CSKIP_A::SKIP_IF_0 => 2,
            CSKIP_A::SET_AND_SKIP => 3,
        }
    }
}
#[doc = "Reader of field `CSKIP`"]
pub type CSKIP_R = crate::R<u8, CSKIP_A>;
impl CSKIP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSKIP_A {
        match self.bits {
            0 => CSKIP_A::NO_SKIP,
            1 => CSKIP_A::SKIP_IF_1,
            2 => CSKIP_A::SKIP_IF_0,
            3 => CSKIP_A::SET_AND_SKIP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_SKIP`"]
    #[inline(always)]
    pub fn is_no_skip(&self) -> bool {
        *self == CSKIP_A::NO_SKIP
    }
    #[doc = "Checks if the value of the field is `SKIP_IF_1`"]
    #[inline(always)]
    pub fn is_skip_if_1(&self) -> bool {
        *self == CSKIP_A::SKIP_IF_1
    }
    #[doc = "Checks if the value of the field is `SKIP_IF_0`"]
    #[inline(always)]
    pub fn is_skip_if_0(&self) -> bool {
        *self == CSKIP_A::SKIP_IF_0
    }
    #[doc = "Checks if the value of the field is `SET_AND_SKIP`"]
    #[inline(always)]
    pub fn is_set_and_skip(&self) -> bool {
        *self == CSKIP_A::SET_AND_SKIP
    }
}
#[doc = "Write proxy for field `CSKIP`"]
pub struct CSKIP_W<'a> {
    w: &'a mut W,
}
impl<'a> CSKIP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSKIP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No Skip"]
    #[inline(always)]
    pub fn no_skip(self) -> &'a mut W {
        self.variant(CSKIP_A::NO_SKIP)
    }
    #[doc = "Skip if Carry is 1"]
    #[inline(always)]
    pub fn skip_if_1(self) -> &'a mut W {
        self.variant(CSKIP_A::SKIP_IF_1)
    }
    #[doc = "Skip if Carry is 0"]
    #[inline(always)]
    pub fn skip_if_0(self) -> &'a mut W {
        self.variant(CSKIP_A::SKIP_IF_0)
    }
    #[doc = "Set CTRLOFF to CDOFF and Skip"]
    #[inline(always)]
    pub fn set_and_skip(self) -> &'a mut W {
        self.variant(CSKIP_A::SET_AND_SKIP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Iteration counter. Is number_cycles - 1. write 0 means Does one cycle - does not iterate."]
    #[inline(always)]
    pub fn iter(&self) -> ITER_R {
        ITER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Operation mode to perform. write 0 means Accelerator is inactive. write others means accelerator is active."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Which bank-pair the offset RESOFF is within. This must be 0 if only 2-up. Ideally this is not the same bank as ABBPAIR (when 4-up supported)"]
    #[inline(always)]
    pub fn resbpair(&self) -> RESBPAIR_R {
        RESBPAIR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 18:28 - Word or DWord Offset of result. Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the AB and CD values"]
    #[inline(always)]
    pub fn resoff(&self) -> RESOFF_R {
        RESOFF_R::new(((self.bits >> 18) & 0x07ff) as u16)
    }
    #[doc = "Bits 30:31 - Skip rules on Carry if needed. This operation will be skipped based on Carry value (from previous operation) if not 0:"]
    #[inline(always)]
    pub fn cskip(&self) -> CSKIP_R {
        CSKIP_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Iteration counter. Is number_cycles - 1. write 0 means Does one cycle - does not iterate."]
    #[inline(always)]
    pub fn iter(&mut self) -> ITER_W {
        ITER_W { w: self }
    }
    #[doc = "Bits 8:15 - Operation mode to perform. write 0 means Accelerator is inactive. write others means accelerator is active."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 16 - Which bank-pair the offset RESOFF is within. This must be 0 if only 2-up. Ideally this is not the same bank as ABBPAIR (when 4-up supported)"]
    #[inline(always)]
    pub fn resbpair(&mut self) -> RESBPAIR_W {
        RESBPAIR_W { w: self }
    }
    #[doc = "Bits 18:28 - Word or DWord Offset of result. Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the AB and CD values"]
    #[inline(always)]
    pub fn resoff(&mut self) -> RESOFF_W {
        RESOFF_W { w: self }
    }
    #[doc = "Bits 30:31 - Skip rules on Carry if needed. This operation will be skipped based on Carry value (from previous operation) if not 0:"]
    #[inline(always)]
    pub fn cskip(&mut self) -> CSKIP_W {
        CSKIP_W { w: self }
    }
}
