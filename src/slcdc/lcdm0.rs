#[doc = "Register `LCDM0` reader"]
pub type R = crate::R<Lcdm0Spec>;
#[doc = "Register `LCDM0` writer"]
pub type W = crate::W<Lcdm0Spec>;
#[doc = "LCD Display Bias Method Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lbas {
    #[doc = "0: 1/2 bias method"]
    _00 = 0,
    #[doc = "1: 1/3 bias method"]
    _01 = 1,
    #[doc = "2: 1/4 bias method"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<Lbas> for u8 {
    #[inline(always)]
    fn from(variant: Lbas) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lbas {
    type Ux = u8;
}
impl crate::IsEnum for Lbas {}
#[doc = "Field `LBAS` reader - LCD Display Bias Method Select"]
pub type LbasR = crate::FieldReader<Lbas>;
impl LbasR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lbas {
        match self.bits {
            0 => Lbas::_00,
            1 => Lbas::_01,
            2 => Lbas::_10,
            3 => Lbas::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "1/2 bias method"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Lbas::_00
    }
    #[doc = "1/3 bias method"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Lbas::_01
    }
    #[doc = "1/4 bias method"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Lbas::_10
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Lbas::_11
    }
}
#[doc = "Field `LBAS` writer - LCD Display Bias Method Select"]
pub type LbasW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lbas, crate::Safe>;
impl<'a, REG> LbasW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1/2 bias method"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Lbas::_00)
    }
    #[doc = "1/3 bias method"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Lbas::_01)
    }
    #[doc = "1/4 bias method"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Lbas::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Lbas::_11)
    }
}
#[doc = "Time Slice of LCD Display Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ldty {
    #[doc = "0: Static"]
    _000 = 0,
    #[doc = "1: 2-time slice"]
    _001 = 1,
    #[doc = "2: 3-time slice"]
    _010 = 2,
    #[doc = "3: 4-time slice"]
    _011 = 3,
    #[doc = "5: 8-time slice"]
    _101 = 5,
    #[doc = "4: Setting prohibited"]
    Others = 4,
}
impl From<Ldty> for u8 {
    #[inline(always)]
    fn from(variant: Ldty) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ldty {
    type Ux = u8;
}
impl crate::IsEnum for Ldty {}
#[doc = "Field `LDTY` reader - Time Slice of LCD Display Select"]
pub type LdtyR = crate::FieldReader<Ldty>;
impl LdtyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ldty {
        match self.bits {
            0 => Ldty::_000,
            1 => Ldty::_001,
            2 => Ldty::_010,
            3 => Ldty::_011,
            5 => Ldty::_101,
            _ => Ldty::Others,
        }
    }
    #[doc = "Static"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Ldty::_000
    }
    #[doc = "2-time slice"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Ldty::_001
    }
    #[doc = "3-time slice"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Ldty::_010
    }
    #[doc = "4-time slice"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Ldty::_011
    }
    #[doc = "8-time slice"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Ldty::_101
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Ldty::Others)
    }
}
#[doc = "Field `LDTY` writer - Time Slice of LCD Display Select"]
pub type LdtyW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ldty, crate::Safe>;
impl<'a, REG> LdtyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Static"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Ldty::_000)
    }
    #[doc = "2-time slice"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Ldty::_001)
    }
    #[doc = "3-time slice"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Ldty::_010)
    }
    #[doc = "4-time slice"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Ldty::_011)
    }
    #[doc = "8-time slice"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Ldty::_101)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Ldty::Others)
    }
}
#[doc = "LCD display waveform selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lwave {
    #[doc = "0: Waveform A"]
    _0 = 0,
    #[doc = "1: Waveform B"]
    _1 = 1,
}
impl From<Lwave> for bool {
    #[inline(always)]
    fn from(variant: Lwave) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LWAVE` reader - LCD display waveform selection"]
pub type LwaveR = crate::BitReader<Lwave>;
impl LwaveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lwave {
        match self.bits {
            false => Lwave::_0,
            true => Lwave::_1,
        }
    }
    #[doc = "Waveform A"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lwave::_0
    }
    #[doc = "Waveform B"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lwave::_1
    }
}
#[doc = "Field `LWAVE` writer - LCD display waveform selection"]
pub type LwaveW<'a, REG> = crate::BitWriter<'a, REG, Lwave>;
impl<'a, REG> LwaveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Waveform A"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lwave::_0)
    }
    #[doc = "Waveform B"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lwave::_1)
    }
}
#[doc = "LCD drive voltage generator selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mdset {
    #[doc = "0: External resistance division method"]
    _00 = 0,
    #[doc = "1: Internal voltage boosting method"]
    _01 = 1,
    #[doc = "2: Capacitor split method"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<Mdset> for u8 {
    #[inline(always)]
    fn from(variant: Mdset) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mdset {
    type Ux = u8;
}
impl crate::IsEnum for Mdset {}
#[doc = "Field `MDSET` reader - LCD drive voltage generator selection"]
pub type MdsetR = crate::FieldReader<Mdset>;
impl MdsetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mdset {
        match self.bits {
            0 => Mdset::_00,
            1 => Mdset::_01,
            2 => Mdset::_10,
            3 => Mdset::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "External resistance division method"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Mdset::_00
    }
    #[doc = "Internal voltage boosting method"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Mdset::_01
    }
    #[doc = "Capacitor split method"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Mdset::_10
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Mdset::_11
    }
}
#[doc = "Field `MDSET` writer - LCD drive voltage generator selection"]
pub type MdsetW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mdset, crate::Safe>;
impl<'a, REG> MdsetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External resistance division method"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Mdset::_00)
    }
    #[doc = "Internal voltage boosting method"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Mdset::_01)
    }
    #[doc = "Capacitor split method"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Mdset::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Mdset::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - LCD Display Bias Method Select"]
    #[inline(always)]
    pub fn lbas(&self) -> LbasR {
        LbasR::new(self.bits & 3)
    }
    #[doc = "Bits 2:4 - Time Slice of LCD Display Select"]
    #[inline(always)]
    pub fn ldty(&self) -> LdtyR {
        LdtyR::new((self.bits >> 2) & 7)
    }
    #[doc = "Bit 5 - LCD display waveform selection"]
    #[inline(always)]
    pub fn lwave(&self) -> LwaveR {
        LwaveR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - LCD drive voltage generator selection"]
    #[inline(always)]
    pub fn mdset(&self) -> MdsetR {
        MdsetR::new((self.bits >> 6) & 3)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCDM0")
            .field("mdset", &self.mdset())
            .field("lwave", &self.lwave())
            .field("ldty", &self.ldty())
            .field("lbas", &self.lbas())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - LCD Display Bias Method Select"]
    #[inline(always)]
    pub fn lbas(&mut self) -> LbasW<Lcdm0Spec> {
        LbasW::new(self, 0)
    }
    #[doc = "Bits 2:4 - Time Slice of LCD Display Select"]
    #[inline(always)]
    pub fn ldty(&mut self) -> LdtyW<Lcdm0Spec> {
        LdtyW::new(self, 2)
    }
    #[doc = "Bit 5 - LCD display waveform selection"]
    #[inline(always)]
    pub fn lwave(&mut self) -> LwaveW<Lcdm0Spec> {
        LwaveW::new(self, 5)
    }
    #[doc = "Bits 6:7 - LCD drive voltage generator selection"]
    #[inline(always)]
    pub fn mdset(&mut self) -> MdsetW<Lcdm0Spec> {
        MdsetW::new(self, 6)
    }
}
#[doc = "LCD Mode Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm0Spec;
impl crate::RegisterSpec for Lcdm0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm0::R`](R) reader structure"]
impl crate::Readable for Lcdm0Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm0::W`](W) writer structure"]
impl crate::Writable for Lcdm0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM0 to value 0"]
impl crate::Resettable for Lcdm0Spec {}
