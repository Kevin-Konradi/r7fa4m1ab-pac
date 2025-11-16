#[doc = "Register `ICMR1` reader"]
pub type R = crate::R<Icmr1Spec>;
#[doc = "Register `ICMR1` writer"]
pub type W = crate::W<Icmr1Spec>;
#[doc = "Bit Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bc {
    #[doc = "0: 9 bits"]
    _000 = 0,
    #[doc = "1: 2 bits"]
    _001 = 1,
    #[doc = "2: 3 bits"]
    _010 = 2,
    #[doc = "3: 4 bits"]
    _011 = 3,
    #[doc = "4: 5 bits"]
    _100 = 4,
    #[doc = "5: 6 bits"]
    _101 = 5,
    #[doc = "6: 7 bits"]
    _110 = 6,
    #[doc = "7: 8 bits"]
    _111 = 7,
}
impl From<Bc> for u8 {
    #[inline(always)]
    fn from(variant: Bc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bc {
    type Ux = u8;
}
impl crate::IsEnum for Bc {}
#[doc = "Field `BC` reader - Bit Counter"]
pub type BcR = crate::FieldReader<Bc>;
impl BcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bc {
        match self.bits {
            0 => Bc::_000,
            1 => Bc::_001,
            2 => Bc::_010,
            3 => Bc::_011,
            4 => Bc::_100,
            5 => Bc::_101,
            6 => Bc::_110,
            7 => Bc::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "9 bits"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Bc::_000
    }
    #[doc = "2 bits"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Bc::_001
    }
    #[doc = "3 bits"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Bc::_010
    }
    #[doc = "4 bits"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Bc::_011
    }
    #[doc = "5 bits"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Bc::_100
    }
    #[doc = "6 bits"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Bc::_101
    }
    #[doc = "7 bits"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Bc::_110
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Bc::_111
    }
}
#[doc = "Field `BC` writer - Bit Counter"]
pub type BcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Bc, crate::Safe>;
impl<'a, REG> BcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "9 bits"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Bc::_000)
    }
    #[doc = "2 bits"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Bc::_001)
    }
    #[doc = "3 bits"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Bc::_010)
    }
    #[doc = "4 bits"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Bc::_011)
    }
    #[doc = "5 bits"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Bc::_100)
    }
    #[doc = "6 bits"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Bc::_101)
    }
    #[doc = "7 bits"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Bc::_110)
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Bc::_111)
    }
}
#[doc = "BC Write Protect (This bit is read as 1.)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bcwp {
    #[doc = "0: Enables a value to be written in the BC\\[2:0\\] bits."]
    _0 = 0,
    #[doc = "1: Disables a value to be written in the BC\\[2:0\\] bits."]
    _1 = 1,
}
impl From<Bcwp> for bool {
    #[inline(always)]
    fn from(variant: Bcwp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCWP` writer - BC Write Protect (This bit is read as 1.)"]
pub type BcwpW<'a, REG> = crate::BitWriter<'a, REG, Bcwp>;
impl<'a, REG> BcwpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enables a value to be written in the BC\\[2:0\\] bits."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bcwp::_0)
    }
    #[doc = "Disables a value to be written in the BC\\[2:0\\] bits."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bcwp::_1)
    }
}
#[doc = "Internal Reference Clock (fIIC) Selection ( fIIC = PCLKB / 2^CKS )\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cks {
    #[doc = "0: PCLKB/1 clock"]
    _000 = 0,
    #[doc = "1: PCLKB/2 clock"]
    _001 = 1,
    #[doc = "2: PCLKB/4 clock"]
    _010 = 2,
    #[doc = "3: PCLKB/8 clock"]
    _011 = 3,
    #[doc = "4: PCLKB/16 clock"]
    _100 = 4,
    #[doc = "5: PCLKB/32 clock"]
    _101 = 5,
    #[doc = "6: PCLKB/64 clock"]
    _110 = 6,
    #[doc = "7: PCLKB/128 clock"]
    _111 = 7,
}
impl From<Cks> for u8 {
    #[inline(always)]
    fn from(variant: Cks) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cks {
    type Ux = u8;
}
impl crate::IsEnum for Cks {}
#[doc = "Field `CKS` reader - Internal Reference Clock (fIIC) Selection ( fIIC = PCLKB / 2^CKS )"]
pub type CksR = crate::FieldReader<Cks>;
impl CksR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cks {
        match self.bits {
            0 => Cks::_000,
            1 => Cks::_001,
            2 => Cks::_010,
            3 => Cks::_011,
            4 => Cks::_100,
            5 => Cks::_101,
            6 => Cks::_110,
            7 => Cks::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "PCLKB/1 clock"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Cks::_000
    }
    #[doc = "PCLKB/2 clock"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Cks::_001
    }
    #[doc = "PCLKB/4 clock"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Cks::_010
    }
    #[doc = "PCLKB/8 clock"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Cks::_011
    }
    #[doc = "PCLKB/16 clock"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Cks::_100
    }
    #[doc = "PCLKB/32 clock"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Cks::_101
    }
    #[doc = "PCLKB/64 clock"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Cks::_110
    }
    #[doc = "PCLKB/128 clock"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Cks::_111
    }
}
#[doc = "Field `CKS` writer - Internal Reference Clock (fIIC) Selection ( fIIC = PCLKB / 2^CKS )"]
pub type CksW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cks, crate::Safe>;
impl<'a, REG> CksW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLKB/1 clock"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_000)
    }
    #[doc = "PCLKB/2 clock"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_001)
    }
    #[doc = "PCLKB/4 clock"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_010)
    }
    #[doc = "PCLKB/8 clock"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_011)
    }
    #[doc = "PCLKB/16 clock"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_100)
    }
    #[doc = "PCLKB/32 clock"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_101)
    }
    #[doc = "PCLKB/64 clock"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_110)
    }
    #[doc = "PCLKB/128 clock"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Cks::_111)
    }
}
#[doc = "MST/TRS Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mtwp {
    #[doc = "0: Disables writing to the MST and TRS bits in ICCR2."]
    _0 = 0,
    #[doc = "1: Enables writing to the MST and TRS bits in ICCR2."]
    _1 = 1,
}
impl From<Mtwp> for bool {
    #[inline(always)]
    fn from(variant: Mtwp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTWP` reader - MST/TRS Write Protect"]
pub type MtwpR = crate::BitReader<Mtwp>;
impl MtwpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mtwp {
        match self.bits {
            false => Mtwp::_0,
            true => Mtwp::_1,
        }
    }
    #[doc = "Disables writing to the MST and TRS bits in ICCR2."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mtwp::_0
    }
    #[doc = "Enables writing to the MST and TRS bits in ICCR2."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mtwp::_1
    }
}
#[doc = "Field `MTWP` writer - MST/TRS Write Protect"]
pub type MtwpW<'a, REG> = crate::BitWriter<'a, REG, Mtwp>;
impl<'a, REG> MtwpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables writing to the MST and TRS bits in ICCR2."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mtwp::_0)
    }
    #[doc = "Enables writing to the MST and TRS bits in ICCR2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mtwp::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Bit Counter"]
    #[inline(always)]
    pub fn bc(&self) -> BcR {
        BcR::new(self.bits & 7)
    }
    #[doc = "Bits 4:6 - Internal Reference Clock (fIIC) Selection ( fIIC = PCLKB / 2^CKS )"]
    #[inline(always)]
    pub fn cks(&self) -> CksR {
        CksR::new((self.bits >> 4) & 7)
    }
    #[doc = "Bit 7 - MST/TRS Write Protect"]
    #[inline(always)]
    pub fn mtwp(&self) -> MtwpR {
        MtwpR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICMR1")
            .field("mtwp", &self.mtwp())
            .field("cks", &self.cks())
            .field("bc", &self.bc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Bit Counter"]
    #[inline(always)]
    pub fn bc(&mut self) -> BcW<Icmr1Spec> {
        BcW::new(self, 0)
    }
    #[doc = "Bit 3 - BC Write Protect (This bit is read as 1.)"]
    #[inline(always)]
    pub fn bcwp(&mut self) -> BcwpW<Icmr1Spec> {
        BcwpW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Internal Reference Clock (fIIC) Selection ( fIIC = PCLKB / 2^CKS )"]
    #[inline(always)]
    pub fn cks(&mut self) -> CksW<Icmr1Spec> {
        CksW::new(self, 4)
    }
    #[doc = "Bit 7 - MST/TRS Write Protect"]
    #[inline(always)]
    pub fn mtwp(&mut self) -> MtwpW<Icmr1Spec> {
        MtwpW::new(self, 7)
    }
}
#[doc = "I2C Bus Mode Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`icmr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icmr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Icmr1Spec;
impl crate::RegisterSpec for Icmr1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`icmr1::R`](R) reader structure"]
impl crate::Readable for Icmr1Spec {}
#[doc = "`write(|w| ..)` method takes [`icmr1::W`](W) writer structure"]
impl crate::Writable for Icmr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICMR1 to value 0x08"]
impl crate::Resettable for Icmr1Spec {
    const RESET_VALUE: u8 = 0x08;
}
