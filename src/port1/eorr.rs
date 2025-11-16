#[doc = "Register `EORR` reader"]
pub type R = crate::R<EorrSpec>;
#[doc = "Register `EORR` writer"]
pub type W = crate::W<EorrSpec>;
#[doc = "Pmn Event Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Eorr {
    #[doc = "0: No affect to output"]
    _0 = 0,
    #[doc = "1: Low output"]
    _1 = 1,
}
impl From<Eorr> for u16 {
    #[inline(always)]
    fn from(variant: Eorr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Eorr {
    type Ux = u16;
}
impl crate::IsEnum for Eorr {}
#[doc = "Field `EORR` reader - Pmn Event Output Reset"]
pub type EorrR = crate::FieldReader<Eorr>;
impl EorrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Eorr> {
        match self.bits {
            0 => Some(Eorr::_0),
            1 => Some(Eorr::_1),
            _ => None,
        }
    }
    #[doc = "No affect to output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eorr::_0
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eorr::_1
    }
}
#[doc = "Field `EORR` writer - Pmn Event Output Reset"]
pub type EorrW<'a, REG> = crate::FieldWriter<'a, REG, 16, Eorr>;
impl<'a, REG> EorrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "No affect to output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr::_0)
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eorr::_1)
    }
}
impl R {
    #[doc = "Bits 0:15 - Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr(&self) -> EorrR {
        EorrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EORR").field("eorr", &self.eorr()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Pmn Event Output Reset"]
    #[inline(always)]
    pub fn eorr(&mut self) -> EorrW<EorrSpec> {
        EorrW::new(self, 0)
    }
}
#[doc = "Event output set register\n\nYou can [`read`](crate::Reg::read) this register and get [`eorr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eorr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EorrSpec;
impl crate::RegisterSpec for EorrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`eorr::R`](R) reader structure"]
impl crate::Readable for EorrSpec {}
#[doc = "`write(|w| ..)` method takes [`eorr::W`](W) writer structure"]
impl crate::Writable for EorrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EORR to value 0"]
impl crate::Resettable for EorrSpec {}
