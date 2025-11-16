#[doc = "Register `DODSR` reader"]
pub type R = crate::R<DodsrSpec>;
#[doc = "Register `DODSR` writer"]
pub type W = crate::W<DodsrSpec>;
#[doc = "Field `DODSR` reader - This register stores 16-bit data for use as a reference in data comparison mode. This register also stores the results of operations in data addition and data subtraction modes."]
pub type DodsrR = crate::FieldReader<u16>;
#[doc = "Field `DODSR` writer - This register stores 16-bit data for use as a reference in data comparison mode. This register also stores the results of operations in data addition and data subtraction modes."]
pub type DodsrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register stores 16-bit data for use as a reference in data comparison mode. This register also stores the results of operations in data addition and data subtraction modes."]
    #[inline(always)]
    pub fn dodsr(&self) -> DodsrR {
        DodsrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DODSR")
            .field("dodsr", &self.dodsr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - This register stores 16-bit data for use as a reference in data comparison mode. This register also stores the results of operations in data addition and data subtraction modes."]
    #[inline(always)]
    pub fn dodsr(&mut self) -> DodsrW<DodsrSpec> {
        DodsrW::new(self, 0)
    }
}
#[doc = "DOC Data Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dodsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dodsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DodsrSpec;
impl crate::RegisterSpec for DodsrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dodsr::R`](R) reader structure"]
impl crate::Readable for DodsrSpec {}
#[doc = "`write(|w| ..)` method takes [`dodsr::W`](W) writer structure"]
impl crate::Writable for DodsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DODSR to value 0"]
impl crate::Resettable for DodsrSpec {}
