#[doc = "Reader of register CTRL_TOG"]
pub type R = crate::R<u32, super::CTRL_TOG>;
#[doc = "Writer for register CTRL_TOG"]
pub type W = crate::W<u32, super::CTRL_TOG>;
#[doc = "Register CTRL_TOG `reset()`'s with value 0xc000_0000"]
impl crate::ResetValue for super::CTRL_TOG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc000_0000
    }
}
#[doc = "Reader of field `ENHOSTDISCONDETECT`"]
pub type ENHOSTDISCONDETECT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENHOSTDISCONDETECT`"]
pub struct ENHOSTDISCONDETECT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENHOSTDISCONDETECT_W<'a> {
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
#[doc = "Reader of field `HOSTDISCONDETECT_IRQ`"]
pub type HOSTDISCONDETECT_IRQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOSTDISCONDETECT_IRQ`"]
pub struct HOSTDISCONDETECT_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> HOSTDISCONDETECT_IRQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Possible values of the field `ENDEVPLUGINDET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEVPLUGINDET_A {
    #[doc = "Disables 200kohm pullup resistors on USB_DP and USB_DM pins (Default)"]
    VALUE0,
    #[doc = "Enables 200kohm pullup resistors on USB_DP and USB_DM pins"]
    VALUE1,
}
impl From<ENDEVPLUGINDET_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEVPLUGINDET_A) -> Self {
        match variant {
            ENDEVPLUGINDET_A::VALUE0 => false,
            ENDEVPLUGINDET_A::VALUE1 => true,
        }
    }
}
#[doc = "Reader of field `ENDEVPLUGINDET`"]
pub type ENDEVPLUGINDET_R = crate::R<bool, ENDEVPLUGINDET_A>;
impl ENDEVPLUGINDET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEVPLUGINDET_A {
        match self.bits {
            false => ENDEVPLUGINDET_A::VALUE0,
            true => ENDEVPLUGINDET_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == ENDEVPLUGINDET_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ENDEVPLUGINDET_A::VALUE1
    }
}
#[doc = "Write proxy for field `ENDEVPLUGINDET`"]
pub struct ENDEVPLUGINDET_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEVPLUGINDET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEVPLUGINDET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables 200kohm pullup resistors on USB_DP and USB_DM pins (Default)"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(ENDEVPLUGINDET_A::VALUE0)
    }
    #[doc = "Enables 200kohm pullup resistors on USB_DP and USB_DM pins"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENDEVPLUGINDET_A::VALUE1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `DEVPLUGIN_IRQ`"]
pub type DEVPLUGIN_IRQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEVPLUGIN_IRQ`"]
pub struct DEVPLUGIN_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVPLUGIN_IRQ_W<'a> {
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
#[doc = "Reader of field `ENUTMILEVEL2`"]
pub type ENUTMILEVEL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENUTMILEVEL2`"]
pub struct ENUTMILEVEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> ENUTMILEVEL2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `ENUTMILEVEL3`"]
pub type ENUTMILEVEL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENUTMILEVEL3`"]
pub struct ENUTMILEVEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> ENUTMILEVEL3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `AUTORESUME_EN`"]
pub type AUTORESUME_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTORESUME_EN`"]
pub struct AUTORESUME_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTORESUME_EN_W<'a> {
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
#[doc = "Reader of field `ENAUTOCLR_CLKGATE`"]
pub type ENAUTOCLR_CLKGATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENAUTOCLR_CLKGATE`"]
pub struct ENAUTOCLR_CLKGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENAUTOCLR_CLKGATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `ENAUTOCLR_PHY_PWD`"]
pub type ENAUTOCLR_PHY_PWD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENAUTOCLR_PHY_PWD`"]
pub struct ENAUTOCLR_PHY_PWD_W<'a> {
    w: &'a mut W,
}
impl<'a> ENAUTOCLR_PHY_PWD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `FSDLL_RST_EN`"]
pub type FSDLL_RST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSDLL_RST_EN`"]
pub struct FSDLL_RST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FSDLL_RST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `OTG_ID_VALUE`"]
pub type OTG_ID_VALUE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OTG_ID_VALUE`"]
pub struct OTG_ID_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> OTG_ID_VALUE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `HOST_FORCE_LS_SE0`"]
pub type HOST_FORCE_LS_SE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOST_FORCE_LS_SE0`"]
pub struct HOST_FORCE_LS_SE0_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_FORCE_LS_SE0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `UTMI_SUSPENDM`"]
pub type UTMI_SUSPENDM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UTMI_SUSPENDM`"]
pub struct UTMI_SUSPENDM_W<'a> {
    w: &'a mut W,
}
impl<'a> UTMI_SUSPENDM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `CLKGATE`"]
pub type CLKGATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKGATE`"]
pub struct CLKGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKGATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `SFTRST`"]
pub type SFTRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SFTRST`"]
pub struct SFTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SFTRST_W<'a> {
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
    #[doc = "Bit 1 - For host mode, enables high-speed disconnect detector"]
    #[inline(always)]
    pub fn enhostdiscondetect(&self) -> ENHOSTDISCONDETECT_R {
        ENHOSTDISCONDETECT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Indicates that the device has disconnected in High-Speed mode"]
    #[inline(always)]
    pub fn hostdiscondetect_irq(&self) -> HOSTDISCONDETECT_IRQ_R {
        HOSTDISCONDETECT_IRQ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode"]
    #[inline(always)]
    pub fn endevplugindet(&self) -> ENDEVPLUGINDET_R {
        ENDEVPLUGINDET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Indicates that the device is connected"]
    #[inline(always)]
    pub fn devplugin_irq(&self) -> DEVPLUGIN_IRQ_R {
        DEVPLUGIN_IRQ_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enables UTMI+ Level 2 operation for the USB HS PHY"]
    #[inline(always)]
    pub fn enutmilevel2(&self) -> ENUTMILEVEL2_R {
        ENUTMILEVEL2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enables UTMI+ Level 3 operation for the USB HS PHY"]
    #[inline(always)]
    pub fn enutmilevel3(&self) -> ENUTMILEVEL3_R {
        ENUTMILEVEL3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enable the auto resume feature, when set, HW will use 32KHz clock to send Resume to respond to the device remote wakeup(for host mode only)"]
    #[inline(always)]
    pub fn autoresume_en(&self) -> AUTORESUME_EN_R {
        AUTORESUME_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub fn enautoclr_clkgate(&self) -> ENAUTOCLR_CLKGATE_R {
        ENAUTOCLR_CLKGATE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enables the feature to auto-clear the PWD register bits in PWD if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub fn enautoclr_phy_pwd(&self) -> ENAUTOCLR_PHY_PWD_R {
        ENAUTOCLR_PHY_PWD_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet."]
    #[inline(always)]
    pub fn fsdll_rst_en(&self) -> FSDLL_RST_EN_R {
        FSDLL_RST_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Indicates the results of USB_ID pin while monitoring the cable plugged into the Micro- or Mini-AB receptacle"]
    #[inline(always)]
    pub fn otg_id_value(&self) -> OTG_ID_VALUE_R {
        OTG_ID_VALUE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Forces the next FS packet that is transmitted to have a EOP with low-speed timing"]
    #[inline(always)]
    pub fn host_force_ls_se0(&self) -> HOST_FORCE_LS_SE0_R {
        HOST_FORCE_LS_SE0_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Used by the PHY to indicate a powered-down state"]
    #[inline(always)]
    pub fn utmi_suspendm(&self) -> UTMI_SUSPENDM_R {
        UTMI_SUSPENDM_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Gate UTMI Clocks"]
    #[inline(always)]
    pub fn clkgate(&self) -> CLKGATE_R {
        CLKGATE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Writing a 1 to this bit will soft-reset the PWD, TX, RX, and CTRL registers"]
    #[inline(always)]
    pub fn sftrst(&self) -> SFTRST_R {
        SFTRST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - For host mode, enables high-speed disconnect detector"]
    #[inline(always)]
    pub fn enhostdiscondetect(&mut self) -> ENHOSTDISCONDETECT_W {
        ENHOSTDISCONDETECT_W { w: self }
    }
    #[doc = "Bit 3 - Indicates that the device has disconnected in High-Speed mode"]
    #[inline(always)]
    pub fn hostdiscondetect_irq(&mut self) -> HOSTDISCONDETECT_IRQ_W {
        HOSTDISCONDETECT_IRQ_W { w: self }
    }
    #[doc = "Bit 4 - Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode"]
    #[inline(always)]
    pub fn endevplugindet(&mut self) -> ENDEVPLUGINDET_W {
        ENDEVPLUGINDET_W { w: self }
    }
    #[doc = "Bit 12 - Indicates that the device is connected"]
    #[inline(always)]
    pub fn devplugin_irq(&mut self) -> DEVPLUGIN_IRQ_W {
        DEVPLUGIN_IRQ_W { w: self }
    }
    #[doc = "Bit 14 - Enables UTMI+ Level 2 operation for the USB HS PHY"]
    #[inline(always)]
    pub fn enutmilevel2(&mut self) -> ENUTMILEVEL2_W {
        ENUTMILEVEL2_W { w: self }
    }
    #[doc = "Bit 15 - Enables UTMI+ Level 3 operation for the USB HS PHY"]
    #[inline(always)]
    pub fn enutmilevel3(&mut self) -> ENUTMILEVEL3_W {
        ENUTMILEVEL3_W { w: self }
    }
    #[doc = "Bit 18 - Enable the auto resume feature, when set, HW will use 32KHz clock to send Resume to respond to the device remote wakeup(for host mode only)"]
    #[inline(always)]
    pub fn autoresume_en(&mut self) -> AUTORESUME_EN_W {
        AUTORESUME_EN_W { w: self }
    }
    #[doc = "Bit 19 - Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub fn enautoclr_clkgate(&mut self) -> ENAUTOCLR_CLKGATE_W {
        ENAUTOCLR_CLKGATE_W { w: self }
    }
    #[doc = "Bit 20 - Enables the feature to auto-clear the PWD register bits in PWD if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub fn enautoclr_phy_pwd(&mut self) -> ENAUTOCLR_PHY_PWD_W {
        ENAUTOCLR_PHY_PWD_W { w: self }
    }
    #[doc = "Bit 24 - Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet."]
    #[inline(always)]
    pub fn fsdll_rst_en(&mut self) -> FSDLL_RST_EN_W {
        FSDLL_RST_EN_W { w: self }
    }
    #[doc = "Bit 27 - Indicates the results of USB_ID pin while monitoring the cable plugged into the Micro- or Mini-AB receptacle"]
    #[inline(always)]
    pub fn otg_id_value(&mut self) -> OTG_ID_VALUE_W {
        OTG_ID_VALUE_W { w: self }
    }
    #[doc = "Bit 28 - Forces the next FS packet that is transmitted to have a EOP with low-speed timing"]
    #[inline(always)]
    pub fn host_force_ls_se0(&mut self) -> HOST_FORCE_LS_SE0_W {
        HOST_FORCE_LS_SE0_W { w: self }
    }
    #[doc = "Bit 29 - Used by the PHY to indicate a powered-down state"]
    #[inline(always)]
    pub fn utmi_suspendm(&mut self) -> UTMI_SUSPENDM_W {
        UTMI_SUSPENDM_W { w: self }
    }
    #[doc = "Bit 30 - Gate UTMI Clocks"]
    #[inline(always)]
    pub fn clkgate(&mut self) -> CLKGATE_W {
        CLKGATE_W { w: self }
    }
    #[doc = "Bit 31 - Writing a 1 to this bit will soft-reset the PWD, TX, RX, and CTRL registers"]
    #[inline(always)]
    pub fn sftrst(&mut self) -> SFTRST_W {
        SFTRST_W { w: self }
    }
}
