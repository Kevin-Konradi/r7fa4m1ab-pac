#[doc = "Register `SOPCCR` reader"]
pub type R = crate::R<SopccrSpec>;
#[doc = "Register `SOPCCR` writer"]
pub type W = crate::W<SopccrSpec>;
#[doc = "Sub Operating Power Control Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sopcm {
    #[doc = "0: Other than Subosc-speed mode"]
    _0 = 0,
    #[doc = "1: Subosc-speed mode"]
    _1 = 1,
}
impl From<Sopcm> for bool {
    #[inline(always)]
    fn from(variant: Sopcm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOPCM` reader - Sub Operating Power Control Mode Select"]
pub type SopcmR = crate::BitReader<Sopcm>;
impl SopcmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sopcm {
        match self.bits {
            false => Sopcm::_0,
            true => Sopcm::_1,
        }
    }
    #[doc = "Other than Subosc-speed mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sopcm::_0
    }
    #[doc = "Subosc-speed mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sopcm::_1
    }
}
#[doc = "Field `SOPCM` writer - Sub Operating Power Control Mode Select"]
pub type SopcmW<'a, REG> = crate::BitWriter<'a, REG, Sopcm>;
impl<'a, REG> SopcmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Other than Subosc-speed mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sopcm::_0)
    }
    #[doc = "Subosc-speed mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sopcm::_1)
    }
}
#[doc = "Sub Operating Power Control Mode Transition Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sopcmtsf {
    #[doc = "0: Transition completed"]
    _0 = 0,
    #[doc = "1: During transition"]
    _1 = 1,
}
impl From<Sopcmtsf> for bool {
    #[inline(always)]
    fn from(variant: Sopcmtsf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOPCMTSF` reader - Sub Operating Power Control Mode Transition Status Flag"]
pub type SopcmtsfR = crate::BitReader<Sopcmtsf>;
impl SopcmtsfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sopcmtsf {
        match self.bits {
            false => Sopcmtsf::_0,
            true => Sopcmtsf::_1,
        }
    }
    #[doc = "Transition completed"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sopcmtsf::_0
    }
    #[doc = "During transition"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sopcmtsf::_1
    }
}
impl R {
    #[doc = "Bit 0 - Sub Operating Power Control Mode Select"]
    #[inline(always)]
    pub fn sopcm(&self) -> SopcmR {
        SopcmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Sub Operating Power Control Mode Transition Status Flag"]
    #[inline(always)]
    pub fn sopcmtsf(&self) -> SopcmtsfR {
        SopcmtsfR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SOPCCR")
            .field("sopcmtsf", &self.sopcmtsf())
            .field("sopcm", &self.sopcm())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Sub Operating Power Control Mode Select"]
    #[inline(always)]
    pub fn sopcm(&mut self) -> SopcmW<SopccrSpec> {
        SopcmW::new(self, 0)
    }
}
#[doc = "Sub Operating Power Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sopccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sopccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SopccrSpec;
impl crate::RegisterSpec for SopccrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sopccr::R`](R) reader structure"]
impl crate::Readable for SopccrSpec {}
#[doc = "`write(|w| ..)` method takes [`sopccr::W`](W) writer structure"]
impl crate::Writable for SopccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SOPCCR to value 0"]
impl crate::Resettable for SopccrSpec {}
