#[doc = "Register `LOCOCR` reader"]
pub type R = crate::R<LococrSpec>;
#[doc = "Register `LOCOCR` writer"]
pub type W = crate::W<LococrSpec>;
#[doc = "LOCO Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcstp {
    #[doc = "0: LOCO is operating."]
    _0 = 0,
    #[doc = "1: LOCO is stopped."]
    _1 = 1,
}
impl From<Lcstp> for bool {
    #[inline(always)]
    fn from(variant: Lcstp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCSTP` reader - LOCO Stop"]
pub type LcstpR = crate::BitReader<Lcstp>;
impl LcstpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcstp {
        match self.bits {
            false => Lcstp::_0,
            true => Lcstp::_1,
        }
    }
    #[doc = "LOCO is operating."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lcstp::_0
    }
    #[doc = "LOCO is stopped."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lcstp::_1
    }
}
#[doc = "Field `LCSTP` writer - LOCO Stop"]
pub type LcstpW<'a, REG> = crate::BitWriter<'a, REG, Lcstp>;
impl<'a, REG> LcstpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LOCO is operating."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcstp::_0)
    }
    #[doc = "LOCO is stopped."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcstp::_1)
    }
}
impl R {
    #[doc = "Bit 0 - LOCO Stop"]
    #[inline(always)]
    pub fn lcstp(&self) -> LcstpR {
        LcstpR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOCOCR")
            .field("lcstp", &self.lcstp())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - LOCO Stop"]
    #[inline(always)]
    pub fn lcstp(&mut self) -> LcstpW<LococrSpec> {
        LcstpW::new(self, 0)
    }
}
#[doc = "Low-Speed On-Chip Oscillator Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lococr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lococr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LococrSpec;
impl crate::RegisterSpec for LococrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lococr::R`](R) reader structure"]
impl crate::Readable for LococrSpec {}
#[doc = "`write(|w| ..)` method takes [`lococr::W`](W) writer structure"]
impl crate::Writable for LococrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOCOCR to value 0"]
impl crate::Resettable for LococrSpec {}
