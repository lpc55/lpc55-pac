#[doc = "Reader of register WAKEINT"]
pub type R = crate::R<u32, super::WAKEINT>;
#[doc = "Writer for register WAKEINT"]
pub type W = crate::W<u32, super::WAKEINT>;
#[doc = "Register WAKEINT `reset()`'s with value 0"]
impl crate::ResetValue for super::WAKEINT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MASK`"]
pub type MASK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MASK`"]
pub struct MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Possible values of the field `FILTER_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTER_MODE_A {
    #[doc = "Bypass mode."]
    BYPASS,
    #[doc = "Filter 1 clock period."]
    FILTER1CLK,
    #[doc = "Filter 2 clock period."]
    FILTER2CLK,
    #[doc = "Filter 3 clock period."]
    FILTER3CLK,
}
impl From<FILTER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: FILTER_MODE_A) -> Self {
        match variant {
            FILTER_MODE_A::BYPASS => 0,
            FILTER_MODE_A::FILTER1CLK => 1,
            FILTER_MODE_A::FILTER2CLK => 2,
            FILTER_MODE_A::FILTER3CLK => 3,
        }
    }
}
#[doc = "Reader of field `FILTER_MODE`"]
pub type FILTER_MODE_R = crate::R<u8, FILTER_MODE_A>;
impl FILTER_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTER_MODE_A {
        match self.bits {
            0 => FILTER_MODE_A::BYPASS,
            1 => FILTER_MODE_A::FILTER1CLK,
            2 => FILTER_MODE_A::FILTER2CLK,
            3 => FILTER_MODE_A::FILTER3CLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == FILTER_MODE_A::BYPASS
    }
    #[doc = "Checks if the value of the field is `FILTER1CLK`"]
    #[inline(always)]
    pub fn is_filter1clk(&self) -> bool {
        *self == FILTER_MODE_A::FILTER1CLK
    }
    #[doc = "Checks if the value of the field is `FILTER2CLK`"]
    #[inline(always)]
    pub fn is_filter2clk(&self) -> bool {
        *self == FILTER_MODE_A::FILTER2CLK
    }
    #[doc = "Checks if the value of the field is `FILTER3CLK`"]
    #[inline(always)]
    pub fn is_filter3clk(&self) -> bool {
        *self == FILTER_MODE_A::FILTER3CLK
    }
}
#[doc = "Write proxy for field `FILTER_MODE`"]
pub struct FILTER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTER_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FILTER_MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Bypass mode."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(FILTER_MODE_A::BYPASS)
    }
    #[doc = "Filter 1 clock period."]
    #[inline(always)]
    pub fn filter1clk(self) -> &'a mut W {
        self.variant(FILTER_MODE_A::FILTER1CLK)
    }
    #[doc = "Filter 2 clock period."]
    #[inline(always)]
    pub fn filter2clk(self) -> &'a mut W {
        self.variant(FILTER_MODE_A::FILTER2CLK)
    }
    #[doc = "Filter 3 clock period."]
    #[inline(always)]
    pub fn filter3clk(self) -> &'a mut W {
        self.variant(FILTER_MODE_A::FILTER3CLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `FILTER_CLKSEL`"]
pub type FILTER_CLKSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FILTER_CLKSEL`"]
pub struct FILTER_CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTER_CLKSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `LATCH_ENABLE`"]
pub type LATCH_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LATCH_ENABLE`"]
pub struct LATCH_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> LATCH_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `INTR_CLEAR`"]
pub type INTR_CLEAR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTR_CLEAR`"]
pub struct INTR_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> INTR_CLEAR_W<'a> {
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
impl R {
    #[doc = "Bits 0:7 - Interrupt mask (which of the 8 PLU Outputs contribute to interrupt)"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - control input of the PLU, add filtering for glitch"]
    #[inline(always)]
    pub fn filter_mode(&self) -> FILTER_MODE_R {
        FILTER_MODE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - hclk is divided by 2**filter_clksel"]
    #[inline(always)]
    pub fn filter_clksel(&self) -> FILTER_CLKSEL_R {
        FILTER_CLKSEL_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - latch the interrupt , then can be cleared with next bit INTR_CLEAR"]
    #[inline(always)]
    pub fn latch_enable(&self) -> LATCH_ENABLE_R {
        LATCH_ENABLE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Write to clear wakeint_latched"]
    #[inline(always)]
    pub fn intr_clear(&self) -> INTR_CLEAR_R {
        INTR_CLEAR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt mask (which of the 8 PLU Outputs contribute to interrupt)"]
    #[inline(always)]
    pub fn mask(&mut self) -> MASK_W {
        MASK_W { w: self }
    }
    #[doc = "Bits 8:9 - control input of the PLU, add filtering for glitch"]
    #[inline(always)]
    pub fn filter_mode(&mut self) -> FILTER_MODE_W {
        FILTER_MODE_W { w: self }
    }
    #[doc = "Bits 10:11 - hclk is divided by 2**filter_clksel"]
    #[inline(always)]
    pub fn filter_clksel(&mut self) -> FILTER_CLKSEL_W {
        FILTER_CLKSEL_W { w: self }
    }
    #[doc = "Bit 12 - latch the interrupt , then can be cleared with next bit INTR_CLEAR"]
    #[inline(always)]
    pub fn latch_enable(&mut self) -> LATCH_ENABLE_W {
        LATCH_ENABLE_W { w: self }
    }
    #[doc = "Bit 13 - Write to clear wakeint_latched"]
    #[inline(always)]
    pub fn intr_clear(&mut self) -> INTR_CLEAR_W {
        INTR_CLEAR_W { w: self }
    }
}
