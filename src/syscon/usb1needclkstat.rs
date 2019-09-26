#[doc = "Reader of register USB1NEEDCLKSTAT"]
pub type R = crate::R<u32, super::USB1NEEDCLKSTAT>;
#[doc = "Writer for register USB1NEEDCLKSTAT"]
pub type W = crate::W<u32, super::USB1NEEDCLKSTAT>;
#[doc = "Register USB1NEEDCLKSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::USB1NEEDCLKSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `DEV_NEEDCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEV_NEEDCLK_A {
    #[doc = "DEV_NEEDCLK is low."]
    LOW,
    #[doc = "DEV_NEEDCLK is high."]
    HIGH,
}
impl From<DEV_NEEDCLK_A> for bool {
    #[inline(always)]
    fn from(variant: DEV_NEEDCLK_A) -> Self {
        match variant {
            DEV_NEEDCLK_A::LOW => false,
            DEV_NEEDCLK_A::HIGH => true,
        }
    }
}
#[doc = "Reader of field `DEV_NEEDCLK`"]
pub type DEV_NEEDCLK_R = crate::R<bool, DEV_NEEDCLK_A>;
impl DEV_NEEDCLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEV_NEEDCLK_A {
        match self.bits {
            false => DEV_NEEDCLK_A::LOW,
            true => DEV_NEEDCLK_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == DEV_NEEDCLK_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == DEV_NEEDCLK_A::HIGH
    }
}
#[doc = "Possible values of the field `HOST_NEEDCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HOST_NEEDCLK_A {
    #[doc = "HOST_NEEDCLK is low."]
    LOW,
    #[doc = "HOST_NEEDCLK is high."]
    HIGH,
}
impl From<HOST_NEEDCLK_A> for bool {
    #[inline(always)]
    fn from(variant: HOST_NEEDCLK_A) -> Self {
        match variant {
            HOST_NEEDCLK_A::LOW => false,
            HOST_NEEDCLK_A::HIGH => true,
        }
    }
}
#[doc = "Reader of field `HOST_NEEDCLK`"]
pub type HOST_NEEDCLK_R = crate::R<bool, HOST_NEEDCLK_A>;
impl HOST_NEEDCLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HOST_NEEDCLK_A {
        match self.bits {
            false => HOST_NEEDCLK_A::LOW,
            true => HOST_NEEDCLK_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == HOST_NEEDCLK_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == HOST_NEEDCLK_A::HIGH
    }
}
impl R {
    #[doc = "Bit 0 - USB1 Device need_clock signal status:."]
    #[inline(always)]
    pub fn dev_needclk(&self) -> DEV_NEEDCLK_R {
        DEV_NEEDCLK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB1 Host need_clock signal status:."]
    #[inline(always)]
    pub fn host_needclk(&self) -> HOST_NEEDCLK_R {
        HOST_NEEDCLK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {}
