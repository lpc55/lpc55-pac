#[doc = "Reader of register CTRL0"]
pub type R = crate::R<u32, super::CTRL0>;
#[doc = "Writer for register CTRL0"]
pub type W = crate::W<u32, super::CTRL0>;
#[doc = "Register CTRL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `ABBPAIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABBPAIR_A {
    #[doc = "Bank-pair 0 (1st)"]
    PAIR0,
    #[doc = "Bank-pair 1 (2nd)"]
    PAIR1,
}
impl From<ABBPAIR_A> for bool {
    #[inline(always)]
    fn from(variant: ABBPAIR_A) -> Self {
        match variant {
            ABBPAIR_A::PAIR0 => false,
            ABBPAIR_A::PAIR1 => true,
        }
    }
}
#[doc = "Reader of field `ABBPAIR`"]
pub type ABBPAIR_R = crate::R<bool, ABBPAIR_A>;
impl ABBPAIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABBPAIR_A {
        match self.bits {
            false => ABBPAIR_A::PAIR0,
            true => ABBPAIR_A::PAIR1,
        }
    }
    #[doc = "Checks if the value of the field is `PAIR0`"]
    #[inline(always)]
    pub fn is_pair0(&self) -> bool {
        *self == ABBPAIR_A::PAIR0
    }
    #[doc = "Checks if the value of the field is `PAIR1`"]
    #[inline(always)]
    pub fn is_pair1(&self) -> bool {
        *self == ABBPAIR_A::PAIR1
    }
}
#[doc = "Write proxy for field `ABBPAIR`"]
pub struct ABBPAIR_W<'a> {
    w: &'a mut W,
}
impl<'a> ABBPAIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABBPAIR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bank-pair 0 (1st)"]
    #[inline(always)]
    pub fn pair0(self) -> &'a mut W {
        self.variant(ABBPAIR_A::PAIR0)
    }
    #[doc = "Bank-pair 1 (2nd)"]
    #[inline(always)]
    pub fn pair1(self) -> &'a mut W {
        self.variant(ABBPAIR_A::PAIR1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `ABOFF`"]
pub type ABOFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ABOFF`"]
pub struct ABOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> ABOFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `CDBPAIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDBPAIR_A {
    #[doc = "Bank-pair 0 (1st)"]
    PAIR0,
    #[doc = "Bank-pair 1 (2nd)"]
    PAIR1,
}
impl From<CDBPAIR_A> for bool {
    #[inline(always)]
    fn from(variant: CDBPAIR_A) -> Self {
        match variant {
            CDBPAIR_A::PAIR0 => false,
            CDBPAIR_A::PAIR1 => true,
        }
    }
}
#[doc = "Reader of field `CDBPAIR`"]
pub type CDBPAIR_R = crate::R<bool, CDBPAIR_A>;
impl CDBPAIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDBPAIR_A {
        match self.bits {
            false => CDBPAIR_A::PAIR0,
            true => CDBPAIR_A::PAIR1,
        }
    }
    #[doc = "Checks if the value of the field is `PAIR0`"]
    #[inline(always)]
    pub fn is_pair0(&self) -> bool {
        *self == CDBPAIR_A::PAIR0
    }
    #[doc = "Checks if the value of the field is `PAIR1`"]
    #[inline(always)]
    pub fn is_pair1(&self) -> bool {
        *self == CDBPAIR_A::PAIR1
    }
}
#[doc = "Write proxy for field `CDBPAIR`"]
pub struct CDBPAIR_W<'a> {
    w: &'a mut W,
}
impl<'a> CDBPAIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDBPAIR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bank-pair 0 (1st)"]
    #[inline(always)]
    pub fn pair0(self) -> &'a mut W {
        self.variant(CDBPAIR_A::PAIR0)
    }
    #[doc = "Bank-pair 1 (2nd)"]
    #[inline(always)]
    pub fn pair1(self) -> &'a mut W {
        self.variant(CDBPAIR_A::PAIR1)
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
#[doc = "Reader of field `CDOFF`"]
pub type CDOFF_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CDOFF`"]
pub struct CDOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> CDOFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 18)) | (((value as u32) & 0x07ff) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Which bank-pair the offset ABOFF is within. This must be 0 if only 2-up"]
    #[inline(always)]
    pub fn abbpair(&self) -> ABBPAIR_R {
        ABBPAIR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Word or DWord Offset of AB values, with B at \\[2\\]=0 and A at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the CD values if 4-up"]
    #[inline(always)]
    pub fn aboff(&self) -> ABOFF_R {
        ABOFF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Which bank-pair the offset CDOFF is within. This must be 0 if only 2-up"]
    #[inline(always)]
    pub fn cdbpair(&self) -> CDBPAIR_R {
        CDBPAIR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 18:28 - Word or DWord Offset of CD, with D at \\[2\\]=0 and C at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the AB values"]
    #[inline(always)]
    pub fn cdoff(&self) -> CDOFF_R {
        CDOFF_R::new(((self.bits >> 18) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Which bank-pair the offset ABOFF is within. This must be 0 if only 2-up"]
    #[inline(always)]
    pub fn abbpair(&mut self) -> ABBPAIR_W {
        ABBPAIR_W { w: self }
    }
    #[doc = "Bit 2 - Word or DWord Offset of AB values, with B at \\[2\\]=0 and A at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the CD values if 4-up"]
    #[inline(always)]
    pub fn aboff(&mut self) -> ABOFF_W {
        ABOFF_W { w: self }
    }
    #[doc = "Bit 16 - Which bank-pair the offset CDOFF is within. This must be 0 if only 2-up"]
    #[inline(always)]
    pub fn cdbpair(&mut self) -> CDBPAIR_W {
        CDBPAIR_W { w: self }
    }
    #[doc = "Bits 18:28 - Word or DWord Offset of CD, with D at \\[2\\]=0 and C at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the AB values"]
    #[inline(always)]
    pub fn cdoff(&mut self) -> CDOFF_W {
        CDOFF_W { w: self }
    }
}
