#[doc = "Reader of register PID"]
pub type R = crate::R<u32, super::PID>;
#[doc = "Reader of field `Aperture`"]
pub type APERTURE_R = crate::R<u8, u8>;
#[doc = "Reader of field `Minor_Rev`"]
pub type MINOR_REV_R = crate::R<u8, u8>;
#[doc = "Reader of field `Major_Rev`"]
pub type MAJOR_REV_R = crate::R<u8, u8>;
#[doc = "Reader of field `ID`"]
pub type ID_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:7 - no description available"]
    #[inline(always)]
    pub fn aperture(&self) -> APERTURE_R {
        APERTURE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Minor revision of module implementation."]
    #[inline(always)]
    pub fn minor_rev(&self) -> MINOR_REV_R {
        MINOR_REV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Major revision of module implementation."]
    #[inline(always)]
    pub fn major_rev(&self) -> MAJOR_REV_R {
        MAJOR_REV_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Module identifier for the selected function."]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
