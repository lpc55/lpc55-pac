#[doc = "Reader of register XO32M_STATUS"]
pub type R = crate::R<u32, super::XO32M_STATUS>;
#[doc = "Possible values of the field `XO_READY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XO_READY_A {
    #[doc = "XO output frequency is not yet stable."]
    NOT_STABLE,
    #[doc = "XO output frequency is stable."]
    STABLE,
}
impl From<XO_READY_A> for bool {
    #[inline(always)]
    fn from(variant: XO_READY_A) -> Self {
        match variant {
            XO_READY_A::NOT_STABLE => false,
            XO_READY_A::STABLE => true,
        }
    }
}
#[doc = "Reader of field `XO_READY`"]
pub type XO_READY_R = crate::R<bool, XO_READY_A>;
impl XO_READY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XO_READY_A {
        match self.bits {
            false => XO_READY_A::NOT_STABLE,
            true => XO_READY_A::STABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_STABLE`"]
    #[inline(always)]
    pub fn is_not_stable(&self) -> bool {
        *self == XO_READY_A::NOT_STABLE
    }
    #[doc = "Checks if the value of the field is `STABLE`"]
    #[inline(always)]
    pub fn is_stable(&self) -> bool {
        *self == XO_READY_A::STABLE
    }
}
impl R {
    #[doc = "Bit 0 - Indicates XO out frequency statibilty."]
    #[inline(always)]
    pub fn xo_ready(&self) -> XO_READY_R {
        XO_READY_R::new((self.bits & 0x01) != 0)
    }
}
