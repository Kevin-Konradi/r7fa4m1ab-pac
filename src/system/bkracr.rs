#[doc = "Register `BKRACR` reader"]
pub type R = crate::R<BkracrSpec>;
#[doc = "Register `BKRACR` writer"]
pub type W = crate::W<BkracrSpec>;
#[doc = "Backup Register Access Control Register\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bkracs {
    #[doc = "0: Access control disable. When System clock source is SOSC or LOCO."]
    _000 = 0,
    #[doc = "6: Access control enable. System clock source is other than SOSC or LOCO."]
    _110 = 6,
    #[doc = "1: Setting prohibited"]
    Others = 1,
}
impl From<Bkracs> for u8 {
    #[inline(always)]
    fn from(variant: Bkracs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bkracs {
    type Ux = u8;
}
impl crate::IsEnum for Bkracs {}
#[doc = "Field `BKRACS` reader - Backup Register Access Control Register"]
pub type BkracsR = crate::FieldReader<Bkracs>;
impl BkracsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bkracs {
        match self.bits {
            0 => Bkracs::_000,
            6 => Bkracs::_110,
            _ => Bkracs::Others,
        }
    }
    #[doc = "Access control disable. When System clock source is SOSC or LOCO."]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Bkracs::_000
    }
    #[doc = "Access control enable. System clock source is other than SOSC or LOCO."]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Bkracs::_110
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Bkracs::Others)
    }
}
#[doc = "Field `BKRACS` writer - Backup Register Access Control Register"]
pub type BkracsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Bkracs, crate::Safe>;
impl<'a, REG> BkracsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Access control disable. When System clock source is SOSC or LOCO."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Bkracs::_000)
    }
    #[doc = "Access control enable. System clock source is other than SOSC or LOCO."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Bkracs::_110)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Bkracs::Others)
    }
}
impl R {
    #[doc = "Bits 0:2 - Backup Register Access Control Register"]
    #[inline(always)]
    pub fn bkracs(&self) -> BkracsR {
        BkracsR::new(self.bits & 7)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BKRACR")
            .field("bkracs", &self.bkracs())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Backup Register Access Control Register"]
    #[inline(always)]
    pub fn bkracs(&mut self) -> BkracsW<BkracrSpec> {
        BkracsW::new(self, 0)
    }
}
#[doc = "Backup Register Access Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkracr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkracr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BkracrSpec;
impl crate::RegisterSpec for BkracrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bkracr::R`](R) reader structure"]
impl crate::Readable for BkracrSpec {}
#[doc = "`write(|w| ..)` method takes [`bkracr::W`](W) writer structure"]
impl crate::Writable for BkracrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BKRACR to value 0x06"]
impl crate::Resettable for BkracrSpec {
    const RESET_VALUE: u8 = 0x06;
}
