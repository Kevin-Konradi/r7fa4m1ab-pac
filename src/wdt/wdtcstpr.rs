#[doc = "Register `WDTCSTPR` reader"]
pub type R = crate::R<WdtcstprSpec>;
#[doc = "Register `WDTCSTPR` writer"]
pub type W = crate::W<WdtcstprSpec>;
#[doc = "Sleep-Mode Count Stop Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slcstp {
    #[doc = "0: Count stop is disabled."]
    _0 = 0,
    #[doc = "1: Count is stopped at a transition to sleep mode."]
    _1 = 1,
}
impl From<Slcstp> for bool {
    #[inline(always)]
    fn from(variant: Slcstp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLCSTP` reader - Sleep-Mode Count Stop Control"]
pub type SlcstpR = crate::BitReader<Slcstp>;
impl SlcstpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slcstp {
        match self.bits {
            false => Slcstp::_0,
            true => Slcstp::_1,
        }
    }
    #[doc = "Count stop is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Slcstp::_0
    }
    #[doc = "Count is stopped at a transition to sleep mode."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Slcstp::_1
    }
}
#[doc = "Field `SLCSTP` writer - Sleep-Mode Count Stop Control"]
pub type SlcstpW<'a, REG> = crate::BitWriter<'a, REG, Slcstp>;
impl<'a, REG> SlcstpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Count stop is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Slcstp::_0)
    }
    #[doc = "Count is stopped at a transition to sleep mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Slcstp::_1)
    }
}
impl R {
    #[doc = "Bit 7 - Sleep-Mode Count Stop Control"]
    #[inline(always)]
    pub fn slcstp(&self) -> SlcstpR {
        SlcstpR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDTCSTPR")
            .field("slcstp", &self.slcstp())
            .finish()
    }
}
impl W {
    #[doc = "Bit 7 - Sleep-Mode Count Stop Control"]
    #[inline(always)]
    pub fn slcstp(&mut self) -> SlcstpW<WdtcstprSpec> {
        SlcstpW::new(self, 7)
    }
}
#[doc = "WDT Count Stop Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtcstpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtcstpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtcstprSpec;
impl crate::RegisterSpec for WdtcstprSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wdtcstpr::R`](R) reader structure"]
impl crate::Readable for WdtcstprSpec {}
#[doc = "`write(|w| ..)` method takes [`wdtcstpr::W`](W) writer structure"]
impl crate::Writable for WdtcstprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDTCSTPR to value 0x80"]
impl crate::Resettable for WdtcstprSpec {
    const RESET_VALUE: u8 = 0x80;
}
