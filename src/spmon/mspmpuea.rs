#[doc = "Register `MSPMPUEA` reader"]
pub type R = crate::R<MspmpueaSpec>;
#[doc = "Register `MSPMPUEA` writer"]
pub type W = crate::W<MspmpueaSpec>;
#[doc = "Field `MSPMPUEA` reader - Region end address register Address where the region starts, for use in region determination. NOTE: Range: 0x1FF00003-0x200FFFFF The low-order 2 bits are fixed to 1."]
pub type MspmpueaR = crate::FieldReader<u32>;
#[doc = "Field `MSPMPUEA` writer - Region end address register Address where the region starts, for use in region determination. NOTE: Range: 0x1FF00003-0x200FFFFF The low-order 2 bits are fixed to 1."]
pub type MspmpueaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region end address register Address where the region starts, for use in region determination. NOTE: Range: 0x1FF00003-0x200FFFFF The low-order 2 bits are fixed to 1."]
    #[inline(always)]
    pub fn mspmpuea(&self) -> MspmpueaR {
        MspmpueaR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSPMPUEA")
            .field("mspmpuea", &self.mspmpuea())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Region end address register Address where the region starts, for use in region determination. NOTE: Range: 0x1FF00003-0x200FFFFF The low-order 2 bits are fixed to 1."]
    #[inline(always)]
    pub fn mspmpuea(&mut self) -> MspmpueaW<MspmpueaSpec> {
        MspmpueaW::new(self, 0)
    }
}
#[doc = "Main Stack Pointer (MSP) Monitor End Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mspmpuea::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mspmpuea::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MspmpueaSpec;
impl crate::RegisterSpec for MspmpueaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mspmpuea::R`](R) reader structure"]
impl crate::Readable for MspmpueaSpec {}
#[doc = "`write(|w| ..)` method takes [`mspmpuea::W`](W) writer structure"]
impl crate::Writable for MspmpueaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MSPMPUEA to value 0x03"]
impl crate::Resettable for MspmpueaSpec {
    const RESET_VALUE: u32 = 0x03;
}
