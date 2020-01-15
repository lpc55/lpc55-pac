#[doc = "Reader of register PLL1STAT"]
pub type R = crate::R<u32, super::PLL1STAT>;
#[doc = "Writer for register PLL1STAT"]
pub type W = crate::W<u32, super::PLL1STAT>;
#[doc = "Register PLL1STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::PLL1STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LOCK`"]
pub type LOCK_R = crate::R<bool, bool>;
#[doc = "Reader of field `PREDIVACK`"]
pub type PREDIVACK_R = crate::R<bool, bool>;
#[doc = "Reader of field `FEEDDIVACK`"]
pub type FEEDDIVACK_R = crate::R<bool, bool>;
#[doc = "Reader of field `POSTDIVACK`"]
pub type POSTDIVACK_R = crate::R<bool, bool>;
#[doc = "Reader of field `FRMDET`"]
pub type FRMDET_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - lock detector output (active high) Warning: The lock signal is only reliable between fref\\[2\\]
:100 kHz to 20 MHz."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - pre-divider ratio change acknowledge."]
    #[inline(always)]
    pub fn predivack(&self) -> PREDIVACK_R {
        PREDIVACK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - feedback divider ratio change acknowledge."]
    #[inline(always)]
    pub fn feeddivack(&self) -> FEEDDIVACK_R {
        FEEDDIVACK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - post-divider ratio change acknowledge."]
    #[inline(always)]
    pub fn postdivack(&self) -> POSTDIVACK_R {
        POSTDIVACK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - free running detector output (active high)."]
    #[inline(always)]
    pub fn frmdet(&self) -> FRMDET_R {
        FRMDET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {}
