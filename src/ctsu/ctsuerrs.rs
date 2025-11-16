#[doc = "Register `CTSUERRS` reader"]
pub type R = crate::R<CtsuerrsSpec>;
#[doc = "TSCAP Voltage Error Monitor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsuicomp {
    #[doc = "0: Normal TSCAP voltage"]
    _0 = 0,
    #[doc = "1: Abnormal TSCAP voltage"]
    _1 = 1,
}
impl From<Ctsuicomp> for bool {
    #[inline(always)]
    fn from(variant: Ctsuicomp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSUICOMP` reader - TSCAP Voltage Error Monitor"]
pub type CtsuicompR = crate::BitReader<Ctsuicomp>;
impl CtsuicompR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsuicomp {
        match self.bits {
            false => Ctsuicomp::_0,
            true => Ctsuicomp::_1,
        }
    }
    #[doc = "Normal TSCAP voltage"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ctsuicomp::_0
    }
    #[doc = "Abnormal TSCAP voltage"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ctsuicomp::_1
    }
}
impl R {
    #[doc = "Bit 15 - TSCAP Voltage Error Monitor"]
    #[inline(always)]
    pub fn ctsuicomp(&self) -> CtsuicompR {
        CtsuicompR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTSUERRS")
            .field("ctsuicomp", &self.ctsuicomp())
            .finish()
    }
}
#[doc = "CTSU Error Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctsuerrs::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtsuerrsSpec;
impl crate::RegisterSpec for CtsuerrsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctsuerrs::R`](R) reader structure"]
impl crate::Readable for CtsuerrsSpec {}
#[doc = "`reset()` method sets CTSUERRS to value 0"]
impl crate::Resettable for CtsuerrsSpec {}
