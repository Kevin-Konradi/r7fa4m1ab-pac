#[doc = "Register `SISR` reader"]
pub type R = crate::R<SisrSpec>;
#[doc = "ACK Reception Data Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iicackr {
    #[doc = "0: ACK received"]
    _0 = 0,
    #[doc = "1: NACK received"]
    _1 = 1,
}
impl From<Iicackr> for bool {
    #[inline(always)]
    fn from(variant: Iicackr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IICACKR` reader - ACK Reception Data Flag"]
pub type IicackrR = crate::BitReader<Iicackr>;
impl IicackrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iicackr {
        match self.bits {
            false => Iicackr::_0,
            true => Iicackr::_1,
        }
    }
    #[doc = "ACK received"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iicackr::_0
    }
    #[doc = "NACK received"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iicackr::_1
    }
}
impl R {
    #[doc = "Bit 0 - ACK Reception Data Flag"]
    #[inline(always)]
    pub fn iicackr(&self) -> IicackrR {
        IicackrR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SISR")
            .field("iicackr", &self.iicackr())
            .finish()
    }
}
#[doc = "I2C Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SisrSpec;
impl crate::RegisterSpec for SisrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sisr::R`](R) reader structure"]
impl crate::Readable for SisrSpec {}
#[doc = "`reset()` method sets SISR to value 0"]
impl crate::Resettable for SisrSpec {}
