#[doc = "Register `MOSCWTCR` reader"]
pub type R = crate::R<MoscwtcrSpec>;
#[doc = "Register `MOSCWTCR` writer"]
pub type W = crate::W<MoscwtcrSpec>;
#[doc = "Main clock oscillator wait time setting\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Msts {
    #[doc = "0: Wait time = 2 cycles (0.25 us)"]
    _0000 = 0,
    #[doc = "1: Wait time = 1024 cycles (128 us)"]
    _0001 = 1,
    #[doc = "2: Wait time = 2048 cycles (256 us)"]
    _0010 = 2,
    #[doc = "3: Wait time = 4096 cycles (512 us)"]
    _0011 = 3,
    #[doc = "4: Wait time = 8192 cycles (1024 us)"]
    _0100 = 4,
    #[doc = "5: Wait time = 16384 cycles (2048 us) (value after reset)"]
    _0101 = 5,
    #[doc = "6: Wait time = 32768 cycles (4096 us)"]
    _0110 = 6,
    #[doc = "7: Wait time = 65536 cycles (8192 us)"]
    _0111 = 7,
    #[doc = "8: Wait time = 131072 cycles (16384 us)"]
    _1000 = 8,
    #[doc = "9: Wait time = 262144 cycles (32768 us)."]
    _1001 = 9,
    #[doc = "10: Setting prohibited"]
    Others = 10,
}
impl From<Msts> for u8 {
    #[inline(always)]
    fn from(variant: Msts) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Msts {
    type Ux = u8;
}
impl crate::IsEnum for Msts {}
#[doc = "Field `MSTS` reader - Main clock oscillator wait time setting"]
pub type MstsR = crate::FieldReader<Msts>;
impl MstsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msts {
        match self.bits {
            0 => Msts::_0000,
            1 => Msts::_0001,
            2 => Msts::_0010,
            3 => Msts::_0011,
            4 => Msts::_0100,
            5 => Msts::_0101,
            6 => Msts::_0110,
            7 => Msts::_0111,
            8 => Msts::_1000,
            9 => Msts::_1001,
            _ => Msts::Others,
        }
    }
    #[doc = "Wait time = 2 cycles (0.25 us)"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == Msts::_0000
    }
    #[doc = "Wait time = 1024 cycles (128 us)"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == Msts::_0001
    }
    #[doc = "Wait time = 2048 cycles (256 us)"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == Msts::_0010
    }
    #[doc = "Wait time = 4096 cycles (512 us)"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == Msts::_0011
    }
    #[doc = "Wait time = 8192 cycles (1024 us)"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == Msts::_0100
    }
    #[doc = "Wait time = 16384 cycles (2048 us) (value after reset)"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == Msts::_0101
    }
    #[doc = "Wait time = 32768 cycles (4096 us)"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == Msts::_0110
    }
    #[doc = "Wait time = 65536 cycles (8192 us)"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == Msts::_0111
    }
    #[doc = "Wait time = 131072 cycles (16384 us)"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == Msts::_1000
    }
    #[doc = "Wait time = 262144 cycles (32768 us)."]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == Msts::_1001
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Msts::Others)
    }
}
#[doc = "Field `MSTS` writer - Main clock oscillator wait time setting"]
pub type MstsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Msts, crate::Safe>;
impl<'a, REG> MstsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Wait time = 2 cycles (0.25 us)"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(Msts::_0000)
    }
    #[doc = "Wait time = 1024 cycles (128 us)"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(Msts::_0001)
    }
    #[doc = "Wait time = 2048 cycles (256 us)"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(Msts::_0010)
    }
    #[doc = "Wait time = 4096 cycles (512 us)"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(Msts::_0011)
    }
    #[doc = "Wait time = 8192 cycles (1024 us)"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(Msts::_0100)
    }
    #[doc = "Wait time = 16384 cycles (2048 us) (value after reset)"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut crate::W<REG> {
        self.variant(Msts::_0101)
    }
    #[doc = "Wait time = 32768 cycles (4096 us)"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut crate::W<REG> {
        self.variant(Msts::_0110)
    }
    #[doc = "Wait time = 65536 cycles (8192 us)"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut crate::W<REG> {
        self.variant(Msts::_0111)
    }
    #[doc = "Wait time = 131072 cycles (16384 us)"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(Msts::_1000)
    }
    #[doc = "Wait time = 262144 cycles (32768 us)."]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut crate::W<REG> {
        self.variant(Msts::_1001)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Msts::Others)
    }
}
impl R {
    #[doc = "Bits 0:3 - Main clock oscillator wait time setting"]
    #[inline(always)]
    pub fn msts(&self) -> MstsR {
        MstsR::new(self.bits & 0x0f)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MOSCWTCR")
            .field("msts", &self.msts())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Main clock oscillator wait time setting"]
    #[inline(always)]
    pub fn msts(&mut self) -> MstsW<MoscwtcrSpec> {
        MstsW::new(self, 0)
    }
}
#[doc = "Main Clock Oscillator Wait Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`moscwtcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moscwtcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MoscwtcrSpec;
impl crate::RegisterSpec for MoscwtcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`moscwtcr::R`](R) reader structure"]
impl crate::Readable for MoscwtcrSpec {}
#[doc = "`write(|w| ..)` method takes [`moscwtcr::W`](W) writer structure"]
impl crate::Writable for MoscwtcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MOSCWTCR to value 0x05"]
impl crate::Resettable for MoscwtcrSpec {
    const RESET_VALUE: u8 = 0x05;
}
