#[doc = "Register `ECC1STSEN` reader"]
pub type R = crate::R<Ecc1stsenSpec>;
#[doc = "Register `ECC1STSEN` writer"]
pub type W = crate::W<Ecc1stsenSpec>;
#[doc = "ECC 1-Bit Error Information Update Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum E1stsen {
    #[doc = "0: Disables updating of the 1-bit ECC error information."]
    _0 = 0,
    #[doc = "1: Enables updating of the 1-bit ECC error information."]
    _1 = 1,
}
impl From<E1stsen> for bool {
    #[inline(always)]
    fn from(variant: E1stsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E1STSEN` reader - ECC 1-Bit Error Information Update Enable"]
pub type E1stsenR = crate::BitReader<E1stsen>;
impl E1stsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> E1stsen {
        match self.bits {
            false => E1stsen::_0,
            true => E1stsen::_1,
        }
    }
    #[doc = "Disables updating of the 1-bit ECC error information."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == E1stsen::_0
    }
    #[doc = "Enables updating of the 1-bit ECC error information."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == E1stsen::_1
    }
}
#[doc = "Field `E1STSEN` writer - ECC 1-Bit Error Information Update Enable"]
pub type E1stsenW<'a, REG> = crate::BitWriter<'a, REG, E1stsen>;
impl<'a, REG> E1stsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables updating of the 1-bit ECC error information."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(E1stsen::_0)
    }
    #[doc = "Enables updating of the 1-bit ECC error information."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(E1stsen::_1)
    }
}
impl R {
    #[doc = "Bit 0 - ECC 1-Bit Error Information Update Enable"]
    #[inline(always)]
    pub fn e1stsen(&self) -> E1stsenR {
        E1stsenR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECC1STSEN")
            .field("e1stsen", &self.e1stsen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - ECC 1-Bit Error Information Update Enable"]
    #[inline(always)]
    pub fn e1stsen(&mut self) -> E1stsenW<Ecc1stsenSpec> {
        E1stsenW::new(self, 0)
    }
}
#[doc = "ECC 1-Bit Error Information Update Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc1stsen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc1stsen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ecc1stsenSpec;
impl crate::RegisterSpec for Ecc1stsenSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ecc1stsen::R`](R) reader structure"]
impl crate::Readable for Ecc1stsenSpec {}
#[doc = "`write(|w| ..)` method takes [`ecc1stsen::W`](W) writer structure"]
impl crate::Writable for Ecc1stsenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECC1STSEN to value 0"]
impl crate::Resettable for Ecc1stsenSpec {}
