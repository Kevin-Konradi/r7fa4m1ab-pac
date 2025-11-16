#[doc = "Register `CRCDIR` reader"]
pub type R = crate::R<CrcdirSpec>;
#[doc = "Register `CRCDIR` writer"]
pub type W = crate::W<CrcdirSpec>;
#[doc = "Field `CRCDIR` reader - Calculation input Data (Case of CRC-32, CRC-32C )"]
pub type CrcdirR = crate::FieldReader<u32>;
#[doc = "Field `CRCDIR` writer - Calculation input Data (Case of CRC-32, CRC-32C )"]
pub type CrcdirW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Calculation input Data (Case of CRC-32, CRC-32C )"]
    #[inline(always)]
    pub fn crcdir(&self) -> CrcdirR {
        CrcdirR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRCDIR")
            .field("crcdir", &self.crcdir())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Calculation input Data (Case of CRC-32, CRC-32C )"]
    #[inline(always)]
    pub fn crcdir(&mut self) -> CrcdirW<CrcdirSpec> {
        CrcdirW::new(self, 0)
    }
}
#[doc = "CRC Data Input Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcdir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcdirSpec;
impl crate::RegisterSpec for CrcdirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcdir::R`](R) reader structure"]
impl crate::Readable for CrcdirSpec {}
#[doc = "`write(|w| ..)` method takes [`crcdir::W`](W) writer structure"]
impl crate::Writable for CrcdirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCDIR to value 0"]
impl crate::Resettable for CrcdirSpec {}
