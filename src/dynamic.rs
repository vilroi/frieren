use std::io::*;

#[derive(Debug, Default)]
#[repr(C)]
pub struct Dyn {
    pub d_tag: usize,
    pub d_val: usize,
}

#[derive(Debug)]
pub struct Dynamic {
    pub tag: EntryType,
    pub val: usize,
}

impl Dynamic {
    pub fn from_dyn(d: &Dyn) -> Result<Self> {
        let tag = match num_traits::cast::FromPrimitive::from_usize(d.d_tag) {
            Some(t) => t,
            None => return Err(Error::other(
                    format!("Failed to cast {:#x} into an dynamic::EntryType", d.d_tag))),
        };

        Ok(Dynamic {
            tag,
            val: d.d_val

        })
    }
}

#[derive(Debug, FromPrimitive)]
pub enum EntryType {
    Null,
    Needed,
    PltRelSize,
    PltGOT,
    Hash,
    Strtab,
    Symtab,
    Rela,
    RelaSize,
    RelaEnt,
    StrSize,
    SymEnt,
    Init,
    Fini,
    SoName,
    Rpath,
    Symbolic,
    Rel,
    Relsize,
    RelEt,
    PltRel,
    Debug,
    TextRel,
    JumpRel,
    BindNow,
    InitArray,
    FiniArray,
    InitArraySize,
    FiniArraySize,
    RunPath,
    Flags,
    Encoding,
    PreInitArray,
    PreInitArraySize,
    SymtabShndx,
    RelRSize,
    RelREnt,
    Num,
    LoOS = 0x6000000d,
    HiOs = 0x6ffff000,
    LoProc = 0x70000000,
    HiProc = 0x7fffffff,
    //PROCNUM	MIPS_NUM	
    ValRangeLow = 	0x6ffffd00,
    GnuPreLinked =  0x6ffffdf5,
    GnuConflictSize =  0x6ffffdf6,
    GnuLibListSize =  0x6ffffdf7,
    CheckSum = 	0x6ffffdf8,
    PltPadSize = 	0x6ffffdf9,
    MoveEnt	= 0x6ffffdfa,
    MoveSize = 	0x6ffffdfb,
    Feature1 = 	0x6ffffdfc,
    PosFlag1 = 	0x6ffffdfd,
    SymInfoSize = 	0x6ffffdfe,
    SymInfoEnt = 	0x6ffffdff,
    //VALTAGIDX(tag)	(VALRNGHI - (tag))	
    //VALNUM 12
    AddrRangeLow = 	0x6ffffe00,
    GnuHash = 	0x6ffffef5,
    TlsDescPlt = 0x6ffffef6,
    TlsDescGot = 	0x6ffffef7,
    GnuConflict = 	0x6ffffef8,
    GnuLibList = 	0x6ffffef9,
    Config = 	0x6ffffefa,
    DepAudit = 	0x6ffffefb,
    Audit = 	0x6ffffefc,
    PltPad = 	0x6ffffefd,
    MoveTab = 	0x6ffffefe,
    SymInfo = 	0x6ffffeff,
    //ADDRTAGIDX(tag)	(ADDRRNGHI - (tag))	
    //ADDRNUM 11
    VerSym = 	0x6ffffff0,
    RelaCount = 0x6ffffff9,
    RelCount = 	0x6ffffffa,
    Flags1 =	0x6ffffffb,
    VerDef = 	0x6ffffffc,
    VerDefNum = 	0x6ffffffd,
    VerNeeded = 	0x6ffffffe,
    VerNeededNum = 	0x6fffffff,
    //VERSIONTAGIDX(tag)	(VERNEEDNUM - (tag))	
    //VERSIONTAGNUM 16
    Auxiliary =     0x7ffffffd,

