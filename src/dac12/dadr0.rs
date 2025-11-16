#[doc = "Register `DADR0` reader"]
pub type R = crate::R<Dadr0Spec>;
#[doc = "Register `DADR0` writer"]
pub type W = crate::W<Dadr0Spec>;
#[doc = "Field `DADR` reader - D/A Data Register NOTE: When DADPR.DPSEL = 0, the high-order 4 bits are fixed to 0: right justified format. When DADPR.DPSEL = 1, the low-order 4 bits are fixed to 0: left justified format."]
pub type DadrR = crate::FieldReader<u16>;
#[doc = "Field `DADR` writer - D/A Data Register NOTE: When DADPR.DPSEL = 0, the high-order 4 bits are fixed to 0: right justified format. When DADPR.DPSEL = 1, the low-order 4 bits are fixed to 0: left justified format."]
pub type DadrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - D/A Data Register NOTE: When DADPR.DPSEL = 0, the high-order 4 bits are fixed to 0: right justified format. When DADPR.DPSEL = 1, the low-order 4 bits are fixed to 0: left justified format."]
    #[inline(always)]
    pub fn dadr(&self) -> DadrR {
        DadrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DADR0").field("dadr", &self.dadr()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - D/A Data Register NOTE: When DADPR.DPSEL = 0, the high-order 4 bits are fixed to 0: right justified format. When DADPR.DPSEL = 1, the low-order 4 bits are fixed to 0: left justified format."]
    #[inline(always)]
    pub fn dadr(&mut self) -> DadrW<Dadr0Spec> {
        DadrW::new(self, 0)
    }
}
#[doc = "D/A Data Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dadr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dadr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dadr0Spec;
impl crate::RegisterSpec for Dadr0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dadr0::R`](R) reader structure"]
impl crate::Readable for Dadr0Spec {}
#[doc = "`write(|w| ..)` method takes [`dadr0::W`](W) writer structure"]
impl crate::Writable for Dadr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DADR0 to value 0"]
impl crate::Resettable for Dadr0Spec {}
