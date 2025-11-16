#[doc = "Register `CRCDOR` reader"]
pub type R = crate::R<CrcdorSpec>;
#[doc = "Register `CRCDOR` writer"]
pub type W = crate::W<CrcdorSpec>;
#[doc = "Field `CRCDOR` reader - Calculation output Data (Case of CRC-32, CRC-32C )"]
pub type CrcdorR = crate::FieldReader<u32>;
#[doc = "Field `CRCDOR` writer - Calculation output Data (Case of CRC-32, CRC-32C )"]
pub type CrcdorW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Calculation output Data (Case of CRC-32, CRC-32C )"]
    #[inline(always)]
    pub fn crcdor(&self) -> CrcdorR {
        CrcdorR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRCDOR")
            .field("crcdor", &self.crcdor())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Calculation output Data (Case of CRC-32, CRC-32C )"]
    #[inline(always)]
    pub fn crcdor(&mut self) -> CrcdorW<CrcdorSpec> {
        CrcdorW::new(self, 0)
    }
}
#[doc = "CRC Data Output Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcdor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcdorSpec;
impl crate::RegisterSpec for CrcdorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcdor::R`](R) reader structure"]
impl crate::Readable for CrcdorSpec {}
#[doc = "`write(|w| ..)` method takes [`crcdor::W`](W) writer structure"]
impl crate::Writable for CrcdorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCDOR to value 0"]
impl crate::Resettable for CrcdorSpec {}
