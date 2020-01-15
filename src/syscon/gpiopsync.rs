#[doc = "Reader of register GPIOPSYNC"]
pub type R = crate::R<u32, super::GPIOPSYNC>;
#[doc = "Writer for register GPIOPSYNC"]
pub type W = crate::W<u32, super::GPIOPSYNC>;
#[doc = "Register GPIOPSYNC `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIOPSYNC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable bypass of the first stage of synchonization inside GPIO_INT module.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSYNC_A {
    #[doc = "0: use the first stage of synchonization inside GPIO_INT module."]
    USED = 0,
    #[doc = "1: bypass of the first stage of synchonization inside GPIO_INT module."]
    BYPASS = 1,
}
impl From<PSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: PSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PSYNC`"]
pub type PSYNC_R = crate::R<bool, PSYNC_A>;
impl PSYNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSYNC_A {
        match self.bits {
            false => PSYNC_A::USED,
            true => PSYNC_A::BYPASS,
        }
    }
    #[doc = "Checks if the value of the field is `USED`"]
    #[inline(always)]
    pub fn is_used(&self) -> bool {
        *self == PSYNC_A::USED
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == PSYNC_A::BYPASS
    }
}
#[doc = "Write proxy for field `PSYNC`"]
pub struct PSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> PSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSYNC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "use the first stage of synchonization inside GPIO_INT module."]
    #[inline(always)]
    pub fn used(self) -> &'a mut W {
        self.variant(PSYNC_A::USED)
    }
    #[doc = "bypass of the first stage of synchonization inside GPIO_INT module."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(PSYNC_A::BYPASS)
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
    #[doc = "Bit 0 - Enable bypass of the first stage of synchonization inside GPIO_INT module."]
    #[inline(always)]
    pub fn psync(&self) -> PSYNC_R {
        PSYNC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable bypass of the first stage of synchonization inside GPIO_INT module."]
    #[inline(always)]
    pub fn psync(&mut self) -> PSYNC_W {
        PSYNC_W { w: self }
    }
}
