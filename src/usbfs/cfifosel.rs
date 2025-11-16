#[doc = "Register `CFIFOSEL` reader"]
pub type R = crate::R<CfifoselSpec>;
#[doc = "Register `CFIFOSEL` writer"]
pub type W = crate::W<CfifoselSpec>;
#[doc = "CFIFO Port Access Pipe Specification\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Curpipe {
    #[doc = "0: DCP (Default control pipe)"]
    _0000 = 0,
    #[doc = "1: Pipe 1"]
    _0001 = 1,
    #[doc = "2: Pipe 2"]
    _0010 = 2,
    #[doc = "3: Pipe 3"]
    _0011 = 3,
    #[doc = "4: Pipe 4"]
    _0100 = 4,
    #[doc = "5: Pipe 5"]
    _0101 = 5,
    #[doc = "6: Pipe 6"]
    _0110 = 6,
    #[doc = "7: Pipe 7"]
    _0111 = 7,
    #[doc = "8: Pipe 8"]
    _1000 = 8,
    #[doc = "9: Pipe 9"]
    _1001 = 9,
    #[doc = "10: Setting prohibited"]
    Others = 10,
}
impl From<Curpipe> for u8 {
    #[inline(always)]
    fn from(variant: Curpipe) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Curpipe {
    type Ux = u8;
}
impl crate::IsEnum for Curpipe {}
#[doc = "Field `CURPIPE` reader - CFIFO Port Access Pipe Specification"]
pub type CurpipeR = crate::FieldReader<Curpipe>;
impl CurpipeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Curpipe {
        match self.bits {
            0 => Curpipe::_0000,
            1 => Curpipe::_0001,
            2 => Curpipe::_0010,
            3 => Curpipe::_0011,
            4 => Curpipe::_0100,
            5 => Curpipe::_0101,
            6 => Curpipe::_0110,
            7 => Curpipe::_0111,
            8 => Curpipe::_1000,
            9 => Curpipe::_1001,
            _ => Curpipe::Others,
        }
    }
    #[doc = "DCP (Default control pipe)"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == Curpipe::_0000
    }
    #[doc = "Pipe 1"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == Curpipe::_0001
    }
    #[doc = "Pipe 2"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == Curpipe::_0010
    }
    #[doc = "Pipe 3"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == Curpipe::_0011
    }
    #[doc = "Pipe 4"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == Curpipe::_0100
    }
    #[doc = "Pipe 5"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == Curpipe::_0101
    }
    #[doc = "Pipe 6"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == Curpipe::_0110
    }
    #[doc = "Pipe 7"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == Curpipe::_0111
    }
    #[doc = "Pipe 8"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == Curpipe::_1000
    }
    #[doc = "Pipe 9"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == Curpipe::_1001
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Curpipe::Others)
    }
}
#[doc = "Field `CURPIPE` writer - CFIFO Port Access Pipe Specification"]
pub type CurpipeW<'a, REG> = crate::FieldWriter<'a, REG, 4, Curpipe, crate::Safe>;
impl<'a, REG> CurpipeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DCP (Default control pipe)"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(Curpipe::_0000)
    }
    #[doc = "Pipe 1"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(Curpipe::_0001)
    }
    #[doc = "Pipe 2"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(Curpipe::_0010)
    }
    #[doc = "Pipe 3"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(Curpipe::_0011)
    }
    #[doc = "Pipe 4"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(Curpipe::_0100)
    }
    #[doc = "Pipe 5"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut crate::W<REG> {
        self.variant(Curpipe::_0101)
    }
    #[doc = "Pipe 6"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut crate::W<REG> {
        self.variant(Curpipe::_0110)
    }
    #[doc = "Pipe 7"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut crate::W<REG> {
        self.variant(Curpipe::_0111)
    }
    #[doc = "Pipe 8"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(Curpipe::_1000)
    }
    #[doc = "Pipe 9"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut crate::W<REG> {
        self.variant(Curpipe::_1001)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Curpipe::Others)
    }
}
#[doc = "CFIFO Port Access Direction When DCP is Selected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isel {
    #[doc = "0: Reading from the buffer memory is selected"]
    _0 = 0,
    #[doc = "1: Writing to the buffer memory is selected"]
    _1 = 1,
}
impl From<Isel> for bool {
    #[inline(always)]
    fn from(variant: Isel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISEL` reader - CFIFO Port Access Direction When DCP is Selected"]
pub type IselR = crate::BitReader<Isel>;
impl IselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isel {
        match self.bits {
            false => Isel::_0,
            true => Isel::_1,
        }
    }
    #[doc = "Reading from the buffer memory is selected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Isel::_0
    }
    #[doc = "Writing to the buffer memory is selected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Isel::_1
    }
}
#[doc = "Field `ISEL` writer - CFIFO Port Access Direction When DCP is Selected"]
pub type IselW<'a, REG> = crate::BitWriter<'a, REG, Isel>;
impl<'a, REG> IselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reading from the buffer memory is selected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Isel::_0)
    }
    #[doc = "Writing to the buffer memory is selected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Isel::_1)
    }
}
#[doc = "CFIFO Port Endian Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bigend {
    #[doc = "0: Little endian"]
    _0 = 0,
    #[doc = "1: Big endian"]
    _1 = 1,
}
impl From<Bigend> for bool {
    #[inline(always)]
    fn from(variant: Bigend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIGEND` reader - CFIFO Port Endian Control"]
pub type BigendR = crate::BitReader<Bigend>;
impl BigendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bigend {
        match self.bits {
            false => Bigend::_0,
            true => Bigend::_1,
        }
    }
    #[doc = "Little endian"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bigend::_0
    }
    #[doc = "Big endian"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bigend::_1
    }
}
#[doc = "Field `BIGEND` writer - CFIFO Port Endian Control"]
pub type BigendW<'a, REG> = crate::BitWriter<'a, REG, Bigend>;
impl<'a, REG> BigendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Little endian"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bigend::_0)
    }
    #[doc = "Big endian"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bigend::_1)
    }
}
#[doc = "CFIFO Port Access Bit Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mbw {
    #[doc = "0: 8-bit width"]
    _0 = 0,
    #[doc = "1: 16-bit width"]
    _1 = 1,
}
impl From<Mbw> for bool {
    #[inline(always)]
    fn from(variant: Mbw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MBW` reader - CFIFO Port Access Bit Width"]
pub type MbwR = crate::BitReader<Mbw>;
impl MbwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mbw {
        match self.bits {
            false => Mbw::_0,
            true => Mbw::_1,
        }
    }
    #[doc = "8-bit width"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mbw::_0
    }
    #[doc = "16-bit width"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mbw::_1
    }
}
#[doc = "Field `MBW` writer - CFIFO Port Access Bit Width"]
pub type MbwW<'a, REG> = crate::BitWriter<'a, REG, Mbw>;
impl<'a, REG> MbwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "8-bit width"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mbw::_0)
    }
    #[doc = "16-bit width"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mbw::_1)
    }
}
#[doc = "Buffer Pointer Rewind\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rew {
    #[doc = "0: The buffer pointer is not rewound."]
    _0 = 0,
    #[doc = "1: The buffer pointer is rewound."]
    _1 = 1,
}
impl From<Rew> for bool {
    #[inline(always)]
    fn from(variant: Rew) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REW` writer - Buffer Pointer Rewind"]
pub type RewW<'a, REG> = crate::BitWriter<'a, REG, Rew>;
impl<'a, REG> RewW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The buffer pointer is not rewound."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rew::_0)
    }
    #[doc = "The buffer pointer is rewound."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rew::_1)
    }
}
#[doc = "Read Count Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcnt {
    #[doc = "0: The DTLN\\[8:0\\] bits (CFIFOCRT.DTLN\\[8:0\\], D0FIFOCRT.DTLN\\[8:0\\], D1FIFOCRT.DTLN\\[8:0\\]) are cleared when all of the receive data has been read from the CFIFO.(In double buffer mode, the DTLN\\[8:0\\] bit value is cleared when all the data has been read from only a single plane.)"]
    _0 = 0,
    #[doc = "1: The DTLN\\[8:0\\] bits are decremented each time the receive data is read from the CFIFO."]
    _1 = 1,
}
impl From<Rcnt> for bool {
    #[inline(always)]
    fn from(variant: Rcnt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCNT` reader - Read Count Mode"]
pub type RcntR = crate::BitReader<Rcnt>;
impl RcntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rcnt {
        match self.bits {
            false => Rcnt::_0,
            true => Rcnt::_1,
        }
    }
    #[doc = "The DTLN\\[8:0\\] bits (CFIFOCRT.DTLN\\[8:0\\], D0FIFOCRT.DTLN\\[8:0\\], D1FIFOCRT.DTLN\\[8:0\\]) are cleared when all of the receive data has been read from the CFIFO.(In double buffer mode, the DTLN\\[8:0\\] bit value is cleared when all the data has been read from only a single plane.)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rcnt::_0
    }
    #[doc = "The DTLN\\[8:0\\] bits are decremented each time the receive data is read from the CFIFO."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rcnt::_1
    }
}
#[doc = "Field `RCNT` writer - Read Count Mode"]
pub type RcntW<'a, REG> = crate::BitWriter<'a, REG, Rcnt>;
impl<'a, REG> RcntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The DTLN\\[8:0\\] bits (CFIFOCRT.DTLN\\[8:0\\], D0FIFOCRT.DTLN\\[8:0\\], D1FIFOCRT.DTLN\\[8:0\\]) are cleared when all of the receive data has been read from the CFIFO.(In double buffer mode, the DTLN\\[8:0\\] bit value is cleared when all the data has been read from only a single plane.)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rcnt::_0)
    }
    #[doc = "The DTLN\\[8:0\\] bits are decremented each time the receive data is read from the CFIFO."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rcnt::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - CFIFO Port Access Pipe Specification"]
    #[inline(always)]
    pub fn curpipe(&self) -> CurpipeR {
        CurpipeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 5 - CFIFO Port Access Direction When DCP is Selected"]
    #[inline(always)]
    pub fn isel(&self) -> IselR {
        IselR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - CFIFO Port Endian Control"]
    #[inline(always)]
    pub fn bigend(&self) -> BigendR {
        BigendR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - CFIFO Port Access Bit Width"]
    #[inline(always)]
    pub fn mbw(&self) -> MbwR {
        MbwR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Read Count Mode"]
    #[inline(always)]
    pub fn rcnt(&self) -> RcntR {
        RcntR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFIFOSEL")
            .field("rcnt", &self.rcnt())
            .field("mbw", &self.mbw())
            .field("bigend", &self.bigend())
            .field("isel", &self.isel())
            .field("curpipe", &self.curpipe())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - CFIFO Port Access Pipe Specification"]
    #[inline(always)]
    pub fn curpipe(&mut self) -> CurpipeW<CfifoselSpec> {
        CurpipeW::new(self, 0)
    }
    #[doc = "Bit 5 - CFIFO Port Access Direction When DCP is Selected"]
    #[inline(always)]
    pub fn isel(&mut self) -> IselW<CfifoselSpec> {
        IselW::new(self, 5)
    }
    #[doc = "Bit 8 - CFIFO Port Endian Control"]
    #[inline(always)]
    pub fn bigend(&mut self) -> BigendW<CfifoselSpec> {
        BigendW::new(self, 8)
    }
    #[doc = "Bit 10 - CFIFO Port Access Bit Width"]
    #[inline(always)]
    pub fn mbw(&mut self) -> MbwW<CfifoselSpec> {
        MbwW::new(self, 10)
    }
    #[doc = "Bit 14 - Buffer Pointer Rewind"]
    #[inline(always)]
    pub fn rew(&mut self) -> RewW<CfifoselSpec> {
        RewW::new(self, 14)
    }
    #[doc = "Bit 15 - Read Count Mode"]
    #[inline(always)]
    pub fn rcnt(&mut self) -> RcntW<CfifoselSpec> {
        RcntW::new(self, 15)
    }
}
#[doc = "CFIFO Port Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfifosel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifosel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfifoselSpec;
impl crate::RegisterSpec for CfifoselSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfifosel::R`](R) reader structure"]
impl crate::Readable for CfifoselSpec {}
#[doc = "`write(|w| ..)` method takes [`cfifosel::W`](W) writer structure"]
impl crate::Writable for CfifoselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFIFOSEL to value 0"]
impl crate::Resettable for CfifoselSpec {}
