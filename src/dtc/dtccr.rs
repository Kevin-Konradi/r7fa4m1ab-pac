#[doc = "Register `DTCCR` reader"]
pub type R = crate::R<DtccrSpec>;
#[doc = "Register `DTCCR` writer"]
pub type W = crate::W<DtccrSpec>;
#[doc = "DTC Transfer Information Read Skip Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rrs {
    #[doc = "0: Do not skip transfer information read"]
    _0 = 0,
    #[doc = "1: Skip transfer information read when vector numbers match"]
    _1 = 1,
}
impl From<Rrs> for bool {
    #[inline(always)]
    fn from(variant: Rrs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RRS` reader - DTC Transfer Information Read Skip Enable."]
pub type RrsR = crate::BitReader<Rrs>;
impl RrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rrs {
        match self.bits {
            false => Rrs::_0,
            true => Rrs::_1,
        }
    }
    #[doc = "Do not skip transfer information read"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rrs::_0
    }
    #[doc = "Skip transfer information read when vector numbers match"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rrs::_1
    }
}
#[doc = "Field `RRS` writer - DTC Transfer Information Read Skip Enable."]
pub type RrsW<'a, REG> = crate::BitWriter<'a, REG, Rrs>;
impl<'a, REG> RrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not skip transfer information read"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rrs::_0)
    }
    #[doc = "Skip transfer information read when vector numbers match"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rrs::_1)
    }
}
impl R {
    #[doc = "Bit 4 - DTC Transfer Information Read Skip Enable."]
    #[inline(always)]
    pub fn rrs(&self) -> RrsR {
        RrsR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTCCR").field("rrs", &self.rrs()).finish()
    }
}
impl W {
    #[doc = "Bit 4 - DTC Transfer Information Read Skip Enable."]
    #[inline(always)]
    pub fn rrs(&mut self) -> RrsW<DtccrSpec> {
        RrsW::new(self, 4)
    }
}
#[doc = "DTC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtccrSpec;
impl crate::RegisterSpec for DtccrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dtccr::R`](R) reader structure"]
impl crate::Readable for DtccrSpec {}
#[doc = "`write(|w| ..)` method takes [`dtccr::W`](W) writer structure"]
impl crate::Writable for DtccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTCCR to value 0x08"]
impl crate::Resettable for DtccrSpec {
    const RESET_VALUE: u8 = 0x08;
}
