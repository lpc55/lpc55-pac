#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MASTER_SEC_ANTI_POL_REG {
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
#[doc = "Possible values of the field `MCM33C`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCM33CR {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
}
impl MCM33CR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MCM33CR::ENUM_S_P => 0,
            MCM33CR::ENUM_S_NP => 1,
            MCM33CR::ENUM_NS_P => 2,
            MCM33CR::ENUM_NS_NP => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MCM33CR {
        match value {
            0 => MCM33CR::ENUM_S_P,
            1 => MCM33CR::ENUM_S_NP,
            2 => MCM33CR::ENUM_NS_P,
            3 => MCM33CR::ENUM_NS_NP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline]
    pub fn is_enum_s_p(&self) -> bool {
        *self == MCM33CR::ENUM_S_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline]
    pub fn is_enum_s_np(&self) -> bool {
        *self == MCM33CR::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == MCM33CR::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == MCM33CR::ENUM_NS_NP
    }
}
#[doc = "Possible values of the field `MCM33S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCM33SR {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
}
impl MCM33SR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MCM33SR::ENUM_S_P => 0,
            MCM33SR::ENUM_S_NP => 1,
            MCM33SR::ENUM_NS_P => 2,
            MCM33SR::ENUM_NS_NP => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MCM33SR {
        match value {
            0 => MCM33SR::ENUM_S_P,
            1 => MCM33SR::ENUM_S_NP,
            2 => MCM33SR::ENUM_NS_P,
            3 => MCM33SR::ENUM_NS_NP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline]
    pub fn is_enum_s_p(&self) -> bool {
        *self == MCM33SR::ENUM_S_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline]
    pub fn is_enum_s_np(&self) -> bool {
        *self == MCM33SR::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == MCM33SR::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == MCM33SR::ENUM_NS_NP
    }
}
#[doc = "Possible values of the field `USBFSD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBFSDR {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
}
impl USBFSDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            USBFSDR::ENUM_S_P => 0,
            USBFSDR::ENUM_S_NP => 1,
            USBFSDR::ENUM_NS_P => 2,
            USBFSDR::ENUM_NS_NP => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> USBFSDR {
        match value {
            0 => USBFSDR::ENUM_S_P,
            1 => USBFSDR::ENUM_S_NP,
            2 => USBFSDR::ENUM_NS_P,
            3 => USBFSDR::ENUM_NS_NP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline]
    pub fn is_enum_s_p(&self) -> bool {
        *self == USBFSDR::ENUM_S_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline]
    pub fn is_enum_s_np(&self) -> bool {
        *self == USBFSDR::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == USBFSDR::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == USBFSDR::ENUM_NS_NP
    }
}
#[doc = "Possible values of the field `SDMA0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMA0R {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
}
impl SDMA0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SDMA0R::ENUM_S_P => 0,
            SDMA0R::ENUM_S_NP => 1,
            SDMA0R::ENUM_NS_P => 2,
            SDMA0R::ENUM_NS_NP => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SDMA0R {
        match value {
            0 => SDMA0R::ENUM_S_P,
            1 => SDMA0R::ENUM_S_NP,
            2 => SDMA0R::ENUM_NS_P,
            3 => SDMA0R::ENUM_NS_NP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline]
    pub fn is_enum_s_p(&self) -> bool {
        *self == SDMA0R::ENUM_S_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline]
    pub fn is_enum_s_np(&self) -> bool {
        *self == SDMA0R::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == SDMA0R::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == SDMA0R::ENUM_NS_NP
    }
}
#[doc = "Possible values of the field `SDIO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDIOR {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
}
impl SDIOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SDIOR::ENUM_S_P => 0,
            SDIOR::ENUM_S_NP => 1,
            SDIOR::ENUM_NS_P => 2,
            SDIOR::ENUM_NS_NP => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SDIOR {
        match value {
            0 => SDIOR::ENUM_S_P,
            1 => SDIOR::ENUM_S_NP,
            2 => SDIOR::ENUM_NS_P,
            3 => SDIOR::ENUM_NS_NP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline]
    pub fn is_enum_s_p(&self) -> bool {
        *self == SDIOR::ENUM_S_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline]
    pub fn is_enum_s_np(&self) -> bool {
        *self == SDIOR::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == SDIOR::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == SDIOR::ENUM_NS_NP
    }
}
#[doc = "Possible values of the field `PQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PQR {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
}
impl PQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PQR::ENUM_S_P => 0,
            PQR::ENUM_S_NP => 1,
            PQR::ENUM_NS_P => 2,
            PQR::ENUM_NS_NP => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PQR {
        match value {
            0 => PQR::ENUM_S_P,
            1 => PQR::ENUM_S_NP,
            2 => PQR::ENUM_NS_P,
            3 => PQR::ENUM_NS_NP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline]
    pub fn is_enum_s_p(&self) -> bool {
        *self == PQR::ENUM_S_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline]
    pub fn is_enum_s_np(&self) -> bool {
        *self == PQR::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == PQR::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == PQR::ENUM_NS_NP
    }
}
#[doc = "Possible values of the field `HASH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HASHR {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
}
impl HASHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HASHR::ENUM_S_P => 0,
            HASHR::ENUM_S_NP => 1,
            HASHR::ENUM_NS_P => 2,
            HASHR::ENUM_NS_NP => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HASHR {
        match value {
            0 => HASHR::ENUM_S_P,
            1 => HASHR::ENUM_S_NP,
            2 => HASHR::ENUM_NS_P,
            3 => HASHR::ENUM_NS_NP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline]
    pub fn is_enum_s_p(&self) -> bool {
        *self == HASHR::ENUM_S_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline]
    pub fn is_enum_s_np(&self) -> bool {
        *self == HASHR::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == HASHR::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == HASHR::ENUM_NS_NP
    }
}
#[doc = "Possible values of the field `USBFSH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBFSHR {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
}
impl USBFSHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            USBFSHR::ENUM_S_P => 0,
            USBFSHR::ENUM_S_NP => 1,
            USBFSHR::ENUM_NS_P => 2,
            USBFSHR::ENUM_NS_NP => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> USBFSHR {
        match value {
            0 => USBFSHR::ENUM_S_P,
            1 => USBFSHR::ENUM_S_NP,
            2 => USBFSHR::ENUM_NS_P,
            3 => USBFSHR::ENUM_NS_NP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline]
    pub fn is_enum_s_p(&self) -> bool {
        *self == USBFSHR::ENUM_S_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline]
    pub fn is_enum_s_np(&self) -> bool {
        *self == USBFSHR::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == USBFSHR::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == USBFSHR::ENUM_NS_NP
    }
}
#[doc = "Possible values of the field `SDMA1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMA1R {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
}
impl SDMA1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SDMA1R::ENUM_S_P => 0,
            SDMA1R::ENUM_S_NP => 1,
            SDMA1R::ENUM_NS_P => 2,
            SDMA1R::ENUM_NS_NP => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SDMA1R {
        match value {
            0 => SDMA1R::ENUM_S_P,
            1 => SDMA1R::ENUM_S_NP,
            2 => SDMA1R::ENUM_NS_P,
            3 => SDMA1R::ENUM_NS_NP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline]
    pub fn is_enum_s_p(&self) -> bool {
        *self == SDMA1R::ENUM_S_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline]
    pub fn is_enum_s_np(&self) -> bool {
        *self == SDMA1R::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == SDMA1R::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == SDMA1R::ENUM_NS_NP
    }
}
#[doc = "Possible values of the field `MASTER_SEC_LEVEL_ANTIPOL_LOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASTER_SEC_LEVEL_ANTIPOL_LOCKR {
    #[doc = "Restricted mode."]
    BLOCKED,
    #[doc = "Writable."]
    WRITABLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MASTER_SEC_LEVEL_ANTIPOL_LOCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MASTER_SEC_LEVEL_ANTIPOL_LOCKR::BLOCKED => 1,
            MASTER_SEC_LEVEL_ANTIPOL_LOCKR::WRITABLE => 2,
            MASTER_SEC_LEVEL_ANTIPOL_LOCKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MASTER_SEC_LEVEL_ANTIPOL_LOCKR {
        match value {
            1 => MASTER_SEC_LEVEL_ANTIPOL_LOCKR::BLOCKED,
            2 => MASTER_SEC_LEVEL_ANTIPOL_LOCKR::WRITABLE,
            i => MASTER_SEC_LEVEL_ANTIPOL_LOCKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == MASTER_SEC_LEVEL_ANTIPOL_LOCKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `WRITABLE`"]
    #[inline]
    pub fn is_writable(&self) -> bool {
        *self == MASTER_SEC_LEVEL_ANTIPOL_LOCKR::WRITABLE
    }
}
#[doc = "Values that can be written to the field `MCM33C`"]
pub enum MCM33CW {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
}
impl MCM33CW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MCM33CW::ENUM_S_P => 0,
            MCM33CW::ENUM_S_NP => 1,
            MCM33CW::ENUM_NS_P => 2,
            MCM33CW::ENUM_NS_NP => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCM33CW<'a> {
    w: &'a mut W,
}
impl<'a> _MCM33CW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCM33CW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(MCM33CW::ENUM_S_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(MCM33CW::ENUM_S_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(MCM33CW::ENUM_NS_P)
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(MCM33CW::ENUM_NS_NP)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MCM33S`"]
pub enum MCM33SW {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
}
impl MCM33SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MCM33SW::ENUM_S_P => 0,
            MCM33SW::ENUM_S_NP => 1,
            MCM33SW::ENUM_NS_P => 2,
            MCM33SW::ENUM_NS_NP => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCM33SW<'a> {
    w: &'a mut W,
}
impl<'a> _MCM33SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCM33SW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(MCM33SW::ENUM_S_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(MCM33SW::ENUM_S_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(MCM33SW::ENUM_NS_P)
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(MCM33SW::ENUM_NS_NP)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USBFSD`"]
pub enum USBFSDW {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
}
impl USBFSDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            USBFSDW::ENUM_S_P => 0,
            USBFSDW::ENUM_S_NP => 1,
            USBFSDW::ENUM_NS_P => 2,
            USBFSDW::ENUM_NS_NP => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBFSDW<'a> {
    w: &'a mut W,
}
impl<'a> _USBFSDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBFSDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(USBFSDW::ENUM_S_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(USBFSDW::ENUM_S_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(USBFSDW::ENUM_NS_P)
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(USBFSDW::ENUM_NS_NP)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SDMA0`"]
pub enum SDMA0W {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
}
impl SDMA0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SDMA0W::ENUM_S_P => 0,
            SDMA0W::ENUM_S_NP => 1,
            SDMA0W::ENUM_NS_P => 2,
            SDMA0W::ENUM_NS_NP => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDMA0W<'a> {
    w: &'a mut W,
}
impl<'a> _SDMA0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDMA0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(SDMA0W::ENUM_S_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(SDMA0W::ENUM_S_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(SDMA0W::ENUM_NS_P)
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(SDMA0W::ENUM_NS_NP)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SDIO`"]
pub enum SDIOW {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
}
impl SDIOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SDIOW::ENUM_S_P => 0,
            SDIOW::ENUM_S_NP => 1,
            SDIOW::ENUM_NS_P => 2,
            SDIOW::ENUM_NS_NP => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDIOW<'a> {
    w: &'a mut W,
}
impl<'a> _SDIOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDIOW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(SDIOW::ENUM_S_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(SDIOW::ENUM_S_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(SDIOW::ENUM_NS_P)
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(SDIOW::ENUM_NS_NP)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PQ`"]
pub enum PQW {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
}
impl PQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PQW::ENUM_S_P => 0,
            PQW::ENUM_S_NP => 1,
            PQW::ENUM_NS_P => 2,
            PQW::ENUM_NS_NP => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PQW<'a> {
    w: &'a mut W,
}
impl<'a> _PQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PQW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(PQW::ENUM_S_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(PQW::ENUM_S_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(PQW::ENUM_NS_P)
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(PQW::ENUM_NS_NP)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HASH`"]
pub enum HASHW {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
}
impl HASHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HASHW::ENUM_S_P => 0,
            HASHW::ENUM_S_NP => 1,
            HASHW::ENUM_NS_P => 2,
            HASHW::ENUM_NS_NP => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HASHW<'a> {
    w: &'a mut W,
}
impl<'a> _HASHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HASHW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(HASHW::ENUM_S_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(HASHW::ENUM_S_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(HASHW::ENUM_NS_P)
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(HASHW::ENUM_NS_NP)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USBFSH`"]
pub enum USBFSHW {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
}
impl USBFSHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            USBFSHW::ENUM_S_P => 0,
            USBFSHW::ENUM_S_NP => 1,
            USBFSHW::ENUM_NS_P => 2,
            USBFSHW::ENUM_NS_NP => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBFSHW<'a> {
    w: &'a mut W,
}
impl<'a> _USBFSHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBFSHW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(USBFSHW::ENUM_S_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(USBFSHW::ENUM_S_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(USBFSHW::ENUM_NS_P)
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(USBFSHW::ENUM_NS_NP)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SDMA1`"]
pub enum SDMA1W {
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
}
impl SDMA1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SDMA1W::ENUM_S_P => 0,
            SDMA1W::ENUM_S_NP => 1,
            SDMA1W::ENUM_NS_P => 2,
            SDMA1W::ENUM_NS_NP => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDMA1W<'a> {
    w: &'a mut W,
}
impl<'a> _SDMA1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDMA1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(SDMA1W::ENUM_S_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(SDMA1W::ENUM_S_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(SDMA1W::ENUM_NS_P)
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(SDMA1W::ENUM_NS_NP)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MASTER_SEC_LEVEL_ANTIPOL_LOCK`"]
pub enum MASTER_SEC_LEVEL_ANTIPOL_LOCKW {
    #[doc = "Restricted mode."]
    BLOCKED,
    #[doc = "Writable."]
    WRITABLE,
}
impl MASTER_SEC_LEVEL_ANTIPOL_LOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MASTER_SEC_LEVEL_ANTIPOL_LOCKW::BLOCKED => 1,
            MASTER_SEC_LEVEL_ANTIPOL_LOCKW::WRITABLE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASTER_SEC_LEVEL_ANTIPOL_LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _MASTER_SEC_LEVEL_ANTIPOL_LOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASTER_SEC_LEVEL_ANTIPOL_LOCKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Restricted mode."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(MASTER_SEC_LEVEL_ANTIPOL_LOCKW::BLOCKED)
    }
    #[doc = "Writable."]
    #[inline]
    pub fn writable(self) -> &'a mut W {
        self.variant(MASTER_SEC_LEVEL_ANTIPOL_LOCKW::WRITABLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
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
    #[doc = "Bits 4:5 - Micro-CM33 (CPU1) Code bus. Must be equal to NOT(MASTER_SEC_LEVEL.MCM33C)"]
    #[inline]
    pub fn mcm33c(&self) -> MCM33CR {
        MCM33CR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Micro-CM33 (CPU1) System bus. Must be equal to NOT(MASTER_SEC_LEVEL.MCM33S)"]
    #[inline]
    pub fn mcm33s(&self) -> MCM33SR {
        MCM33SR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - USB Full Speed Device. Must be equal to NOT(MASTER_SEC_LEVEL.USBFSD)"]
    #[inline]
    pub fn usbfsd(&self) -> USBFSDR {
        USBFSDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - System DMA 0. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA0)"]
    #[inline]
    pub fn sdma0(&self) -> SDMA0R {
        SDMA0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - SDIO. Must be equal to NOT(MASTER_SEC_LEVEL.SDIO)"]
    #[inline]
    pub fn sdio(&self) -> SDIOR {
        SDIOR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - Power Quad. Must be equal to NOT(MASTER_SEC_LEVEL.PQ)"]
    #[inline]
    pub fn pq(&self) -> PQR {
        PQR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Hash. Must be equal to NOT(MASTER_SEC_LEVEL.HASH)"]
    #[inline]
    pub fn hash(&self) -> HASHR {
        HASHR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - USB Full speed Host. Must be equal to NOT(MASTER_SEC_LEVEL.USBFSH)"]
    #[inline]
    pub fn usbfsh(&self) -> USBFSHR {
        USBFSHR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - System DMA 1 security level. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA1)"]
    #[inline]
    pub fn sdma1(&self) -> SDMA1R {
        SDMA1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 30:31 - MASTER_SEC_ANTI_POL_REG register write-lock."]
    #[inline]
    pub fn master_sec_level_antipol_lock(&self) -> MASTER_SEC_LEVEL_ANTIPOL_LOCKR {
        MASTER_SEC_LEVEL_ANTIPOL_LOCKR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3221225471 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 4:5 - Micro-CM33 (CPU1) Code bus. Must be equal to NOT(MASTER_SEC_LEVEL.MCM33C)"]
    #[inline]
    pub fn mcm33c(&mut self) -> _MCM33CW {
        _MCM33CW { w: self }
    }
    #[doc = "Bits 6:7 - Micro-CM33 (CPU1) System bus. Must be equal to NOT(MASTER_SEC_LEVEL.MCM33S)"]
    #[inline]
    pub fn mcm33s(&mut self) -> _MCM33SW {
        _MCM33SW { w: self }
    }
    #[doc = "Bits 8:9 - USB Full Speed Device. Must be equal to NOT(MASTER_SEC_LEVEL.USBFSD)"]
    #[inline]
    pub fn usbfsd(&mut self) -> _USBFSDW {
        _USBFSDW { w: self }
    }
    #[doc = "Bits 10:11 - System DMA 0. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA0)"]
    #[inline]
    pub fn sdma0(&mut self) -> _SDMA0W {
        _SDMA0W { w: self }
    }
    #[doc = "Bits 16:17 - SDIO. Must be equal to NOT(MASTER_SEC_LEVEL.SDIO)"]
    #[inline]
    pub fn sdio(&mut self) -> _SDIOW {
        _SDIOW { w: self }
    }
    #[doc = "Bits 18:19 - Power Quad. Must be equal to NOT(MASTER_SEC_LEVEL.PQ)"]
    #[inline]
    pub fn pq(&mut self) -> _PQW {
        _PQW { w: self }
    }
    #[doc = "Bits 20:21 - Hash. Must be equal to NOT(MASTER_SEC_LEVEL.HASH)"]
    #[inline]
    pub fn hash(&mut self) -> _HASHW {
        _HASHW { w: self }
    }
    #[doc = "Bits 22:23 - USB Full speed Host. Must be equal to NOT(MASTER_SEC_LEVEL.USBFSH)"]
    #[inline]
    pub fn usbfsh(&mut self) -> _USBFSHW {
        _USBFSHW { w: self }
    }
    #[doc = "Bits 24:25 - System DMA 1 security level. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA1)"]
    #[inline]
    pub fn sdma1(&mut self) -> _SDMA1W {
        _SDMA1W { w: self }
    }
    #[doc = "Bits 30:31 - MASTER_SEC_ANTI_POL_REG register write-lock."]
    #[inline]
    pub fn master_sec_level_antipol_lock(&mut self) -> _MASTER_SEC_LEVEL_ANTIPOL_LOCKW {
        _MASTER_SEC_LEVEL_ANTIPOL_LOCKW { w: self }
    }
}
