#[doc = "Register `TRACECTR` reader"]
pub type R = crate::R<TracectrSpec>;
#[doc = "Register `TRACECTR` writer"]
pub type W = crate::W<TracectrSpec>;
#[doc = "Enable bit for halt request by ETB full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enetbfull {
    #[doc = "0: ETB full does not cause CPU halt"]
    _0 = 0,
    #[doc = "1: ETB full cause CPU halt"]
    _1 = 1,
}
impl From<Enetbfull> for bool {
    #[inline(always)]
    fn from(variant: Enetbfull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENETBFULL` reader - Enable bit for halt request by ETB full"]
pub type EnetbfullR = crate::BitReader<Enetbfull>;
impl EnetbfullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enetbfull {
        match self.bits {
            false => Enetbfull::_0,
            true => Enetbfull::_1,
        }
    }
    #[doc = "ETB full does not cause CPU halt"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Enetbfull::_0
    }
    #[doc = "ETB full cause CPU halt"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Enetbfull::_1
    }
}
#[doc = "Field `ENETBFULL` writer - Enable bit for halt request by ETB full"]
pub type EnetbfullW<'a, REG> = crate::BitWriter<'a, REG, Enetbfull>;
impl<'a, REG> EnetbfullW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ETB full does not cause CPU halt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Enetbfull::_0)
    }
    #[doc = "ETB full cause CPU halt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Enetbfull::_1)
    }
}
impl R {
    #[doc = "Bit 31 - Enable bit for halt request by ETB full"]
    #[inline(always)]
    pub fn enetbfull(&self) -> EnetbfullR {
        EnetbfullR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRACECTR")
            .field("enetbfull", &self.enetbfull())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - Enable bit for halt request by ETB full"]
    #[inline(always)]
    pub fn enetbfull(&mut self) -> EnetbfullW<TracectrSpec> {
        EnetbfullW::new(self, 31)
    }
}
#[doc = "Trace Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tracectr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tracectr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TracectrSpec;
impl crate::RegisterSpec for TracectrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tracectr::R`](R) reader structure"]
impl crate::Readable for TracectrSpec {}
#[doc = "`write(|w| ..)` method takes [`tracectr::W`](W) writer structure"]
impl crate::Writable for TracectrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRACECTR to value 0"]
impl crate::Resettable for TracectrSpec {}
