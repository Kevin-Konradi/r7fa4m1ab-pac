#[doc = "Register `CTSUCHAC2` reader"]
pub type R = crate::R<Ctsuchac2Spec>;
#[doc = "Register `CTSUCHAC2` writer"]
pub type W = crate::W<Ctsuchac2Spec>;
#[doc = "CTSU Channel Enable Control 2. 0: Not measurement target 1: Measurement target Note: CTSUCHAC2\\[0\\] corresponds to TS16 and CTSUCHAC2\\[7\\] corresponds to TS23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctsuchac2 {
    #[doc = "0: TS pin which correspond to the bit number of CTSUCHAC2 register set whether the measurement target."]
    Ctsuchac2 = 0,
}
impl From<Ctsuchac2> for u8 {
    #[inline(always)]
    fn from(variant: Ctsuchac2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctsuchac2 {
    type Ux = u8;
}
impl crate::IsEnum for Ctsuchac2 {}
#[doc = "Field `CTSUCHAC2` reader - CTSU Channel Enable Control 2. 0: Not measurement target 1: Measurement target Note: CTSUCHAC2\\[0\\] corresponds to TS16 and CTSUCHAC2\\[7\\] corresponds to TS23."]
pub type Ctsuchac2R = crate::FieldReader<Ctsuchac2>;
impl Ctsuchac2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsuchac2 {
        match self.bits {
            _ => Ctsuchac2::Ctsuchac2,
        }
    }
    #[doc = "TS pin which correspond to the bit number of CTSUCHAC2 register set whether the measurement target."]
    #[inline(always)]
    pub fn is_ctsuchac2(&self) -> bool {
        matches!(self.variant(), Ctsuchac2::Ctsuchac2)
    }
}
#[doc = "Field `CTSUCHAC2` writer - CTSU Channel Enable Control 2. 0: Not measurement target 1: Measurement target Note: CTSUCHAC2\\[0\\] corresponds to TS16 and CTSUCHAC2\\[7\\] corresponds to TS23."]
pub type Ctsuchac2W<'a, REG> = crate::FieldWriter<'a, REG, 8, Ctsuchac2, crate::Safe>;
impl<'a, REG> Ctsuchac2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TS pin which correspond to the bit number of CTSUCHAC2 register set whether the measurement target."]
    #[inline(always)]
    pub fn ctsuchac2(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsuchac2::Ctsuchac2)
    }
}
impl R {
    #[doc = "Bits 0:7 - CTSU Channel Enable Control 2. 0: Not measurement target 1: Measurement target Note: CTSUCHAC2\\[0\\] corresponds to TS16 and CTSUCHAC2\\[7\\] corresponds to TS23."]
    #[inline(always)]
    pub fn ctsuchac2(&self) -> Ctsuchac2R {
        Ctsuchac2R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTSUCHAC2")
            .field("ctsuchac2", &self.ctsuchac2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - CTSU Channel Enable Control 2. 0: Not measurement target 1: Measurement target Note: CTSUCHAC2\\[0\\] corresponds to TS16 and CTSUCHAC2\\[7\\] corresponds to TS23."]
    #[inline(always)]
    pub fn ctsuchac2(&mut self) -> Ctsuchac2W<Ctsuchac2Spec> {
        Ctsuchac2W::new(self, 0)
    }
}
#[doc = "CTSU Channel Enable Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctsuchac2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchac2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctsuchac2Spec;
impl crate::RegisterSpec for Ctsuchac2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctsuchac2::R`](R) reader structure"]
impl crate::Readable for Ctsuchac2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctsuchac2::W`](W) writer structure"]
impl crate::Writable for Ctsuchac2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTSUCHAC2 to value 0"]
impl crate::Resettable for Ctsuchac2Spec {}
