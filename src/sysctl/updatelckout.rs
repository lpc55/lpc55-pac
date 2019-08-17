#[doc = "Reader of register UPDATELCKOUT"]
pub type R = crate::R<u32, super::UPDATELCKOUT>;
#[doc = "Writer for register UPDATELCKOUT"]
pub type W = crate::W<u32, super::UPDATELCKOUT>;
#[doc = "Register UPDATELCKOUT `reset()`'s with value 0"]
impl crate::ResetValue for super::UPDATELCKOUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `UPDATELCKOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPDATELCKOUT_A {
    #[doc = "Normal Mode. Can be written to."]
    NORMAL_MODE,
    #[doc = "Protected Mode. Cannot be written to."]
    PROTECTED_MODE,
}
impl From<UPDATELCKOUT_A> for bool {
    #[inline(always)]
    fn from(variant: UPDATELCKOUT_A) -> Self {
        match variant {
            UPDATELCKOUT_A::NORMAL_MODE => false,
            UPDATELCKOUT_A::PROTECTED_MODE => true,
        }
    }
}
#[doc = "Reader of field `UPDATELCKOUT`"]
pub type UPDATELCKOUT_R = crate::R<bool, UPDATELCKOUT_A>;
impl UPDATELCKOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPDATELCKOUT_A {
        match self.bits {
            false => UPDATELCKOUT_A::NORMAL_MODE,
            true => UPDATELCKOUT_A::PROTECTED_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_MODE`"]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == UPDATELCKOUT_A::NORMAL_MODE
    }
    #[doc = "Checks if the value of the field is `PROTECTED_MODE`"]
    #[inline(always)]
    pub fn is_protected_mode(&self) -> bool {
        *self == UPDATELCKOUT_A::PROTECTED_MODE
    }
}
#[doc = "Write proxy for field `UPDATELCKOUT`"]
pub struct UPDATELCKOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDATELCKOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UPDATELCKOUT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal Mode. Can be written to."]
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut W {
        self.variant(UPDATELCKOUT_A::NORMAL_MODE)
    }
    #[doc = "Protected Mode. Cannot be written to."]
    #[inline(always)]
    pub fn protected_mode(self) -> &'a mut W {
        self.variant(UPDATELCKOUT_A::PROTECTED_MODE)
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
impl R {
    #[doc = "Bit 0 - All Registers"]
    #[inline(always)]
    pub fn updatelckout(&self) -> UPDATELCKOUT_R {
        UPDATELCKOUT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - All Registers"]
    #[inline(always)]
    pub fn updatelckout(&mut self) -> UPDATELCKOUT_W {
        UPDATELCKOUT_W { w: self }
    }
}
