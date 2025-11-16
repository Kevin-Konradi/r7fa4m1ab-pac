#[doc = "Register `RYRAREN` reader"]
pub type R = crate::R<RyrarenSpec>;
#[doc = "Register `RYRAREN` writer"]
pub type W = crate::W<RyrarenSpec>;
#[doc = "Compare enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enb {
    #[doc = "0: The register value is not compared with the RYRCNT counter value."]
    _0 = 0,
    #[doc = "1: The register value is compared with the RYRCNT counter value."]
    _1 = 1,
}
impl From<Enb> for bool {
    #[inline(always)]
    fn from(variant: Enb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENB` reader - Compare enable"]
pub type EnbR = crate::BitReader<Enb>;
impl EnbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enb {
        match self.bits {
            false => Enb::_0,
            true => Enb::_1,
        }
    }
    #[doc = "The register value is not compared with the RYRCNT counter value."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Enb::_0
    }
    #[doc = "The register value is compared with the RYRCNT counter value."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Enb::_1
    }
}
#[doc = "Field `ENB` writer - Compare enable"]
pub type EnbW<'a, REG> = crate::BitWriter<'a, REG, Enb>;
impl<'a, REG> EnbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The register value is not compared with the RYRCNT counter value."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Enb::_0)
    }
    #[doc = "The register value is compared with the RYRCNT counter value."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Enb::_1)
    }
}
impl R {
    #[doc = "Bit 7 - Compare enable"]
    #[inline(always)]
    pub fn enb(&self) -> EnbR {
        EnbR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RYRAREN").field("enb", &self.enb()).finish()
    }
}
impl W {
    #[doc = "Bit 7 - Compare enable"]
    #[inline(always)]
    pub fn enb(&mut self) -> EnbW<RyrarenSpec> {
        EnbW::new(self, 7)
    }
}
#[doc = "Year Alarm Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ryraren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ryraren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RyrarenSpec;
impl crate::RegisterSpec for RyrarenSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ryraren::R`](R) reader structure"]
impl crate::Readable for RyrarenSpec {}
#[doc = "`write(|w| ..)` method takes [`ryraren::W`](W) writer structure"]
impl crate::Writable for RyrarenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RYRAREN to value 0"]
impl crate::Resettable for RyrarenSpec {}
