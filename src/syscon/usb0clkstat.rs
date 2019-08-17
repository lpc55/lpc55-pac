#[doc = "Reader of register USB0CLKSTAT"]
pub type R = crate::R<u32, super::USB0CLKSTAT>;
#[doc = "Writer for register USB0CLKSTAT"]
pub type W = crate::W<u32, super::USB0CLKSTAT>;
#[doc = "Register USB0CLKSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::USB0CLKSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `DEV_NEED_CLKST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEV_NEED_CLKST_A {
    #[doc = "USB0 Device clock is low."]
    LOW,
    #[doc = "USB0 Device clock is high."]
    HIGH,
}
impl From<DEV_NEED_CLKST_A> for bool {
    #[inline(always)]
    fn from(variant: DEV_NEED_CLKST_A) -> Self {
        match variant {
            DEV_NEED_CLKST_A::LOW => false,
            DEV_NEED_CLKST_A::HIGH => true,
        }
    }
}
#[doc = "Reader of field `DEV_NEED_CLKST`"]
pub type DEV_NEED_CLKST_R = crate::R<bool, DEV_NEED_CLKST_A>;
impl DEV_NEED_CLKST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEV_NEED_CLKST_A {
        match self.bits {
            false => DEV_NEED_CLKST_A::LOW,
            true => DEV_NEED_CLKST_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == DEV_NEED_CLKST_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == DEV_NEED_CLKST_A::HIGH
    }
}
#[doc = "Possible values of the field `HOST_NEED_CLKST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HOST_NEED_CLKST_A {
    #[doc = "USB0 Host clock is low."]
    LOW,
    #[doc = "USB0 Host clock is high."]
    HIGH,
}
impl From<HOST_NEED_CLKST_A> for bool {
    #[inline(always)]
    fn from(variant: HOST_NEED_CLKST_A) -> Self {
        match variant {
            HOST_NEED_CLKST_A::LOW => false,
            HOST_NEED_CLKST_A::HIGH => true,
        }
    }
}
#[doc = "Reader of field `HOST_NEED_CLKST`"]
pub type HOST_NEED_CLKST_R = crate::R<bool, HOST_NEED_CLKST_A>;
impl HOST_NEED_CLKST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HOST_NEED_CLKST_A {
        match self.bits {
            false => HOST_NEED_CLKST_A::LOW,
            true => HOST_NEED_CLKST_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == HOST_NEED_CLKST_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == HOST_NEED_CLKST_A::HIGH
    }
}
impl R {
    #[doc = "Bit 0 - USB0 Device USB0_NEEDCLK signal status:."]
    #[inline(always)]
    pub fn dev_need_clkst(&self) -> DEV_NEED_CLKST_R {
        DEV_NEED_CLKST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB0 Host USB0_NEEDCLK signal status:."]
    #[inline(always)]
    pub fn host_need_clkst(&self) -> HOST_NEED_CLKST_R {
        HOST_NEED_CLKST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {}
