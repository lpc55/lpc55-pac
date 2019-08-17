#[doc = "Reader of register FLASHBANKENABLE"]
pub type R = crate::R<u32, super::FLASHBANKENABLE>;
#[doc = "Writer for register FLASHBANKENABLE"]
pub type W = crate::W<u32, super::FLASHBANKENABLE>;
#[doc = "Register FLASHBANKENABLE `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASHBANKENABLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `BANK0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BANK0_A {
    #[doc = "Flash BANK0 checker is enabled (all Flash pages inside this bank cannot be erased nor programmed)."]
    ENABLE,
    #[doc = "1010: Flash BANK0 checker is disabled (all Flash pages inside this bank can be erased and programmed)."]
    DISABLE,
}
impl From<BANK0_A> for u8 {
    #[inline(always)]
    fn from(variant: BANK0_A) -> Self {
        match variant {
            BANK0_A::ENABLE => 0,
            BANK0_A::DISABLE => 10,
        }
    }
}
#[doc = "Reader of field `BANK0`"]
pub type BANK0_R = crate::R<u8, BANK0_A>;
impl BANK0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BANK0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BANK0_A::ENABLE),
            10 => Val(BANK0_A::DISABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BANK0_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BANK0_A::DISABLE
    }
}
#[doc = "Write proxy for field `BANK0`"]
pub struct BANK0_W<'a> {
    w: &'a mut W,
}
impl<'a> BANK0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BANK0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Flash BANK0 checker is enabled (all Flash pages inside this bank cannot be erased nor programmed)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BANK0_A::ENABLE)
    }
    #[doc = "1010: Flash BANK0 checker is disabled (all Flash pages inside this bank can be erased and programmed)."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BANK0_A::DISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Possible values of the field `BANK1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BANK1_A {
    #[doc = "Flash BANK1 checker is enabled (all Flash pages inside this bank cannot be erased nor programmed)."]
    ENABLE,
    #[doc = "1010: Flash BANK1 checker is disabled (all Flash pages inside this bank can be erased and programmed)."]
    DISABLE,
}
impl From<BANK1_A> for u8 {
    #[inline(always)]
    fn from(variant: BANK1_A) -> Self {
        match variant {
            BANK1_A::ENABLE => 0,
            BANK1_A::DISABLE => 10,
        }
    }
}
#[doc = "Reader of field `BANK1`"]
pub type BANK1_R = crate::R<u8, BANK1_A>;
impl BANK1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BANK1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BANK1_A::ENABLE),
            10 => Val(BANK1_A::DISABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BANK1_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BANK1_A::DISABLE
    }
}
#[doc = "Write proxy for field `BANK1`"]
pub struct BANK1_W<'a> {
    w: &'a mut W,
}
impl<'a> BANK1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BANK1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Flash BANK1 checker is enabled (all Flash pages inside this bank cannot be erased nor programmed)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BANK1_A::ENABLE)
    }
    #[doc = "1010: Flash BANK1 checker is disabled (all Flash pages inside this bank can be erased and programmed)."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BANK1_A::DISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `BANK2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BANK2_A {
    #[doc = "Flash BANK2 checker is enabled (all Flash pages inside this bank cannot be erased nor programmed)."]
    ENABLE,
    #[doc = "1010: Flash BANK2 checker is disabled (all Flash pages inside this bank can be erased and programmed)."]
    DISABLE,
}
impl From<BANK2_A> for u8 {
    #[inline(always)]
    fn from(variant: BANK2_A) -> Self {
        match variant {
            BANK2_A::ENABLE => 0,
            BANK2_A::DISABLE => 10,
        }
    }
}
#[doc = "Reader of field `BANK2`"]
pub type BANK2_R = crate::R<u8, BANK2_A>;
impl BANK2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BANK2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BANK2_A::ENABLE),
            10 => Val(BANK2_A::DISABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BANK2_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BANK2_A::DISABLE
    }
}
#[doc = "Write proxy for field `BANK2`"]
pub struct BANK2_W<'a> {
    w: &'a mut W,
}
impl<'a> BANK2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BANK2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Flash BANK2 checker is enabled (all Flash pages inside this bank cannot be erased nor programmed)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BANK2_A::ENABLE)
    }
    #[doc = "1010: Flash BANK2 checker is disabled (all Flash pages inside this bank can be erased and programmed)."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BANK2_A::DISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Flash Bank0 control."]
    #[inline(always)]
    pub fn bank0(&self) -> BANK0_R {
        BANK0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Flash Bank1 control."]
    #[inline(always)]
    pub fn bank1(&self) -> BANK1_R {
        BANK1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Flash Bank2 control."]
    #[inline(always)]
    pub fn bank2(&self) -> BANK2_R {
        BANK2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Flash Bank0 control."]
    #[inline(always)]
    pub fn bank0(&mut self) -> BANK0_W {
        BANK0_W { w: self }
    }
    #[doc = "Bits 4:7 - Flash Bank1 control."]
    #[inline(always)]
    pub fn bank1(&mut self) -> BANK1_W {
        BANK1_W { w: self }
    }
    #[doc = "Bits 8:11 - Flash Bank2 control."]
    #[inline(always)]
    pub fn bank2(&mut self) -> BANK2_W {
        BANK2_W { w: self }
    }
}
