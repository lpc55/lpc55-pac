#[doc = "Reader of register ANACTRL_SET"]
pub type R = crate::R<u32, super::ANACTRL_SET>;
#[doc = "Writer for register ANACTRL_SET"]
pub type W = crate::W<u32, super::ANACTRL_SET>;
#[doc = "Register ANACTRL_SET `reset()`'s with value 0x0a00_0402"]
impl crate::ResetValue for super::ANACTRL_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0a00_0402
    }
}
#[doc = "Reader of field `LVI_EN`"]
pub type LVI_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LVI_EN`"]
pub struct LVI_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LVI_EN_W<'a> {
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
#[doc = "Reader of field `PFD_CLK_SEL`"]
pub type PFD_CLK_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PFD_CLK_SEL`"]
pub struct PFD_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PFD_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `DEV_PULLDOWN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEV_PULLDOWN_A {
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare disabled in device mode."]
    VALUE0,
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare enabled in device mode."]
    VALUE1,
}
impl From<DEV_PULLDOWN_A> for bool {
    #[inline(always)]
    fn from(variant: DEV_PULLDOWN_A) -> Self {
        match variant {
            DEV_PULLDOWN_A::VALUE0 => false,
            DEV_PULLDOWN_A::VALUE1 => true,
        }
    }
}
#[doc = "Reader of field `DEV_PULLDOWN`"]
pub type DEV_PULLDOWN_R = crate::R<bool, DEV_PULLDOWN_A>;
impl DEV_PULLDOWN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEV_PULLDOWN_A {
        match self.bits {
            false => DEV_PULLDOWN_A::VALUE0,
            true => DEV_PULLDOWN_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == DEV_PULLDOWN_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DEV_PULLDOWN_A::VALUE1
    }
}
#[doc = "Write proxy for field `DEV_PULLDOWN`"]
pub struct DEV_PULLDOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_PULLDOWN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEV_PULLDOWN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare disabled in device mode."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(DEV_PULLDOWN_A::VALUE0)
    }
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare enabled in device mode."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DEV_PULLDOWN_A::VALUE1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Vow voltage detector enable bit."]
    #[inline(always)]
    pub fn lvi_en(&self) -> LVI_EN_R {
        LVI_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - For normal USB operation, this bit field must remain at value 2'b00."]
    #[inline(always)]
    pub fn pfd_clk_sel(&self) -> PFD_CLK_SEL_R {
        PFD_CLK_SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins"]
    #[inline(always)]
    pub fn dev_pulldown(&self) -> DEV_PULLDOWN_R {
        DEV_PULLDOWN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Vow voltage detector enable bit."]
    #[inline(always)]
    pub fn lvi_en(&mut self) -> LVI_EN_W {
        LVI_EN_W { w: self }
    }
    #[doc = "Bits 2:3 - For normal USB operation, this bit field must remain at value 2'b00."]
    #[inline(always)]
    pub fn pfd_clk_sel(&mut self) -> PFD_CLK_SEL_W {
        PFD_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 10 - Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins"]
    #[inline(always)]
    pub fn dev_pulldown(&mut self) -> DEV_PULLDOWN_W {
        DEV_PULLDOWN_W { w: self }
    }
}
