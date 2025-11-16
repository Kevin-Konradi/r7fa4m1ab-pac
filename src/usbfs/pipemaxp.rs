#[doc = "Register `PIPEMAXP` reader"]
pub type R = crate::R<PipemaxpSpec>;
#[doc = "Register `PIPEMAXP` writer"]
pub type W = crate::W<PipemaxpSpec>;
#[doc = "Field `MXPS` reader - Maximum Packet Size PIPE1 and PIPE2: 1 byte (001h) to 256 bytes (100h) PIPE3 to PIPE5: 8 bytes (008h), 16 bytes (010h), 32 bytes (020h), 64 bytes (040h) (Bits \\[8:7\\] and \\[2:0\\] are not provided.) PIPE6 to PIPE9: 1 byte (001h) to 64 bytes (040h) (Bits \\[8:7\\] are not provided.)"]
pub type MxpsR = crate::FieldReader<u16>;
#[doc = "Field `MXPS` writer - Maximum Packet Size PIPE1 and PIPE2: 1 byte (001h) to 256 bytes (100h) PIPE3 to PIPE5: 8 bytes (008h), 16 bytes (010h), 32 bytes (020h), 64 bytes (040h) (Bits \\[8:7\\] and \\[2:0\\] are not provided.) PIPE6 to PIPE9: 1 byte (001h) to 64 bytes (040h) (Bits \\[8:7\\] are not provided.)"]
pub type MxpsW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Device Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Devsel {
    #[doc = "0: Address 0000"]
    _0000 = 0,
    #[doc = "1: Address 0001"]
    _0001 = 1,
    #[doc = "2: Address 0010"]
    _0010 = 2,
    #[doc = "3: Address 0011"]
    _0011 = 3,
    #[doc = "4: Address 0100"]
    _0100 = 4,
    #[doc = "5: Address 0101"]
    _0101 = 5,
    #[doc = "6: Settings prohibited."]
    Others = 6,
}
impl From<Devsel> for u8 {
    #[inline(always)]
    fn from(variant: Devsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Devsel {
    type Ux = u8;
}
impl crate::IsEnum for Devsel {}
#[doc = "Field `DEVSEL` reader - Device Select"]
pub type DevselR = crate::FieldReader<Devsel>;
impl DevselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Devsel {
        match self.bits {
            0 => Devsel::_0000,
            1 => Devsel::_0001,
            2 => Devsel::_0010,
            3 => Devsel::_0011,
            4 => Devsel::_0100,
            5 => Devsel::_0101,
            _ => Devsel::Others,
        }
    }
    #[doc = "Address 0000"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == Devsel::_0000
    }
    #[doc = "Address 0001"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == Devsel::_0001
    }
    #[doc = "Address 0010"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == Devsel::_0010
    }
    #[doc = "Address 0011"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == Devsel::_0011
    }
    #[doc = "Address 0100"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == Devsel::_0100
    }
    #[doc = "Address 0101"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == Devsel::_0101
    }
    #[doc = "Settings prohibited."]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Devsel::Others)
    }
}
#[doc = "Field `DEVSEL` writer - Device Select"]
pub type DevselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Devsel, crate::Safe>;
impl<'a, REG> DevselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Address 0000"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(Devsel::_0000)
    }
    #[doc = "Address 0001"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(Devsel::_0001)
    }
    #[doc = "Address 0010"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(Devsel::_0010)
    }
    #[doc = "Address 0011"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(Devsel::_0011)
    }
    #[doc = "Address 0100"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(Devsel::_0100)
    }
    #[doc = "Address 0101"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut crate::W<REG> {
        self.variant(Devsel::_0101)
    }
    #[doc = "Settings prohibited."]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Devsel::Others)
    }
}
impl R {
    #[doc = "Bits 0:8 - Maximum Packet Size PIPE1 and PIPE2: 1 byte (001h) to 256 bytes (100h) PIPE3 to PIPE5: 8 bytes (008h), 16 bytes (010h), 32 bytes (020h), 64 bytes (040h) (Bits \\[8:7\\] and \\[2:0\\] are not provided.) PIPE6 to PIPE9: 1 byte (001h) to 64 bytes (040h) (Bits \\[8:7\\] are not provided.)"]
    #[inline(always)]
    pub fn mxps(&self) -> MxpsR {
        MxpsR::new(self.bits & 0x01ff)
    }
    #[doc = "Bits 12:15 - Device Select"]
    #[inline(always)]
    pub fn devsel(&self) -> DevselR {
        DevselR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIPEMAXP")
            .field("devsel", &self.devsel())
            .field("mxps", &self.mxps())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - Maximum Packet Size PIPE1 and PIPE2: 1 byte (001h) to 256 bytes (100h) PIPE3 to PIPE5: 8 bytes (008h), 16 bytes (010h), 32 bytes (020h), 64 bytes (040h) (Bits \\[8:7\\] and \\[2:0\\] are not provided.) PIPE6 to PIPE9: 1 byte (001h) to 64 bytes (040h) (Bits \\[8:7\\] are not provided.)"]
    #[inline(always)]
    pub fn mxps(&mut self) -> MxpsW<PipemaxpSpec> {
        MxpsW::new(self, 0)
    }
    #[doc = "Bits 12:15 - Device Select"]
    #[inline(always)]
    pub fn devsel(&mut self) -> DevselW<PipemaxpSpec> {
        DevselW::new(self, 12)
    }
}
#[doc = "Pipe Maximum Packet Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pipemaxp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipemaxp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PipemaxpSpec;
impl crate::RegisterSpec for PipemaxpSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pipemaxp::R`](R) reader structure"]
impl crate::Readable for PipemaxpSpec {}
#[doc = "`write(|w| ..)` method takes [`pipemaxp::W`](W) writer structure"]
impl crate::Writable for PipemaxpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PIPEMAXP to value 0"]
impl crate::Resettable for PipemaxpSpec {}
