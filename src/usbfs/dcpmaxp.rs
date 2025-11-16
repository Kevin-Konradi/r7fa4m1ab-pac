#[doc = "Register `DCPMAXP` reader"]
pub type R = crate::R<DcpmaxpSpec>;
#[doc = "Register `DCPMAXP` writer"]
pub type W = crate::W<DcpmaxpSpec>;
#[doc = "Maximum Packet Size These bits set the maximum amount of data (maximum packet size) in payloads for the DCP.\n\nValue on reset: 64"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mxps {
    #[doc = "8: 8 bytes"]
    _0x08 = 8,
    #[doc = "16: 16 bytes"]
    _0x10 = 16,
    #[doc = "24: 24 bytes"]
    _0x18 = 24,
    #[doc = "32: 32 bytes"]
    _0x20 = 32,
    #[doc = "40: 40 bytes"]
    _0x28 = 40,
    #[doc = "48: 48 bytes"]
    _0x30 = 48,
    #[doc = "56: 56 bytes"]
    _0x38 = 56,
    #[doc = "64: 64 bytes"]
    _0x40 = 64,
    #[doc = "72: 72 bytes"]
    _0x48 = 72,
    #[doc = "80: 80 bytes"]
    _0x50 = 80,
    #[doc = "88: 88 bytes"]
    _0x58 = 88,
    #[doc = "96: 96 bytes"]
    _0x60 = 96,
    #[doc = "104: 104 bytes"]
    _0x68 = 104,
    #[doc = "112: 112 bytes"]
    _0x70 = 112,
    #[doc = "120: 120 bytes"]
    _0x78 = 120,
    #[doc = "0: Setting prohibited"]
    Others = 0,
}
impl From<Mxps> for u8 {
    #[inline(always)]
    fn from(variant: Mxps) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mxps {
    type Ux = u8;
}
impl crate::IsEnum for Mxps {}
#[doc = "Field `MXPS` reader - Maximum Packet Size These bits set the maximum amount of data (maximum packet size) in payloads for the DCP."]
pub type MxpsR = crate::FieldReader<Mxps>;
impl MxpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mxps {
        match self.bits {
            8 => Mxps::_0x08,
            16 => Mxps::_0x10,
            24 => Mxps::_0x18,
            32 => Mxps::_0x20,
            40 => Mxps::_0x28,
            48 => Mxps::_0x30,
            56 => Mxps::_0x38,
            64 => Mxps::_0x40,
            72 => Mxps::_0x48,
            80 => Mxps::_0x50,
            88 => Mxps::_0x58,
            96 => Mxps::_0x60,
            104 => Mxps::_0x68,
            112 => Mxps::_0x70,
            120 => Mxps::_0x78,
            _ => Mxps::Others,
        }
    }
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn is_0x08(&self) -> bool {
        *self == Mxps::_0x08
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn is_0x10(&self) -> bool {
        *self == Mxps::_0x10
    }
    #[doc = "24 bytes"]
    #[inline(always)]
    pub fn is_0x18(&self) -> bool {
        *self == Mxps::_0x18
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn is_0x20(&self) -> bool {
        *self == Mxps::_0x20
    }
    #[doc = "40 bytes"]
    #[inline(always)]
    pub fn is_0x28(&self) -> bool {
        *self == Mxps::_0x28
    }
    #[doc = "48 bytes"]
    #[inline(always)]
    pub fn is_0x30(&self) -> bool {
        *self == Mxps::_0x30
    }
    #[doc = "56 bytes"]
    #[inline(always)]
    pub fn is_0x38(&self) -> bool {
        *self == Mxps::_0x38
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn is_0x40(&self) -> bool {
        *self == Mxps::_0x40
    }
    #[doc = "72 bytes"]
    #[inline(always)]
    pub fn is_0x48(&self) -> bool {
        *self == Mxps::_0x48
    }
    #[doc = "80 bytes"]
    #[inline(always)]
    pub fn is_0x50(&self) -> bool {
        *self == Mxps::_0x50
    }
    #[doc = "88 bytes"]
    #[inline(always)]
    pub fn is_0x58(&self) -> bool {
        *self == Mxps::_0x58
    }
    #[doc = "96 bytes"]
    #[inline(always)]
    pub fn is_0x60(&self) -> bool {
        *self == Mxps::_0x60
    }
    #[doc = "104 bytes"]
    #[inline(always)]
    pub fn is_0x68(&self) -> bool {
        *self == Mxps::_0x68
    }
    #[doc = "112 bytes"]
    #[inline(always)]
    pub fn is_0x70(&self) -> bool {
        *self == Mxps::_0x70
    }
    #[doc = "120 bytes"]
    #[inline(always)]
    pub fn is_0x78(&self) -> bool {
        *self == Mxps::_0x78
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Mxps::Others)
    }
}
#[doc = "Field `MXPS` writer - Maximum Packet Size These bits set the maximum amount of data (maximum packet size) in payloads for the DCP."]
pub type MxpsW<'a, REG> = crate::FieldWriter<'a, REG, 7, Mxps, crate::Safe>;
impl<'a, REG> MxpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn _0x08(self) -> &'a mut crate::W<REG> {
        self.variant(Mxps::_0x08)
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn _0x10(self) -> &'a mut crate::W<REG> {
        self.variant(Mxps::_0x10)
    }
    #[doc = "24 bytes"]
    #[inline(always)]
    pub fn _0x18(self) -> &'a mut crate::W<REG> {
        self.variant(Mxps::_0x18)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn _0x20(self) -> &'a mut crate::W<REG> {
        self.variant(Mxps::_0x20)
    }
    #[doc = "40 bytes"]
    #[inline(always)]
    pub fn _0x28(self) -> &'a mut crate::W<REG> {
        self.variant(Mxps::_0x28)
    }
    #[doc = "48 bytes"]
    #[inline(always)]
    pub fn _0x30(self) -> &'a mut crate::W<REG> {
        self.variant(Mxps::_0x30)
    }
    #[doc = "56 bytes"]
    #[inline(always)]
    pub fn _0x38(self) -> &'a mut crate::W<REG> {
        self.variant(Mxps::_0x38)
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn _0x40(self) -> &'a mut crate::W<REG> {
        self.variant(Mxps::_0x40)
    }
    #[doc = "72 bytes"]
    #[inline(always)]
    pub fn _0x48(self) -> &'a mut crate::W<REG> {
        self.variant(Mxps::_0x48)
    }
    #[doc = "80 bytes"]
    #[inline(always)]
    pub fn _0x50(self) -> &'a mut crate::W<REG> {
        self.variant(Mxps::_0x50)
    }
    #[doc = "88 bytes"]
    #[inline(always)]
    pub fn _0x58(self) -> &'a mut crate::W<REG> {
        self.variant(Mxps::_0x58)
    }
    #[doc = "96 bytes"]
    #[inline(always)]
    pub fn _0x60(self) -> &'a mut crate::W<REG> {
        self.variant(Mxps::_0x60)
    }
    #[doc = "104 bytes"]
    #[inline(always)]
    pub fn _0x68(self) -> &'a mut crate::W<REG> {
        self.variant(Mxps::_0x68)
    }
    #[doc = "112 bytes"]
    #[inline(always)]
    pub fn _0x70(self) -> &'a mut crate::W<REG> {
        self.variant(Mxps::_0x70)
    }
    #[doc = "120 bytes"]
    #[inline(always)]
    pub fn _0x78(self) -> &'a mut crate::W<REG> {
        self.variant(Mxps::_0x78)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Mxps::Others)
    }
}
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
    #[doc = "Bits 0:6 - Maximum Packet Size These bits set the maximum amount of data (maximum packet size) in payloads for the DCP."]
    #[inline(always)]
    pub fn mxps(&self) -> MxpsR {
        MxpsR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 12:15 - Device Select"]
    #[inline(always)]
    pub fn devsel(&self) -> DevselR {
        DevselR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCPMAXP")
            .field("devsel", &self.devsel())
            .field("mxps", &self.mxps())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - Maximum Packet Size These bits set the maximum amount of data (maximum packet size) in payloads for the DCP."]
    #[inline(always)]
    pub fn mxps(&mut self) -> MxpsW<DcpmaxpSpec> {
        MxpsW::new(self, 0)
    }
    #[doc = "Bits 12:15 - Device Select"]
    #[inline(always)]
    pub fn devsel(&mut self) -> DevselW<DcpmaxpSpec> {
        DevselW::new(self, 12)
    }
}
#[doc = "DCP Maximum Packet Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcpmaxp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcpmaxp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcpmaxpSpec;
impl crate::RegisterSpec for DcpmaxpSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dcpmaxp::R`](R) reader structure"]
impl crate::Readable for DcpmaxpSpec {}
#[doc = "`write(|w| ..)` method takes [`dcpmaxp::W`](W) writer structure"]
impl crate::Writable for DcpmaxpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCPMAXP to value 0x40"]
impl crate::Resettable for DcpmaxpSpec {
    const RESET_VALUE: u16 = 0x40;
}
