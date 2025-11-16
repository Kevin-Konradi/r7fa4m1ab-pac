#[doc = "Register `KRCTL` reader"]
pub type R = crate::R<KrctlSpec>;
#[doc = "Register `KRCTL` writer"]
pub type W = crate::W<KrctlSpec>;
#[doc = "Detection Edge Selection (KRF0 to KRF7)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kreg {
    #[doc = "0: Falling edge"]
    _0 = 0,
    #[doc = "1: Rising edge"]
    _1 = 1,
}
impl From<Kreg> for bool {
    #[inline(always)]
    fn from(variant: Kreg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KREG` reader - Detection Edge Selection (KRF0 to KRF7)"]
pub type KregR = crate::BitReader<Kreg>;
impl KregR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Kreg {
        match self.bits {
            false => Kreg::_0,
            true => Kreg::_1,
        }
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Kreg::_0
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Kreg::_1
    }
}
#[doc = "Field `KREG` writer - Detection Edge Selection (KRF0 to KRF7)"]
pub type KregW<'a, REG> = crate::BitWriter<'a, REG, Kreg>;
impl<'a, REG> KregW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Kreg::_0)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Kreg::_1)
    }
}
#[doc = "Usage of Key Interrupt Flags(KR0 to KR7)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Krmd {
    #[doc = "0: Do not use key interrupt flags"]
    _0 = 0,
    #[doc = "1: Use key interrupt flags."]
    _1 = 1,
}
impl From<Krmd> for bool {
    #[inline(always)]
    fn from(variant: Krmd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KRMD` reader - Usage of Key Interrupt Flags(KR0 to KR7)"]
pub type KrmdR = crate::BitReader<Krmd>;
impl KrmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Krmd {
        match self.bits {
            false => Krmd::_0,
            true => Krmd::_1,
        }
    }
    #[doc = "Do not use key interrupt flags"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Krmd::_0
    }
    #[doc = "Use key interrupt flags."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Krmd::_1
    }
}
#[doc = "Field `KRMD` writer - Usage of Key Interrupt Flags(KR0 to KR7)"]
pub type KrmdW<'a, REG> = crate::BitWriter<'a, REG, Krmd>;
impl<'a, REG> KrmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not use key interrupt flags"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Krmd::_0)
    }
    #[doc = "Use key interrupt flags."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Krmd::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Detection Edge Selection (KRF0 to KRF7)"]
    #[inline(always)]
    pub fn kreg(&self) -> KregR {
        KregR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - Usage of Key Interrupt Flags(KR0 to KR7)"]
    #[inline(always)]
    pub fn krmd(&self) -> KrmdR {
        KrmdR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KRCTL")
            .field("krmd", &self.krmd())
            .field("kreg", &self.kreg())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Detection Edge Selection (KRF0 to KRF7)"]
    #[inline(always)]
    pub fn kreg(&mut self) -> KregW<KrctlSpec> {
        KregW::new(self, 0)
    }
    #[doc = "Bit 7 - Usage of Key Interrupt Flags(KR0 to KR7)"]
    #[inline(always)]
    pub fn krmd(&mut self) -> KrmdW<KrctlSpec> {
        KrmdW::new(self, 7)
    }
}
#[doc = "KEY Return Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`krctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`krctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KrctlSpec;
impl crate::RegisterSpec for KrctlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`krctl::R`](R) reader structure"]
impl crate::Readable for KrctlSpec {}
#[doc = "`write(|w| ..)` method takes [`krctl::W`](W) writer structure"]
impl crate::Writable for KrctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KRCTL to value 0"]
impl crate::Resettable for KrctlSpec {}
