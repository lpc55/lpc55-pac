#[doc = "Reader of register DE"]
pub type R = crate::R<u32, super::DE>;
#[doc = "Writer for register DE"]
pub type W = crate::W<u32, super::DE>;
#[doc = "Register DE `reset()`'s with value 0"]
impl crate::ResetValue for super::DE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "FIFO 0 Watermark DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWMDE0_A {
    #[doc = "0: DMA request disabled."]
    FWMDE0_0 = 0,
    #[doc = "1: DMA request enabled."]
    FWMDE0_1 = 1,
}
impl From<FWMDE0_A> for bool {
    #[inline(always)]
    fn from(variant: FWMDE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FWMDE0`"]
pub type FWMDE0_R = crate::R<bool, FWMDE0_A>;
impl FWMDE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWMDE0_A {
        match self.bits {
            false => FWMDE0_A::FWMDE0_0,
            true => FWMDE0_A::FWMDE0_1,
        }
    }
    #[doc = "Checks if the value of the field is `FWMDE0_0`"]
    #[inline(always)]
    pub fn is_fwmde0_0(&self) -> bool {
        *self == FWMDE0_A::FWMDE0_0
    }
    #[doc = "Checks if the value of the field is `FWMDE0_1`"]
    #[inline(always)]
    pub fn is_fwmde0_1(&self) -> bool {
        *self == FWMDE0_A::FWMDE0_1
    }
}
#[doc = "Write proxy for field `FWMDE0`"]
pub struct FWMDE0_W<'a> {
    w: &'a mut W,
}
impl<'a> FWMDE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FWMDE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA request disabled."]
    #[inline(always)]
    pub fn fwmde0_0(self) -> &'a mut W {
        self.variant(FWMDE0_A::FWMDE0_0)
    }
    #[doc = "DMA request enabled."]
    #[inline(always)]
    pub fn fwmde0_1(self) -> &'a mut W {
        self.variant(FWMDE0_A::FWMDE0_1)
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
#[doc = "FIFO1 Watermark DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWMDE1_A {
    #[doc = "0: DMA request disabled."]
    FWMDE1_0 = 0,
    #[doc = "1: DMA request enabled."]
    FWMDE1_1 = 1,
}
impl From<FWMDE1_A> for bool {
    #[inline(always)]
    fn from(variant: FWMDE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FWMDE1`"]
pub type FWMDE1_R = crate::R<bool, FWMDE1_A>;
impl FWMDE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWMDE1_A {
        match self.bits {
            false => FWMDE1_A::FWMDE1_0,
            true => FWMDE1_A::FWMDE1_1,
        }
    }
    #[doc = "Checks if the value of the field is `FWMDE1_0`"]
    #[inline(always)]
    pub fn is_fwmde1_0(&self) -> bool {
        *self == FWMDE1_A::FWMDE1_0
    }
    #[doc = "Checks if the value of the field is `FWMDE1_1`"]
    #[inline(always)]
    pub fn is_fwmde1_1(&self) -> bool {
        *self == FWMDE1_A::FWMDE1_1
    }
}
#[doc = "Write proxy for field `FWMDE1`"]
pub struct FWMDE1_W<'a> {
    w: &'a mut W,
}
impl<'a> FWMDE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FWMDE1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA request disabled."]
    #[inline(always)]
    pub fn fwmde1_0(self) -> &'a mut W {
        self.variant(FWMDE1_A::FWMDE1_0)
    }
    #[doc = "DMA request enabled."]
    #[inline(always)]
    pub fn fwmde1_1(self) -> &'a mut W {
        self.variant(FWMDE1_A::FWMDE1_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - FIFO 0 Watermark DMA Enable"]
    #[inline(always)]
    pub fn fwmde0(&self) -> FWMDE0_R {
        FWMDE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FIFO1 Watermark DMA Enable"]
    #[inline(always)]
    pub fn fwmde1(&self) -> FWMDE1_R {
        FWMDE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO 0 Watermark DMA Enable"]
    #[inline(always)]
    pub fn fwmde0(&mut self) -> FWMDE0_W {
        FWMDE0_W { w: self }
    }
    #[doc = "Bit 1 - FIFO1 Watermark DMA Enable"]
    #[inline(always)]
    pub fn fwmde1(&mut self) -> FWMDE1_W {
        FWMDE1_W { w: self }
    }
}
