#[doc = "Register `DMSAR` reader"]
pub type R = crate::R<DmsarSpec>;
#[doc = "Register `DMSAR` writer"]
pub type W = crate::W<DmsarSpec>;
#[doc = "Field `DMSAR` reader - Specifies the transfer source start address."]
pub type DmsarR = crate::FieldReader<u32>;
#[doc = "Field `DMSAR` writer - Specifies the transfer source start address."]
pub type DmsarW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies the transfer source start address."]
    #[inline(always)]
    pub fn dmsar(&self) -> DmsarR {
        DmsarR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMSAR")
            .field("dmsar", &self.dmsar())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies the transfer source start address."]
    #[inline(always)]
    pub fn dmsar(&mut self) -> DmsarW<DmsarSpec> {
        DmsarW::new(self, 0)
    }
}
#[doc = "DMA Source Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmsar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmsar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmsarSpec;
impl crate::RegisterSpec for DmsarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmsar::R`](R) reader structure"]
impl crate::Readable for DmsarSpec {}
#[doc = "`write(|w| ..)` method takes [`dmsar::W`](W) writer structure"]
impl crate::Writable for DmsarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMSAR to value 0"]
impl crate::Resettable for DmsarSpec {}
