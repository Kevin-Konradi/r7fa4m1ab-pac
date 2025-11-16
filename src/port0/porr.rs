#[doc = "Register `PORR` writer"]
pub type W = crate::W<PorrSpec>;
#[doc = "Pmn Output Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Porr {
    #[doc = "0: No affect to output"]
    _0 = 0,
    #[doc = "1: Low output."]
    _1 = 1,
}
impl From<Porr> for u16 {
    #[inline(always)]
    fn from(variant: Porr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Porr {
    type Ux = u16;
}
impl crate::IsEnum for Porr {}
#[doc = "Field `PORR` writer - Pmn Output Reset"]
pub type PorrW<'a, REG> = crate::FieldWriter<'a, REG, 16, Porr>;
impl<'a, REG> PorrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "No affect to output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Porr::_0)
    }
    #[doc = "Low output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Porr::_1)
    }
}
impl core::fmt::Debug for crate::generic::Reg<PorrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:15 - Pmn Output Reset"]
    #[inline(always)]
    pub fn porr(&mut self) -> PorrW<PorrSpec> {
        PorrW::new(self, 0)
    }
}
#[doc = "Output reset register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`porr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PorrSpec;
impl crate::RegisterSpec for PorrSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`porr::W`](W) writer structure"]
impl crate::Writable for PorrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PORR to value 0"]
impl crate::Resettable for PorrSpec {}
