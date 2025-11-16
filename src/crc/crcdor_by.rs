#[doc = "Register `CRCDOR_BY` reader"]
pub type R = crate::R<CrcdorBySpec>;
#[doc = "Register `CRCDOR_BY` writer"]
pub type W = crate::W<CrcdorBySpec>;
#[doc = "Field `CRCDOR_BY` reader - Calculation output Data (Case of CRC-8 )"]
pub type CrcdorByR = crate::FieldReader;
#[doc = "Field `CRCDOR_BY` writer - Calculation output Data (Case of CRC-8 )"]
pub type CrcdorByW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Calculation output Data (Case of CRC-8 )"]
    #[inline(always)]
    pub fn crcdor_by(&self) -> CrcdorByR {
        CrcdorByR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRCDOR_BY")
            .field("crcdor_by", &self.crcdor_by())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Calculation output Data (Case of CRC-8 )"]
    #[inline(always)]
    pub fn crcdor_by(&mut self) -> CrcdorByW<CrcdorBySpec> {
        CrcdorByW::new(self, 0)
    }
}
#[doc = "CRC Data Output Register(byte access)\n\nYou can [`read`](crate::Reg::read) this register and get [`crcdor_by::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdor_by::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcdorBySpec;
impl crate::RegisterSpec for CrcdorBySpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`crcdor_by::R`](R) reader structure"]
impl crate::Readable for CrcdorBySpec {}
#[doc = "`write(|w| ..)` method takes [`crcdor_by::W`](W) writer structure"]
impl crate::Writable for CrcdorBySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCDOR_BY to value 0"]
impl crate::Resettable for CrcdorBySpec {}
