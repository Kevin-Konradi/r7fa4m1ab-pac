#[doc = "Register `SSLND` reader"]
pub type R = crate::R<SslndSpec>;
#[doc = "Register `SSLND` writer"]
pub type W = crate::W<SslndSpec>;
#[doc = "SSL Negation Delay Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Slndl {
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
impl From<Slndl> for u8 {
    #[inline(always)]
    fn from(variant: Slndl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Slndl {
    type Ux = u8;
}
impl crate::IsEnum for Slndl {}
#[doc = "Field `SLNDL` reader - SSL Negation Delay Setting"]
pub type SlndlR = crate::FieldReader<Slndl>;
impl SlndlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slndl {
        match self.bits {
            0 => Slndl::_000,
            1 => Slndl::_001,
            2 => Slndl::_010,
            3 => Slndl::_011,
            4 => Slndl::_100,
            5 => Slndl::_101,
            6 => Slndl::_110,
            7 => Slndl::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "1 RSPCK"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Slndl::_000
    }
    #[doc = "2 RSPCK"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Slndl::_001
    }
    #[doc = "3 RSPCK"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Slndl::_010
    }
    #[doc = "4 RSPCK"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Slndl::_011
    }
    #[doc = "5 RSPCK"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Slndl::_100
    }
    #[doc = "6 RSPCK"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Slndl::_101
    }
    #[doc = "7 RSPCK"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Slndl::_110
    }
    #[doc = "8 RSPCK"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Slndl::_111
    }
}
#[doc = "Field `SLNDL` writer - SSL Negation Delay Setting"]
pub type SlndlW<'a, REG> = crate::FieldWriter<'a, REG, 3, Slndl, crate::Safe>;
impl<'a, REG> SlndlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 RSPCK"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Slndl::_000)
    }
    #[doc = "2 RSPCK"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Slndl::_001)
    }
    #[doc = "3 RSPCK"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Slndl::_010)
    }
    #[doc = "4 RSPCK"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Slndl::_011)
    }
    #[doc = "5 RSPCK"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Slndl::_100)
    }
    #[doc = "6 RSPCK"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Slndl::_101)
    }
    #[doc = "7 RSPCK"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Slndl::_110)
    }
    #[doc = "8 RSPCK"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Slndl::_111)
    }
}
impl R {
    #[doc = "Bits 0:2 - SSL Negation Delay Setting"]
    #[inline(always)]
    pub fn slndl(&self) -> SlndlR {
        SlndlR::new(self.bits & 7)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSLND")
            .field("slndl", &self.slndl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - SSL Negation Delay Setting"]
    #[inline(always)]
    pub fn slndl(&mut self) -> SlndlW<SslndSpec> {
        SlndlW::new(self, 0)
    }
}
#[doc = "SPI Slave Select Negation Delay Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sslnd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sslnd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SslndSpec;
impl crate::RegisterSpec for SslndSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sslnd::R`](R) reader structure"]
impl crate::Readable for SslndSpec {}
#[doc = "`write(|w| ..)` method takes [`sslnd::W`](W) writer structure"]
impl crate::Writable for SslndSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSLND to value 0"]
impl crate::Resettable for SslndSpec {}
