#[doc = "Register `LOCOUTCR` reader"]
pub type R = crate::R<LocoutcrSpec>;
#[doc = "Register `LOCOUTCR` writer"]
pub type W = crate::W<LocoutcrSpec>;
#[doc = "Field `LOCOUTRM` reader - LOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127 These bits are added to original LOCO trimming bits"]
pub type LocoutrmR = crate::FieldReader;
#[doc = "Field `LOCOUTRM` writer - LOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127 These bits are added to original LOCO trimming bits"]
pub type LocoutrmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - LOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127 These bits are added to original LOCO trimming bits"]
    #[inline(always)]
    pub fn locoutrm(&self) -> LocoutrmR {
        LocoutrmR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOCOUTCR")
            .field("locoutrm", &self.locoutrm())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - LOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127 These bits are added to original LOCO trimming bits"]
    #[inline(always)]
    pub fn locoutrm(&mut self) -> LocoutrmW<LocoutcrSpec> {
        LocoutrmW::new(self, 0)
    }
}
#[doc = "LOCO User Trimming Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`locoutcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`locoutcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LocoutcrSpec;
impl crate::RegisterSpec for LocoutcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`locoutcr::R`](R) reader structure"]
impl crate::Readable for LocoutcrSpec {}
#[doc = "`write(|w| ..)` method takes [`locoutcr::W`](W) writer structure"]
impl crate::Writable for LocoutcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOCOUTCR to value 0"]
impl crate::Resettable for LocoutcrSpec {}
