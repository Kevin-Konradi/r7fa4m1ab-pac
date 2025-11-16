#[doc = "Register `CTSUCHAC3` reader"]
pub type R = crate::R<Ctsuchac3Spec>;
#[doc = "Register `CTSUCHAC3` writer"]
pub type W = crate::W<Ctsuchac3Spec>;
#[doc = "CTSU Channel Enable Control 3. 0: Not measurement target 1: Measurement target Note: CTSUCHAC3\\[0\\] corresponds to TS24 and CTSUCHAC3\\[7\\] corresponds to TS31.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctsuchac3 {
    #[doc = "0: TS pin which correspond to the bit number of CTSUCHAC3 register set whether the measurement target."]
    Ctsuchac3 = 0,
}
impl From<Ctsuchac3> for u8 {
    #[inline(always)]
    fn from(variant: Ctsuchac3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctsuchac3 {
    type Ux = u8;
}
impl crate::IsEnum for Ctsuchac3 {}
#[doc = "Field `CTSUCHAC3` reader - CTSU Channel Enable Control 3. 0: Not measurement target 1: Measurement target Note: CTSUCHAC3\\[0\\] corresponds to TS24 and CTSUCHAC3\\[7\\] corresponds to TS31."]
pub type Ctsuchac3R = crate::FieldReader<Ctsuchac3>;
impl Ctsuchac3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsuchac3 {
        match self.bits {
            _ => Ctsuchac3::Ctsuchac3,
        }
    }
    #[doc = "TS pin which correspond to the bit number of CTSUCHAC3 register set whether the measurement target."]
    #[inline(always)]
    pub fn is_ctsuchac3(&self) -> bool {
        matches!(self.variant(), Ctsuchac3::Ctsuchac3)
    }
}
#[doc = "Field `CTSUCHAC3` writer - CTSU Channel Enable Control 3. 0: Not measurement target 1: Measurement target Note: CTSUCHAC3\\[0\\] corresponds to TS24 and CTSUCHAC3\\[7\\] corresponds to TS31."]
pub type Ctsuchac3W<'a, REG> = crate::FieldWriter<'a, REG, 8, Ctsuchac3, crate::Safe>;
impl<'a, REG> Ctsuchac3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TS pin which correspond to the bit number of CTSUCHAC3 register set whether the measurement target."]
    #[inline(always)]
    pub fn ctsuchac3(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsuchac3::Ctsuchac3)
    }
}
impl R {
    #[doc = "Bits 0:7 - CTSU Channel Enable Control 3. 0: Not measurement target 1: Measurement target Note: CTSUCHAC3\\[0\\] corresponds to TS24 and CTSUCHAC3\\[7\\] corresponds to TS31."]
    #[inline(always)]
    pub fn ctsuchac3(&self) -> Ctsuchac3R {
        Ctsuchac3R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTSUCHAC3")
            .field("ctsuchac3", &self.ctsuchac3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - CTSU Channel Enable Control 3. 0: Not measurement target 1: Measurement target Note: CTSUCHAC3\\[0\\] corresponds to TS24 and CTSUCHAC3\\[7\\] corresponds to TS31."]
    #[inline(always)]
    pub fn ctsuchac3(&mut self) -> Ctsuchac3W<Ctsuchac3Spec> {
        Ctsuchac3W::new(self, 0)
    }
}
#[doc = "CTSU Channel Enable Control Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ctsuchac3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchac3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctsuchac3Spec;
impl crate::RegisterSpec for Ctsuchac3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctsuchac3::R`](R) reader structure"]
impl crate::Readable for Ctsuchac3Spec {}
#[doc = "`write(|w| ..)` method takes [`ctsuchac3::W`](W) writer structure"]
impl crate::Writable for Ctsuchac3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTSUCHAC3 to value 0"]
impl crate::Resettable for Ctsuchac3Spec {}
