#[doc = "Register `MSPMPUSA` reader"]
pub type R = crate::R<MspmpusaSpec>;
#[doc = "Register `MSPMPUSA` writer"]
pub type W = crate::W<MspmpusaSpec>;
#[doc = "Field `MSPMPUSA` reader - Region start address register Address where the region starts, for use in region determination. NOTE: Range: 0x1FF00000-0x200FFFFC The low-order 2 bits are fixed to 0."]
pub type MspmpusaR = crate::FieldReader<u32>;
#[doc = "Field `MSPMPUSA` writer - Region start address register Address where the region starts, for use in region determination. NOTE: Range: 0x1FF00000-0x200FFFFC The low-order 2 bits are fixed to 0."]
pub type MspmpusaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region start address register Address where the region starts, for use in region determination. NOTE: Range: 0x1FF00000-0x200FFFFC The low-order 2 bits are fixed to 0."]
    #[inline(always)]
    pub fn mspmpusa(&self) -> MspmpusaR {
        MspmpusaR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSPMPUSA")
            .field("mspmpusa", &self.mspmpusa())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Region start address register Address where the region starts, for use in region determination. NOTE: Range: 0x1FF00000-0x200FFFFC The low-order 2 bits are fixed to 0."]
    #[inline(always)]
    pub fn mspmpusa(&mut self) -> MspmpusaW<MspmpusaSpec> {
        MspmpusaW::new(self, 0)
    }
}
#[doc = "Main Stack Pointer (MSP) Monitor Start Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mspmpusa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mspmpusa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MspmpusaSpec;
impl crate::RegisterSpec for MspmpusaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mspmpusa::R`](R) reader structure"]
impl crate::Readable for MspmpusaSpec {}
#[doc = "`write(|w| ..)` method takes [`mspmpusa::W`](W) writer structure"]
impl crate::Writable for MspmpusaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MSPMPUSA to value 0"]
impl crate::Resettable for MspmpusaSpec {}
