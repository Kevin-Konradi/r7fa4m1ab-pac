#[doc = "Register `HOCOWTCR` reader"]
pub type R = crate::R<HocowtcrSpec>;
#[doc = "Register `HOCOWTCR` writer"]
pub type W = crate::W<HocowtcrSpec>;
#[doc = "HOCO wait time setting\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hsts {
    #[doc = "5: If HOCO frequency is other than 64MHz, should set the value to 101b."]
    _101 = 5,
    #[doc = "6: If HOCO frequency = 64MHz, should set the value to 110b."]
    _110 = 6,
    #[doc = "0: Setting prohibited"]
    Others = 0,
}
impl From<Hsts> for u8 {
    #[inline(always)]
    fn from(variant: Hsts) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hsts {
    type Ux = u8;
}
impl crate::IsEnum for Hsts {}
#[doc = "Field `HSTS` reader - HOCO wait time setting"]
pub type HstsR = crate::FieldReader<Hsts>;
impl HstsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsts {
        match self.bits {
            5 => Hsts::_101,
            6 => Hsts::_110,
            _ => Hsts::Others,
        }
    }
    #[doc = "If HOCO frequency is other than 64MHz, should set the value to 101b."]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Hsts::_101
    }
    #[doc = "If HOCO frequency = 64MHz, should set the value to 110b."]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Hsts::_110
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Hsts::Others)
    }
}
#[doc = "Field `HSTS` writer - HOCO wait time setting"]
pub type HstsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Hsts, crate::Safe>;
impl<'a, REG> HstsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "If HOCO frequency is other than 64MHz, should set the value to 101b."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Hsts::_101)
    }
    #[doc = "If HOCO frequency = 64MHz, should set the value to 110b."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Hsts::_110)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Hsts::Others)
    }
}
impl R {
    #[doc = "Bits 0:2 - HOCO wait time setting"]
    #[inline(always)]
    pub fn hsts(&self) -> HstsR {
        HstsR::new(self.bits & 7)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOCOWTCR")
            .field("hsts", &self.hsts())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - HOCO wait time setting"]
    #[inline(always)]
    pub fn hsts(&mut self) -> HstsW<HocowtcrSpec> {
        HstsW::new(self, 0)
    }
}
#[doc = "High-Speed On-Chip Oscillator Wait Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hocowtcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hocowtcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HocowtcrSpec;
impl crate::RegisterSpec for HocowtcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hocowtcr::R`](R) reader structure"]
impl crate::Readable for HocowtcrSpec {}
#[doc = "`write(|w| ..)` method takes [`hocowtcr::W`](W) writer structure"]
impl crate::Writable for HocowtcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HOCOWTCR to value 0x05"]
impl crate::Resettable for HocowtcrSpec {
    const RESET_VALUE: u8 = 0x05;
}