    /* Unsupported for now
    Filter =        0x7fffffff,
    EXTRATAGIDX(tag)	((Elf32_Word)-((Elf32_Sword) (tag) <<1>>1)-1)
    EXTRANUM	3
    DTF_1_PARINIT	0x00000001
    DTF_1_CONFEXP	0x00000002
    SPARC_REGISTER	0x70000001
    SPARC_NUM		2
    MIPS_RLD_VERSION  0x70000001	
    MIPS_TIME_STAMP   0x70000002	
    MIPS_ICHECKSUM    0x70000003	
    MIPS_IVERSION     0x70000004	
    MIPS_FLAGS	     0x70000005	
    MIPS_BASE_ADDRESS 0x70000006	
    MIPS_MSYM	     0x70000007
    MIPS_CONFLICT     0x70000008	
    MIPS_LIBLIST	     0x70000009	
    MIPS_LOCAL_GOTNO  0x7000000a	
    MIPS_CONFLICTNO   0x7000000b	
    MIPS_LIBLISTNO    0x70000010	
    MIPS_SYMTABNO     0x70000011	
    MIPS_UNREFEXTNO   0x70000012	
    MIPS_GOTSYM	     0x70000013	
    MIPS_HIPAGENO     0x70000014	
    MIPS_RLD_MAP	     0x70000016	
    MIPS_DELTA_CLASS  0x70000017	
    MIPS_DELTA_CLASS_NO    0x70000018 
    MIPS_DELTA_INSTANCE    0x70000019 
    MIPS_DELTA_INSTANCE_NO 0x7000001a 
    MIPS_DELTA_RELOC  0x7000001b 
    MIPS_DELTA_RELOC_NO 0x7000001c 
    MIPS_DELTA_SYM    0x7000001d 
    MIPS_DELTA_SYM_NO 0x7000001e 
    MIPS_DELTA_CLASSSYM 0x70000020 
    MIPS_DELTA_CLASSSYM_NO 0x70000021 
    MIPS_CXX_FLAGS    0x70000022 
    MIPS_PIXIE_INIT   0x70000023
    MIPS_SYMBOL_LIB   0x70000024
    MIPS_LOCALPAGE_GOTIDX 0x70000025
    MIPS_LOCAL_GOTIDX 0x70000026
    MIPS_HIDDEN_GOTIDX 0x70000027
    MIPS_PROTECTED_GOTIDX 0x70000028
    MIPS_OPTIONS	     0x70000029 
    MIPS_INTERFACE    0x7000002a 
    MIPS_DYNSTR_ALIGN 0x7000002b
    MIPS_INTERFACE_SIZE 0x7000002c 
    MIPS_RLD_TEXT_RESOLVE_ADDR 0x7000002d 
    MIPS_PERF_SUFFIX  0x7000002e 
    MIPS_COMPACT_SIZE 0x7000002f 
    MIPS_GP_VALUE     0x70000030 
    MIPS_AUX_DYNAMIC  0x70000031 
    MIPS_PLTGOT	     0x70000032
    MIPS_RWPLT        0x70000034
    MIPS_XHASH	     0x70000036
    MIPS_NUM	     0x37
    ALPHA_PLTRO		(LOPROC + 0)
    ALPHA_NUM		1
    PPC_GOT		(LOPROC + 0)
    PPC_OPT		(LOPROC + 1)
    PPC_NUM		2
    PPC64_GLINK  (LOPROC + 0)
    PPC64_OPD	(LOPROC + 1)
    PPC64_OPDSZ	(LOPROC + 2)
    PPC64_OPT	(LOPROC + 3)
    PPC64_NUM    4
    AARCH64_BTI_PLT	(LOPROC + 1)
    AARCH64_PAC_PLT	(LOPROC + 3)
    AARCH64_VARIANT_PCS	(LOPROC + 5)
    AARCH64_NUM		6
    IA_64_PLT_RESERVE	(LOPROC + 0)
    IA_64_NUM		1
    X86_64_PLT = 		(LOPROC + 0),
    X86_64_PLTSZ		(LOPROC + 1)
    X86_64_PLTENT	(LOPROC + 3)
    X86_64_NUM		4
    //NIOS2_GP             0x70000002 
    //RISCV_VARIANT_CC	(LOPROC + 1)

    */
}

