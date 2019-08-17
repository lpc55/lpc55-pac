#[doc = "Reader of register USBHS_PHY_TRIM"]
pub type R = crate::R<u32, super::USBHS_PHY_TRIM>;
#[doc = "Writer for register USBHS_PHY_TRIM"]
pub type W = crate::W<u32, super::USBHS_PHY_TRIM>;
#[doc = "Register USBHS_PHY_TRIM `reset()`'s with value 0"]
impl crate::ResetValue for super::USBHS_PHY_TRIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `trim_usb_reg_env_tail_adj_vd`"]
pub type TRIM_USB_REG_ENV_TAIL_ADJ_VD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `trim_usb_reg_env_tail_adj_vd`"]
pub struct TRIM_USB_REG_ENV_TAIL_ADJ_VD_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_USB_REG_ENV_TAIL_ADJ_VD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `trim_usbphy_tx_d_cal`"]
pub type TRIM_USBPHY_TX_D_CAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `trim_usbphy_tx_d_cal`"]
pub struct TRIM_USBPHY_TX_D_CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_USBPHY_TX_D_CAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
#[doc = "Reader of field `trim_usbphy_tx_cal45dp`"]
pub type TRIM_USBPHY_TX_CAL45DP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `trim_usbphy_tx_cal45dp`"]
pub struct TRIM_USBPHY_TX_CAL45DP_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_USBPHY_TX_CAL45DP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | (((value as u32) & 0x1f) << 6);
        self.w
    }
}
#[doc = "Reader of field `trim_usbphy_tx_cal45dm`"]
pub type TRIM_USBPHY_TX_CAL45DM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `trim_usbphy_tx_cal45dm`"]
pub struct TRIM_USBPHY_TX_CAL45DM_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_USBPHY_TX_CAL45DM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u32) & 0x1f) << 11);
        self.w
    }
}
#[doc = "Reader of field `trim_usb2_refbias_tst`"]
pub type TRIM_USB2_REFBIAS_TST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `trim_usb2_refbias_tst`"]
pub struct TRIM_USB2_REFBIAS_TST_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_USB2_REFBIAS_TST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `trim_usb2_refbias_vbgadj`"]
pub type TRIM_USB2_REFBIAS_VBGADJ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `trim_usb2_refbias_vbgadj`"]
pub struct TRIM_USB2_REFBIAS_VBGADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_USB2_REFBIAS_VBGADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Reader of field `trim_pll_ctrl0_div_sel`"]
pub type TRIM_PLL_CTRL0_DIV_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `trim_pll_ctrl0_div_sel`"]
pub struct TRIM_PLL_CTRL0_DIV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_PLL_CTRL0_DIV_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | (((value as u32) & 0x07) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Adjusts time constant of HS RX squelch (envelope) comparator."]
    #[inline(always)]
    pub fn trim_usb_reg_env_tail_adj_vd(&self) -> TRIM_USB_REG_ENV_TAIL_ADJ_VD_R {
        TRIM_USB_REG_ENV_TAIL_ADJ_VD_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:5 - ."]
    #[inline(always)]
    pub fn trim_usbphy_tx_d_cal(&self) -> TRIM_USBPHY_TX_D_CAL_R {
        TRIM_USBPHY_TX_D_CAL_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:10 - ."]
    #[inline(always)]
    pub fn trim_usbphy_tx_cal45dp(&self) -> TRIM_USBPHY_TX_CAL45DP_R {
        TRIM_USBPHY_TX_CAL45DP_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - ."]
    #[inline(always)]
    pub fn trim_usbphy_tx_cal45dm(&self) -> TRIM_USBPHY_TX_CAL45DM_R {
        TRIM_USBPHY_TX_CAL45DM_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:17 - ."]
    #[inline(always)]
    pub fn trim_usb2_refbias_tst(&self) -> TRIM_USB2_REFBIAS_TST_R {
        TRIM_USB2_REFBIAS_TST_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20 - ."]
    #[inline(always)]
    pub fn trim_usb2_refbias_vbgadj(&self) -> TRIM_USB2_REFBIAS_VBGADJ_R {
        TRIM_USB2_REFBIAS_VBGADJ_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 21:23 - ."]
    #[inline(always)]
    pub fn trim_pll_ctrl0_div_sel(&self) -> TRIM_PLL_CTRL0_DIV_SEL_R {
        TRIM_PLL_CTRL0_DIV_SEL_R::new(((self.bits >> 21) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Adjusts time constant of HS RX squelch (envelope) comparator."]
    #[inline(always)]
    pub fn trim_usb_reg_env_tail_adj_vd(&mut self) -> TRIM_USB_REG_ENV_TAIL_ADJ_VD_W {
        TRIM_USB_REG_ENV_TAIL_ADJ_VD_W { w: self }
    }
    #[doc = "Bits 2:5 - ."]
    #[inline(always)]
    pub fn trim_usbphy_tx_d_cal(&mut self) -> TRIM_USBPHY_TX_D_CAL_W {
        TRIM_USBPHY_TX_D_CAL_W { w: self }
    }
    #[doc = "Bits 6:10 - ."]
    #[inline(always)]
    pub fn trim_usbphy_tx_cal45dp(&mut self) -> TRIM_USBPHY_TX_CAL45DP_W {
        TRIM_USBPHY_TX_CAL45DP_W { w: self }
    }
    #[doc = "Bits 11:15 - ."]
    #[inline(always)]
    pub fn trim_usbphy_tx_cal45dm(&mut self) -> TRIM_USBPHY_TX_CAL45DM_W {
        TRIM_USBPHY_TX_CAL45DM_W { w: self }
    }
    #[doc = "Bits 16:17 - ."]
    #[inline(always)]
    pub fn trim_usb2_refbias_tst(&mut self) -> TRIM_USB2_REFBIAS_TST_W {
        TRIM_USB2_REFBIAS_TST_W { w: self }
    }
    #[doc = "Bits 18:20 - ."]
    #[inline(always)]
    pub fn trim_usb2_refbias_vbgadj(&mut self) -> TRIM_USB2_REFBIAS_VBGADJ_W {
        TRIM_USB2_REFBIAS_VBGADJ_W { w: self }
    }
    #[doc = "Bits 21:23 - ."]
    #[inline(always)]
    pub fn trim_pll_ctrl0_div_sel(&mut self) -> TRIM_PLL_CTRL0_DIV_SEL_W {
        TRIM_PLL_CTRL0_DIV_SEL_W { w: self }
    }
}
