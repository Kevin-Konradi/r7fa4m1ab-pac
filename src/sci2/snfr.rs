#[doc = "Register `SNFR` reader"]
pub type R = crate::R<SnfrSpec>;
#[doc = "Register `SNFR` writer"]
pub type W = crate::W<SnfrSpec>;
#[doc = "Noise Filter Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nfcs {
    #[doc = "0: The clock signal divided by 1 is used with the noise filter.(In asynchronous mode)"]
    _000 = 0,
    #[doc = "1: The clock signal divided by 1 is used with the noise filter.(In simple I2C mode)"]
    _001 = 1,
    #[doc = "2: The clock signal divided by 2 is used with the noise filter.(In simple I2C mode)"]
    _010 = 2,
    #[doc = "3: The clock signal divided by 4 is used with the noise filter.(In simple I2C mode)"]
    _011 = 3,
    #[doc = "4: The clock signal divided by 8 is used with the noise filter.(In simple I2C mode)"]
    _100 = 4,
    #[doc = "5: Settings prohibited."]
    Others = 5,
}
impl From<Nfcs> for u8 {
    #[inline(always)]
    fn from(variant: Nfcs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nfcs {
    type Ux = u8;
}
impl crate::IsEnum for Nfcs {}
#[doc = "Field `NFCS` reader - Noise Filter Clock Select"]
pub type NfcsR = crate::FieldReader<Nfcs>;
impl NfcsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nfcs {
        match self.bits {
            0 => Nfcs::_000,
            1 => Nfcs::_001,
            2 => Nfcs::_010,
            3 => Nfcs::_011,
            4 => Nfcs::_100,
            _ => Nfcs::Others,
        }
    }
    #[doc = "The clock signal divided by 1 is used with the noise filter.(In asynchronous mode)"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Nfcs::_000
    }
    #[doc = "The clock signal divided by 1 is used with the noise filter.(In simple I2C mode)"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Nfcs::_001
    }
    #[doc = "The clock signal divided by 2 is used with the noise filter.(In simple I2C mode)"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Nfcs::_010
    }
    #[doc = "The clock signal divided by 4 is used with the noise filter.(In simple I2C mode)"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Nfcs::_011
    }
    #[doc = "The clock signal divided by 8 is used with the noise filter.(In simple I2C mode)"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Nfcs::_100
    }
    #[doc = "Settings prohibited."]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Nfcs::Others)
    }
}
#[doc = "Field `NFCS` writer - Noise Filter Clock Select"]
pub type NfcsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Nfcs, crate::Safe>;
impl<'a, REG> NfcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The clock signal divided by 1 is used with the noise filter.(In asynchronous mode)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcs::_000)
    }
    #[doc = "The clock signal divided by 1 is used with the noise filter.(In simple I2C mode)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcs::_001)
    }
    #[doc = "The clock signal divided by 2 is used with the noise filter.(In simple I2C mode)"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcs::_010)
    }
    #[doc = "The clock signal divided by 4 is used with the noise filter.(In simple I2C mode)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcs::_011)
    }
    #[doc = "The clock signal divided by 8 is used with the noise filter.(In simple I2C mode)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcs::_100)
    }
    #[doc = "Settings prohibited."]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Nfcs::Others)
    }
}
impl R {
    #[doc = "Bits 0:2 - Noise Filter Clock Select"]
    #[inline(always)]
    pub fn nfcs(&self) -> NfcsR {
        NfcsR::new(self.bits & 7)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SNFR").field("nfcs", &self.nfcs()).finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Noise Filter Clock Select"]
    #[inline(always)]
    pub fn nfcs(&mut self) -> NfcsW<SnfrSpec> {
        NfcsW::new(self, 0)
    }
}
#[doc = "Noise Filter Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`snfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SnfrSpec;
impl crate::RegisterSpec for SnfrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`snfr::R`](R) reader structure"]
impl crate::Readable for SnfrSpec {}
#[doc = "`write(|w| ..)` method takes [`snfr::W`](W) writer structure"]
impl crate::Writable for SnfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SNFR to value 0"]
impl crate::Resettable for SnfrSpec {}
