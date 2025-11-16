#[doc = "Register `WDTRR` reader"]
pub type R = crate::R<WdtrrSpec>;
#[doc = "Register `WDTRR` writer"]
pub type W = crate::W<WdtrrSpec>;
#[doc = "Field `WDTRR` reader - WDTRR is an 8-bit register that refreshes the down-counter of the WDT."]
pub type WdtrrR = crate::FieldReader;
#[doc = "Field `WDTRR` writer - WDTRR is an 8-bit register that refreshes the down-counter of the WDT."]
pub type WdtrrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - WDTRR is an 8-bit register that refreshes the down-counter of the WDT."]
    #[inline(always)]
    pub fn wdtrr(&self) -> WdtrrR {
        WdtrrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDTRR")
            .field("wdtrr", &self.wdtrr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - WDTRR is an 8-bit register that refreshes the down-counter of the WDT."]
    #[inline(always)]
    pub fn wdtrr(&mut self) -> WdtrrW<WdtrrSpec> {
        WdtrrW::new(self, 0)
    }
}
#[doc = "WDT Refresh Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtrrSpec;
impl crate::RegisterSpec for WdtrrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wdtrr::R`](R) reader structure"]
impl crate::Readable for WdtrrSpec {}
#[doc = "`write(|w| ..)` method takes [`wdtrr::W`](W) writer structure"]
impl crate::Writable for WdtrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDTRR to value 0xff"]
impl crate::Resettable for WdtrrSpec {
    const RESET_VALUE: u8 = 0xff;
}
