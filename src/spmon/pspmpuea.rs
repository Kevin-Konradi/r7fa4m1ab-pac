#[doc = "Register `PSPMPUEA` reader"]
pub type R = crate::R<PspmpueaSpec>;
#[doc = "Register `PSPMPUEA` writer"]
pub type W = crate::W<PspmpueaSpec>;
#[doc = "Field `PSPMPUEA` reader - Region end address register Address where the region starts, for use in region determination. NOTE: Range: 0x1FF00003-0x200FFFFF The low-order 2 bits are fixed to 1."]
pub type PspmpueaR = crate::FieldReader<u32>;
#[doc = "Field `PSPMPUEA` writer - Region end address register Address where the region starts, for use in region determination. NOTE: Range: 0x1FF00003-0x200FFFFF The low-order 2 bits are fixed to 1."]
pub type PspmpueaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region end address register Address where the region starts, for use in region determination. NOTE: Range: 0x1FF00003-0x200FFFFF The low-order 2 bits are fixed to 1."]
    #[inline(always)]
    pub fn pspmpuea(&self) -> PspmpueaR {
        PspmpueaR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSPMPUEA")
            .field("pspmpuea", &self.pspmpuea())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Region end address register Address where the region starts, for use in region determination. NOTE: Range: 0x1FF00003-0x200FFFFF The low-order 2 bits are fixed to 1."]
    #[inline(always)]
    pub fn pspmpuea(&mut self) -> PspmpueaW<PspmpueaSpec> {
        PspmpueaW::new(self, 0)
    }
}
#[doc = "Process Stack Pointer (PSP) Monitor End Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pspmpuea::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pspmpuea::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PspmpueaSpec;
impl crate::RegisterSpec for PspmpueaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pspmpuea::R`](R) reader structure"]
impl crate::Readable for PspmpueaSpec {}
#[doc = "`write(|w| ..)` method takes [`pspmpuea::W`](W) writer structure"]
impl crate::Writable for PspmpueaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSPMPUEA to value 0x03"]
impl crate::Resettable for PspmpueaSpec {
    const RESET_VALUE: u32 = 0x03;
}
