#[doc = "Register `DMOFR` reader"]
pub type R = crate::R<DmofrSpec>;
#[doc = "Register `DMOFR` writer"]
pub type W = crate::W<DmofrSpec>;
#[doc = "Field `DMOFR` reader - Specifies the offset when offset addition is selected as the address update mode for transfer source or destination."]
pub type DmofrR = crate::FieldReader<u32>;
#[doc = "Field `DMOFR` writer - Specifies the offset when offset addition is selected as the address update mode for transfer source or destination."]
pub type DmofrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies the offset when offset addition is selected as the address update mode for transfer source or destination."]
    #[inline(always)]
    pub fn dmofr(&self) -> DmofrR {
        DmofrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMOFR")
            .field("dmofr", &self.dmofr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies the offset when offset addition is selected as the address update mode for transfer source or destination."]
    #[inline(always)]
    pub fn dmofr(&mut self) -> DmofrW<DmofrSpec> {
        DmofrW::new(self, 0)
    }
}
#[doc = "DMA Offset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmofr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmofr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmofrSpec;
impl crate::RegisterSpec for DmofrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmofr::R`](R) reader structure"]
impl crate::Readable for DmofrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmofr::W`](W) writer structure"]
impl crate::Writable for DmofrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMOFR to value 0"]
impl crate::Resettable for DmofrSpec {}
