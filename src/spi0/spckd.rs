#[doc = "Register `SPCKD` reader"]
pub type R = crate::R<SpckdSpec>;
#[doc = "Register `SPCKD` writer"]
pub type W = crate::W<SpckdSpec>;
#[doc = "RSPCK Delay Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sckdl {
    #[doc = "0: 1 RSPCK"]
    _000 = 0,
    #[doc = "1: 2 RSPCK"]
    _001 = 1,
    #[doc = "2: 3 RSPCK"]
    _010 = 2,
    #[doc = "3: 4 RSPCK"]
    _011 = 3,
    #[doc = "4: 5 RSPCK"]
    _100 = 4,
    #[doc = "5: 6 RSPCK"]
    _101 = 5,
    #[doc = "6: 7 RSPCK"]
    _110 = 6,
    #[doc = "7: 8 RSPCK"]
    _111 = 7,
}
impl From<Sckdl> for u8 {
    #[inline(always)]
    fn from(variant: Sckdl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sckdl {
    type Ux = u8;
}
impl crate::IsEnum for Sckdl {}
#[doc = "Field `SCKDL` reader - RSPCK Delay Setting"]
pub type SckdlR = crate::FieldReader<Sckdl>;
impl SckdlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sckdl {
        match self.bits {
            0 => Sckdl::_000,
            1 => Sckdl::_001,
            2 => Sckdl::_010,
            3 => Sckdl::_011,
            4 => Sckdl::_100,
            5 => Sckdl::_101,
            6 => Sckdl::_110,
            7 => Sckdl::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "1 RSPCK"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Sckdl::_000
    }
    #[doc = "2 RSPCK"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Sckdl::_001
    }
    #[doc = "3 RSPCK"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Sckdl::_010
    }
    #[doc = "4 RSPCK"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Sckdl::_011
    }
    #[doc = "5 RSPCK"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Sckdl::_100
    }
    #[doc = "6 RSPCK"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Sckdl::_101
    }
    #[doc = "7 RSPCK"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Sckdl::_110
    }
    #[doc = "8 RSPCK"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Sckdl::_111
    }
}
#[doc = "Field `SCKDL` writer - RSPCK Delay Setting"]
pub type SckdlW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sckdl, crate::Safe>;
impl<'a, REG> SckdlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 RSPCK"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Sckdl::_000)
    }
    #[doc = "2 RSPCK"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Sckdl::_001)
    }
    #[doc = "3 RSPCK"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Sckdl::_010)
    }
    #[doc = "4 RSPCK"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Sckdl::_011)
    }
    #[doc = "5 RSPCK"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Sckdl::_100)
    }
    #[doc = "6 RSPCK"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Sckdl::_101)
    }
    #[doc = "7 RSPCK"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Sckdl::_110)
    }
    #[doc = "8 RSPCK"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Sckdl::_111)
    }
}
impl R {
    #[doc = "Bits 0:2 - RSPCK Delay Setting"]
    #[inline(always)]
    pub fn sckdl(&self) -> SckdlR {
        SckdlR::new(self.bits & 7)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPCKD")
            .field("sckdl", &self.sckdl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - RSPCK Delay Setting"]
    #[inline(always)]
    pub fn sckdl(&mut self) -> SckdlW<SpckdSpec> {
        SckdlW::new(self, 0)
    }
}
#[doc = "SPI Clock Delay Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spckd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spckd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpckdSpec;
impl crate::RegisterSpec for SpckdSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spckd::R`](R) reader structure"]
impl crate::Readable for SpckdSpec {}
#[doc = "`write(|w| ..)` method takes [`spckd::W`](W) writer structure"]
impl crate::Writable for SpckdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPCKD to value 0"]
impl crate::Resettable for SpckdSpec {}
