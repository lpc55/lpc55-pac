#[doc = "Reader of register PAUSE"]
pub type R = crate::R<u32, super::PAUSE>;
#[doc = "Writer for register PAUSE"]
pub type W = crate::W<u32, super::PAUSE>;
#[doc = "Register PAUSE `reset()`'s with value 0"]
impl crate::ResetValue for super::PAUSE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PAUSEDLY`"]
pub type PAUSEDLY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PAUSEDLY`"]
pub struct PAUSEDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> PAUSEDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
#[doc = "Possible values of the field `PAUSEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAUSEEN_A {
    #[doc = "Pause operation disabled"]
    PAUSEEN_0,
    #[doc = "Pause operation enabled"]
    PAUSEEN_1,
}
impl From<PAUSEEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAUSEEN_A) -> Self {
        match variant {
            PAUSEEN_A::PAUSEEN_0 => false,
            PAUSEEN_A::PAUSEEN_1 => true,
        }
    }
}
#[doc = "Reader of field `PAUSEEN`"]
pub type PAUSEEN_R = crate::R<bool, PAUSEEN_A>;
impl PAUSEEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAUSEEN_A {
        match self.bits {
            false => PAUSEEN_A::PAUSEEN_0,
            true => PAUSEEN_A::PAUSEEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `PAUSEEN_0`"]
    #[inline(always)]
    pub fn is_pauseen_0(&self) -> bool {
        *self == PAUSEEN_A::PAUSEEN_0
    }
    #[doc = "Checks if the value of the field is `PAUSEEN_1`"]
    #[inline(always)]
    pub fn is_pauseen_1(&self) -> bool {
        *self == PAUSEEN_A::PAUSEEN_1
    }
}
#[doc = "Write proxy for field `PAUSEEN`"]
pub struct PAUSEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAUSEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAUSEEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pause operation disabled"]
    #[inline(always)]
    pub fn pauseen_0(self) -> &'a mut W {
        self.variant(PAUSEEN_A::PAUSEEN_0)
    }
    #[doc = "Pause operation enabled"]
    #[inline(always)]
    pub fn pauseen_1(self) -> &'a mut W {
        self.variant(PAUSEEN_A::PAUSEEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Pause Delay"]
    #[inline(always)]
    pub fn pausedly(&self) -> PAUSEDLY_R {
        PAUSEDLY_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 31 - PAUSE Option Enable"]
    #[inline(always)]
    pub fn pauseen(&self) -> PAUSEEN_R {
        PAUSEEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Pause Delay"]
    #[inline(always)]
    pub fn pausedly(&mut self) -> PAUSEDLY_W {
        PAUSEDLY_W { w: self }
    }
    #[doc = "Bit 31 - PAUSE Option Enable"]
    #[inline(always)]
    pub fn pauseen(&mut self) -> PAUSEEN_W {
        PAUSEEN_W { w: self }
    }
}
