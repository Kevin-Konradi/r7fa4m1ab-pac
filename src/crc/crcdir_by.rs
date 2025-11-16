#[doc = "Register `CRCDIR_BY` reader"]
pub type R = crate::R<CrcdirBySpec>;
#[doc = "Register `CRCDIR_BY` writer"]
pub type W = crate::W<CrcdirBySpec>;
#[doc = "Field `CRCDIR_BY` reader - Calculation input Data ( Case of CRC-8, CRC-16 or CRC-CCITT )"]
pub type CrcdirByR = crate::FieldReader;
#[doc = "Field `CRCDIR_BY` writer - Calculation input Data ( Case of CRC-8, CRC-16 or CRC-CCITT )"]
pub type CrcdirByW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Calculation input Data ( Case of CRC-8, CRC-16 or CRC-CCITT )"]
    #[inline(always)]
    pub fn crcdir_by(&self) -> CrcdirByR {
        CrcdirByR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRCDIR_BY")
            .field("crcdir_by", &self.crcdir_by())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Calculation input Data ( Case of CRC-8, CRC-16 or CRC-CCITT )"]
    #[inline(always)]
    pub fn crcdir_by(&mut self) -> CrcdirByW<CrcdirBySpec> {
        CrcdirByW::new(self, 0)
    }
}
#[doc = "CRC Data Input Register (byte access)\n\nYou can [`read`](crate::Reg::read) this register and get [`crcdir_by::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdir_by::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcdirBySpec;
impl crate::RegisterSpec for CrcdirBySpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`crcdir_by::R`](R) reader structure"]
impl crate::Readable for CrcdirBySpec {}
#[doc = "`write(|w| ..)` method takes [`crcdir_by::W`](W) writer structure"]
impl crate::Writable for CrcdirBySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCDIR_BY to value 0"]
impl crate::Resettable for CrcdirBySpec {}
