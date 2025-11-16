#[doc = "Register `CALLVR` reader"]
pub type R = crate::R<CallvrSpec>;
#[doc = "Register `CALLVR` writer"]
pub type W = crate::W<CallvrSpec>;
#[doc = "Field `CALLVR` reader - CALLVR is a 16-bit readable/writable register that stores the lower-limit value of the frequency."]
pub type CallvrR = crate::FieldReader<u16>;
#[doc = "Field `CALLVR` writer - CALLVR is a 16-bit readable/writable register that stores the lower-limit value of the frequency."]
pub type CallvrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CALLVR is a 16-bit readable/writable register that stores the lower-limit value of the frequency."]
    #[inline(always)]
    pub fn callvr(&self) -> CallvrR {
        CallvrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CALLVR")
            .field("callvr", &self.callvr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - CALLVR is a 16-bit readable/writable register that stores the lower-limit value of the frequency."]
    #[inline(always)]
    pub fn callvr(&mut self) -> CallvrW<CallvrSpec> {
        CallvrW::new(self, 0)
    }
}
#[doc = "CAC Lower-Limit Value Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`callvr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`callvr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CallvrSpec;
impl crate::RegisterSpec for CallvrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`callvr::R`](R) reader structure"]
impl crate::Readable for CallvrSpec {}
#[doc = "`write(|w| ..)` method takes [`callvr::W`](W) writer structure"]
impl crate::Writable for CallvrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CALLVR to value 0"]
impl crate::Resettable for CallvrSpec {}
