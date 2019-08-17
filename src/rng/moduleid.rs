#[doc = "Reader of register MODULEID"]
pub type R = crate::R<u32, super::MODULEID>;
#[doc = "Reader of field `APERTURE`"]
pub type APERTURE_R = crate::R<u8, u8>;
#[doc = "Reader of field `MIN_REV`"]
pub type MIN_REV_R = crate::R<u8, u8>;
#[doc = "Reader of field `MAJ_REV`"]
pub type MAJ_REV_R = crate::R<u8, u8>;
#[doc = "Reader of field `ID`"]
pub type ID_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:7 - Aperture i."]
    #[inline(always)]
    pub fn aperture(&self) -> APERTURE_R {
        APERTURE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Minor revision i."]
    #[inline(always)]
    pub fn min_rev(&self) -> MIN_REV_R {
        MIN_REV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Major revision i."]
    #[inline(always)]
    pub fn maj_rev(&self) -> MAJ_REV_R {
        MAJ_REV_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Identifier."]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
