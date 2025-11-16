#[doc = "Register `SMPUMBIU` reader"]
pub type R = crate::R<SmpumbiuSpec>;
#[doc = "Register `SMPUMBIU` writer"]
pub type W = crate::W<SmpumbiuSpec>;
#[doc = "Master Group A Read protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rpgrpa {
    #[doc = "0: Master group A read of memory protection disabled."]
    _0 = 0,
    #[doc = "1: Master group A read of memory protection enabled."]
    _1 = 1,
}
impl From<Rpgrpa> for bool {
    #[inline(always)]
    fn from(variant: Rpgrpa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPGRPA` reader - Master Group A Read protection"]
pub type RpgrpaR = crate::BitReader<Rpgrpa>;
impl RpgrpaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rpgrpa {
        match self.bits {
            false => Rpgrpa::_0,
            true => Rpgrpa::_1,
        }
    }
    #[doc = "Master group A read of memory protection disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rpgrpa::_0
    }
    #[doc = "Master group A read of memory protection enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rpgrpa::_1
    }
}
#[doc = "Field `RPGRPA` writer - Master Group A Read protection"]
pub type RpgrpaW<'a, REG> = crate::BitWriter<'a, REG, Rpgrpa>;
impl<'a, REG> RpgrpaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master group A read of memory protection disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rpgrpa::_0)
    }
    #[doc = "Master group A read of memory protection enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rpgrpa::_1)
    }
}
#[doc = "Master Group A Write protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wpgrpa {
    #[doc = "0: Master group A write of memory protection disabled."]
    _0 = 0,
    #[doc = "1: Master group A write of memory protection enabled."]
    _1 = 1,
}
impl From<Wpgrpa> for bool {
    #[inline(always)]
    fn from(variant: Wpgrpa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPGRPA` reader - Master Group A Write protection"]
pub type WpgrpaR = crate::BitReader<Wpgrpa>;
impl WpgrpaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wpgrpa {
        match self.bits {
            false => Wpgrpa::_0,
            true => Wpgrpa::_1,
        }
    }
    #[doc = "Master group A write of memory protection disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wpgrpa::_0
    }
    #[doc = "Master group A write of memory protection enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wpgrpa::_1
    }
}
#[doc = "Field `WPGRPA` writer - Master Group A Write protection"]
pub type WpgrpaW<'a, REG> = crate::BitWriter<'a, REG, Wpgrpa>;
impl<'a, REG> WpgrpaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master group A write of memory protection disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wpgrpa::_0)
    }
    #[doc = "Master group A write of memory protection enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wpgrpa::_1)
    }
}
impl R {
    #[doc = "Bit 2 - Master Group A Read protection"]
    #[inline(always)]
    pub fn rpgrpa(&self) -> RpgrpaR {
        RpgrpaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master Group A Write protection"]
    #[inline(always)]
    pub fn wpgrpa(&self) -> WpgrpaR {
        WpgrpaR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPUMBIU")
            .field("wpgrpa", &self.wpgrpa())
            .field("rpgrpa", &self.rpgrpa())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - Master Group A Read protection"]
    #[inline(always)]
    pub fn rpgrpa(&mut self) -> RpgrpaW<SmpumbiuSpec> {
        RpgrpaW::new(self, 2)
    }
    #[doc = "Bit 3 - Master Group A Write protection"]
    #[inline(always)]
    pub fn wpgrpa(&mut self) -> WpgrpaW<SmpumbiuSpec> {
        WpgrpaW::new(self, 3)
    }
}
#[doc = "Access Control Register for MBIU\n\nYou can [`read`](crate::Reg::read) this register and get [`smpumbiu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpumbiu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmpumbiuSpec;
impl crate::RegisterSpec for SmpumbiuSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`smpumbiu::R`](R) reader structure"]
impl crate::Readable for SmpumbiuSpec {}
#[doc = "`write(|w| ..)` method takes [`smpumbiu::W`](W) writer structure"]
impl crate::Writable for SmpumbiuSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMPUMBIU to value 0"]
impl crate::Resettable for SmpumbiuSpec {}
