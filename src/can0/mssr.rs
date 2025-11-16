#[doc = "Register `MSSR` reader"]
pub type R = crate::R<MssrSpec>;
#[doc = "Field `MBNST` reader - Search Result Mailbox Number Status These bits output the smallest mailbox number that is searched in each mode of MSMR."]
pub type MbnstR = crate::FieldReader;
#[doc = "Search Result Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sest {
    #[doc = "0: Search result found"]
    _0 = 0,
    #[doc = "1: No search result"]
    _1 = 1,
}
impl From<Sest> for bool {
    #[inline(always)]
    fn from(variant: Sest) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEST` reader - Search Result Status"]
pub type SestR = crate::BitReader<Sest>;
impl SestR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sest {
        match self.bits {
            false => Sest::_0,
            true => Sest::_1,
        }
    }
    #[doc = "Search result found"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sest::_0
    }
    #[doc = "No search result"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sest::_1
    }
}
impl R {
    #[doc = "Bits 0:4 - Search Result Mailbox Number Status These bits output the smallest mailbox number that is searched in each mode of MSMR."]
    #[inline(always)]
    pub fn mbnst(&self) -> MbnstR {
        MbnstR::new(self.bits & 0x1f)
    }
    #[doc = "Bit 7 - Search Result Status"]
    #[inline(always)]
    pub fn sest(&self) -> SestR {
        SestR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSSR")
            .field("sest", &self.sest())
            .field("mbnst", &self.mbnst())
            .finish()
    }
}
#[doc = "Mailbox Search Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mssr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssrSpec;
impl crate::RegisterSpec for MssrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mssr::R`](R) reader structure"]
impl crate::Readable for MssrSpec {}
#[doc = "`reset()` method sets MSSR to value 0x80"]
impl crate::Resettable for MssrSpec {
    const RESET_VALUE: u8 = 0x80;
}
