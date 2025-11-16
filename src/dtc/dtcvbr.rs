#[doc = "Register `DTCVBR` reader"]
pub type R = crate::R<DtcvbrSpec>;
#[doc = "Register `DTCVBR` writer"]
pub type W = crate::W<DtcvbrSpec>;
#[doc = "Field `DTCVBR` reader - DTC Vector Base Address. Note: A value cannot be set in the lower-order 10 bits. These bits are fixed to 0."]
pub type DtcvbrR = crate::FieldReader<u32>;
#[doc = "Field `DTCVBR` writer - DTC Vector Base Address. Note: A value cannot be set in the lower-order 10 bits. These bits are fixed to 0."]
pub type DtcvbrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DTC Vector Base Address. Note: A value cannot be set in the lower-order 10 bits. These bits are fixed to 0."]
    #[inline(always)]
    pub fn dtcvbr(&self) -> DtcvbrR {
        DtcvbrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTCVBR")
            .field("dtcvbr", &self.dtcvbr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - DTC Vector Base Address. Note: A value cannot be set in the lower-order 10 bits. These bits are fixed to 0."]
    #[inline(always)]
    pub fn dtcvbr(&mut self) -> DtcvbrW<DtcvbrSpec> {
        DtcvbrW::new(self, 0)
    }
}
#[doc = "DTC Vector Base Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtcvbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtcvbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtcvbrSpec;
impl crate::RegisterSpec for DtcvbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtcvbr::R`](R) reader structure"]
impl crate::Readable for DtcvbrSpec {}
#[doc = "`write(|w| ..)` method takes [`dtcvbr::W`](W) writer structure"]
impl crate::Writable for DtcvbrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTCVBR to value 0"]
impl crate::Resettable for DtcvbrSpec {}
