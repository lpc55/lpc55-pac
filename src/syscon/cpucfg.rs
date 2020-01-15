#[doc = "Reader of register CPUCFG"]
pub type R = crate::R<u32, super::CPUCFG>;
#[doc = "Writer for register CPUCFG"]
pub type W = crate::W<u32, super::CPUCFG>;
#[doc = "Register CPUCFG `reset()`'s with value 0x02"]
impl crate::ResetValue for super::CPUCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Enable CPU1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU1ENABLE_A {
    #[doc = "0: CPU1 is disable (Processor in reset)."]
    DISABLE = 0,
    #[doc = "1: CPU1 is enable."]
    ENABLE = 1,
}
impl From<CPU1ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: CPU1ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPU1ENABLE`"]
pub type CPU1ENABLE_R = crate::R<bool, CPU1ENABLE_A>;
impl CPU1ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPU1ENABLE_A {
        match self.bits {
            false => CPU1ENABLE_A::DISABLE,
            true => CPU1ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CPU1ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CPU1ENABLE_A::ENABLE
    }
}
#[doc = "Write proxy for field `CPU1ENABLE`"]
pub struct CPU1ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU1ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPU1ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CPU1 is disable (Processor in reset)."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CPU1ENABLE_A::DISABLE)
    }
    #[doc = "CPU1 is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CPU1ENABLE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Enable CPU1."]
    #[inline(always)]
    pub fn cpu1enable(&self) -> CPU1ENABLE_R {
        CPU1ENABLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Enable CPU1."]
    #[inline(always)]
    pub fn cpu1enable(&mut self) -> CPU1ENABLE_W {
        CPU1ENABLE_W { w: self }
    }
}
