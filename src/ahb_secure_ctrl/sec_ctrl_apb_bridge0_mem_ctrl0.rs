#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SEC_CTRL_APB_BRIDGE0_MEM_CTRL0 {
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
#[doc = "Possible values of the field `SYSCON_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCON_RULER {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl SYSCON_RULER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSCON_RULER::ENUM_NS_NP => 0,
            SYSCON_RULER::ENUM_NS_P => 1,
            SYSCON_RULER::ENUM_S_NP => 2,
            SYSCON_RULER::ENUM_S_P => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SYSCON_RULER {
        match value {
            0 => SYSCON_RULER::ENUM_NS_NP,
            1 => SYSCON_RULER::ENUM_NS_P,
            2 => SYSCON_RULER::ENUM_S_NP,
            3 => SYSCON_RULER::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == SYSCON_RULER::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == SYSCON_RULER::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline]
    pub fn is_enum_s_np(&self) -> bool {
        *self == SYSCON_RULER::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline]
    pub fn is_enum_s_p(&self) -> bool {
        *self == SYSCON_RULER::ENUM_S_P
    }
}
#[doc = "Possible values of the field `IOCON_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOCON_RULER {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl IOCON_RULER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IOCON_RULER::ENUM_NS_NP => 0,
            IOCON_RULER::ENUM_NS_P => 1,
            IOCON_RULER::ENUM_S_NP => 2,
            IOCON_RULER::ENUM_S_P => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IOCON_RULER {
        match value {
            0 => IOCON_RULER::ENUM_NS_NP,
            1 => IOCON_RULER::ENUM_NS_P,
            2 => IOCON_RULER::ENUM_S_NP,
            3 => IOCON_RULER::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == IOCON_RULER::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == IOCON_RULER::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline]
    pub fn is_enum_s_np(&self) -> bool {
        *self == IOCON_RULER::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline]
    pub fn is_enum_s_p(&self) -> bool {
        *self == IOCON_RULER::ENUM_S_P
    }
}
#[doc = "Possible values of the field `GINT0_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GINT0_RULER {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl GINT0_RULER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GINT0_RULER::ENUM_NS_NP => 0,
            GINT0_RULER::ENUM_NS_P => 1,
            GINT0_RULER::ENUM_S_NP => 2,
            GINT0_RULER::ENUM_S_P => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GINT0_RULER {
        match value {
            0 => GINT0_RULER::ENUM_NS_NP,
            1 => GINT0_RULER::ENUM_NS_P,
            2 => GINT0_RULER::ENUM_S_NP,
            3 => GINT0_RULER::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == GINT0_RULER::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == GINT0_RULER::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline]
    pub fn is_enum_s_np(&self) -> bool {
        *self == GINT0_RULER::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline]
    pub fn is_enum_s_p(&self) -> bool {
        *self == GINT0_RULER::ENUM_S_P
    }
}
#[doc = "Possible values of the field `GINT1_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GINT1_RULER {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl GINT1_RULER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GINT1_RULER::ENUM_NS_NP => 0,
            GINT1_RULER::ENUM_NS_P => 1,
            GINT1_RULER::ENUM_S_NP => 2,
            GINT1_RULER::ENUM_S_P => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GINT1_RULER {
        match value {
            0 => GINT1_RULER::ENUM_NS_NP,
            1 => GINT1_RULER::ENUM_NS_P,
            2 => GINT1_RULER::ENUM_S_NP,
            3 => GINT1_RULER::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == GINT1_RULER::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == GINT1_RULER::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline]
    pub fn is_enum_s_np(&self) -> bool {
        *self == GINT1_RULER::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline]
    pub fn is_enum_s_p(&self) -> bool {
        *self == GINT1_RULER::ENUM_S_P
    }
}
#[doc = "Possible values of the field `PINT_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINT_RULER {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl PINT_RULER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PINT_RULER::ENUM_NS_NP => 0,
            PINT_RULER::ENUM_NS_P => 1,
            PINT_RULER::ENUM_S_NP => 2,
            PINT_RULER::ENUM_S_P => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PINT_RULER {
        match value {
            0 => PINT_RULER::ENUM_NS_NP,
            1 => PINT_RULER::ENUM_NS_P,
            2 => PINT_RULER::ENUM_S_NP,
            3 => PINT_RULER::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == PINT_RULER::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == PINT_RULER::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline]
    pub fn is_enum_s_np(&self) -> bool {
        *self == PINT_RULER::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline]
    pub fn is_enum_s_p(&self) -> bool {
        *self == PINT_RULER::ENUM_S_P
    }
}
#[doc = "Possible values of the field `SEC_PINT_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_PINT_RULER {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl SEC_PINT_RULER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SEC_PINT_RULER::ENUM_NS_NP => 0,
            SEC_PINT_RULER::ENUM_NS_P => 1,
            SEC_PINT_RULER::ENUM_S_NP => 2,
            SEC_PINT_RULER::ENUM_S_P => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SEC_PINT_RULER {
        match value {
            0 => SEC_PINT_RULER::ENUM_NS_NP,
            1 => SEC_PINT_RULER::ENUM_NS_P,
            2 => SEC_PINT_RULER::ENUM_S_NP,
            3 => SEC_PINT_RULER::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == SEC_PINT_RULER::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == SEC_PINT_RULER::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline]
    pub fn is_enum_s_np(&self) -> bool {
        *self == SEC_PINT_RULER::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline]
    pub fn is_enum_s_p(&self) -> bool {
        *self == SEC_PINT_RULER::ENUM_S_P
    }
}
#[doc = "Possible values of the field `PMUX_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMUX_RULER {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl PMUX_RULER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PMUX_RULER::ENUM_NS_NP => 0,
            PMUX_RULER::ENUM_NS_P => 1,
            PMUX_RULER::ENUM_S_NP => 2,
            PMUX_RULER::ENUM_S_P => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PMUX_RULER {
        match value {
            0 => PMUX_RULER::ENUM_NS_NP,
            1 => PMUX_RULER::ENUM_NS_P,
            2 => PMUX_RULER::ENUM_S_NP,
            3 => PMUX_RULER::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == PMUX_RULER::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == PMUX_RULER::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline]
    pub fn is_enum_s_np(&self) -> bool {
        *self == PMUX_RULER::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline]
    pub fn is_enum_s_p(&self) -> bool {
        *self == PMUX_RULER::ENUM_S_P
    }
}
#[doc = "Values that can be written to the field `SYSCON_RULE`"]
pub enum SYSCON_RULEW {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl SYSCON_RULEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYSCON_RULEW::ENUM_NS_NP => 0,
            SYSCON_RULEW::ENUM_NS_P => 1,
            SYSCON_RULEW::ENUM_S_NP => 2,
            SYSCON_RULEW::ENUM_S_P => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYSCON_RULEW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCON_RULEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYSCON_RULEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(SYSCON_RULEW::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(SYSCON_RULEW::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(SYSCON_RULEW::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(SYSCON_RULEW::ENUM_S_P)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IOCON_RULE`"]
pub enum IOCON_RULEW {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl IOCON_RULEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IOCON_RULEW::ENUM_NS_NP => 0,
            IOCON_RULEW::ENUM_NS_P => 1,
            IOCON_RULEW::ENUM_S_NP => 2,
            IOCON_RULEW::ENUM_S_P => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IOCON_RULEW<'a> {
    w: &'a mut W,
}
impl<'a> _IOCON_RULEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOCON_RULEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(IOCON_RULEW::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(IOCON_RULEW::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(IOCON_RULEW::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(IOCON_RULEW::ENUM_S_P)
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
#[doc = "Values that can be written to the field `GINT0_RULE`"]
pub enum GINT0_RULEW {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl GINT0_RULEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GINT0_RULEW::ENUM_NS_NP => 0,
            GINT0_RULEW::ENUM_NS_P => 1,
            GINT0_RULEW::ENUM_S_NP => 2,
            GINT0_RULEW::ENUM_S_P => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GINT0_RULEW<'a> {
    w: &'a mut W,
}
impl<'a> _GINT0_RULEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GINT0_RULEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(GINT0_RULEW::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(GINT0_RULEW::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(GINT0_RULEW::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(GINT0_RULEW::ENUM_S_P)
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
#[doc = "Values that can be written to the field `GINT1_RULE`"]
pub enum GINT1_RULEW {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl GINT1_RULEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GINT1_RULEW::ENUM_NS_NP => 0,
            GINT1_RULEW::ENUM_NS_P => 1,
            GINT1_RULEW::ENUM_S_NP => 2,
            GINT1_RULEW::ENUM_S_P => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GINT1_RULEW<'a> {
    w: &'a mut W,
}
impl<'a> _GINT1_RULEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GINT1_RULEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(GINT1_RULEW::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(GINT1_RULEW::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(GINT1_RULEW::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(GINT1_RULEW::ENUM_S_P)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PINT_RULE`"]
pub enum PINT_RULEW {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl PINT_RULEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PINT_RULEW::ENUM_NS_NP => 0,
            PINT_RULEW::ENUM_NS_P => 1,
            PINT_RULEW::ENUM_S_NP => 2,
            PINT_RULEW::ENUM_S_P => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PINT_RULEW<'a> {
    w: &'a mut W,
}
impl<'a> _PINT_RULEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PINT_RULEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(PINT_RULEW::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(PINT_RULEW::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(PINT_RULEW::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(PINT_RULEW::ENUM_S_P)
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
#[doc = "Values that can be written to the field `SEC_PINT_RULE`"]
pub enum SEC_PINT_RULEW {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl SEC_PINT_RULEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SEC_PINT_RULEW::ENUM_NS_NP => 0,
            SEC_PINT_RULEW::ENUM_NS_P => 1,
            SEC_PINT_RULEW::ENUM_S_NP => 2,
            SEC_PINT_RULEW::ENUM_S_P => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEC_PINT_RULEW<'a> {
    w: &'a mut W,
}
impl<'a> _SEC_PINT_RULEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEC_PINT_RULEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(SEC_PINT_RULEW::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(SEC_PINT_RULEW::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(SEC_PINT_RULEW::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(SEC_PINT_RULEW::ENUM_S_P)
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
#[doc = "Values that can be written to the field `PMUX_RULE`"]
pub enum PMUX_RULEW {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl PMUX_RULEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PMUX_RULEW::ENUM_NS_NP => 0,
            PMUX_RULEW::ENUM_NS_P => 1,
            PMUX_RULEW::ENUM_S_NP => 2,
            PMUX_RULEW::ENUM_S_P => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PMUX_RULEW<'a> {
    w: &'a mut W,
}
impl<'a> _PMUX_RULEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PMUX_RULEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(PMUX_RULEW::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(PMUX_RULEW::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(PMUX_RULEW::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(PMUX_RULEW::ENUM_S_P)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - System Configuration"]
    #[inline]
    pub fn syscon_rule(&self) -> SYSCON_RULER {
        SYSCON_RULER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - I/O Configuration"]
    #[inline]
    pub fn iocon_rule(&self) -> IOCON_RULER {
        IOCON_RULER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - GPIO input Interrupt 0"]
    #[inline]
    pub fn gint0_rule(&self) -> GINT0_RULER {
        GINT0_RULER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - GPIO input Interrupt 1"]
    #[inline]
    pub fn gint1_rule(&self) -> GINT1_RULER {
        GINT1_RULER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Pin Interrupt and Pattern match"]
    #[inline]
    pub fn pint_rule(&self) -> PINT_RULER {
        PINT_RULER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Secure Pin Interrupt and Pattern match"]
    #[inline]
    pub fn sec_pint_rule(&self) -> SEC_PINT_RULER {
        SEC_PINT_RULER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - Peripherals mux"]
    #[inline]
    pub fn pmux_rule(&self) -> PMUX_RULER {
        PMUX_RULER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bits 0:1 - System Configuration"]
    #[inline]
    pub fn syscon_rule(&mut self) -> _SYSCON_RULEW {
        _SYSCON_RULEW { w: self }
    }
    #[doc = "Bits 4:5 - I/O Configuration"]
    #[inline]
    pub fn iocon_rule(&mut self) -> _IOCON_RULEW {
        _IOCON_RULEW { w: self }
    }
    #[doc = "Bits 8:9 - GPIO input Interrupt 0"]
    #[inline]
    pub fn gint0_rule(&mut self) -> _GINT0_RULEW {
        _GINT0_RULEW { w: self }
    }
    #[doc = "Bits 12:13 - GPIO input Interrupt 1"]
    #[inline]
    pub fn gint1_rule(&mut self) -> _GINT1_RULEW {
        _GINT1_RULEW { w: self }
    }
    #[doc = "Bits 16:17 - Pin Interrupt and Pattern match"]
    #[inline]
    pub fn pint_rule(&mut self) -> _PINT_RULEW {
        _PINT_RULEW { w: self }
    }
    #[doc = "Bits 20:21 - Secure Pin Interrupt and Pattern match"]
    #[inline]
    pub fn sec_pint_rule(&mut self) -> _SEC_PINT_RULEW {
        _SEC_PINT_RULEW { w: self }
    }
    #[doc = "Bits 24:25 - Peripherals mux"]
    #[inline]
    pub fn pmux_rule(&mut self) -> _PMUX_RULEW {
        _PMUX_RULEW { w: self }
    }
}
