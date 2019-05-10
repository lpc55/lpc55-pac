#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SEC_CTRL_AHB2_1_SLAVE_RULE {
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
#[doc = "Possible values of the field `GPIO1_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO1_RULER {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl GPIO1_RULER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO1_RULER::ENUM_NS_NP => 0,
            GPIO1_RULER::ENUM_NS_P => 1,
            GPIO1_RULER::ENUM_S_NP => 2,
            GPIO1_RULER::ENUM_S_P => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO1_RULER {
        match value {
            0 => GPIO1_RULER::ENUM_NS_NP,
            1 => GPIO1_RULER::ENUM_NS_P,
            2 => GPIO1_RULER::ENUM_S_NP,
            3 => GPIO1_RULER::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == GPIO1_RULER::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == GPIO1_RULER::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline]
    pub fn is_enum_s_np(&self) -> bool {
        *self == GPIO1_RULER::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline]
    pub fn is_enum_s_p(&self) -> bool {
        *self == GPIO1_RULER::ENUM_S_P
    }
}
#[doc = "Possible values of the field `AHB_SEC_CTRL_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHB_SEC_CTRL_RULER {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl AHB_SEC_CTRL_RULER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AHB_SEC_CTRL_RULER::ENUM_NS_NP => 0,
            AHB_SEC_CTRL_RULER::ENUM_NS_P => 1,
            AHB_SEC_CTRL_RULER::ENUM_S_NP => 2,
            AHB_SEC_CTRL_RULER::ENUM_S_P => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AHB_SEC_CTRL_RULER {
        match value {
            0 => AHB_SEC_CTRL_RULER::ENUM_NS_NP,
            1 => AHB_SEC_CTRL_RULER::ENUM_NS_P,
            2 => AHB_SEC_CTRL_RULER::ENUM_S_NP,
            3 => AHB_SEC_CTRL_RULER::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == AHB_SEC_CTRL_RULER::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == AHB_SEC_CTRL_RULER::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline]
    pub fn is_enum_s_np(&self) -> bool {
        *self == AHB_SEC_CTRL_RULER::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline]
    pub fn is_enum_s_p(&self) -> bool {
        *self == AHB_SEC_CTRL_RULER::ENUM_S_P
    }
}
#[doc = "Values that can be written to the field `GPIO1_RULE`"]
pub enum GPIO1_RULEW {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl GPIO1_RULEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GPIO1_RULEW::ENUM_NS_NP => 0,
            GPIO1_RULEW::ENUM_NS_P => 1,
            GPIO1_RULEW::ENUM_S_NP => 2,
            GPIO1_RULEW::ENUM_S_P => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO1_RULEW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO1_RULEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO1_RULEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(GPIO1_RULEW::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(GPIO1_RULEW::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(GPIO1_RULEW::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(GPIO1_RULEW::ENUM_S_P)
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
#[doc = "Values that can be written to the field `AHB_SEC_CTRL_RULE`"]
pub enum AHB_SEC_CTRL_RULEW {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl AHB_SEC_CTRL_RULEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AHB_SEC_CTRL_RULEW::ENUM_NS_NP => 0,
            AHB_SEC_CTRL_RULEW::ENUM_NS_P => 1,
            AHB_SEC_CTRL_RULEW::ENUM_S_NP => 2,
            AHB_SEC_CTRL_RULEW::ENUM_S_P => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AHB_SEC_CTRL_RULEW<'a> {
    w: &'a mut W,
}
impl<'a> _AHB_SEC_CTRL_RULEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AHB_SEC_CTRL_RULEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(AHB_SEC_CTRL_RULEW::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(AHB_SEC_CTRL_RULEW::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(AHB_SEC_CTRL_RULEW::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(AHB_SEC_CTRL_RULEW::ENUM_S_P)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Secure High Speed GPIO"]
    #[inline]
    pub fn gpio1_rule(&self) -> GPIO1_RULER {
        GPIO1_RULER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - AHB Secure Controller"]
    #[inline]
    pub fn ahb_sec_ctrl_rule(&self) -> AHB_SEC_CTRL_RULER {
        AHB_SEC_CTRL_RULER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
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
    #[doc = "Bits 0:1 - Secure High Speed GPIO"]
    #[inline]
    pub fn gpio1_rule(&mut self) -> _GPIO1_RULEW {
        _GPIO1_RULEW { w: self }
    }
    #[doc = "Bits 4:5 - AHB Secure Controller"]
    #[inline]
    pub fn ahb_sec_ctrl_rule(&mut self) -> _AHB_SEC_CTRL_RULEW {
        _AHB_SEC_CTRL_RULEW { w: self }
    }
}
