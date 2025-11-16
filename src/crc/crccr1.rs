#[doc = "Register `CRCCR1` reader"]
pub type R = crate::R<Crccr1Spec>;
#[doc = "Register `CRCCR1` writer"]
pub type W = crate::W<Crccr1Spec>;
#[doc = "Snoop-on-write/read switch bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crcswr {
    #[doc = "0: Snoop-on-read"]
    _0 = 0,
    #[doc = "1: Snoop-on-write"]
    _1 = 1,
}
impl From<Crcswr> for bool {
    #[inline(always)]
    fn from(variant: Crcswr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCSWR` reader - Snoop-on-write/read switch bit"]
pub type CrcswrR = crate::BitReader<Crcswr>;
impl CrcswrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crcswr {
        match self.bits {
            false => Crcswr::_0,
            true => Crcswr::_1,
        }
    }
    #[doc = "Snoop-on-read"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Crcswr::_0
    }
    #[doc = "Snoop-on-write"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Crcswr::_1
    }
}
#[doc = "Field `CRCSWR` writer - Snoop-on-write/read switch bit"]
pub type CrcswrW<'a, REG> = crate::BitWriter<'a, REG, Crcswr>;
impl<'a, REG> CrcswrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Snoop-on-read"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Crcswr::_0)
    }
    #[doc = "Snoop-on-write"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Crcswr::_1)
    }
}
#[doc = "Snoop enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crcsen {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<Crcsen> for bool {
    #[inline(always)]
    fn from(variant: Crcsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCSEN` reader - Snoop enable bit"]
pub type CrcsenR = crate::BitReader<Crcsen>;
impl CrcsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crcsen {
        match self.bits {
            false => Crcsen::_0,
            true => Crcsen::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Crcsen::_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Crcsen::_1
    }
}
#[doc = "Field `CRCSEN` writer - Snoop enable bit"]
pub type CrcsenW<'a, REG> = crate::BitWriter<'a, REG, Crcsen>;
impl<'a, REG> CrcsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsen::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsen::_1)
    }
}
impl R {
    #[doc = "Bit 6 - Snoop-on-write/read switch bit"]
    #[inline(always)]
    pub fn crcswr(&self) -> CrcswrR {
        CrcswrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Snoop enable bit"]
    #[inline(always)]
    pub fn crcsen(&self) -> CrcsenR {
        CrcsenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRCCR1")
            .field("crcsen", &self.crcsen())
            .field("crcswr", &self.crcswr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 6 - Snoop-on-write/read switch bit"]
    #[inline(always)]
    pub fn crcswr(&mut self) -> CrcswrW<Crccr1Spec> {
        CrcswrW::new(self, 6)
    }
    #[doc = "Bit 7 - Snoop enable bit"]
    #[inline(always)]
    pub fn crcsen(&mut self) -> CrcsenW<Crccr1Spec> {
        CrcsenW::new(self, 7)
    }
}
#[doc = "CRC Control Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`crccr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crccr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Crccr1Spec;
impl crate::RegisterSpec for Crccr1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`crccr1::R`](R) reader structure"]
impl crate::Readable for Crccr1Spec {}
#[doc = "`write(|w| ..)` method takes [`crccr1::W`](W) writer structure"]
impl crate::Writable for Crccr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCCR1 to value 0"]
impl crate::Resettable for Crccr1Spec {}
