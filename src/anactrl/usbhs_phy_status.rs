#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::USBHS_PHY_STATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct PFD_STABLER {
    bits: bool,
}
impl PFD_STABLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct VBUSVALID_3VDETECT_1P8VR {
    bits: bool,
}
impl VBUSVALID_3VDETECT_1P8VR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct SESS_VLD_1P8VR {
    bits: bool,
}
impl SESS_VLD_1P8VR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct USB2_RX_VPIN_FS_1P8VR {
    bits: bool,
}
impl USB2_RX_VPIN_FS_1P8VR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct USB2_RX_VMIN_FS_1P8VR {
    bits: bool,
}
impl USB2_RX_VMIN_FS_1P8VR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct USB2_PLUGGED_IN_1P8VR {
    bits: bool,
}
impl USB2_PLUGGED_IN_1P8VR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct USB2_IDDIG_1P8VR {
    bits: bool,
}
impl USB2_IDDIG_1P8VR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - pfd output is stable."]
    #[inline]
    pub fn pfd_stable(&self) -> PFD_STABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PFD_STABLER { bits }
    }
    #[doc = "Bit 1 - Can be left disconnected if not using High volt interrupts."]
    #[inline]
    pub fn vbusvalid_3vdetect_1p8v(&self) -> VBUSVALID_3VDETECT_1P8VR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VBUSVALID_3VDETECT_1P8VR { bits }
    }
    #[doc = "Bit 2 - Same as utmi_sessend."]
    #[inline]
    pub fn sess_vld_1p8v(&self) -> SESS_VLD_1P8VR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SESS_VLD_1P8VR { bits }
    }
    #[doc = "Bit 3 - Full speed single ended receiver for 1."]
    #[inline]
    pub fn usb2_rx_vpin_fs_1p8v(&self) -> USB2_RX_VPIN_FS_1P8VR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        USB2_RX_VPIN_FS_1P8VR { bits }
    }
    #[doc = "Bit 4 - Full speed single ended receiver for 1."]
    #[inline]
    pub fn usb2_rx_vmin_fs_1p8v(&self) -> USB2_RX_VMIN_FS_1P8VR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        USB2_RX_VMIN_FS_1P8VR { bits }
    }
    #[doc = "Bit 5 - this is a proprietary mode described in the reference manual."]
    #[inline]
    pub fn usb2_plugged_in_1p8v(&self) -> USB2_PLUGGED_IN_1P8VR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        USB2_PLUGGED_IN_1P8VR { bits }
    }
    #[doc = "Bit 6 - ID value in the 1."]
    #[inline]
    pub fn usb2_iddig_1p8v(&self) -> USB2_IDDIG_1P8VR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        USB2_IDDIG_1P8VR { bits }
    }
}
