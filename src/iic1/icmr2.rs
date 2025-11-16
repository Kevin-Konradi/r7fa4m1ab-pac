#[doc = "Register `ICMR2` reader"]
pub type R = crate::R<Icmr2Spec>;
#[doc = "Register `ICMR2` writer"]
pub type W = crate::W<Icmr2Spec>;
#[doc = "Timeout Detection Time Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmos {
    #[doc = "0: Long mode is selected."]
    _0 = 0,
    #[doc = "1: Short mode is selected."]
    _1 = 1,
}
impl From<Tmos> for bool {
    #[inline(always)]
    fn from(variant: Tmos) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMOS` reader - Timeout Detection Time Select"]
pub type TmosR = crate::BitReader<Tmos>;
impl TmosR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmos {
        match self.bits {
            false => Tmos::_0,
            true => Tmos::_1,
        }
    }
    #[doc = "Long mode is selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tmos::_0
    }
    #[doc = "Short mode is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tmos::_1
    }
}
#[doc = "Field `TMOS` writer - Timeout Detection Time Select"]
pub type TmosW<'a, REG> = crate::BitWriter<'a, REG, Tmos>;
impl<'a, REG> TmosW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Long mode is selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tmos::_0)
    }
    #[doc = "Short mode is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tmos::_1)
    }
}
#[doc = "Timeout L Count Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmol {
    #[doc = "0: Count is disabled while the SCLn line is at a low level."]
    _0 = 0,
    #[doc = "1: Count is enabled while the SCLn line is at a low level."]
    _1 = 1,
}
impl From<Tmol> for bool {
    #[inline(always)]
    fn from(variant: Tmol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMOL` reader - Timeout L Count Control"]
pub type TmolR = crate::BitReader<Tmol>;
impl TmolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmol {
        match self.bits {
            false => Tmol::_0,
            true => Tmol::_1,
        }
    }
    #[doc = "Count is disabled while the SCLn line is at a low level."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tmol::_0
    }
    #[doc = "Count is enabled while the SCLn line is at a low level."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tmol::_1
    }
}
#[doc = "Field `TMOL` writer - Timeout L Count Control"]
pub type TmolW<'a, REG> = crate::BitWriter<'a, REG, Tmol>;
impl<'a, REG> TmolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Count is disabled while the SCLn line is at a low level."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tmol::_0)
    }
    #[doc = "Count is enabled while the SCLn line is at a low level."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tmol::_1)
    }
}
#[doc = "Timeout H Count Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmoh {
    #[doc = "0: Count is disabled while the SCLn line is at a high level."]
    _0 = 0,
    #[doc = "1: Count is enabled while the SCLn line is at a high level."]
    _1 = 1,
}
impl From<Tmoh> for bool {
    #[inline(always)]
    fn from(variant: Tmoh) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMOH` reader - Timeout H Count Control"]
pub type TmohR = crate::BitReader<Tmoh>;
impl TmohR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmoh {
        match self.bits {
            false => Tmoh::_0,
            true => Tmoh::_1,
        }
    }
    #[doc = "Count is disabled while the SCLn line is at a high level."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tmoh::_0
    }
    #[doc = "Count is enabled while the SCLn line is at a high level."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tmoh::_1
    }
}
#[doc = "Field `TMOH` writer - Timeout H Count Control"]
pub type TmohW<'a, REG> = crate::BitWriter<'a, REG, Tmoh>;
impl<'a, REG> TmohW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Count is disabled while the SCLn line is at a high level."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tmoh::_0)
    }
    #[doc = "Count is enabled while the SCLn line is at a high level."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tmoh::_1)
    }
}
#[doc = "SDA Output Delay Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sddl {
    #[doc = "0: No output delay"]
    _000 = 0,
    #[doc = "1: 1 fIIC cycle (ICMR2.DLCS=0) / 1 or 2 fIIC cycles (ICMR2.DLCS=1)"]
    _001 = 1,
    #[doc = "2: 2 fIIC cycles (ICMR2.DLCS=0) / 3 or 4 fIIC cycles (ICMR2.DLCS=1)"]
    _010 = 2,
    #[doc = "3: 3 fIIC cycles (ICMR2.DLCS=0) / 5 or 6 fIIC cycles (ICMR2.DLCS=1)"]
    _011 = 3,
    #[doc = "4: 4 fIIC cycles (ICMR2.DLCS=0) / 7 or 8 fIIC cycles (ICMR2.DLCS=1)"]
    _100 = 4,
    #[doc = "5: 5 fIIC cycles (ICMR2.DLCS=0) / 9 or 10 fIIC cycles (ICMR2.DLCS=1)"]
    _101 = 5,
    #[doc = "6: 6 fIIC cycles (ICMR2.DLCS=0) / 11 or 12 fIIC cycles (ICMR2.DLCS=1)"]
    _110 = 6,
    #[doc = "7: 7 fIIC cycles (ICMR2.DLCS=0) / 13 or 14 fIIC cycles (ICMR2.DLCS=1)"]
    _111 = 7,
}
impl From<Sddl> for u8 {
    #[inline(always)]
    fn from(variant: Sddl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sddl {
    type Ux = u8;
}
impl crate::IsEnum for Sddl {}
#[doc = "Field `SDDL` reader - SDA Output Delay Counter"]
pub type SddlR = crate::FieldReader<Sddl>;
impl SddlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sddl {
        match self.bits {
            0 => Sddl::_000,
            1 => Sddl::_001,
            2 => Sddl::_010,
            3 => Sddl::_011,
            4 => Sddl::_100,
            5 => Sddl::_101,
            6 => Sddl::_110,
            7 => Sddl::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "No output delay"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Sddl::_000
    }
    #[doc = "1 fIIC cycle (ICMR2.DLCS=0) / 1 or 2 fIIC cycles (ICMR2.DLCS=1)"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Sddl::_001
    }
    #[doc = "2 fIIC cycles (ICMR2.DLCS=0) / 3 or 4 fIIC cycles (ICMR2.DLCS=1)"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Sddl::_010
    }
    #[doc = "3 fIIC cycles (ICMR2.DLCS=0) / 5 or 6 fIIC cycles (ICMR2.DLCS=1)"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Sddl::_011
    }
    #[doc = "4 fIIC cycles (ICMR2.DLCS=0) / 7 or 8 fIIC cycles (ICMR2.DLCS=1)"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Sddl::_100
    }
    #[doc = "5 fIIC cycles (ICMR2.DLCS=0) / 9 or 10 fIIC cycles (ICMR2.DLCS=1)"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Sddl::_101
    }
    #[doc = "6 fIIC cycles (ICMR2.DLCS=0) / 11 or 12 fIIC cycles (ICMR2.DLCS=1)"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Sddl::_110
    }
    #[doc = "7 fIIC cycles (ICMR2.DLCS=0) / 13 or 14 fIIC cycles (ICMR2.DLCS=1)"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Sddl::_111
    }
}
#[doc = "Field `SDDL` writer - SDA Output Delay Counter"]
pub type SddlW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sddl, crate::Safe>;
impl<'a, REG> SddlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No output delay"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Sddl::_000)
    }
    #[doc = "1 fIIC cycle (ICMR2.DLCS=0) / 1 or 2 fIIC cycles (ICMR2.DLCS=1)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Sddl::_001)
    }
    #[doc = "2 fIIC cycles (ICMR2.DLCS=0) / 3 or 4 fIIC cycles (ICMR2.DLCS=1)"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Sddl::_010)
    }
    #[doc = "3 fIIC cycles (ICMR2.DLCS=0) / 5 or 6 fIIC cycles (ICMR2.DLCS=1)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Sddl::_011)
    }
    #[doc = "4 fIIC cycles (ICMR2.DLCS=0) / 7 or 8 fIIC cycles (ICMR2.DLCS=1)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Sddl::_100)
    }
    #[doc = "5 fIIC cycles (ICMR2.DLCS=0) / 9 or 10 fIIC cycles (ICMR2.DLCS=1)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Sddl::_101)
    }
    #[doc = "6 fIIC cycles (ICMR2.DLCS=0) / 11 or 12 fIIC cycles (ICMR2.DLCS=1)"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Sddl::_110)
    }
    #[doc = "7 fIIC cycles (ICMR2.DLCS=0) / 13 or 14 fIIC cycles (ICMR2.DLCS=1)"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Sddl::_111)
    }
}
#[doc = "SDA Output Delay Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dlcs {
    #[doc = "0: The internal reference clock (fIIC) is selected as the clock source of the SDA output delay counter."]
    _0 = 0,
    #[doc = "1: The internal reference clock divided by 2 (fIIC/2) is selected as the clock source of the SDA output delay counter."]
    _1 = 1,
}
impl From<Dlcs> for bool {
    #[inline(always)]
    fn from(variant: Dlcs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLCS` reader - SDA Output Delay Clock Source Select"]
pub type DlcsR = crate::BitReader<Dlcs>;
impl DlcsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dlcs {
        match self.bits {
            false => Dlcs::_0,
            true => Dlcs::_1,
        }
    }
    #[doc = "The internal reference clock (fIIC) is selected as the clock source of the SDA output delay counter."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dlcs::_0
    }
    #[doc = "The internal reference clock divided by 2 (fIIC/2) is selected as the clock source of the SDA output delay counter."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dlcs::_1
    }
}
#[doc = "Field `DLCS` writer - SDA Output Delay Clock Source Select"]
pub type DlcsW<'a, REG> = crate::BitWriter<'a, REG, Dlcs>;
impl<'a, REG> DlcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The internal reference clock (fIIC) is selected as the clock source of the SDA output delay counter."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dlcs::_0)
    }
    #[doc = "The internal reference clock divided by 2 (fIIC/2) is selected as the clock source of the SDA output delay counter."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dlcs::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Timeout Detection Time Select"]
    #[inline(always)]
    pub fn tmos(&self) -> TmosR {
        TmosR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timeout L Count Control"]
    #[inline(always)]
    pub fn tmol(&self) -> TmolR {
        TmolR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timeout H Count Control"]
    #[inline(always)]
    pub fn tmoh(&self) -> TmohR {
        TmohR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - SDA Output Delay Counter"]
    #[inline(always)]
    pub fn sddl(&self) -> SddlR {
        SddlR::new((self.bits >> 4) & 7)
    }
    #[doc = "Bit 7 - SDA Output Delay Clock Source Select"]
    #[inline(always)]
    pub fn dlcs(&self) -> DlcsR {
        DlcsR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICMR2")
            .field("dlcs", &self.dlcs())
            .field("sddl", &self.sddl())
            .field("tmoh", &self.tmoh())
            .field("tmol", &self.tmol())
            .field("tmos", &self.tmos())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Timeout Detection Time Select"]
    #[inline(always)]
    pub fn tmos(&mut self) -> TmosW<Icmr2Spec> {
        TmosW::new(self, 0)
    }
    #[doc = "Bit 1 - Timeout L Count Control"]
    #[inline(always)]
    pub fn tmol(&mut self) -> TmolW<Icmr2Spec> {
        TmolW::new(self, 1)
    }
    #[doc = "Bit 2 - Timeout H Count Control"]
    #[inline(always)]
    pub fn tmoh(&mut self) -> TmohW<Icmr2Spec> {
        TmohW::new(self, 2)
    }
    #[doc = "Bits 4:6 - SDA Output Delay Counter"]
    #[inline(always)]
    pub fn sddl(&mut self) -> SddlW<Icmr2Spec> {
        SddlW::new(self, 4)
    }
    #[doc = "Bit 7 - SDA Output Delay Clock Source Select"]
    #[inline(always)]
    pub fn dlcs(&mut self) -> DlcsW<Icmr2Spec> {
        DlcsW::new(self, 7)
    }
}
#[doc = "I2C Bus Mode Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`icmr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icmr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Icmr2Spec;
impl crate::RegisterSpec for Icmr2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`icmr2::R`](R) reader structure"]
impl crate::Readable for Icmr2Spec {}
#[doc = "`write(|w| ..)` method takes [`icmr2::W`](W) writer structure"]
impl crate::Writable for Icmr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICMR2 to value 0x06"]
impl crate::Resettable for Icmr2Spec {
    const RESET_VALUE: u8 = 0x06;
}
