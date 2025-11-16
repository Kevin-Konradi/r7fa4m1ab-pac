#[doc = "Register `SBYCR` reader"]
pub type R = crate::R<SbycrSpec>;
#[doc = "Register `SBYCR` writer"]
pub type W = crate::W<SbycrSpec>;
#[doc = "Software Standby\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssby {
    #[doc = "0: Sleep mode"]
    _0 = 0,
    #[doc = "1: Software Standby mode"]
    _1 = 1,
}
impl From<Ssby> for bool {
    #[inline(always)]
    fn from(variant: Ssby) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSBY` reader - Software Standby"]
pub type SsbyR = crate::BitReader<Ssby>;
impl SsbyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssby {
        match self.bits {
            false => Ssby::_0,
            true => Ssby::_1,
        }
    }
    #[doc = "Sleep mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ssby::_0
    }
    #[doc = "Software Standby mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ssby::_1
    }
}
#[doc = "Field `SSBY` writer - Software Standby"]
pub type SsbyW<'a, REG> = crate::BitWriter<'a, REG, Ssby>;
impl<'a, REG> SsbyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sleep mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ssby::_0)
    }
    #[doc = "Software Standby mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ssby::_1)
    }
}
impl R {
    #[doc = "Bit 15 - Software Standby"]
    #[inline(always)]
    pub fn ssby(&self) -> SsbyR {
        SsbyR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SBYCR").field("ssby", &self.ssby()).finish()
    }
}
impl W {
    #[doc = "Bit 15 - Software Standby"]
    #[inline(always)]
    pub fn ssby(&mut self) -> SsbyW<SbycrSpec> {
        SsbyW::new(self, 15)
    }
}
#[doc = "Standby Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sbycr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbycr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SbycrSpec;
impl crate::RegisterSpec for SbycrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sbycr::R`](R) reader structure"]
impl crate::Readable for SbycrSpec {}
#[doc = "`write(|w| ..)` method takes [`sbycr::W`](W) writer structure"]
impl crate::Writable for SbycrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SBYCR to value 0x4000"]
impl crate::Resettable for SbycrSpec {
    const RESET_VALUE: u16 = 0x4000;
}
