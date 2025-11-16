#[doc = "Register `HOCOUTCR` reader"]
pub type R = crate::R<HocoutcrSpec>;
#[doc = "Register `HOCOUTCR` writer"]
pub type W = crate::W<HocoutcrSpec>;
#[doc = "Field `HOCOUTRM` reader - HOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127 These bits are added to original HOCO trimming bits"]
pub type HocoutrmR = crate::FieldReader;
#[doc = "Field `HOCOUTRM` writer - HOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127 These bits are added to original HOCO trimming bits"]
pub type HocoutrmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - HOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127 These bits are added to original HOCO trimming bits"]
    #[inline(always)]
    pub fn hocoutrm(&self) -> HocoutrmR {
        HocoutrmR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOCOUTCR")
            .field("hocoutrm", &self.hocoutrm())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - HOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127 These bits are added to original HOCO trimming bits"]
    #[inline(always)]
    pub fn hocoutrm(&mut self) -> HocoutrmW<HocoutcrSpec> {
        HocoutrmW::new(self, 0)
    }
}
#[doc = "HOCO User Trimming Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hocoutcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hocoutcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HocoutcrSpec;
impl crate::RegisterSpec for HocoutcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hocoutcr::R`](R) reader structure"]
impl crate::Readable for HocoutcrSpec {}
#[doc = "`write(|w| ..)` method takes [`hocoutcr::W`](W) writer structure"]
impl crate::Writable for HocoutcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HOCOUTCR to value 0"]
impl crate::Resettable for HocoutcrSpec {}
