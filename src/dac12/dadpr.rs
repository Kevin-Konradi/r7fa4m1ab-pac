#[doc = "Register `DADPR` reader"]
pub type R = crate::R<DadprSpec>;
#[doc = "Register `DADPR` writer"]
pub type W = crate::W<DadprSpec>;
#[doc = "DADRm Format Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpsel {
    #[doc = "0: Right justified format."]
    _0 = 0,
    #[doc = "1: Left justified format."]
    _1 = 1,
}
impl From<Dpsel> for bool {
    #[inline(always)]
    fn from(variant: Dpsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPSEL` reader - DADRm Format Select"]
pub type DpselR = crate::BitReader<Dpsel>;
impl DpselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dpsel {
        match self.bits {
            false => Dpsel::_0,
            true => Dpsel::_1,
        }
    }
    #[doc = "Right justified format."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dpsel::_0
    }
    #[doc = "Left justified format."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dpsel::_1
    }
}
#[doc = "Field `DPSEL` writer - DADRm Format Select"]
pub type DpselW<'a, REG> = crate::BitWriter<'a, REG, Dpsel>;
impl<'a, REG> DpselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Right justified format."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dpsel::_0)
    }
    #[doc = "Left justified format."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpsel::_1)
    }
}
impl R {
    #[doc = "Bit 7 - DADRm Format Select"]
    #[inline(always)]
    pub fn dpsel(&self) -> DpselR {
        DpselR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DADPR")
            .field("dpsel", &self.dpsel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 7 - DADRm Format Select"]
    #[inline(always)]
    pub fn dpsel(&mut self) -> DpselW<DadprSpec> {
        DpselW::new(self, 7)
    }
}
#[doc = "DADR0 Format Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dadpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dadpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DadprSpec;
impl crate::RegisterSpec for DadprSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dadpr::R`](R) reader structure"]
impl crate::Readable for DadprSpec {}
#[doc = "`write(|w| ..)` method takes [`dadpr::W`](W) writer structure"]
impl crate::Writable for DadprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DADPR to value 0"]
impl crate::Resettable for DadprSpec {}
