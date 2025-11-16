#[doc = "Register `ECC1STS` reader"]
pub type R = crate::R<Ecc1stsSpec>;
#[doc = "Register `ECC1STS` writer"]
pub type W = crate::W<Ecc1stsSpec>;
#[doc = "ECC 1-Bit Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecc1err {
    #[doc = "0: No 1-bit ECC error occurred"]
    _0 = 0,
    #[doc = "1: 1-bit ECC error occurred"]
    _1 = 1,
}
impl From<Ecc1err> for bool {
    #[inline(always)]
    fn from(variant: Ecc1err) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECC1ERR` reader - ECC 1-Bit Error Status\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Ecc1errR = crate::BitReader<Ecc1err>;
impl Ecc1errR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ecc1err {
        match self.bits {
            false => Ecc1err::_0,
            true => Ecc1err::_1,
        }
    }
    #[doc = "No 1-bit ECC error occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ecc1err::_0
    }
    #[doc = "1-bit ECC error occurred"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ecc1err::_1
    }
}
#[doc = "Field `ECC1ERR` writer - ECC 1-Bit Error Status"]
pub type Ecc1errW<'a, REG> = crate::BitWriter0C<'a, REG, Ecc1err>;
impl<'a, REG> Ecc1errW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No 1-bit ECC error occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ecc1err::_0)
    }
    #[doc = "1-bit ECC error occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ecc1err::_1)
    }
}
impl R {
    #[doc = "Bit 0 - ECC 1-Bit Error Status"]
    #[inline(always)]
    pub fn ecc1err(&self) -> Ecc1errR {
        Ecc1errR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECC1STS").finish()
    }
}
impl W {
    #[doc = "Bit 0 - ECC 1-Bit Error Status"]
    #[inline(always)]
    pub fn ecc1err(&mut self) -> Ecc1errW<Ecc1stsSpec> {
        Ecc1errW::new(self, 0)
    }
}
#[doc = "ECC 1-Bit Error Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc1sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc1sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ecc1stsSpec;
impl crate::RegisterSpec for Ecc1stsSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ecc1sts::R`](R) reader structure"]
impl crate::Readable for Ecc1stsSpec {}
#[doc = "`write(|w| ..)` method takes [`ecc1sts::W`](W) writer structure"]
impl crate::Writable for Ecc1stsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0x01;
}
#[doc = "`reset()` method sets ECC1STS to value 0"]
impl crate::Resettable for Ecc1stsSpec {}
