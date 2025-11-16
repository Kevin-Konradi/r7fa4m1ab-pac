#[doc = "Register `MMPUEA%s` reader"]
pub type R = crate::R<MmpueaSpec>;
#[doc = "Register `MMPUEA%s` writer"]
pub type W = crate::W<MmpueaSpec>;
#[doc = "Field `MMPUEA` reader - Region end address register Address where the region end, for use in region determination. NOTE: The low-order 2 bits are fixed to 1."]
pub type MmpueaR = crate::FieldReader<u32>;
#[doc = "Field `MMPUEA` writer - Region end address register Address where the region end, for use in region determination. NOTE: The low-order 2 bits are fixed to 1."]
pub type MmpueaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region end address register Address where the region end, for use in region determination. NOTE: The low-order 2 bits are fixed to 1."]
    #[inline(always)]
    pub fn mmpuea(&self) -> MmpueaR {
        MmpueaR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMPUEA")
            .field("mmpuea", &self.mmpuea())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Region end address register Address where the region end, for use in region determination. NOTE: The low-order 2 bits are fixed to 1."]
    #[inline(always)]
    pub fn mmpuea(&mut self) -> MmpueaW<MmpueaSpec> {
        MmpueaW::new(self, 0)
    }
}
#[doc = "Group A Region %s End Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmpuea::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuea::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmpueaSpec;
impl crate::RegisterSpec for MmpueaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmpuea::R`](R) reader structure"]
impl crate::Readable for MmpueaSpec {}
#[doc = "`write(|w| ..)` method takes [`mmpuea::W`](W) writer structure"]
impl crate::Writable for MmpueaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MMPUEA%s to value 0x03"]
impl crate::Resettable for MmpueaSpec {
    const RESET_VALUE: u32 = 0x03;
}
