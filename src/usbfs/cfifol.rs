#[doc = "Register `CFIFOL` reader"]
pub type R = crate::R<CfifolSpec>;
#[doc = "Register `CFIFOL` writer"]
pub type W = crate::W<CfifolSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "CFIFO Port Register L\n\nYou can [`read`](crate::Reg::read) this register and get [`cfifol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfifolSpec;
impl crate::RegisterSpec for CfifolSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cfifol::R`](R) reader structure"]
impl crate::Readable for CfifolSpec {}
#[doc = "`write(|w| ..)` method takes [`cfifol::W`](W) writer structure"]
impl crate::Writable for CfifolSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFIFOL to value 0"]
impl crate::Resettable for CfifolSpec {}
