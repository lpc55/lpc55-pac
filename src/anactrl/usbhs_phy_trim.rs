#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USBHS_PHY_TRIM {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct TRIM_USB_REG_ENV_TAIL_ADJ_VDR {
    bits: u8,
}
impl TRIM_USB_REG_ENV_TAIL_ADJ_VDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRIM_USBPHY_TX_D_CALR {
    bits: u8,
}
impl TRIM_USBPHY_TX_D_CALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRIM_USBPHY_TX_CAL45DPR {
    bits: u8,
}
impl TRIM_USBPHY_TX_CAL45DPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRIM_USBPHY_TX_CAL45DMR {
    bits: u8,
}
impl TRIM_USBPHY_TX_CAL45DMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRIM_USB2_REFBIAS_TSTR {
    bits: u8,
}
impl TRIM_USB2_REFBIAS_TSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRIM_USB2_REFBIAS_VBGADJR {
    bits: u8,
}
impl TRIM_USB2_REFBIAS_VBGADJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRIM_PLL_CTRL0_DIV_SELR {
    bits: u8,
}
impl TRIM_PLL_CTRL0_DIV_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _TRIM_USB_REG_ENV_TAIL_ADJ_VDW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIM_USB_REG_ENV_TAIL_ADJ_VDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRIM_USBPHY_TX_D_CALW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIM_USBPHY_TX_D_CALW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRIM_USBPHY_TX_CAL45DPW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIM_USBPHY_TX_CAL45DPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRIM_USBPHY_TX_CAL45DMW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIM_USBPHY_TX_CAL45DMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRIM_USB2_REFBIAS_TSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIM_USB2_REFBIAS_TSTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRIM_USB2_REFBIAS_VBGADJW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIM_USB2_REFBIAS_VBGADJW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRIM_PLL_CTRL0_DIV_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIM_PLL_CTRL0_DIV_SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Adjusts time constant of HS RX squelch (envelope) comparator."]
    #[inline]
    pub fn trim_usb_reg_env_tail_adj_vd(&self) -> TRIM_USB_REG_ENV_TAIL_ADJ_VDR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRIM_USB_REG_ENV_TAIL_ADJ_VDR { bits }
    }
    #[doc = "Bits 2:5 - ."]
    #[inline]
    pub fn trim_usbphy_tx_d_cal(&self) -> TRIM_USBPHY_TX_D_CALR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRIM_USBPHY_TX_D_CALR { bits }
    }
    #[doc = "Bits 6:10 - ."]
    #[inline]
    pub fn trim_usbphy_tx_cal45dp(&self) -> TRIM_USBPHY_TX_CAL45DPR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRIM_USBPHY_TX_CAL45DPR { bits }
    }
    #[doc = "Bits 11:15 - ."]
    #[inline]
    pub fn trim_usbphy_tx_cal45dm(&self) -> TRIM_USBPHY_TX_CAL45DMR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRIM_USBPHY_TX_CAL45DMR { bits }
    }
    #[doc = "Bits 16:17 - ."]
    #[inline]
    pub fn trim_usb2_refbias_tst(&self) -> TRIM_USB2_REFBIAS_TSTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRIM_USB2_REFBIAS_TSTR { bits }
    }
    #[doc = "Bits 18:20 - ."]
    #[inline]
    pub fn trim_usb2_refbias_vbgadj(&self) -> TRIM_USB2_REFBIAS_VBGADJR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRIM_USB2_REFBIAS_VBGADJR { bits }
    }
    #[doc = "Bits 21:23 - ."]
    #[inline]
    pub fn trim_pll_ctrl0_div_sel(&self) -> TRIM_PLL_CTRL0_DIV_SELR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRIM_PLL_CTRL0_DIV_SELR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Adjusts time constant of HS RX squelch (envelope) comparator."]
    #[inline]
    pub fn trim_usb_reg_env_tail_adj_vd(&mut self) -> _TRIM_USB_REG_ENV_TAIL_ADJ_VDW {
        _TRIM_USB_REG_ENV_TAIL_ADJ_VDW { w: self }
    }
    #[doc = "Bits 2:5 - ."]
    #[inline]
    pub fn trim_usbphy_tx_d_cal(&mut self) -> _TRIM_USBPHY_TX_D_CALW {
        _TRIM_USBPHY_TX_D_CALW { w: self }
    }
    #[doc = "Bits 6:10 - ."]
    #[inline]
    pub fn trim_usbphy_tx_cal45dp(&mut self) -> _TRIM_USBPHY_TX_CAL45DPW {
        _TRIM_USBPHY_TX_CAL45DPW { w: self }
    }
    #[doc = "Bits 11:15 - ."]
    #[inline]
    pub fn trim_usbphy_tx_cal45dm(&mut self) -> _TRIM_USBPHY_TX_CAL45DMW {
        _TRIM_USBPHY_TX_CAL45DMW { w: self }
    }
    #[doc = "Bits 16:17 - ."]
    #[inline]
    pub fn trim_usb2_refbias_tst(&mut self) -> _TRIM_USB2_REFBIAS_TSTW {
        _TRIM_USB2_REFBIAS_TSTW { w: self }
    }
    #[doc = "Bits 18:20 - ."]
    #[inline]
    pub fn trim_usb2_refbias_vbgadj(&mut self) -> _TRIM_USB2_REFBIAS_VBGADJW {
        _TRIM_USB2_REFBIAS_VBGADJW { w: self }
    }
    #[doc = "Bits 21:23 - ."]
    #[inline]
    pub fn trim_pll_ctrl0_div_sel(&mut self) -> _TRIM_PLL_CTRL0_DIV_SELW {
        _TRIM_PLL_CTRL0_DIV_SELW { w: self }
    }
}
