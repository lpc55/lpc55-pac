#[doc = "Reader of register USBHS_PHY_STATUS"]
pub type R = crate::R<u32, super::USBHS_PHY_STATUS>;
#[doc = "Reader of field `pfd_stable`"]
pub type PFD_STABLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `vbusvalid_3vdetect_1p8v`"]
pub type VBUSVALID_3VDETECT_1P8V_R = crate::R<bool, bool>;
#[doc = "Reader of field `sess_vld_1p8v`"]
pub type SESS_VLD_1P8V_R = crate::R<bool, bool>;
#[doc = "Reader of field `usb2_rx_vpin_fs_1p8v`"]
pub type USB2_RX_VPIN_FS_1P8V_R = crate::R<bool, bool>;
#[doc = "Reader of field `usb2_rx_vmin_fs_1p8v`"]
pub type USB2_RX_VMIN_FS_1P8V_R = crate::R<bool, bool>;
#[doc = "Reader of field `usb2_plugged_in_1p8v`"]
pub type USB2_PLUGGED_IN_1P8V_R = crate::R<bool, bool>;
#[doc = "Reader of field `usb2_iddig_1p8v`"]
pub type USB2_IDDIG_1P8V_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - pfd output is stable."]
    #[inline(always)]
    pub fn pfd_stable(&self) -> PFD_STABLE_R {
        PFD_STABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Can be left disconnected if not using High volt interrupts."]
    #[inline(always)]
    pub fn vbusvalid_3vdetect_1p8v(&self) -> VBUSVALID_3VDETECT_1P8V_R {
        VBUSVALID_3VDETECT_1P8V_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Same as utmi_sessend."]
    #[inline(always)]
    pub fn sess_vld_1p8v(&self) -> SESS_VLD_1P8V_R {
        SESS_VLD_1P8V_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Full speed single ended receiver for 1."]
    #[inline(always)]
    pub fn usb2_rx_vpin_fs_1p8v(&self) -> USB2_RX_VPIN_FS_1P8V_R {
        USB2_RX_VPIN_FS_1P8V_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Full speed single ended receiver for 1."]
    #[inline(always)]
    pub fn usb2_rx_vmin_fs_1p8v(&self) -> USB2_RX_VMIN_FS_1P8V_R {
        USB2_RX_VMIN_FS_1P8V_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - this is a proprietary mode described in the reference manual."]
    #[inline(always)]
    pub fn usb2_plugged_in_1p8v(&self) -> USB2_PLUGGED_IN_1P8V_R {
        USB2_PLUGGED_IN_1P8V_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ID value in the 1."]
    #[inline(always)]
    pub fn usb2_iddig_1p8v(&self) -> USB2_IDDIG_1P8V_R {
        USB2_IDDIG_1P8V_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
