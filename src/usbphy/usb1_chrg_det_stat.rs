#[doc = "Reader of register USB1_CHRG_DET_STAT"]
pub type R = crate::R<u32, super::USB1_CHRG_DET_STAT>;
#[doc = "Possible values of the field `PLUG_CONTACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLUG_CONTACT_A {
    #[doc = "No USB cable attachment has been detected"]
    VALUE0,
    #[doc = "A USB cable attachment between the device and host has been detected"]
    VALUE1,
}
impl From<PLUG_CONTACT_A> for bool {
    #[inline(always)]
    fn from(variant: PLUG_CONTACT_A) -> Self {
        match variant {
            PLUG_CONTACT_A::VALUE0 => false,
            PLUG_CONTACT_A::VALUE1 => true,
        }
    }
}
#[doc = "Reader of field `PLUG_CONTACT`"]
pub type PLUG_CONTACT_R = crate::R<bool, PLUG_CONTACT_A>;
impl PLUG_CONTACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLUG_CONTACT_A {
        match self.bits {
            false => PLUG_CONTACT_A::VALUE0,
            true => PLUG_CONTACT_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == PLUG_CONTACT_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PLUG_CONTACT_A::VALUE1
    }
}
#[doc = "Possible values of the field `CHRG_DETECTED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHRG_DETECTED_A {
    #[doc = "Standard Downstream Port (SDP) has been detected"]
    VALUE0,
    #[doc = "Charging Port has been detected"]
    VALUE1,
}
impl From<CHRG_DETECTED_A> for bool {
    #[inline(always)]
    fn from(variant: CHRG_DETECTED_A) -> Self {
        match variant {
            CHRG_DETECTED_A::VALUE0 => false,
            CHRG_DETECTED_A::VALUE1 => true,
        }
    }
}
#[doc = "Reader of field `CHRG_DETECTED`"]
pub type CHRG_DETECTED_R = crate::R<bool, CHRG_DETECTED_A>;
impl CHRG_DETECTED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHRG_DETECTED_A {
        match self.bits {
            false => CHRG_DETECTED_A::VALUE0,
            true => CHRG_DETECTED_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == CHRG_DETECTED_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHRG_DETECTED_A::VALUE1
    }
}
#[doc = "Possible values of the field `DM_STATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DM_STATE_A {
    #[doc = "USB_DM pin voltage is < 0.8V"]
    VALUE0,
    #[doc = "USB_DM pin voltage is > 2.0V"]
    VALUE1,
}
impl From<DM_STATE_A> for bool {
    #[inline(always)]
    fn from(variant: DM_STATE_A) -> Self {
        match variant {
            DM_STATE_A::VALUE0 => false,
            DM_STATE_A::VALUE1 => true,
        }
    }
}
#[doc = "Reader of field `DM_STATE`"]
pub type DM_STATE_R = crate::R<bool, DM_STATE_A>;
impl DM_STATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DM_STATE_A {
        match self.bits {
            false => DM_STATE_A::VALUE0,
            true => DM_STATE_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == DM_STATE_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DM_STATE_A::VALUE1
    }
}
#[doc = "Possible values of the field `DP_STATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DP_STATE_A {
    #[doc = "USB_DP pin voltage is < 0.8V"]
    VALUE0,
    #[doc = "USB_DP pin voltage is > 2.0V"]
    VALUE1,
}
impl From<DP_STATE_A> for bool {
    #[inline(always)]
    fn from(variant: DP_STATE_A) -> Self {
        match variant {
            DP_STATE_A::VALUE0 => false,
            DP_STATE_A::VALUE1 => true,
        }
    }
}
#[doc = "Reader of field `DP_STATE`"]
pub type DP_STATE_R = crate::R<bool, DP_STATE_A>;
impl DP_STATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DP_STATE_A {
        match self.bits {
            false => DP_STATE_A::VALUE0,
            true => DP_STATE_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == DP_STATE_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DP_STATE_A::VALUE1
    }
}
#[doc = "Possible values of the field `SECDET_DCP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECDET_DCP_A {
    #[doc = "Charging Downstream Port (CDP) has been detected"]
    VALUE0,
    #[doc = "Downstream Charging Port (DCP) has been detected"]
    VALUE1,
}
impl From<SECDET_DCP_A> for bool {
    #[inline(always)]
    fn from(variant: SECDET_DCP_A) -> Self {
        match variant {
            SECDET_DCP_A::VALUE0 => false,
            SECDET_DCP_A::VALUE1 => true,
        }
    }
}
#[doc = "Reader of field `SECDET_DCP`"]
pub type SECDET_DCP_R = crate::R<bool, SECDET_DCP_A>;
impl SECDET_DCP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECDET_DCP_A {
        match self.bits {
            false => SECDET_DCP_A::VALUE0,
            true => SECDET_DCP_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == SECDET_DCP_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SECDET_DCP_A::VALUE1
    }
}
impl R {
    #[doc = "Bit 0 - Battery Charging Data Contact Detection phase output During the Data Contact Detection phase per the USB Battery Charging Specification Revision 1"]
    #[inline(always)]
    pub fn plug_contact(&self) -> PLUG_CONTACT_R {
        PLUG_CONTACT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Battery Charging Primary Detection phase output During the USB Battery Charging Primary Detection phase using the USBHSDCD module, this bit field indicates whether a Standard Downstream Port or Charging Port was detected"]
    #[inline(always)]
    pub fn chrg_detected(&self) -> CHRG_DETECTED_R {
        CHRG_DETECTED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Single ended receiver output for the USB_DM pin, from charger detection circuits."]
    #[inline(always)]
    pub fn dm_state(&self) -> DM_STATE_R {
        DM_STATE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Single ended receiver output for the USB_DP pin, from charger detection circuits."]
    #[inline(always)]
    pub fn dp_state(&self) -> DP_STATE_R {
        DP_STATE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Battery Charging Secondary Detection phase output During the USB Battery Charging Secondary Detection phase using the USBHSDCD module, this bit field indicates which kind of Charging Port was detected"]
    #[inline(always)]
    pub fn secdet_dcp(&self) -> SECDET_DCP_R {
        SECDET_DCP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
