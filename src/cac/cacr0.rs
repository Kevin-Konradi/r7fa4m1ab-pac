#[doc = "Register `CACR0` reader"]
pub type R = crate::R<Cacr0Spec>;
#[doc = "Register `CACR0` writer"]
pub type W = crate::W<Cacr0Spec>;
#[doc = "Clock Frequency Measurement Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfme {
    #[doc = "0: Disable"]
    _0 = 0,
    #[doc = "1: Enable"]
    _1 = 1,
}
impl From<Cfme> for bool {
    #[inline(always)]
    fn from(variant: Cfme) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFME` reader - Clock Frequency Measurement Enable."]
pub type CfmeR = crate::BitReader<Cfme>;
impl CfmeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cfme {
        match self.bits {
            false => Cfme::_0,
            true => Cfme::_1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cfme::_0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cfme::_1
    }
}
#[doc = "Field `CFME` writer - Clock Frequency Measurement Enable."]
pub type CfmeW<'a, REG> = crate::BitWriter<'a, REG, Cfme>;
impl<'a, REG> CfmeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cfme::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfme::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Clock Frequency Measurement Enable."]
    #[inline(always)]
    pub fn cfme(&self) -> CfmeR {
        CfmeR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACR0").field("cfme", &self.cfme()).finish()
    }
}
impl W {
    #[doc = "Bit 0 - Clock Frequency Measurement Enable."]
    #[inline(always)]
    pub fn cfme(&mut self) -> CfmeW<Cacr0Spec> {
        CfmeW::new(self, 0)
    }
}
#[doc = "CAC Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cacr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cacr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cacr0Spec;
impl crate::RegisterSpec for Cacr0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cacr0::R`](R) reader structure"]
impl crate::Readable for Cacr0Spec {}
#[doc = "`write(|w| ..)` method takes [`cacr0::W`](W) writer structure"]
impl crate::Writable for Cacr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACR0 to value 0"]
impl crate::Resettable for Cacr0Spec {}
