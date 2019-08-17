#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Writer for register STATUS"]
pub type W = crate::W<u32, super::STATUS>;
#[doc = "Register STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `DONE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DONE_A {
    #[doc = "Busy or just cleared"]
    BUSY,
    #[doc = "Completed last operation"]
    COMPLETED,
}
impl From<DONE_A> for bool {
    #[inline(always)]
    fn from(variant: DONE_A) -> Self {
        match variant {
            DONE_A::BUSY => false,
            DONE_A::COMPLETED => true,
        }
    }
}
#[doc = "Reader of field `DONE`"]
pub type DONE_R = crate::R<bool, DONE_A>;
impl DONE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DONE_A {
        match self.bits {
            false => DONE_A::BUSY,
            true => DONE_A::COMPLETED,
        }
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == DONE_A::BUSY
    }
    #[doc = "Checks if the value of the field is `COMPLETED`"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == DONE_A::COMPLETED
    }
}
#[doc = "Write proxy for field `DONE`"]
pub struct DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DONE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Busy or just cleared"]
    #[inline(always)]
    pub fn busy(self) -> &'a mut W {
        self.variant(DONE_A::BUSY)
    }
    #[doc = "Completed last operation"]
    #[inline(always)]
    pub fn completed(self) -> &'a mut W {
        self.variant(DONE_A::COMPLETED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Possible values of the field `CARRY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARRY_A {
    #[doc = "Carry was 0 or no carry"]
    NO_CARRY,
    #[doc = "Carry was 1"]
    CARRY,
}
impl From<CARRY_A> for bool {
    #[inline(always)]
    fn from(variant: CARRY_A) -> Self {
        match variant {
            CARRY_A::NO_CARRY => false,
            CARRY_A::CARRY => true,
        }
    }
}
#[doc = "Reader of field `CARRY`"]
pub type CARRY_R = crate::R<bool, CARRY_A>;
impl CARRY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CARRY_A {
        match self.bits {
            false => CARRY_A::NO_CARRY,
            true => CARRY_A::CARRY,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CARRY`"]
    #[inline(always)]
    pub fn is_no_carry(&self) -> bool {
        *self == CARRY_A::NO_CARRY
    }
    #[doc = "Checks if the value of the field is `CARRY`"]
    #[inline(always)]
    pub fn is_carry(&self) -> bool {
        *self == CARRY_A::CARRY
    }
}
#[doc = "Possible values of the field `BUSY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "Not busy - is idle"]
    IDLE,
    #[doc = "Is busy"]
    BUSY,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        match variant {
            BUSY_A::IDLE => false,
            BUSY_A::BUSY => true,
        }
    }
}
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, BUSY_A>;
impl BUSY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::IDLE,
            true => BUSY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BUSY_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY_A::BUSY
    }
}
impl R {
    #[doc = "Bit 0 - Indicates if the accelerator has finished an operation. Write 1 to clear, or write CTRL1 to clear."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Last carry value if operation produced a carry bit"]
    #[inline(always)]
    pub fn carry(&self) -> CARRY_R {
        CARRY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Indicates if the accelerator is busy performing an operation"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates if the accelerator has finished an operation. Write 1 to clear, or write CTRL1 to clear."]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W {
        DONE_W { w: self }
    }
}
