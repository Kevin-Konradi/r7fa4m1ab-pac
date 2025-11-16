#[doc = "Register `FCACHEIV` reader"]
pub type R = crate::R<FcacheivSpec>;
#[doc = "Register `FCACHEIV` writer"]
pub type W = crate::W<FcacheivSpec>;
#[doc = "FCACHE Invalidation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fcacheiv {
    #[doc = "0: (Read)not in progress / (Write) no effect."]
    _0 = 0,
    #[doc = "1: (Read)in progress /(Write) Starting Cache Invalidation"]
    _1 = 1,
}
impl From<Fcacheiv> for bool {
    #[inline(always)]
    fn from(variant: Fcacheiv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCACHEIV` reader - FCACHE Invalidation\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type FcacheivR = crate::BitReader<Fcacheiv>;
impl FcacheivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fcacheiv {
        match self.bits {
            false => Fcacheiv::_0,
            true => Fcacheiv::_1,
        }
    }
    #[doc = "(Read)not in progress / (Write) no effect."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fcacheiv::_0
    }
    #[doc = "(Read)in progress /(Write) Starting Cache Invalidation"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fcacheiv::_1
    }
}
#[doc = "Field `FCACHEIV` writer - FCACHE Invalidation"]
pub type FcacheivW<'a, REG> = crate::BitWriter1S<'a, REG, Fcacheiv>;
impl<'a, REG> FcacheivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "(Read)not in progress / (Write) no effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fcacheiv::_0)
    }
    #[doc = "(Read)in progress /(Write) Starting Cache Invalidation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fcacheiv::_1)
    }
}
impl R {
    #[doc = "Bit 0 - FCACHE Invalidation"]
    #[inline(always)]
    pub fn fcacheiv(&self) -> FcacheivR {
        FcacheivR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCACHEIV").finish()
    }
}
impl W {
    #[doc = "Bit 0 - FCACHE Invalidation"]
    #[inline(always)]
    pub fn fcacheiv(&mut self) -> FcacheivW<FcacheivSpec> {
        FcacheivW::new(self, 0)
    }
}
#[doc = "Flash Cache Invalidate Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcacheiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcacheiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcacheivSpec;
impl crate::RegisterSpec for FcacheivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fcacheiv::R`](R) reader structure"]
impl crate::Readable for FcacheivSpec {}
#[doc = "`write(|w| ..)` method takes [`fcacheiv::W`](W) writer structure"]
impl crate::Writable for FcacheivSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0x01;
}
#[doc = "`reset()` method sets FCACHEIV to value 0"]
impl crate::Resettable for FcacheivSpec {}
