#[doc = "Register `AGTISR` reader"]
pub type R = crate::R<AgtisrSpec>;
#[doc = "Register `AGTISR` writer"]
pub type W = crate::W<AgtisrSpec>;
#[doc = "AGTEE polarty selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eeps {
    #[doc = "0: An event is counted during the low-level period"]
    _0 = 0,
    #[doc = "1: An event is counted during the high-level period"]
    _1 = 1,
}
impl From<Eeps> for bool {
    #[inline(always)]
    fn from(variant: Eeps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEPS` reader - AGTEE polarty selection"]
pub type EepsR = crate::BitReader<Eeps>;
impl EepsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eeps {
        match self.bits {
            false => Eeps::_0,
            true => Eeps::_1,
        }
    }
    #[doc = "An event is counted during the low-level period"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Eeps::_0
    }
    #[doc = "An event is counted during the high-level period"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Eeps::_1
    }
}
#[doc = "Field `EEPS` writer - AGTEE polarty selection"]
pub type EepsW<'a, REG> = crate::BitWriter<'a, REG, Eeps>;
impl<'a, REG> EepsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "An event is counted during the low-level period"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Eeps::_0)
    }
    #[doc = "An event is counted during the high-level period"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Eeps::_1)
    }
}
impl R {
    #[doc = "Bit 2 - AGTEE polarty selection"]
    #[inline(always)]
    pub fn eeps(&self) -> EepsR {
        EepsR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AGTISR")
            .field("eeps", &self.eeps())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - AGTEE polarty selection"]
    #[inline(always)]
    pub fn eeps(&mut self) -> EepsW<AgtisrSpec> {
        EepsW::new(self, 2)
    }
}
#[doc = "AGT Event Pin Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`agtisr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtisr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AgtisrSpec;
impl crate::RegisterSpec for AgtisrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`agtisr::R`](R) reader structure"]
impl crate::Readable for AgtisrSpec {}
#[doc = "`write(|w| ..)` method takes [`agtisr::W`](W) writer structure"]
impl crate::Writable for AgtisrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AGTISR to value 0"]
impl crate::Resettable for AgtisrSpec {}
