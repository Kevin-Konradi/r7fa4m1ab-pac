#[doc = "Register `ECC2STS` reader"]
pub type R = crate::R<Ecc2stsSpec>;
#[doc = "Register `ECC2STS` writer"]
pub type W = crate::W<Ecc2stsSpec>;
#[doc = "ECC 2-Bit Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecc2err {
    #[doc = "0: No 2-bit ECC error occurred"]
    _0 = 0,
    #[doc = "1: 2-bit ECC error occurred."]
    _1 = 1,
}
impl From<Ecc2err> for bool {
    #[inline(always)]
    fn from(variant: Ecc2err) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECC2ERR` reader - ECC 2-Bit Error Status\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type Ecc2errR = crate::BitReader<Ecc2err>;
impl Ecc2errR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ecc2err {
        match self.bits {
            false => Ecc2err::_0,
            true => Ecc2err::_1,
        }
    }
    #[doc = "No 2-bit ECC error occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ecc2err::_0
    }
    #[doc = "2-bit ECC error occurred."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ecc2err::_1
    }
}
#[doc = "Field `ECC2ERR` writer - ECC 2-Bit Error Status"]
pub type Ecc2errW<'a, REG> = crate::BitWriter0C<'a, REG, Ecc2err>;
impl<'a, REG> Ecc2errW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No 2-bit ECC error occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ecc2err::_0)
    }
    #[doc = "2-bit ECC error occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ecc2err::_1)
    }
}
impl R {
    #[doc = "Bit 0 - ECC 2-Bit Error Status"]
    #[inline(always)]
    pub fn ecc2err(&self) -> Ecc2errR {
        Ecc2errR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECC2STS").finish()
    }
}
impl W {
    #[doc = "Bit 0 - ECC 2-Bit Error Status"]
    #[inline(always)]
    pub fn ecc2err(&mut self) -> Ecc2errW<Ecc2stsSpec> {
        Ecc2errW::new(self, 0)
    }
}
#[doc = "ECC 2-Bit Error Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc2sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc2sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ecc2stsSpec;
impl crate::RegisterSpec for Ecc2stsSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ecc2sts::R`](R) reader structure"]
impl crate::Readable for Ecc2stsSpec {}
#[doc = "`write(|w| ..)` method takes [`ecc2sts::W`](W) writer structure"]
impl crate::Writable for Ecc2stsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0x01;
}
#[doc = "`reset()` method sets ECC2STS to value 0"]
impl crate::Resettable for Ecc2stsSpec {}
