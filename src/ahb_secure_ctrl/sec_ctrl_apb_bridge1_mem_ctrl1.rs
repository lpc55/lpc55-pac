#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SEC_CTRL_APB_BRIDGE1_MEM_CTRL1 {
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
#[doc = "Possible values of the field `CTIMER2_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIMER2_RULER {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl CTIMER2_RULER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CTIMER2_RULER::ENUM_NS_NP => 0,
            CTIMER2_RULER::ENUM_NS_P => 1,
            CTIMER2_RULER::ENUM_S_NP => 2,
            CTIMER2_RULER::ENUM_S_P => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CTIMER2_RULER {
        match value {
            0 => CTIMER2_RULER::ENUM_NS_NP,
            1 => CTIMER2_RULER::ENUM_NS_P,
            2 => CTIMER2_RULER::ENUM_S_NP,
            3 => CTIMER2_RULER::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == CTIMER2_RULER::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == CTIMER2_RULER::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline]
    pub fn is_enum_s_np(&self) -> bool {
        *self == CTIMER2_RULER::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline]
    pub fn is_enum_s_p(&self) -> bool {
        *self == CTIMER2_RULER::ENUM_S_P
    }
}
#[doc = "Possible values of the field `CTIMER3_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIMER3_RULER {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl CTIMER3_RULER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CTIMER3_RULER::ENUM_NS_NP => 0,
            CTIMER3_RULER::ENUM_NS_P => 1,
            CTIMER3_RULER::ENUM_S_NP => 2,
            CTIMER3_RULER::ENUM_S_P => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CTIMER3_RULER {
        match value {
            0 => CTIMER3_RULER::ENUM_NS_NP,
            1 => CTIMER3_RULER::ENUM_NS_P,
            2 => CTIMER3_RULER::ENUM_S_NP,
            3 => CTIMER3_RULER::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == CTIMER3_RULER::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == CTIMER3_RULER::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline]
    pub fn is_enum_s_np(&self) -> bool {
        *self == CTIMER3_RULER::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline]
    pub fn is_enum_s_p(&self) -> bool {
        *self == CTIMER3_RULER::ENUM_S_P
    }
}
#[doc = "Possible values of the field `CTIMER4_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIMER4_RULER {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl CTIMER4_RULER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CTIMER4_RULER::ENUM_NS_NP => 0,
            CTIMER4_RULER::ENUM_NS_P => 1,
            CTIMER4_RULER::ENUM_S_NP => 2,
            CTIMER4_RULER::ENUM_S_P => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CTIMER4_RULER {
        match value {
            0 => CTIMER4_RULER::ENUM_NS_NP,
            1 => CTIMER4_RULER::ENUM_NS_P,
            2 => CTIMER4_RULER::ENUM_S_NP,
            3 => CTIMER4_RULER::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == CTIMER4_RULER::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == CTIMER4_RULER::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline]
    pub fn is_enum_s_np(&self) -> bool {
        *self == CTIMER4_RULER::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline]
    pub fn is_enum_s_p(&self) -> bool {
        *self == CTIMER4_RULER::ENUM_S_P
    }
}
#[doc = "Possible values of the field `RTC_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_RULER {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl RTC_RULER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RTC_RULER::ENUM_NS_NP => 0,
            RTC_RULER::ENUM_NS_P => 1,
            RTC_RULER::ENUM_S_NP => 2,
            RTC_RULER::ENUM_S_P => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RTC_RULER {
        match value {
            0 => RTC_RULER::ENUM_NS_NP,
            1 => RTC_RULER::ENUM_NS_P,
            2 => RTC_RULER::ENUM_S_NP,
            3 => RTC_RULER::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == RTC_RULER::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == RTC_RULER::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline]
    pub fn is_enum_s_np(&self) -> bool {
        *self == RTC_RULER::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline]
    pub fn is_enum_s_p(&self) -> bool {
        *self == RTC_RULER::ENUM_S_P
    }
}
#[doc = "Possible values of the field `OSEVENT_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSEVENT_RULER {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl OSEVENT_RULER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OSEVENT_RULER::ENUM_NS_NP => 0,
            OSEVENT_RULER::ENUM_NS_P => 1,
            OSEVENT_RULER::ENUM_S_NP => 2,
            OSEVENT_RULER::ENUM_S_P => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OSEVENT_RULER {
        match value {
            0 => OSEVENT_RULER::ENUM_NS_NP,
            1 => OSEVENT_RULER::ENUM_NS_P,
            2 => OSEVENT_RULER::ENUM_S_NP,
            3 => OSEVENT_RULER::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == OSEVENT_RULER::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == OSEVENT_RULER::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline]
    pub fn is_enum_s_np(&self) -> bool {
        *self == OSEVENT_RULER::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline]
    pub fn is_enum_s_p(&self) -> bool {
        *self == OSEVENT_RULER::ENUM_S_P
    }
}
#[doc = "Values that can be written to the field `CTIMER2_RULE`"]
pub enum CTIMER2_RULEW {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl CTIMER2_RULEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CTIMER2_RULEW::ENUM_NS_NP => 0,
            CTIMER2_RULEW::ENUM_NS_P => 1,
            CTIMER2_RULEW::ENUM_S_NP => 2,
            CTIMER2_RULEW::ENUM_S_P => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTIMER2_RULEW<'a> {
    w: &'a mut W,
}
impl<'a> _CTIMER2_RULEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTIMER2_RULEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(CTIMER2_RULEW::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(CTIMER2_RULEW::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(CTIMER2_RULEW::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(CTIMER2_RULEW::ENUM_S_P)
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
#[doc = "Values that can be written to the field `CTIMER3_RULE`"]
pub enum CTIMER3_RULEW {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl CTIMER3_RULEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CTIMER3_RULEW::ENUM_NS_NP => 0,
            CTIMER3_RULEW::ENUM_NS_P => 1,
            CTIMER3_RULEW::ENUM_S_NP => 2,
            CTIMER3_RULEW::ENUM_S_P => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTIMER3_RULEW<'a> {
    w: &'a mut W,
}
impl<'a> _CTIMER3_RULEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTIMER3_RULEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(CTIMER3_RULEW::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(CTIMER3_RULEW::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(CTIMER3_RULEW::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(CTIMER3_RULEW::ENUM_S_P)
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
#[doc = "Values that can be written to the field `CTIMER4_RULE`"]
pub enum CTIMER4_RULEW {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl CTIMER4_RULEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CTIMER4_RULEW::ENUM_NS_NP => 0,
            CTIMER4_RULEW::ENUM_NS_P => 1,
            CTIMER4_RULEW::ENUM_S_NP => 2,
            CTIMER4_RULEW::ENUM_S_P => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTIMER4_RULEW<'a> {
    w: &'a mut W,
}
impl<'a> _CTIMER4_RULEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTIMER4_RULEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(CTIMER4_RULEW::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(CTIMER4_RULEW::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(CTIMER4_RULEW::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(CTIMER4_RULEW::ENUM_S_P)
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
#[doc = "Values that can be written to the field `RTC_RULE`"]
pub enum RTC_RULEW {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl RTC_RULEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RTC_RULEW::ENUM_NS_NP => 0,
            RTC_RULEW::ENUM_NS_P => 1,
            RTC_RULEW::ENUM_S_NP => 2,
            RTC_RULEW::ENUM_S_P => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTC_RULEW<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_RULEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTC_RULEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(RTC_RULEW::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(RTC_RULEW::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(RTC_RULEW::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(RTC_RULEW::ENUM_S_P)
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
#[doc = "Values that can be written to the field `OSEVENT_RULE`"]
pub enum OSEVENT_RULEW {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl OSEVENT_RULEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OSEVENT_RULEW::ENUM_NS_NP => 0,
            OSEVENT_RULEW::ENUM_NS_P => 1,
            OSEVENT_RULEW::ENUM_S_NP => 2,
            OSEVENT_RULEW::ENUM_S_P => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSEVENT_RULEW<'a> {
    w: &'a mut W,
}
impl<'a> _OSEVENT_RULEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSEVENT_RULEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(OSEVENT_RULEW::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(OSEVENT_RULEW::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(OSEVENT_RULEW::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(OSEVENT_RULEW::ENUM_S_P)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Standard counter/Timer 2"]
    #[inline]
    pub fn ctimer2_rule(&self) -> CTIMER2_RULER {
        CTIMER2_RULER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Standard counter/Timer 3"]
    #[inline]
    pub fn ctimer3_rule(&self) -> CTIMER3_RULER {
        CTIMER3_RULER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Standard counter/Timer 4"]
    #[inline]
    pub fn ctimer4_rule(&self) -> CTIMER4_RULER {
        CTIMER4_RULER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Real Time Counter"]
    #[inline]
    pub fn rtc_rule(&self) -> RTC_RULER {
        RTC_RULER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - OS Event Timer"]
    #[inline]
    pub fn osevent_rule(&self) -> OSEVENT_RULER {
        OSEVENT_RULER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
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
    #[doc = "Bits 0:1 - Standard counter/Timer 2"]
    #[inline]
    pub fn ctimer2_rule(&mut self) -> _CTIMER2_RULEW {
        _CTIMER2_RULEW { w: self }
    }
    #[doc = "Bits 4:5 - Standard counter/Timer 3"]
    #[inline]
    pub fn ctimer3_rule(&mut self) -> _CTIMER3_RULEW {
        _CTIMER3_RULEW { w: self }
    }
    #[doc = "Bits 8:9 - Standard counter/Timer 4"]
    #[inline]
    pub fn ctimer4_rule(&mut self) -> _CTIMER4_RULEW {
        _CTIMER4_RULEW { w: self }
    }
    #[doc = "Bits 16:17 - Real Time Counter"]
    #[inline]
    pub fn rtc_rule(&mut self) -> _RTC_RULEW {
        _RTC_RULEW { w: self }
    }
    #[doc = "Bits 20:21 - OS Event Timer"]
    #[inline]
    pub fn osevent_rule(&mut self) -> _OSEVENT_RULEW {
        _OSEVENT_RULEW { w: self }
    }
}
