#[doc = "Register `MDDR` reader"]
pub type R = crate::R<MddrSpec>;
#[doc = "Register `MDDR` writer"]
pub type W = crate::W<MddrSpec>;
#[doc = "Field `MDDR` reader - MDDR corrects the bit rate adjusted by the BRR register."]
pub type MddrR = crate::FieldReader;
#[doc = "Field `MDDR` writer - MDDR corrects the bit rate adjusted by the BRR register."]
pub type MddrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - MDDR corrects the bit rate adjusted by the BRR register."]
    #[inline(always)]
    pub fn mddr(&self) -> MddrR {
        MddrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDDR").field("mddr", &self.mddr()).finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - MDDR corrects the bit rate adjusted by the BRR register."]
    #[inline(always)]
    pub fn mddr(&mut self) -> MddrW<MddrSpec> {
        MddrW::new(self, 0)
    }
}
#[doc = "Modulation Duty Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MddrSpec;
impl crate::RegisterSpec for MddrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mddr::R`](R) reader structure"]
impl crate::Readable for MddrSpec {}
#[doc = "`write(|w| ..)` method takes [`mddr::W`](W) writer structure"]
impl crate::Writable for MddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDDR to value 0xff"]
impl crate::Resettable for MddrSpec {
    const RESET_VALUE: u8 = 0xff;
}
