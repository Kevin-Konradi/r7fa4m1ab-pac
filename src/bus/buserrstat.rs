#[doc = "Register `BUS%sERRSTAT` reader"]
pub type R = crate::R<BuserrstatSpec>;
#[doc = "Error Access Status The status at the time of the error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Accstat {
    #[doc = "0: Read access"]
    _0 = 0,
    #[doc = "1: Write Access"]
    _1 = 1,
}
impl From<Accstat> for bool {
    #[inline(always)]
    fn from(variant: Accstat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACCSTAT` reader - Error Access Status The status at the time of the error"]
pub type AccstatR = crate::BitReader<Accstat>;
impl AccstatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Accstat {
        match self.bits {
            false => Accstat::_0,
            true => Accstat::_1,
        }
    }
    #[doc = "Read access"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Accstat::_0
    }
    #[doc = "Write Access"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Accstat::_1
    }
}
#[doc = "Bus Error Status When bus error assert, error flag occurs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errstat {
    #[doc = "0: No bus error occurred"]
    _0 = 0,
    #[doc = "1: Bus error occurred."]
    _1 = 1,
}
impl From<Errstat> for bool {
    #[inline(always)]
    fn from(variant: Errstat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRSTAT` reader - Bus Error Status When bus error assert, error flag occurs."]
pub type ErrstatR = crate::BitReader<Errstat>;
impl ErrstatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errstat {
        match self.bits {
            false => Errstat::_0,
            true => Errstat::_1,
        }
    }
    #[doc = "No bus error occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Errstat::_0
    }
    #[doc = "Bus error occurred."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Errstat::_1
    }
}
impl R {
    #[doc = "Bit 0 - Error Access Status The status at the time of the error"]
    #[inline(always)]
    pub fn accstat(&self) -> AccstatR {
        AccstatR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - Bus Error Status When bus error assert, error flag occurs."]
    #[inline(always)]
    pub fn errstat(&self) -> ErrstatR {
        ErrstatR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUSERRSTAT")
            .field("errstat", &self.errstat())
            .field("accstat", &self.accstat())
            .finish()
    }
}
#[doc = "Bus Error Status Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`buserrstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BuserrstatSpec;
impl crate::RegisterSpec for BuserrstatSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`buserrstat::R`](R) reader structure"]
impl crate::Readable for BuserrstatSpec {}
#[doc = "`reset()` method sets BUS%sERRSTAT to value 0"]
impl crate::Resettable for BuserrstatSpec {}
