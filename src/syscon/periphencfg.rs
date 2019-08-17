#[doc = "Reader of register PERIPHENCFG"]
pub type R = crate::R<u32, super::PERIPHENCFG>;
#[doc = "Writer for register PERIPHENCFG"]
pub type W = crate::W<u32, super::PERIPHENCFG>;
#[doc = "Register PERIPHENCFG `reset()`'s with value 0x5c47"]
impl crate::ResetValue for super::PERIPHENCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x5c47
    }
}
#[doc = "Possible values of the field `SCTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCTEN_A {
    #[doc = "peripheral is disable."]
    DISABLE,
    #[doc = "peripheral is enable."]
    ENABLE,
}
impl From<SCTEN_A> for bool {
    #[inline(always)]
    fn from(variant: SCTEN_A) -> Self {
        match variant {
            SCTEN_A::DISABLE => false,
            SCTEN_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `SCTEN`"]
pub type SCTEN_R = crate::R<bool, SCTEN_A>;
impl SCTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCTEN_A {
        match self.bits {
            false => SCTEN_A::DISABLE,
            true => SCTEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SCTEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SCTEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `SCTEN`"]
pub struct SCTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "peripheral is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SCTEN_A::DISABLE)
    }
    #[doc = "peripheral is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SCTEN_A::ENABLE)
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
#[doc = "Possible values of the field `ADCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCEN_A {
    #[doc = "peripheral is disable."]
    DISABLE,
    #[doc = "peripheral is enable."]
    ENABLE,
}
impl From<ADCEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADCEN_A) -> Self {
        match variant {
            ADCEN_A::DISABLE => false,
            ADCEN_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `ADCEN`"]
pub type ADCEN_R = crate::R<bool, ADCEN_A>;
impl ADCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCEN_A {
        match self.bits {
            false => ADCEN_A::DISABLE,
            true => ADCEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADCEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADCEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `ADCEN`"]
pub struct ADCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "peripheral is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADCEN_A::DISABLE)
    }
    #[doc = "peripheral is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADCEN_A::ENABLE)
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
#[doc = "Possible values of the field `USB0EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB0EN_A {
    #[doc = "peripheral is disable."]
    DISABLE,
    #[doc = "peripheral is enable."]
    ENABLE,
}
impl From<USB0EN_A> for bool {
    #[inline(always)]
    fn from(variant: USB0EN_A) -> Self {
        match variant {
            USB0EN_A::DISABLE => false,
            USB0EN_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `USB0EN`"]
pub type USB0EN_R = crate::R<bool, USB0EN_A>;
impl USB0EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB0EN_A {
        match self.bits {
            false => USB0EN_A::DISABLE,
            true => USB0EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == USB0EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == USB0EN_A::ENABLE
    }
}
#[doc = "Write proxy for field `USB0EN`"]
pub struct USB0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB0EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "peripheral is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB0EN_A::DISABLE)
    }
    #[doc = "peripheral is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB0EN_A::ENABLE)
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
#[doc = "Possible values of the field `PUFFEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PUFFEN_A {
    #[doc = "peripheral is disable."]
    DISABLE,
    #[doc = "peripheral is enable."]
    ENABLE,
}
impl From<PUFFEN_A> for bool {
    #[inline(always)]
    fn from(variant: PUFFEN_A) -> Self {
        match variant {
            PUFFEN_A::DISABLE => false,
            PUFFEN_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `PUFFEN`"]
pub type PUFFEN_R = crate::R<bool, PUFFEN_A>;
impl PUFFEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PUFFEN_A {
        match self.bits {
            false => PUFFEN_A::DISABLE,
            true => PUFFEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PUFFEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PUFFEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `PUFFEN`"]
pub struct PUFFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PUFFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUFFEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "peripheral is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PUFFEN_A::DISABLE)
    }
    #[doc = "peripheral is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PUFFEN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Possible values of the field `USB1EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1EN_A {
    #[doc = "peripheral is disable."]
    DISABLE,
    #[doc = "peripheral is enable."]
    ENABLE,
}
impl From<USB1EN_A> for bool {
    #[inline(always)]
    fn from(variant: USB1EN_A) -> Self {
        match variant {
            USB1EN_A::DISABLE => false,
            USB1EN_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `USB1EN`"]
pub type USB1EN_R = crate::R<bool, USB1EN_A>;
impl USB1EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB1EN_A {
        match self.bits {
            false => USB1EN_A::DISABLE,
            true => USB1EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == USB1EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == USB1EN_A::ENABLE
    }
}
#[doc = "Write proxy for field `USB1EN`"]
pub struct USB1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB1EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "peripheral is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB1EN_A::DISABLE)
    }
    #[doc = "peripheral is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB1EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Possible values of the field `SDIOEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDIOEN_A {
    #[doc = "peripheral is disable."]
    DISABLE,
    #[doc = "peripheral is enable."]
    ENABLE,
}
impl From<SDIOEN_A> for bool {
    #[inline(always)]
    fn from(variant: SDIOEN_A) -> Self {
        match variant {
            SDIOEN_A::DISABLE => false,
            SDIOEN_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `SDIOEN`"]
pub type SDIOEN_R = crate::R<bool, SDIOEN_A>;
impl SDIOEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIOEN_A {
        match self.bits {
            false => SDIOEN_A::DISABLE,
            true => SDIOEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SDIOEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SDIOEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `SDIOEN`"]
pub struct SDIOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIOEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDIOEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "peripheral is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SDIOEN_A::DISABLE)
    }
    #[doc = "peripheral is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SDIOEN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Possible values of the field `HASHEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HASHEN_A {
    #[doc = "peripheral is disable."]
    DISABLE,
    #[doc = "peripheral is enable."]
    ENABLE,
}
impl From<HASHEN_A> for bool {
    #[inline(always)]
    fn from(variant: HASHEN_A) -> Self {
        match variant {
            HASHEN_A::DISABLE => false,
            HASHEN_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `HASHEN`"]
pub type HASHEN_R = crate::R<bool, HASHEN_A>;
impl HASHEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HASHEN_A {
        match self.bits {
            false => HASHEN_A::DISABLE,
            true => HASHEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HASHEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HASHEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `HASHEN`"]
pub struct HASHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HASHEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HASHEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "peripheral is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HASHEN_A::DISABLE)
    }
    #[doc = "peripheral is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HASHEN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Possible values of the field `PRINCEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRINCEEN_A {
    #[doc = "peripheral is disable."]
    DISABLE,
    #[doc = "peripheral is enable."]
    ENABLE,
}
impl From<PRINCEEN_A> for bool {
    #[inline(always)]
    fn from(variant: PRINCEEN_A) -> Self {
        match variant {
            PRINCEEN_A::DISABLE => false,
            PRINCEEN_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `PRINCEEN`"]
pub type PRINCEEN_R = crate::R<bool, PRINCEEN_A>;
impl PRINCEEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRINCEEN_A {
        match self.bits {
            false => PRINCEEN_A::DISABLE,
            true => PRINCEEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PRINCEEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PRINCEEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `PRINCEEN`"]
pub struct PRINCEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRINCEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRINCEEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "peripheral is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PRINCEEN_A::DISABLE)
    }
    #[doc = "peripheral is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PRINCEEN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SCT enable."]
    #[inline(always)]
    pub fn scten(&self) -> SCTEN_R {
        SCTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC enable."]
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USB0 enable."]
    #[inline(always)]
    pub fn usb0en(&self) -> USB0EN_R {
        USB0EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Puff enable."]
    #[inline(always)]
    pub fn puffen(&self) -> PUFFEN_R {
        PUFFEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 10 - USB1 enable."]
    #[inline(always)]
    pub fn usb1en(&self) -> USB1EN_R {
        USB1EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SDIO enable."]
    #[inline(always)]
    pub fn sdioen(&self) -> SDIOEN_R {
        SDIOEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - HASH enable."]
    #[inline(always)]
    pub fn hashen(&self) -> HASHEN_R {
        HASHEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PRINCE enable."]
    #[inline(always)]
    pub fn princeen(&self) -> PRINCEEN_R {
        PRINCEEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCT enable."]
    #[inline(always)]
    pub fn scten(&mut self) -> SCTEN_W {
        SCTEN_W { w: self }
    }
    #[doc = "Bit 1 - ADC enable."]
    #[inline(always)]
    pub fn adcen(&mut self) -> ADCEN_W {
        ADCEN_W { w: self }
    }
    #[doc = "Bit 2 - USB0 enable."]
    #[inline(always)]
    pub fn usb0en(&mut self) -> USB0EN_W {
        USB0EN_W { w: self }
    }
    #[doc = "Bit 6 - Puff enable."]
    #[inline(always)]
    pub fn puffen(&mut self) -> PUFFEN_W {
        PUFFEN_W { w: self }
    }
    #[doc = "Bit 10 - USB1 enable."]
    #[inline(always)]
    pub fn usb1en(&mut self) -> USB1EN_W {
        USB1EN_W { w: self }
    }
    #[doc = "Bit 11 - SDIO enable."]
    #[inline(always)]
    pub fn sdioen(&mut self) -> SDIOEN_W {
        SDIOEN_W { w: self }
    }
    #[doc = "Bit 12 - HASH enable."]
    #[inline(always)]
    pub fn hashen(&mut self) -> HASHEN_W {
        HASHEN_W { w: self }
    }
    #[doc = "Bit 14 - PRINCE enable."]
    #[inline(always)]
    pub fn princeen(&mut self) -> PRINCEEN_W {
        PRINCEEN_W { w: self }
    }
}
