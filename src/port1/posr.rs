#[doc = "Register `POSR` writer"]
pub type W = crate::W<PosrSpec>;
#[doc = "Pmn Output Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Posr {
    #[doc = "0: No affect to output"]
    _0 = 0,
    #[doc = "1: High output."]
    _1 = 1,
}
impl From<Posr> for u16 {
    #[inline(always)]
    fn from(variant: Posr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Posr {
    type Ux = u16;
}
impl crate::IsEnum for Posr {}
#[doc = "Field `POSR` writer - Pmn Output Set"]
pub type PosrW<'a, REG> = crate::FieldWriter<'a, REG, 16, Posr>;
impl<'a, REG> PosrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "No affect to output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Posr::_0)
    }
    #[doc = "High output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Posr::_1)
    }
}
impl core::fmt::Debug for crate::generic::Reg<PosrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:15 - Pmn Output Set"]
    #[inline(always)]
    pub fn posr(&mut self) -> PosrW<PosrSpec> {
        PosrW::new(self, 0)
    }
}
#[doc = "Output reset register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`posr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PosrSpec;
impl crate::RegisterSpec for PosrSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`posr::W`](W) writer structure"]
impl crate::Writable for PosrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POSR to value 0"]
impl crate::Resettable for PosrSpec {}
