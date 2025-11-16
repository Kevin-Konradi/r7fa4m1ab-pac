#[doc = "Register `GTDVU` reader"]
pub type R = crate::R<GtdvuSpec>;
#[doc = "Register `GTDVU` writer"]
pub type W = crate::W<GtdvuSpec>;
#[doc = "Field `GTDVU` reader - Dead Time Value Register U"]
pub type GtdvuR = crate::FieldReader<u32>;
#[doc = "Field `GTDVU` writer - Dead Time Value Register U"]
pub type GtdvuW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Dead Time Value Register U"]
    #[inline(always)]
    pub fn gtdvu(&self) -> GtdvuR {
        GtdvuR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTDVU")
            .field("gtdvu", &self.gtdvu())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Dead Time Value Register U"]
    #[inline(always)]
    pub fn gtdvu(&mut self) -> GtdvuW<GtdvuSpec> {
        GtdvuW::new(self, 0)
    }
}
#[doc = "General PWM Timer Dead Time Value Register U\n\nYou can [`read`](crate::Reg::read) this register and get [`gtdvu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtdvu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtdvuSpec;
impl crate::RegisterSpec for GtdvuSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtdvu::R`](R) reader structure"]
impl crate::Readable for GtdvuSpec {}
#[doc = "`write(|w| ..)` method takes [`gtdvu::W`](W) writer structure"]
impl crate::Writable for GtdvuSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GTDVU to value 0xffff"]
impl crate::Resettable for GtdvuSpec {
    const RESET_VALUE: u32 = 0xffff;
}
