#[doc = "Reader of register ERRSTAT"]
pub type R = crate::R<u32, super::ERRSTAT>;
#[doc = "Writer for register ERRSTAT"]
pub type W = crate::W<u32, super::ERRSTAT>;
#[doc = "Register ERRSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::ERRSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OVERFLOW`"]
pub type OVERFLOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVERFLOW`"]
pub struct OVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERFLOW_W<'a> {
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
#[doc = "Reader of field `NAN`"]
pub type NAN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NAN`"]
pub struct NAN_W<'a> {
    w: &'a mut W,
}
impl<'a> NAN_W<'a> {
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
#[doc = "Reader of field `FIXEDOVERFLOW`"]
pub type FIXEDOVERFLOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIXEDOVERFLOW`"]
pub struct FIXEDOVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> FIXEDOVERFLOW_W<'a> {
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
#[doc = "Reader of field `UNDERFLOW`"]
pub type UNDERFLOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UNDERFLOW`"]
pub struct UNDERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> UNDERFLOW_W<'a> {
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
#[doc = "Reader of field `BUSERROR`"]
pub type BUSERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUSERROR`"]
pub struct BUSERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSERROR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - overflow"]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - nan"]
    #[inline(always)]
    pub fn nan(&self) -> NAN_R {
        NAN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - fixed_pt_overflow"]
    #[inline(always)]
    pub fn fixedoverflow(&self) -> FIXEDOVERFLOW_R {
        FIXEDOVERFLOW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - underflow"]
    #[inline(always)]
    pub fn underflow(&self) -> UNDERFLOW_R {
        UNDERFLOW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - bus_error"]
    #[inline(always)]
    pub fn buserror(&self) -> BUSERROR_R {
        BUSERROR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - overflow"]
    #[inline(always)]
    pub fn overflow(&mut self) -> OVERFLOW_W {
        OVERFLOW_W { w: self }
    }
    #[doc = "Bit 1 - nan"]
    #[inline(always)]
    pub fn nan(&mut self) -> NAN_W {
        NAN_W { w: self }
    }
    #[doc = "Bit 2 - fixed_pt_overflow"]
    #[inline(always)]
    pub fn fixedoverflow(&mut self) -> FIXEDOVERFLOW_W {
        FIXEDOVERFLOW_W { w: self }
    }
    #[doc = "Bit 3 - underflow"]
    #[inline(always)]
    pub fn underflow(&mut self) -> UNDERFLOW_W {
        UNDERFLOW_W { w: self }
    }
    #[doc = "Bit 4 - bus_error"]
    #[inline(always)]
    pub fn buserror(&mut self) -> BUSERROR_W {
        BUSERROR_W { w: self }
    }
}
