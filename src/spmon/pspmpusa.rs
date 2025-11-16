#[doc = "Register `PSPMPUSA` reader"]
pub type R = crate::R<PspmpusaSpec>;
#[doc = "Register `PSPMPUSA` writer"]
pub type W = crate::W<PspmpusaSpec>;
#[doc = "Field `PSPMPUSA` reader - Region start address register Address where the region starts, for use in region determination. NOTE: Range: 0x1FF00000-0x200FFFFC The low-order 2 bits are fixed to 0."]
pub type PspmpusaR = crate::FieldReader<u32>;
#[doc = "Field `PSPMPUSA` writer - Region start address register Address where the region starts, for use in region determination. NOTE: Range: 0x1FF00000-0x200FFFFC The low-order 2 bits are fixed to 0."]
pub type PspmpusaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region start address register Address where the region starts, for use in region determination. NOTE: Range: 0x1FF00000-0x200FFFFC The low-order 2 bits are fixed to 0."]
    #[inline(always)]
    pub fn pspmpusa(&self) -> PspmpusaR {
        PspmpusaR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSPMPUSA")
            .field("pspmpusa", &self.pspmpusa())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Region start address register Address where the region starts, for use in region determination. NOTE: Range: 0x1FF00000-0x200FFFFC The low-order 2 bits are fixed to 0."]
    #[inline(always)]
    pub fn pspmpusa(&mut self) -> PspmpusaW<PspmpusaSpec> {
        PspmpusaW::new(self, 0)
    }
}
#[doc = "Process Stack Pointer (PSP) Monitor Start Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pspmpusa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pspmpusa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PspmpusaSpec;
impl crate::RegisterSpec for PspmpusaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pspmpusa::R`](R) reader structure"]
impl crate::Readable for PspmpusaSpec {}
#[doc = "`write(|w| ..)` method takes [`pspmpusa::W`](W) writer structure"]
impl crate::Writable for PspmpusaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSPMPUSA to value 0"]
impl crate::Resettable for PspmpusaSpec {}
