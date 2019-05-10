#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::USB1_CHRG_DET_STAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `PLUG_CONTACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLUG_CONTACTR {
    #[doc = "No USB cable attachment has been detected"]
    VALUE0,
    #[doc = "A USB cable attachment between the device and host has been detected"]
    VALUE1,
}
impl PLUG_CONTACTR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PLUG_CONTACTR::VALUE0 => false,
            PLUG_CONTACTR::VALUE1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLUG_CONTACTR {
        match value {
            false => PLUG_CONTACTR::VALUE0,
            true => PLUG_CONTACTR::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == PLUG_CONTACTR::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PLUG_CONTACTR::VALUE1
    }
}
#[doc = "Possible values of the field `CHRG_DETECTED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHRG_DETECTEDR {
    #[doc = "Standard Downstream Port (SDP) has been detected"]
    VALUE0,
    #[doc = "Charging Port has been detected"]
    VALUE1,
}
impl CHRG_DETECTEDR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CHRG_DETECTEDR::VALUE0 => false,
            CHRG_DETECTEDR::VALUE1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHRG_DETECTEDR {
        match value {
            false => CHRG_DETECTEDR::VALUE0,
            true => CHRG_DETECTEDR::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == CHRG_DETECTEDR::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CHRG_DETECTEDR::VALUE1
    }
}
#[doc = "Possible values of the field `DM_STATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DM_STATER {
    #[doc = "USB_DM pin voltage is < 0.8V"]
    VALUE0,
    #[doc = "USB_DM pin voltage is > 2.0V"]
    VALUE1,
}
impl DM_STATER {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DM_STATER::VALUE0 => false,
            DM_STATER::VALUE1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DM_STATER {
        match value {
            false => DM_STATER::VALUE0,
            true => DM_STATER::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == DM_STATER::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DM_STATER::VALUE1
    }
}
#[doc = "Possible values of the field `DP_STATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DP_STATER {
    #[doc = "USB_DP pin voltage is < 0.8V"]
    VALUE0,
    #[doc = "USB_DP pin voltage is > 2.0V"]
    VALUE1,
}
impl DP_STATER {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DP_STATER::VALUE0 => false,
            DP_STATER::VALUE1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DP_STATER {
        match value {
            false => DP_STATER::VALUE0,
            true => DP_STATER::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == DP_STATER::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DP_STATER::VALUE1
    }
}
#[doc = "Possible values of the field `SECDET_DCP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECDET_DCPR {
    #[doc = "Charging Downstream Port (CDP) has been detected"]
    VALUE0,
    #[doc = "Downstream Charging Port (DCP) has been detected"]
    VALUE1,
}
impl SECDET_DCPR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SECDET_DCPR::VALUE0 => false,
            SECDET_DCPR::VALUE1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SECDET_DCPR {
        match value {
            false => SECDET_DCPR::VALUE0,
            true => SECDET_DCPR::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == SECDET_DCPR::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SECDET_DCPR::VALUE1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Battery Charging Data Contact Detection phase output During the Data Contact Detection phase per the USB Battery Charging Specification Revision 1"]
    #[inline]
    pub fn plug_contact(&self) -> PLUG_CONTACTR {
        PLUG_CONTACTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Battery Charging Primary Detection phase output During the USB Battery Charging Primary Detection phase using the USBHSDCD module, this bit field indicates whether a Standard Downstream Port or Charging Port was detected"]
    #[inline]
    pub fn chrg_detected(&self) -> CHRG_DETECTEDR {
        CHRG_DETECTEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Single ended receiver output for the USB_DM pin, from charger detection circuits."]
    #[inline]
    pub fn dm_state(&self) -> DM_STATER {
        DM_STATER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Single ended receiver output for the USB_DP pin, from charger detection circuits."]
    #[inline]
    pub fn dp_state(&self) -> DP_STATER {
        DP_STATER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Battery Charging Secondary Detection phase output During the USB Battery Charging Secondary Detection phase using the USBHSDCD module, this bit field indicates which kind of Charging Port was detected"]
    #[inline]
    pub fn secdet_dcp(&self) -> SECDET_DCPR {
        SECDET_DCPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
