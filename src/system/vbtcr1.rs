#[doc = "Register `VBTCR1` reader"]
pub type R = crate::R<Vbtcr1Spec>;
#[doc = "Register `VBTCR1` writer"]
pub type W = crate::W<Vbtcr1Spec>;
#[doc = "Battery Power supply Switch Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpwswstp {
    #[doc = "0: Battery Power supply Switch Enable"]
    _0 = 0,
    #[doc = "1: Battery Power supply Switch stop"]
    _1 = 1,
}
impl From<Bpwswstp> for bool {
    #[inline(always)]
    fn from(variant: Bpwswstp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPWSWSTP` reader - Battery Power supply Switch Stop"]
pub type BpwswstpR = crate::BitReader<Bpwswstp>;
impl BpwswstpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpwswstp {
        match self.bits {
            false => Bpwswstp::_0,
            true => Bpwswstp::_1,
        }
    }
    #[doc = "Battery Power supply Switch Enable"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bpwswstp::_0
    }
    #[doc = "Battery Power supply Switch stop"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bpwswstp::_1
    }
}
#[doc = "Field `BPWSWSTP` writer - Battery Power supply Switch Stop"]
pub type BpwswstpW<'a, REG> = crate::BitWriter<'a, REG, Bpwswstp>;
impl<'a, REG> BpwswstpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Battery Power supply Switch Enable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpwswstp::_0)
    }
    #[doc = "Battery Power supply Switch stop"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpwswstp::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Battery Power supply Switch Stop"]
    #[inline(always)]
    pub fn bpwswstp(&self) -> BpwswstpR {
        BpwswstpR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VBTCR1")
            .field("bpwswstp", &self.bpwswstp())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Battery Power supply Switch Stop"]
    #[inline(always)]
    pub fn bpwswstp(&mut self) -> BpwswstpW<Vbtcr1Spec> {
        BpwswstpW::new(self, 0)
    }
}
#[doc = "VBATT Control Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`vbtcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbtcr1Spec;
impl crate::RegisterSpec for Vbtcr1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vbtcr1::R`](R) reader structure"]
impl crate::Readable for Vbtcr1Spec {}
#[doc = "`write(|w| ..)` method takes [`vbtcr1::W`](W) writer structure"]
impl crate::Writable for Vbtcr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VBTCR1 to value 0"]
impl crate::Resettable for Vbtcr1Spec {}
