#[doc = "Register `DTCST` reader"]
pub type R = crate::R<DtcstSpec>;
#[doc = "Register `DTCST` writer"]
pub type W = crate::W<DtcstSpec>;
#[doc = "DTC Module Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtcst {
    #[doc = "0: DTC module stop"]
    _0 = 0,
    #[doc = "1: DTC module start"]
    _1 = 1,
}
impl From<Dtcst> for bool {
    #[inline(always)]
    fn from(variant: Dtcst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTCST` reader - DTC Module Start"]
pub type DtcstR = crate::BitReader<Dtcst>;
impl DtcstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dtcst {
        match self.bits {
            false => Dtcst::_0,
            true => Dtcst::_1,
        }
    }
    #[doc = "DTC module stop"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dtcst::_0
    }
    #[doc = "DTC module start"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dtcst::_1
    }
}
#[doc = "Field `DTCST` writer - DTC Module Start"]
pub type DtcstW<'a, REG> = crate::BitWriter<'a, REG, Dtcst>;
impl<'a, REG> DtcstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DTC module stop"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dtcst::_0)
    }
    #[doc = "DTC module start"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtcst::_1)
    }
}
impl R {
    #[doc = "Bit 0 - DTC Module Start"]
    #[inline(always)]
    pub fn dtcst(&self) -> DtcstR {
        DtcstR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTCST")
            .field("dtcst", &self.dtcst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - DTC Module Start"]
    #[inline(always)]
    pub fn dtcst(&mut self) -> DtcstW<DtcstSpec> {
        DtcstW::new(self, 0)
    }
}
#[doc = "DTC Module Start Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtcst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtcst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtcstSpec;
impl crate::RegisterSpec for DtcstSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dtcst::R`](R) reader structure"]
impl crate::Readable for DtcstSpec {}
#[doc = "`write(|w| ..)` method takes [`dtcst::W`](W) writer structure"]
impl crate::Writable for DtcstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTCST to value 0"]
impl crate::Resettable for DtcstSpec {}
