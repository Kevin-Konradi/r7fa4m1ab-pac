#[doc = "Register `BRR` reader"]
pub type R = crate::R<BrrSpec>;
#[doc = "Register `BRR` writer"]
pub type W = crate::W<BrrSpec>;
#[doc = "Field `BRR` reader - BRR is an 8-bit register that adjusts the bit rate."]
pub type BrrR = crate::FieldReader;
#[doc = "Field `BRR` writer - BRR is an 8-bit register that adjusts the bit rate."]
pub type BrrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - BRR is an 8-bit register that adjusts the bit rate."]
    #[inline(always)]
    pub fn brr(&self) -> BrrR {
        BrrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BRR").field("brr", &self.brr()).finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - BRR is an 8-bit register that adjusts the bit rate."]
    #[inline(always)]
    pub fn brr(&mut self) -> BrrW<BrrSpec> {
        BrrW::new(self, 0)
    }
}
#[doc = "Bit Rate Register\n\nYou can [`read`](crate::Reg::read) this register and get [`brr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrrSpec;
impl crate::RegisterSpec for BrrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`brr::R`](R) reader structure"]
impl crate::Readable for BrrSpec {}
#[doc = "`write(|w| ..)` method takes [`brr::W`](W) writer structure"]
impl crate::Writable for BrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BRR to value 0xff"]
impl crate::Resettable for BrrSpec {
    const RESET_VALUE: u8 = 0xff;
}
