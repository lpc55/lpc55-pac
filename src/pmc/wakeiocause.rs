#[doc = "Reader of register WAKEIOCAUSE"]
pub type R = crate::R<u32, super::WAKEIOCAUSE>;
#[doc = "Writer for register WAKEIOCAUSE"]
pub type W = crate::W<u32, super::WAKEIOCAUSE>;
#[doc = "Register WAKEIOCAUSE `reset()`'s with value 0"]
impl crate::ResetValue for super::WAKEIOCAUSE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `WAKEUP0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP0_A {
    #[doc = "Last wake up from Deep Power down mode was NOT triggred by wake up I/O 0."]
    NOEVENT,
    #[doc = "Last wake up from Deep Power down mode was triggred by wake up I/O 0."]
    EVENT,
}
impl From<WAKEUP0_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEUP0_A) -> Self {
        match variant {
            WAKEUP0_A::NOEVENT => false,
            WAKEUP0_A::EVENT => true,
        }
    }
}
#[doc = "Reader of field `WAKEUP0`"]
pub type WAKEUP0_R = crate::R<bool, WAKEUP0_A>;
impl WAKEUP0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEUP0_A {
        match self.bits {
            false => WAKEUP0_A::NOEVENT,
            true => WAKEUP0_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_noevent(&self) -> bool {
        *self == WAKEUP0_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == WAKEUP0_A::EVENT
    }
}
#[doc = "Possible values of the field `WAKEUP1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP1_A {
    #[doc = "Last wake up from Deep Power down mode was NOT triggred by wake up I/O 1."]
    NOEVENT,
    #[doc = "Last wake up from Deep Power down mode was triggred by wake up I/O 1."]
    EVENT,
}
impl From<WAKEUP1_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEUP1_A) -> Self {
        match variant {
            WAKEUP1_A::NOEVENT => false,
            WAKEUP1_A::EVENT => true,
        }
    }
}
#[doc = "Reader of field `WAKEUP1`"]
pub type WAKEUP1_R = crate::R<bool, WAKEUP1_A>;
impl WAKEUP1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEUP1_A {
        match self.bits {
            false => WAKEUP1_A::NOEVENT,
            true => WAKEUP1_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_noevent(&self) -> bool {
        *self == WAKEUP1_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == WAKEUP1_A::EVENT
    }
}
#[doc = "Write proxy for field `WAKEUP1`"]
pub struct WAKEUP1_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKEUP1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Last wake up from Deep Power down mode was NOT triggred by wake up I/O 1."]
    #[inline(always)]
    pub fn noevent(self) -> &'a mut W {
        self.variant(WAKEUP1_A::NOEVENT)
    }
    #[doc = "Last wake up from Deep Power down mode was triggred by wake up I/O 1."]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(WAKEUP1_A::EVENT)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Possible values of the field `WAKEUP2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP2_A {
    #[doc = "Last wake up from Deep Power down mode was NOT triggred by wake up I/O 2."]
    NOEVENT,
    #[doc = "Last wake up from Deep Power down mode was triggred by wake up I/O 2."]
    EVENT,
}
impl From<WAKEUP2_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEUP2_A) -> Self {
        match variant {
            WAKEUP2_A::NOEVENT => false,
            WAKEUP2_A::EVENT => true,
        }
    }
}
#[doc = "Reader of field `WAKEUP2`"]
pub type WAKEUP2_R = crate::R<bool, WAKEUP2_A>;
impl WAKEUP2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEUP2_A {
        match self.bits {
            false => WAKEUP2_A::NOEVENT,
            true => WAKEUP2_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_noevent(&self) -> bool {
        *self == WAKEUP2_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == WAKEUP2_A::EVENT
    }
}
#[doc = "Write proxy for field `WAKEUP2`"]
pub struct WAKEUP2_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKEUP2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Last wake up from Deep Power down mode was NOT triggred by wake up I/O 2."]
    #[inline(always)]
    pub fn noevent(self) -> &'a mut W {
        self.variant(WAKEUP2_A::NOEVENT)
    }
    #[doc = "Last wake up from Deep Power down mode was triggred by wake up I/O 2."]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(WAKEUP2_A::EVENT)
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
#[doc = "Possible values of the field `WAKEUP3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP3_A {
    #[doc = "Last wake up from Deep Power down mode was NOT triggred by wake up I/O 3."]
    NOEVENT,
    #[doc = "Last wake up from Deep Power down mode was triggred by wake up I/O 3."]
    EVENT,
}
impl From<WAKEUP3_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEUP3_A) -> Self {
        match variant {
            WAKEUP3_A::NOEVENT => false,
            WAKEUP3_A::EVENT => true,
        }
    }
}
#[doc = "Reader of field `WAKEUP3`"]
pub type WAKEUP3_R = crate::R<bool, WAKEUP3_A>;
impl WAKEUP3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEUP3_A {
        match self.bits {
            false => WAKEUP3_A::NOEVENT,
            true => WAKEUP3_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_noevent(&self) -> bool {
        *self == WAKEUP3_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == WAKEUP3_A::EVENT
    }
}
#[doc = "Write proxy for field `WAKEUP3`"]
pub struct WAKEUP3_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKEUP3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Last wake up from Deep Power down mode was NOT triggred by wake up I/O 3."]
    #[inline(always)]
    pub fn noevent(self) -> &'a mut W {
        self.variant(WAKEUP3_A::NOEVENT)
    }
    #[doc = "Last wake up from Deep Power down mode was triggred by wake up I/O 3."]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(WAKEUP3_A::EVENT)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Allows to identify Wake up I/O 0 as the wake-up source from Deep Power Down mode."]
    #[inline(always)]
    pub fn wakeup0(&self) -> WAKEUP0_R {
        WAKEUP0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Allows to identify Wake up I/O 1 as the wake-up source from Deep Power Down mode."]
    #[inline(always)]
    pub fn wakeup1(&self) -> WAKEUP1_R {
        WAKEUP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Allows to identify Wake up I/O 2 as the wake-up source from Deep Power Down mode."]
    #[inline(always)]
    pub fn wakeup2(&self) -> WAKEUP2_R {
        WAKEUP2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Allows to identify Wake up I/O 3 as the wake-up source from Deep Power Down mode."]
    #[inline(always)]
    pub fn wakeup3(&self) -> WAKEUP3_R {
        WAKEUP3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Allows to identify Wake up I/O 1 as the wake-up source from Deep Power Down mode."]
    #[inline(always)]
    pub fn wakeup1(&mut self) -> WAKEUP1_W {
        WAKEUP1_W { w: self }
    }
    #[doc = "Bit 2 - Allows to identify Wake up I/O 2 as the wake-up source from Deep Power Down mode."]
    #[inline(always)]
    pub fn wakeup2(&mut self) -> WAKEUP2_W {
        WAKEUP2_W { w: self }
    }
    #[doc = "Bit 3 - Allows to identify Wake up I/O 3 as the wake-up source from Deep Power Down mode."]
    #[inline(always)]
    pub fn wakeup3(&mut self) -> WAKEUP3_W {
        WAKEUP3_W { w: self }
    }
}
