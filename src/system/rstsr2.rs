#[doc = "Register `RSTSR2` reader"]
pub type R = crate::R<Rstsr2Spec>;
#[doc = "Register `RSTSR2` writer"]
pub type W = crate::W<Rstsr2Spec>;
#[doc = "Cold/Warm Start Determination Flag Note: Only 1 can be written to set the flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cwsf {
    #[doc = "0: Cold start"]
    _0 = 0,
    #[doc = "1: Warm start"]
    _1 = 1,
}
impl From<Cwsf> for bool {
    #[inline(always)]
    fn from(variant: Cwsf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CWSF` reader - Cold/Warm Start Determination Flag Note: Only 1 can be written to set the flag.\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type CwsfR = crate::BitReader<Cwsf>;
impl CwsfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cwsf {
        match self.bits {
            false => Cwsf::_0,
            true => Cwsf::_1,
        }
    }
    #[doc = "Cold start"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cwsf::_0
    }
    #[doc = "Warm start"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cwsf::_1
    }
}
#[doc = "Field `CWSF` writer - Cold/Warm Start Determination Flag Note: Only 1 can be written to set the flag."]
pub type CwsfW<'a, REG> = crate::BitWriter1S<'a, REG, Cwsf>;
impl<'a, REG> CwsfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cold start"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cwsf::_0)
    }
    #[doc = "Warm start"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cwsf::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Cold/Warm Start Determination Flag Note: Only 1 can be written to set the flag."]
    #[inline(always)]
    pub fn cwsf(&self) -> CwsfR {
        CwsfR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSTSR2").finish()
    }
}
impl W {
    #[doc = "Bit 0 - Cold/Warm Start Determination Flag Note: Only 1 can be written to set the flag."]
    #[inline(always)]
    pub fn cwsf(&mut self) -> CwsfW<Rstsr2Spec> {
        CwsfW::new(self, 0)
    }
}
#[doc = "Reset Status Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`rstsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rstsr2Spec;
impl crate::RegisterSpec for Rstsr2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rstsr2::R`](R) reader structure"]
impl crate::Readable for Rstsr2Spec {}
#[doc = "`write(|w| ..)` method takes [`rstsr2::W`](W) writer structure"]
impl crate::Writable for Rstsr2Spec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0x01;
}
#[doc = "`reset()` method sets RSTSR2 to value 0"]
impl crate::Resettable for Rstsr2Spec {}
