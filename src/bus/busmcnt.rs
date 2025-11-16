#[doc = "Register `BUSMCNT%s` reader"]
pub type R = crate::R<BusmcntSpec>;
#[doc = "Register `BUSMCNT%s` writer"]
pub type W = crate::W<BusmcntSpec>;
#[doc = "Ignore Error Responses\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ieres {
    #[doc = "0: A bus error is reported"]
    _0 = 0,
    #[doc = "1: A bus error is not reported."]
    _1 = 1,
}
impl From<Ieres> for bool {
    #[inline(always)]
    fn from(variant: Ieres) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IERES` reader - Ignore Error Responses"]
pub type IeresR = crate::BitReader<Ieres>;
impl IeresR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ieres {
        match self.bits {
            false => Ieres::_0,
            true => Ieres::_1,
        }
    }
    #[doc = "A bus error is reported"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ieres::_0
    }
    #[doc = "A bus error is not reported."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ieres::_1
    }
}
#[doc = "Field `IERES` writer - Ignore Error Responses"]
pub type IeresW<'a, REG> = crate::BitWriter<'a, REG, Ieres>;
impl<'a, REG> IeresW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A bus error is reported"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ieres::_0)
    }
    #[doc = "A bus error is not reported."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ieres::_1)
    }
}
impl R {
    #[doc = "Bit 15 - Ignore Error Responses"]
    #[inline(always)]
    pub fn ieres(&self) -> IeresR {
        IeresR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUSMCNT")
            .field("ieres", &self.ieres())
            .finish()
    }
}
impl W {
    #[doc = "Bit 15 - Ignore Error Responses"]
    #[inline(always)]
    pub fn ieres(&mut self) -> IeresW<BusmcntSpec> {
        IeresW::new(self, 15)
    }
}
#[doc = "Master Bus Control Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`busmcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busmcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BusmcntSpec;
impl crate::RegisterSpec for BusmcntSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`busmcnt::R`](R) reader structure"]
impl crate::Readable for BusmcntSpec {}
#[doc = "`write(|w| ..)` method takes [`busmcnt::W`](W) writer structure"]
impl crate::Writable for BusmcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUSMCNT%s to value 0"]
impl crate::Resettable for BusmcntSpec {}
