#[doc = "Register `SYOCDCR` reader"]
pub type R = crate::R<SyocdcrSpec>;
#[doc = "Register `SYOCDCR` writer"]
pub type W = crate::W<SyocdcrSpec>;
#[doc = "Debugger Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbgen {
    #[doc = "0: On-chip debugger is disabled"]
    _0 = 0,
    #[doc = "1: On-chip debugger is enabled"]
    _1 = 1,
}
impl From<Dbgen> for bool {
    #[inline(always)]
    fn from(variant: Dbgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGEN` reader - Debugger Enable bit"]
pub type DbgenR = crate::BitReader<Dbgen>;
impl DbgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbgen {
        match self.bits {
            false => Dbgen::_0,
            true => Dbgen::_1,
        }
    }
    #[doc = "On-chip debugger is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dbgen::_0
    }
    #[doc = "On-chip debugger is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dbgen::_1
    }
}
#[doc = "Field `DBGEN` writer - Debugger Enable bit"]
pub type DbgenW<'a, REG> = crate::BitWriter<'a, REG, Dbgen>;
impl<'a, REG> DbgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "On-chip debugger is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgen::_0)
    }
    #[doc = "On-chip debugger is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgen::_1)
    }
}
impl R {
    #[doc = "Bit 7 - Debugger Enable bit"]
    #[inline(always)]
    pub fn dbgen(&self) -> DbgenR {
        DbgenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYOCDCR")
            .field("dbgen", &self.dbgen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 7 - Debugger Enable bit"]
    #[inline(always)]
    pub fn dbgen(&mut self) -> DbgenW<SyocdcrSpec> {
        DbgenW::new(self, 7)
    }
}
#[doc = "System Control OCD Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syocdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syocdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyocdcrSpec;
impl crate::RegisterSpec for SyocdcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`syocdcr::R`](R) reader structure"]
impl crate::Readable for SyocdcrSpec {}
#[doc = "`write(|w| ..)` method takes [`syocdcr::W`](W) writer structure"]
impl crate::Writable for SyocdcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYOCDCR to value 0"]
impl crate::Resettable for SyocdcrSpec {}
