#[doc = "Register `DBGSTR` reader"]
pub type R = crate::R<DbgstrSpec>;
#[doc = "Debug power-up request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cdbgpwrupreq {
    #[doc = "0: OCD is not requesting debug power-up"]
    _0 = 0,
    #[doc = "1: OCD is requesting debug power-up"]
    _1 = 1,
}
impl From<Cdbgpwrupreq> for bool {
    #[inline(always)]
    fn from(variant: Cdbgpwrupreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDBGPWRUPREQ` reader - Debug power-up request"]
pub type CdbgpwrupreqR = crate::BitReader<Cdbgpwrupreq>;
impl CdbgpwrupreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cdbgpwrupreq {
        match self.bits {
            false => Cdbgpwrupreq::_0,
            true => Cdbgpwrupreq::_1,
        }
    }
    #[doc = "OCD is not requesting debug power-up"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cdbgpwrupreq::_0
    }
    #[doc = "OCD is requesting debug power-up"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cdbgpwrupreq::_1
    }
}
#[doc = "Debug power-up acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cdbgpwrupack {
    #[doc = "0: Debug power-up request is not acknowledged"]
    _0 = 0,
    #[doc = "1: Debug power-up request is acknowledged"]
    _1 = 1,
}
impl From<Cdbgpwrupack> for bool {
    #[inline(always)]
    fn from(variant: Cdbgpwrupack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDBGPWRUPACK` reader - Debug power-up acknowledge"]
pub type CdbgpwrupackR = crate::BitReader<Cdbgpwrupack>;
impl CdbgpwrupackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cdbgpwrupack {
        match self.bits {
            false => Cdbgpwrupack::_0,
            true => Cdbgpwrupack::_1,
        }
    }
    #[doc = "Debug power-up request is not acknowledged"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cdbgpwrupack::_0
    }
    #[doc = "Debug power-up request is acknowledged"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cdbgpwrupack::_1
    }
}
impl R {
    #[doc = "Bit 28 - Debug power-up request"]
    #[inline(always)]
    pub fn cdbgpwrupreq(&self) -> CdbgpwrupreqR {
        CdbgpwrupreqR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Debug power-up acknowledge"]
    #[inline(always)]
    pub fn cdbgpwrupack(&self) -> CdbgpwrupackR {
        CdbgpwrupackR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGSTR")
            .field("cdbgpwrupack", &self.cdbgpwrupack())
            .field("cdbgpwrupreq", &self.cdbgpwrupreq())
            .finish()
    }
}
#[doc = "Debug Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgstr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgstrSpec;
impl crate::RegisterSpec for DbgstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgstr::R`](R) reader structure"]
impl crate::Readable for DbgstrSpec {}
#[doc = "`reset()` method sets DBGSTR to value 0"]
impl crate::Resettable for DbgstrSpec {}
