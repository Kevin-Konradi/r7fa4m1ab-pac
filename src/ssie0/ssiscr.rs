#[doc = "Register `SSISCR` reader"]
pub type R = crate::R<SsiscrSpec>;
#[doc = "Register `SSISCR` writer"]
pub type W = crate::W<SsiscrSpec>;
#[doc = "RDF Setting Condition Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rdfs {
    #[doc = "0: SSIFRDR has one stage or more data size"]
    _000 = 0,
    #[doc = "1: SSIFRDR has two stages or more data size (snip)"]
    _001 = 1,
    #[doc = "6: SSIFRDR has seven stages or more data size"]
    _110 = 6,
    #[doc = "7: SSIFRDR has eight stages or more data size."]
    _111 = 7,
}
impl From<Rdfs> for u8 {
    #[inline(always)]
    fn from(variant: Rdfs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rdfs {
    type Ux = u8;
}
impl crate::IsEnum for Rdfs {}
#[doc = "Field `RDFS` reader - RDF Setting Condition Select"]
pub type RdfsR = crate::FieldReader<Rdfs>;
impl RdfsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rdfs> {
        match self.bits {
            0 => Some(Rdfs::_000),
            1 => Some(Rdfs::_001),
            6 => Some(Rdfs::_110),
            7 => Some(Rdfs::_111),
            _ => None,
        }
    }
    #[doc = "SSIFRDR has one stage or more data size"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Rdfs::_000
    }
    #[doc = "SSIFRDR has two stages or more data size (snip)"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Rdfs::_001
    }
    #[doc = "SSIFRDR has seven stages or more data size"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Rdfs::_110
    }
    #[doc = "SSIFRDR has eight stages or more data size."]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Rdfs::_111
    }
}
#[doc = "Field `RDFS` writer - RDF Setting Condition Select"]
pub type RdfsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rdfs>;
impl<'a, REG> RdfsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SSIFRDR has one stage or more data size"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Rdfs::_000)
    }
    #[doc = "SSIFRDR has two stages or more data size (snip)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Rdfs::_001)
    }
    #[doc = "SSIFRDR has seven stages or more data size"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Rdfs::_110)
    }
    #[doc = "SSIFRDR has eight stages or more data size."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Rdfs::_111)
    }
}
#[doc = "TDE Setting Condition Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tdes {
    #[doc = "0: SSIFTDR has one stage or more free space"]
    _000 = 0,
    #[doc = "1: SSIFTDR has two stages or more free space (snip)"]
    _001 = 1,
    #[doc = "6: SSIFTDR has seven stages or more free space"]
    _110 = 6,
    #[doc = "7: SSIFTDR has eight stages or more free space."]
    _111 = 7,
}
impl From<Tdes> for u8 {
    #[inline(always)]
    fn from(variant: Tdes) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tdes {
    type Ux = u8;
}
impl crate::IsEnum for Tdes {}
#[doc = "Field `TDES` reader - TDE Setting Condition Select"]
pub type TdesR = crate::FieldReader<Tdes>;
impl TdesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tdes> {
        match self.bits {
            0 => Some(Tdes::_000),
            1 => Some(Tdes::_001),
            6 => Some(Tdes::_110),
            7 => Some(Tdes::_111),
            _ => None,
        }
    }
    #[doc = "SSIFTDR has one stage or more free space"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Tdes::_000
    }
    #[doc = "SSIFTDR has two stages or more free space (snip)"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Tdes::_001
    }
    #[doc = "SSIFTDR has seven stages or more free space"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Tdes::_110
    }
    #[doc = "SSIFTDR has eight stages or more free space."]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Tdes::_111
    }
}
#[doc = "Field `TDES` writer - TDE Setting Condition Select"]
pub type TdesW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tdes>;
impl<'a, REG> TdesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SSIFTDR has one stage or more free space"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Tdes::_000)
    }
    #[doc = "SSIFTDR has two stages or more free space (snip)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Tdes::_001)
    }
    #[doc = "SSIFTDR has seven stages or more free space"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Tdes::_110)
    }
    #[doc = "SSIFTDR has eight stages or more free space."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Tdes::_111)
    }
}
impl R {
    #[doc = "Bits 0:2 - RDF Setting Condition Select"]
    #[inline(always)]
    pub fn rdfs(&self) -> RdfsR {
        RdfsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - TDE Setting Condition Select"]
    #[inline(always)]
    pub fn tdes(&self) -> TdesR {
        TdesR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSISCR")
            .field("tdes", &self.tdes())
            .field("rdfs", &self.rdfs())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - RDF Setting Condition Select"]
    #[inline(always)]
    pub fn rdfs(&mut self) -> RdfsW<SsiscrSpec> {
        RdfsW::new(self, 0)
    }
    #[doc = "Bits 8:10 - TDE Setting Condition Select"]
    #[inline(always)]
    pub fn tdes(&mut self) -> TdesW<SsiscrSpec> {
        TdesW::new(self, 8)
    }
}
#[doc = "Status Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssiscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssiscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsiscrSpec;
impl crate::RegisterSpec for SsiscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssiscr::R`](R) reader structure"]
impl crate::Readable for SsiscrSpec {}
#[doc = "`write(|w| ..)` method takes [`ssiscr::W`](W) writer structure"]
impl crate::Writable for SsiscrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSISCR to value 0"]
impl crate::Resettable for SsiscrSpec {}
