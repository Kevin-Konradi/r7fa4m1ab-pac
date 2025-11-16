#[doc = "Register `VBTBKR[%s]` reader"]
pub type R = crate::R<VbtbkrSpec>;
#[doc = "Register `VBTBKR[%s]` writer"]
pub type W = crate::W<VbtbkrSpec>;
#[doc = "Field `VBTBKR` reader - VBTBKR is a 512-byte readable/writable register to store data powered by VBATT. The value of this register is retained even when VCC is not powered but VBATT is powered. VBTBKR is initialized by VBATT selected voltage power-on-reset."]
pub type VbtbkrR = crate::FieldReader;
#[doc = "Field `VBTBKR` writer - VBTBKR is a 512-byte readable/writable register to store data powered by VBATT. The value of this register is retained even when VCC is not powered but VBATT is powered. VBTBKR is initialized by VBATT selected voltage power-on-reset."]
pub type VbtbkrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - VBTBKR is a 512-byte readable/writable register to store data powered by VBATT. The value of this register is retained even when VCC is not powered but VBATT is powered. VBTBKR is initialized by VBATT selected voltage power-on-reset."]
    #[inline(always)]
    pub fn vbtbkr(&self) -> VbtbkrR {
        VbtbkrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VBTBKR")
            .field("vbtbkr", &self.vbtbkr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - VBTBKR is a 512-byte readable/writable register to store data powered by VBATT. The value of this register is retained even when VCC is not powered but VBATT is powered. VBTBKR is initialized by VBATT selected voltage power-on-reset."]
    #[inline(always)]
    pub fn vbtbkr(&mut self) -> VbtbkrW<VbtbkrSpec> {
        VbtbkrW::new(self, 0)
    }
}
#[doc = "VBATT Backup Register \\[%s\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`vbtbkr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtbkr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VbtbkrSpec;
impl crate::RegisterSpec for VbtbkrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vbtbkr::R`](R) reader structure"]
impl crate::Readable for VbtbkrSpec {}
#[doc = "`write(|w| ..)` method takes [`vbtbkr::W`](W) writer structure"]
impl crate::Writable for VbtbkrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VBTBKR[%s] to value 0"]
impl crate::Resettable for VbtbkrSpec {}
