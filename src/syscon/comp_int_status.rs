#[doc = "Reader of register COMP_INT_STATUS"]
pub type R = crate::R<u32, super::COMP_INT_STATUS>;
#[doc = "Writer for register COMP_INT_STATUS"]
pub type W = crate::W<u32, super::COMP_INT_STATUS>;
#[doc = "Register COMP_INT_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::COMP_INT_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATUS_A {
    #[doc = "no interrupt pending."]
    NO_INT,
    #[doc = "interrupt pending."]
    PENDING,
}
impl From<STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: STATUS_A) -> Self {
        match variant {
            STATUS_A::NO_INT => false,
            STATUS_A::PENDING => true,
        }
    }
}
#[doc = "Reader of field `STATUS`"]
pub type STATUS_R = crate::R<bool, STATUS_A>;
impl STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATUS_A {
        match self.bits {
            false => STATUS_A::NO_INT,
            true => STATUS_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INT`"]
    #[inline(always)]
    pub fn is_no_int(&self) -> bool {
        *self == STATUS_A::NO_INT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == STATUS_A::PENDING
    }
}
#[doc = "Possible values of the field `INT_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_STATUS_A {
    #[doc = "no interrupt pending."]
    NO_INT,
    #[doc = "interrupt pending."]
    PENDING,
}
impl From<INT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: INT_STATUS_A) -> Self {
        match variant {
            INT_STATUS_A::NO_INT => false,
            INT_STATUS_A::PENDING => true,
        }
    }
}
#[doc = "Reader of field `INT_STATUS`"]
pub type INT_STATUS_R = crate::R<bool, INT_STATUS_A>;
impl INT_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_STATUS_A {
        match self.bits {
            false => INT_STATUS_A::NO_INT,
            true => INT_STATUS_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INT`"]
    #[inline(always)]
    pub fn is_no_int(&self) -> bool {
        *self == INT_STATUS_A::NO_INT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == INT_STATUS_A::PENDING
    }
}
#[doc = "Possible values of the field `VAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VAL_A {
    #[doc = "P+ is smaller than P-."]
    SMALLER,
    #[doc = "P+ is greater than P-."]
    GREATER,
}
impl From<VAL_A> for bool {
    #[inline(always)]
    fn from(variant: VAL_A) -> Self {
        match variant {
            VAL_A::SMALLER => false,
            VAL_A::GREATER => true,
        }
    }
}
#[doc = "Reader of field `VAL`"]
pub type VAL_R = crate::R<bool, VAL_A>;
impl VAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VAL_A {
        match self.bits {
            false => VAL_A::SMALLER,
            true => VAL_A::GREATER,
        }
    }
    #[doc = "Checks if the value of the field is `SMALLER`"]
    #[inline(always)]
    pub fn is_smaller(&self) -> bool {
        *self == VAL_A::SMALLER
    }
    #[doc = "Checks if the value of the field is `GREATER`"]
    #[inline(always)]
    pub fn is_greater(&self) -> bool {
        *self == VAL_A::GREATER
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt status BEFORE Interrupt Enable."]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt status AFTER Interrupt Enable."]
    #[inline(always)]
    pub fn int_status(&self) -> INT_STATUS_R {
        INT_STATUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - comparator analog output."]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {}
