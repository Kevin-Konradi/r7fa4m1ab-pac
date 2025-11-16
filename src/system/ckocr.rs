#[doc = "Register `CKOCR` reader"]
pub type R = crate::R<CkocrSpec>;
#[doc = "Register `CKOCR` writer"]
pub type W = crate::W<CkocrSpec>;
#[doc = "Clock out source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ckosel {
    #[doc = "0: HOCO"]
    _000 = 0,
    #[doc = "1: MOCO"]
    _001 = 1,
    #[doc = "2: LOCO"]
    _010 = 2,
    #[doc = "3: MOSC"]
    _011 = 3,
    #[doc = "4: SOSC"]
    _100 = 4,
    #[doc = "5: Setting prohibited"]
    Others = 5,
}
impl From<Ckosel> for u8 {
    #[inline(always)]
    fn from(variant: Ckosel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ckosel {
    type Ux = u8;
}
impl crate::IsEnum for Ckosel {}
#[doc = "Field `CKOSEL` reader - Clock out source select"]
pub type CkoselR = crate::FieldReader<Ckosel>;
impl CkoselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ckosel {
        match self.bits {
            0 => Ckosel::_000,
            1 => Ckosel::_001,
            2 => Ckosel::_010,
            3 => Ckosel::_011,
            4 => Ckosel::_100,
            _ => Ckosel::Others,
        }
    }
    #[doc = "HOCO"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Ckosel::_000
    }
    #[doc = "MOCO"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Ckosel::_001
    }
    #[doc = "LOCO"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Ckosel::_010
    }
    #[doc = "MOSC"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Ckosel::_011
    }
    #[doc = "SOSC"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Ckosel::_100
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Ckosel::Others)
    }
}
#[doc = "Field `CKOSEL` writer - Clock out source select"]
pub type CkoselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ckosel, crate::Safe>;
impl<'a, REG> CkoselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HOCO"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Ckosel::_000)
    }
    #[doc = "MOCO"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Ckosel::_001)
    }
    #[doc = "LOCO"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Ckosel::_010)
    }
    #[doc = "MOSC"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Ckosel::_011)
    }
    #[doc = "SOSC"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Ckosel::_100)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Ckosel::Others)
    }
}
#[doc = "Clock out input frequency Division Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ckodiv {
    #[doc = "0: /1"]
    _000 = 0,
    #[doc = "1: /2"]
    _001 = 1,
    #[doc = "2: /4"]
    _010 = 2,
    #[doc = "3: /8"]
    _011 = 3,
    #[doc = "4: /16"]
    _100 = 4,
    #[doc = "5: /32"]
    _101 = 5,
    #[doc = "6: /64"]
    _110 = 6,
    #[doc = "7: /128"]
    _111 = 7,
}
impl From<Ckodiv> for u8 {
    #[inline(always)]
    fn from(variant: Ckodiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ckodiv {
    type Ux = u8;
}
impl crate::IsEnum for Ckodiv {}
#[doc = "Field `CKODIV` reader - Clock out input frequency Division Select"]
pub type CkodivR = crate::FieldReader<Ckodiv>;
impl CkodivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ckodiv {
        match self.bits {
            0 => Ckodiv::_000,
            1 => Ckodiv::_001,
            2 => Ckodiv::_010,
            3 => Ckodiv::_011,
            4 => Ckodiv::_100,
            5 => Ckodiv::_101,
            6 => Ckodiv::_110,
            7 => Ckodiv::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Ckodiv::_000
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Ckodiv::_001
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Ckodiv::_010
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Ckodiv::_011
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Ckodiv::_100
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Ckodiv::_101
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Ckodiv::_110
    }
    #[doc = "/128"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Ckodiv::_111
    }
}
#[doc = "Field `CKODIV` writer - Clock out input frequency Division Select"]
pub type CkodivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ckodiv, crate::Safe>;
impl<'a, REG> CkodivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Ckodiv::_000)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Ckodiv::_001)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Ckodiv::_010)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Ckodiv::_011)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Ckodiv::_100)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Ckodiv::_101)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Ckodiv::_110)
    }
    #[doc = "/128"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Ckodiv::_111)
    }
}
#[doc = "Clock out enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ckoen {
    #[doc = "0: Clock Out disable"]
    _0 = 0,
    #[doc = "1: Clock Out enable"]
    _1 = 1,
}
impl From<Ckoen> for bool {
    #[inline(always)]
    fn from(variant: Ckoen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKOEN` reader - Clock out enable"]
pub type CkoenR = crate::BitReader<Ckoen>;
impl CkoenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ckoen {
        match self.bits {
            false => Ckoen::_0,
            true => Ckoen::_1,
        }
    }
    #[doc = "Clock Out disable"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ckoen::_0
    }
    #[doc = "Clock Out enable"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ckoen::_1
    }
}
#[doc = "Field `CKOEN` writer - Clock out enable"]
pub type CkoenW<'a, REG> = crate::BitWriter<'a, REG, Ckoen>;
impl<'a, REG> CkoenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock Out disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ckoen::_0)
    }
    #[doc = "Clock Out enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ckoen::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock out source select"]
    #[inline(always)]
    pub fn ckosel(&self) -> CkoselR {
        CkoselR::new(self.bits & 7)
    }
    #[doc = "Bits 4:6 - Clock out input frequency Division Select"]
    #[inline(always)]
    pub fn ckodiv(&self) -> CkodivR {
        CkodivR::new((self.bits >> 4) & 7)
    }
    #[doc = "Bit 7 - Clock out enable"]
    #[inline(always)]
    pub fn ckoen(&self) -> CkoenR {
        CkoenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CKOCR")
            .field("ckoen", &self.ckoen())
            .field("ckodiv", &self.ckodiv())
            .field("ckosel", &self.ckosel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock out source select"]
    #[inline(always)]
    pub fn ckosel(&mut self) -> CkoselW<CkocrSpec> {
        CkoselW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Clock out input frequency Division Select"]
    #[inline(always)]
    pub fn ckodiv(&mut self) -> CkodivW<CkocrSpec> {
        CkodivW::new(self, 4)
    }
    #[doc = "Bit 7 - Clock out enable"]
    #[inline(always)]
    pub fn ckoen(&mut self) -> CkoenW<CkocrSpec> {
        CkoenW::new(self, 7)
    }
}
#[doc = "Clock Out Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ckocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CkocrSpec;
impl crate::RegisterSpec for CkocrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ckocr::R`](R) reader structure"]
impl crate::Readable for CkocrSpec {}
#[doc = "`write(|w| ..)` method takes [`ckocr::W`](W) writer structure"]
impl crate::Writable for CkocrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CKOCR to value 0"]
impl crate::Resettable for CkocrSpec {}
