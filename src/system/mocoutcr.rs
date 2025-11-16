#[doc = "Register `MOCOUTCR` reader"]
pub type R = crate::R<MocoutcrSpec>;
#[doc = "Register `MOCOUTCR` writer"]
pub type W = crate::W<MocoutcrSpec>;
#[doc = "Field `MOCOUTRM` reader - MOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127 These bits are added to original MOCO trimming bits"]
pub type MocoutrmR = crate::FieldReader;
#[doc = "Field `MOCOUTRM` writer - MOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127 These bits are added to original MOCO trimming bits"]
pub type MocoutrmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - MOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127 These bits are added to original MOCO trimming bits"]
    #[inline(always)]
    pub fn mocoutrm(&self) -> MocoutrmR {
        MocoutrmR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MOCOUTCR")
            .field("mocoutrm", &self.mocoutrm())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - MOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127 These bits are added to original MOCO trimming bits"]
    #[inline(always)]
    pub fn mocoutrm(&mut self) -> MocoutrmW<MocoutcrSpec> {
        MocoutrmW::new(self, 0)
    }
}
#[doc = "MOCO User Trimming Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mocoutcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mocoutcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MocoutcrSpec;
impl crate::RegisterSpec for MocoutcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mocoutcr::R`](R) reader structure"]
impl crate::Readable for MocoutcrSpec {}
#[doc = "`write(|w| ..)` method takes [`mocoutcr::W`](W) writer structure"]
impl crate::Writable for MocoutcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MOCOUTCR to value 0"]
impl crate::Resettable for MocoutcrSpec {}
