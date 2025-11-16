#[doc = "Register `MMPUSA%s` reader"]
pub type R = crate::R<MmpusaSpec>;
#[doc = "Register `MMPUSA%s` writer"]
pub type W = crate::W<MmpusaSpec>;
#[doc = "Field `MMPUSA` reader - Address where the region starts, for use in region determination. NOTE: The low-order 2 bits are fixed to 0."]
pub type MmpusaR = crate::FieldReader<u32>;
#[doc = "Field `MMPUSA` writer - Address where the region starts, for use in region determination. NOTE: The low-order 2 bits are fixed to 0."]
pub type MmpusaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Address where the region starts, for use in region determination. NOTE: The low-order 2 bits are fixed to 0."]
    #[inline(always)]
    pub fn mmpusa(&self) -> MmpusaR {
        MmpusaR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMPUSA")
            .field("mmpusa", &self.mmpusa())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Address where the region starts, for use in region determination. NOTE: The low-order 2 bits are fixed to 0."]
    #[inline(always)]
    pub fn mmpusa(&mut self) -> MmpusaW<MmpusaSpec> {
        MmpusaW::new(self, 0)
    }
}
#[doc = "Group A Region %s Start Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmpusa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpusa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmpusaSpec;
impl crate::RegisterSpec for MmpusaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmpusa::R`](R) reader structure"]
impl crate::Readable for MmpusaSpec {}
#[doc = "`write(|w| ..)` method takes [`mmpusa::W`](W) writer structure"]
impl crate::Writable for MmpusaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MMPUSA%s to value 0"]
impl crate::Resettable for MmpusaSpec {}
