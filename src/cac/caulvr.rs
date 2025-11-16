#[doc = "Register `CAULVR` reader"]
pub type R = crate::R<CaulvrSpec>;
#[doc = "Register `CAULVR` writer"]
pub type W = crate::W<CaulvrSpec>;
#[doc = "Field `CAULVR` reader - CAULVR is a 16-bit readable/writable register that stores the upper-limit value of the frequency."]
pub type CaulvrR = crate::FieldReader<u16>;
#[doc = "Field `CAULVR` writer - CAULVR is a 16-bit readable/writable register that stores the upper-limit value of the frequency."]
pub type CaulvrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CAULVR is a 16-bit readable/writable register that stores the upper-limit value of the frequency."]
    #[inline(always)]
    pub fn caulvr(&self) -> CaulvrR {
        CaulvrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAULVR")
            .field("caulvr", &self.caulvr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - CAULVR is a 16-bit readable/writable register that stores the upper-limit value of the frequency."]
    #[inline(always)]
    pub fn caulvr(&mut self) -> CaulvrW<CaulvrSpec> {
        CaulvrW::new(self, 0)
    }
}
#[doc = "CAC Upper-Limit Value Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`caulvr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`caulvr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CaulvrSpec;
impl crate::RegisterSpec for CaulvrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`caulvr::R`](R) reader structure"]
impl crate::Readable for CaulvrSpec {}
#[doc = "`write(|w| ..)` method takes [`caulvr::W`](W) writer structure"]
impl crate::Writable for CaulvrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAULVR to value 0"]
impl crate::Resettable for CaulvrSpec {}
