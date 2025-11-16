#[doc = "Register `CSSR` reader"]
pub type R = crate::R<CssrSpec>;
#[doc = "Register `CSSR` writer"]
pub type W = crate::W<CssrSpec>;
#[doc = "Field `CSSR` reader - When the value for the channel search is input, the channel number is output to MSSR."]
pub type CssrR = crate::FieldReader;
#[doc = "Field `CSSR` writer - When the value for the channel search is input, the channel number is output to MSSR."]
pub type CssrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - When the value for the channel search is input, the channel number is output to MSSR."]
    #[inline(always)]
    pub fn cssr(&self) -> CssrR {
        CssrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSSR").field("cssr", &self.cssr()).finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - When the value for the channel search is input, the channel number is output to MSSR."]
    #[inline(always)]
    pub fn cssr(&mut self) -> CssrW<CssrSpec> {
        CssrW::new(self, 0)
    }
}
#[doc = "Channel Search Support Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cssr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cssr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CssrSpec;
impl crate::RegisterSpec for CssrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cssr::R`](R) reader structure"]
impl crate::Readable for CssrSpec {}
#[doc = "`write(|w| ..)` method takes [`cssr::W`](W) writer structure"]
impl crate::Writable for CssrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSSR to value 0"]
impl crate::Resettable for CssrSpec {}
