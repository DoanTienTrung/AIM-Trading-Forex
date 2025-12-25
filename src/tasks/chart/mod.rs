use slint::{Model, ModelRc, VecModel};
// Import StockData with a more specific name to avoid conflicts
use crate::slint_generatedAppWindow::{
    MarketWatchData as SlintMarketWatchData, ShortType, StockData as SlintStockData,
    StockGroup as SlintStockGroup,
};
use chrono::{Datelike, Timelike};

mod chart_update;
mod company_profile;
mod data_update;
mod finance_report;
mod finance_sheet;
mod stock_update;
mod ui_chart;
mod quantitative_streamgraph;

pub use chart_update::*;
pub use company_profile::*;
pub use data_update::*;
pub use finance_report::*;
pub use finance_sheet::*;
pub use stock_update::*;
pub use ui_chart::*;
pub use quantitative_streamgraph::*;

use aim_data::explorer::vci::market_watch::VCIMarketWatch;

const VN30_LIST: [&str; 30] = [
    "ACB", "BCM", "BID", "BVH", "CTG", "FPT", "GAS", "GVR", "HDB", "HPG", "LPB", "MBB", "MSN",
    "MWG", "PLX", "SAB", "SHB", "SSB", "SSI", "STB", "TCB", "TPB", "VCB", "VHM", "VIB", "VIC",
    "VJC", "VNM", "VPB", "VRE",
];

// Comprehensive list of major Vietnam stock symbols
// VN30 Index + additional major stocks from HSX, HNX, and UPCOM
pub static ALL_STOCK_LIST: [&str; 1604] = [
    "A32", "AAA", "AAH", "AAM", "AAS", "AAT", "AAV", "ABB", "ABC", "ABI", "ABR", "ABS", "ABT",
    "ABW", "ACB", "ACC", "ACE", "ACG", "ACL", "ACM", "ACS", "ACV", "ADC", "ADG", "ADP", "ADS",
    "AFX", "AG1", "AGF", "AGG", "AGM", "AGP", "AGR", "AGX", "AIC", "AIG", "ALT", "ALV", "AMC",
    "AMD", "AME", "AMP", "AMS", "AMV", "ANT", "ANV", "APC", "APF", "APG", "APH", "API", "APL",
    "APP", "APS", "APT", "ARM", "ART", "ASA", "ASG", "ASM", "ASP", "AST", "ATA", "ATB", "ATG",
    "ATS", "AVC", "AVF", "AVG", "BAB", "BAF", "BAL", "BAX", "BBC", "BBH", "BBM", "BBS", "BBT",
    "BCA", "BCB", "BCC", "BCE", "BCF", "BCG", "BCM", "BCP", "BCR", "BCV", "BDB", "BDG", "BDT",
    "BDW", "BED", "BEL", "BFC", "BGE", "BGW", "BHA", "BHC", "BHG", "BHH", "BHI", "BHK", "BHN",
    "BHP", "BIC", "BID", "BIG", "BII", "BIO", "BKC", "BKG", "BLF", "BLI", "BLN", "BLT", "BMC",
    "BMD", "BMF", "BMG", "BMI", "BMJ", "BMK", "BMP", "BMS", "BMV", "BNA", "BNW", "BOT", "BPC",
    "BQB", "BRC", "BRR", "BRS", "BSA", "BSC", "BSD", "BSG", "BSH", "BSI", "BSL", "BSP", "BSQ",
    "BSR", "BST", "BT1", "BT6", "BTB", "BTD", "BTG", "BTH", "BTN", "BTP", "BTS", "BTT", "BTU",
    "BTV", "BTW", "BVB", "BVG", "BVH", "BVL", "BVN", "BVS", "BWA", "BWE", "BWS", "BXH", "C12",
    "C21", "C22", "C32", "C47", "C4G", "C69", "C92", "CAD", "CAG", "CAN", "CAP", "CAR", "CAT",
    "CBI", "CBS", "CC1", "CCA", "CCC", "CCI", "CCL", "CCM", "CCP", "CCR", "CCT", "CCV", "CDC",
    "CDG", "CDH", "CDN", "CDO", "CDP", "CDR", "CEN", "CEO", "CET", "CFM", "CFV", "CGV", "CH5",
    "CHC", "CHP", "CHS", "CI5", "CIA", "CID", "CIG", "CII", "CIP", "CJC", "CK8", "CKA", "CKD",
    "CKG", "CKV", "CLC", "CLG", "CLH", "CLL", "CLM", "CLW", "CLX", "CMC", "CMD", "CMF", "CMG",
    "CMI", "CMK", "CMM", "CMN", "CMP", "CMS", "CMT", "CMV", "CMW", "CMX", "CNA", "CNC", "CNG",
    "CNN", "CNT", "COM", "CPA", "CPC", "CPH", "CPI", "CQN", "CQT", "CRC", "CRE", "CSC", "CSI",
    "CSM", "CST", "CSV", "CT3", "CT6", "CTA", "CTB", "CTD", "CTF", "CTG", "CTI", "CTN", "CTP",
    "CTR", "CTS", "CTT", "CTW", "CTX", "CVN", "CVT", "CX8", "CYC", "D11", "D2D", "DAC", "DAD",
    "DAE", "DAG", "DAH", "DAN", "DAS", "DAT", "DBC", "DBD", "DBM", "DBT", "DC1", "DC2", "DC4",
    "DCF", "DCG", "DCH", "DCL", "DCM", "DCR", "DCS", "DCT", "DDB", "DDG", "DDH", "DDM", "DDN",
    "DDV", "DFC", "DFF", "DGC", "DGT", "DGW", "DHA", "DHB", "DHC", "DHD", "DHG", "DHM", "DHN",
    "DHP", "DHT", "DIC", "DID", "DIG", "DIH", "DKC", "DKG", "DKW", "DL1", "DLD", "DLG", "DLR",
    "DLT", "DM7", "DMC", "DMN", "DMS", "DNA", "DNC", "DND", "DNE", "DNH", "DNL", "DNM", "DNN",
    "DNP", "DNT", "DNW", "DOC", "DOP", "DP1", "DP2", "DP3", "DPC", "DPG", "DPH", "DPM", "DPP",
    "DPR", "DPS", "DQC", "DRC", "DRG", "DRH", "DRI", "DRL", "DS3", "DSC", "DSD", "DSE", "DSG",
    "DSH", "DSN", "DSP", "DST", "DTA", "DTB", "DTC", "DTD", "DTE", "DTG", "DTH", "DTI", "DTK",
    "DTL", "DTP", "DTT", "DUS", "DVC", "DVG", "DVM", "DVN", "DVP", "DVT", "DVW", "DWC", "DWS",
    "DXG", "DXL", "DXP", "DXS", "DXV", "DZM", "E12", "E1VFVN30", "E29", "EBS", "ECI", "ECO", "EFI",
    "EIB", "EIC", "EID", "EIN", "ELC", "EME", "EMG", "EMS", "EPC", "EVE", "EVF", "EVG", "EVS",
    "FBA", "FBC", "FCC", "FCM", "FCN", "FCS", "FDC", "FGL", "FHN", "FHS", "FIC", "FID", "FIR",
    "FIT", "FLC", "FMC", "FOC", "FOX", "FPT", "FRC", "FRM", "FRT", "FSO", "FT1", "FTI", "FTM",
    "FTS", "FUCTVGF3", "FUCTVGF4", "FUCTVGF5", "FUCVREIT", "FUEABVND", "FUEBFVND", "FUEDCMID",
    "FUEFCV50", "FUEIP100", "FUEKIV30", "FUEKIVFS", "FUEKIVND", "FUEMAV30", "FUEMAVND", "FUESSV30",
    "FUESSV50", "FUESSVFL", "FUETCC50", "FUEVFVND", "FUEVN100", "G20", "G36", "GAB", "GAS", "GCB",
    "GCF", "GDA", "GDT", "GDW", "GEE", "GEG", "GER", "GEX", "GGG", "GH3", "GHC", "GIC", "GIL",
    "GKM", "GLC", "GLT", "GLW", "GMA", "GMC", "GMD", "GMH", "GMX", "GND", "GPC", "GSM", "GSP",
    "GTA", "GTD", "GTS", "GTT", "GVR", "GVT", "H11", "HAC", "HAD", "HAF", "HAG", "HAH", "HAI",
    "HAM", "HAN", "HAP", "HAR", "HAS", "HAT", "HAV", "HAX", "HBC", "HBD", "HBH", "HBS", "HC1",
    "HC3", "HCC", "HCD", "HCI", "HCM", "HCT", "HD2", "HD6", "HD8", "HDA", "HDB", "HDC", "HDG",
    "HDM", "HDO", "HDP", "HDW", "HEC", "HEJ", "HEP", "HES", "HEV", "HFB", "HFC", "HFX", "HGM",
    "HGT", "HHC", "HHG", "HHN", "HHP", "HHS", "HHV", "HID", "HIG", "HII", "HIO", "HJC", "HJS",
    "HKB", "HKT", "HLA", "HLB", "HLC", "HLD", "HLO", "HLS", "HLT", "HLY", "HMC", "HMD", "HMG",
    "HMH", "HMR", "HMS", "HNA", "HNB", "HND", "HNF", "HNG", "HNI", "HNM", "HNP", "HNR", "HOM",
    "HOT", "HPB", "HPD", "HPG", "HPH", "HPI", "HPM", "HPP", "HPT", "HPW", "HPX", "HQC", "HRB",
    "HRC", "HSA", "HSG", "HSI", "HSL", "HSM", "HSP", "HSV", "HT1", "HTC", "HTE", "HTG", "HTI",
    "HTL", "HTM", "HTN", "HTP", "HTT", "HTV", "HU1", "HU3", "HU4", "HU6", "HUB", "HUG", "HUT",
    "HVA", "HVG", "HVH", "HVN", "HVT", "HVX", "HWS", "IBC", "IBD", "ICC", "ICF", "ICG", "ICI",
    "ICN", "ICT", "IDC", "IDI", "IDJ", "IDP", "IDV", "IFS", "IHK", "IJC", "ILA", "ILB", "ILC",
    "ILS", "IME", "IMP", "IN4", "INC", "ING", "INN", "IPA", "IRC", "ISG", "ISH", "IST", "ITA",
    "ITC", "ITD", "ITQ", "ITS", "IVS", "JOS", "JVC", "KAC", "KBC", "KCB", "KDC", "KDH", "KDM",
    "KGM", "KHD", "KHG", "KHL", "KHP", "KHS", "KHW", "KIP", "KKC", "KLB", "KLF", "KMR", "KMT",
    "KOS", "KPF", "KSB", "KSD", "KSF", "KSH", "KSQ", "KST", "KSV", "KTC", "KTL", "KTS", "KTT",
    "KVC", "KWA", "L10", "L12", "L14", "L18", "L35", "L40", "L43", "L44", "L45", "L61", "L62",
    "L63", "LAF", "LAI", "LAS", "LAW", "LBE", "LBM", "LCC", "LCD", "LCG", "LCM", "LCS", "LDG",
    "LDP", "LDW", "LEC", "LG9", "LGC", "LGL", "LGM", "LHC", "LHG", "LIC", "LIG", "LIX", "LKW",
    "LLM", "LM3", "LM7", "LM8", "LMC", "LMH", "LMI", "LNC", "LO5", "LPB", "LPT", "LQN", "LSG",
    "LSS", "LTC", "LTG", "LUT", "M10", "MA1", "MAC", "MAS", "MBB", "MBG", "MBN", "MBS", "MBT",
    "MCC", "MCF", "MCG", "MCH", "MCM", "MCO", "MCP", "MDA", "MDC", "MDF", "MDG", "MEC", "MED",
    "MEF", "MEL", "MES", "MFS", "MGC", "MGG", "MGR", "MH3", "MHC", "MHL", "MIC", "MIE", "MIG",
    "MIM", "MKP", "MKV", "MLC", "MLS", "MML", "MNB", "MND", "MPC", "MPT", "MPY", "MQB", "MQN",
    "MRF", "MSB", "MSH", "MSN", "MSR", "MST", "MTA", "MTB", "MTG", "MTH", "MTL", "MTP", "MTS",
    "MTV", "MTX", "MVB", "MVC", "MVN", "MWG", "MZG", "NAB", "NAC", "NAF", "NAG", "NAP", "NAS",
    "NAU", "NAV", "NAW", "NBB", "NBC", "NBE", "NBP", "NBT", "NBW", "NCG", "NCS", "NCT", "ND2",
    "NDC", "NDF", "NDN", "NDP", "NDT", "NDW", "NDX", "NED", "NEM", "NET", "NFC", "NGC", "NHA",
    "NHC", "NHH", "NHP", "NHT", "NHV", "NJC", "NKG", "NLG", "NLS", "NNC", "NNT", "NO1", "NOS",
    "NQB", "NQN", "NRC", "NS2", "NSC", "NSG", "NSH", "NSL", "NSS", "NST", "NT2", "NTB", "NTC",
    "NTF", "NTH", "NTL", "NTP", "NTT", "NTW", "NUE", "NVB", "NVL", "NVP", "NVT", "NWT", "NXT",
    "OCB", "OCH", "ODE", "OGC", "OIL", "ONE", "ONW", "OPC", "ORS", "PAC", "PAI", "PAN", "PAP",
    "PAS", "PAT", "PBC", "PBP", "PBT", "PC1", "PCC", "PCE", "PCF", "PCG", "PCH", "PCM", "PCT",
    "PDB", "PDC", "PDN", "PDR", "PDV", "PEC", "PEG", "PEN", "PEQ", "PET", "PFL", "PGB", "PGC",
    "PGD", "PGI", "PGN", "PGS", "PGT", "PGV", "PHC", "PHH", "PHN", "PHP", "PHR", "PHS", "PIA",
    "PIC", "PID", "PIS", "PIT", "PIV", "PJC", "PJS", "PJT", "PLA", "PLC", "PLE", "PLO", "PLP",
    "PLX", "PMB", "PMC", "PMG", "PMJ", "PMP", "PMS", "PMT", "PMW", "PNC", "PND", "PNG", "PNJ",
    "PNP", "PNT", "POB", "POM", "POS", "POT", "POV", "POW", "PPC", "PPE", "PPH", "PPI", "PPP",
    "PPS", "PPT", "PPY", "PQN", "PRC", "PRE", "PRO", "PRT", "PSB", "PSC", "PSD", "PSE", "PSG",
    "PSH", "PSI", "PSL", "PSN", "PSP", "PSW", "PTB", "PTC", "PTD", "PTE", "PTG", "PTH", "PTI",
    "PTL", "PTO", "PTP", "PTS", "PTT", "PTV", "PTX", "PV2", "PVA", "PVB", "PVC", "PVD", "PVE",
    "PVG", "PVH", "PVI", "PVL", "PVM", "PVO", "PVP", "PVR", "PVS", "PVT", "PVV", "PVX", "PVY",
    "PWA", "PWS", "PX1", "PXA", "PXC", "PXI", "PXL", "PXM", "PXS", "PXT", "QBS", "QCC", "QCG",
    "QHD", "QHW", "QNC", "QNP", "QNS", "QNT", "QNU", "QNW", "QPH", "QSP", "QST", "QTC", "QTP",
    "RAL", "RAT", "RBC", "RCC", "RCD", "RCL", "RDP", "REE", "RIC", "RTB", "RYG", "S12", "S27",
    "S4A", "S55", "S72", "S74", "S96", "S99", "SAB", "SAC", "SAF", "SAL", "SAM", "SAP", "SAS",
    "SAV", "SB1", "SBA", "SBB", "SBD", "SBG", "SBH", "SBL", "SBM", "SBR", "SBS", "SBT", "SBV",
    "SC5", "SCC", "SCD", "SCG", "SCI", "SCJ", "SCL", "SCO", "SCR", "SCS", "SD1", "SD2", "SD3",
    "SD4", "SD5", "SD6", "SD7", "SD8", "SD9", "SDA", "SDB", "SDC", "SDD", "SDG", "SDK", "SDN",
    "SDP", "SDT", "SDU", "SDV", "SDX", "SDY", "SEA", "SEB", "SED", "SEP", "SFC", "SFG", "SFI",
    "SFN", "SGB", "SGC", "SGD", "SGH", "SGI", "SGN", "SGP", "SGR", "SGS", "SGT", "SHA", "SHB",
    "SHC", "SHE", "SHG", "SHI", "SHN", "SHP", "SHS", "SID", "SIG", "SII", "SIP", "SIV", "SJ1",
    "SJC", "SJD", "SJE", "SJF", "SJG", "SJM", "SJS", "SKG", "SKH", "SKN", "SKV", "SLS", "SMA",
    "SMB", "SMC", "SMN", "SMT", "SNC", "SNZ", "SP2", "SPB", "SPC", "SPD", "SPH", "SPI", "SPM",
    "SPV", "SQC", "SRA", "SRB", "SRC", "SRF", "SSB", "SSC", "SSF", "SSG", "SSH", "SSI", "SSM",
    "SSN", "ST8", "STB", "STC", "STG", "STH", "STK", "STL", "STP", "STS", "STT", "STW", "SVC",
    "SVD", "SVG", "SVH", "SVI", "SVN", "SVT", "SWC", "SZB", "SZC", "SZE", "SZG", "SZL", "TA6",
    "TA9", "TAB", "TAN", "TAR", "TAW", "TB8", "TBC", "TBD", "TBH", "TBR", "TBW", "TBX", "TCB",
    "TCD", "TCH", "TCI", "TCJ", "TCK", "TCL", "TCM", "TCO", "TCR", "TCT", "TCW", "TD6", "TDB",
    "TDC", "TDF", "TDG", "TDH", "TDM", "TDP", "TDS", "TDT", "TDW", "TED", "TEG", "TEL", "TET",
    "TFC", "TGG", "TGP", "TH1", "THB", "THD", "THG", "THM", "THN", "THP", "THS", "THT", "THU",
    "THW", "TID", "TIE", "TIG", "TIN", "TIP", "TIS", "TIX", "TJC", "TKA", "TKC", "TKG", "TKU",
    "TL4", "TLD", "TLG", "TLH", "TLI", "TLP", "TLT", "TMB", "TMC", "TMG", "TMP", "TMS", "TMT",
    "TMW", "TMX", "TN1", "TNA", "TNB", "TNC", "TNG", "TNH", "TNI", "TNM", "TNP", "TNS", "TNT",
    "TNV", "TNW", "TOP", "TOS", "TOT", "TOW", "TPB", "TPC", "TPP", "TPS", "TQN", "TQW", "TR1",
    "TRA", "TRC", "TRS", "TRT", "TRV", "TS3", "TS4", "TSA", "TSB", "TSC", "TSD", "TSG", "TSJ",
    "TST", "TT6", "TTA", "TTB", "TTC", "TTD", "TTE", "TTF", "TTG", "TTH", "TTL", "TTN", "TTS",
    "TTT", "TTZ", "TUG", "TV1", "TV2", "TV3", "TV4", "TV6", "TVA", "TVB", "TVC", "TVD", "TVG",
    "TVH", "TVM", "TVN", "TVS", "TVT", "TW3", "TXM", "TYA", "UCT", "UDC", "UDJ", "UDL", "UEM",
    "UIC", "UMC", "UNI", "UPC", "UPH", "USC", "USD", "UXC", "V12", "V15", "V21", "VAB", "VAF",
    "VAV", "VBB", "VBC", "VBG", "VBH", "VC1", "VC2", "VC3", "VC5", "VC6", "VC7", "VC9", "VCA",
    "VCB", "VCC", "VCE", "VCF", "VCG", "VCI", "VCM", "VCP", "VCR", "VCS", "VCT", "VCW", "VCX",
    "VDB", "VDG", "VDL", "VDN", "VDP", "VDS", "VDT", "VE1", "VE2", "VE3", "VE4", "VE8", "VE9",
    "VEA", "VEC", "VEF", "VES", "VET", "VFC", "VFG", "VFR", "VFS", "VGC", "VGG", "VGI", "VGL",
    "VGP", "VGR", "VGS", "VGT", "VGV", "VHC", "VHD", "VHE", "VHF", "VHG", "VHH", "VHL", "VHM",
    "VIB", "VIC", "VID", "VIE", "VIF", "VIG", "VIH", "VIM", "VIN", "VIP", "VIR", "VIT", "VIW",
    "VIX", "VJC", "VKC", "VKP", "VLA", "VLB", "VLC", "VLF", "VLG", "VLP", "VLW", "VMA", "VMC",
    "VMD", "VMG", "VMK", "VMS", "VMT", "VNA", "VNB", "VNC", "VND", "VNE", "VNF", "VNG", "VNH",
    "VNI", "VNL", "VNM", "VNP", "VNR", "VNS", "VNT", "VNX", "VNY", "VNZ", "VOC", "VOS", "VPA",
    "VPB", "VPC", "VPD", "VPG", "VPH", "VPI", "VPL", "VPR", "VPS", "VPW", "VQC", "VRC", "VRE",
    "VRG", "VSA", "VSC", "VSE", "VSF", "VSG", "VSH", "VSI", "VSM", "VSN", "VST", "VTA", "VTB",
    "VTC", "VTD", "VTE", "VTG", "VTH", "VTI", "VTJ", "VTK", "VTL", "VTM", "VTO", "VTP", "VTQ",
    "VTR", "VTS", "VTV", "VTX", "VTZ", "VUA", "VVN", "VVS", "VW3", "VWS", "VXB", "VXP", "VXT",
    "WCS", "WSB", "WSS", "WTC", "X20", "X26", "X77", "XDH", "XHC", "XLV", "XMC", "XMD", "XMP",
    "XPH", "YBC", "YBM", "YEG", "YTC",
];

fn convert_to_market_data(market_watch: &VCIMarketWatch) -> SlintMarketWatchData {
    // Divide all price values by 1000 to get the actual price
    let price = market_watch.match_price.match_price / 1000.0;
    let ref_price = market_watch.listing_info.ref_price / 1000.0;
    let change = price - ref_price;
    let change_percent = if ref_price != 0.0 {
        (change / ref_price) * 100.0
    } else {
        0.0
    };

    // Get bid/ask prices
    let (ask3_price, ask2_price, ask1_price) = if market_watch.bid_ask.ask_prices.len() >= 3 {
        (
            market_watch.bid_ask.ask_prices[2].price / 1000.0,
            market_watch.bid_ask.ask_prices[1].price / 1000.0,
            market_watch.bid_ask.ask_prices[0].price / 1000.0,
        )
    } else {
        (0.0, 0.0, 0.0)
    };

    let (bid1_price, bid2_price, bid3_price) = if market_watch.bid_ask.bid_prices.len() >= 3 {
        (
            market_watch.bid_ask.bid_prices[0].price / 1000.0,
            market_watch.bid_ask.bid_prices[1].price / 1000.0,
            market_watch.bid_ask.bid_prices[2].price / 1000.0,
        )
    } else {
        (0.0, 0.0, 0.0)
    };

    let volume = market_watch.match_price.match_vol as f64;
    let total_volume = market_watch.match_price.accumulated_volume as f64;
    let ceil_price = market_watch.listing_info.ceiling / 1000.0;
    let floor_price = market_watch.listing_info.floor / 1000.0;
    let high = market_watch.match_price.highest / 1000.0;
    let low = market_watch.match_price.lowest / 1000.0;

    SlintMarketWatchData {
        symbol: market_watch.listing_info.symbol.clone().into(),
        info: market_watch.listing_info.organ_name.clone().into(),
        match_price: price as f32,
        match_volume: volume as f32,
        change: change as f32,
        change_percent: change_percent as f32,
        volume: total_volume as f32,
        high: high as f32,
        low: low as f32,
        ref_price: ref_price as f32,
        ceil_price: ceil_price as f32,
        floor_price: floor_price as f32,
        ask_price1: ask1_price as f32,
        ask_price2: ask2_price as f32,
        ask_price3: ask3_price as f32,
        ask_volume1: if !market_watch.bid_ask.ask_prices.is_empty() {
            market_watch.bid_ask.ask_prices[0].volume as f32
        } else {
            0.0
        },
        ask_volume2: if market_watch.bid_ask.ask_prices.len() > 1 {
            market_watch.bid_ask.ask_prices[1].volume as f32
        } else {
            0.0
        },
        ask_volume3: if market_watch.bid_ask.ask_prices.len() > 2 {
            market_watch.bid_ask.ask_prices[2].volume as f32
        } else {
            0.0
        },
        bid_price1: bid1_price as f32,
        bid_price2: bid2_price as f32,
        bid_price3: bid3_price as f32,
        bid_volume1: if !market_watch.bid_ask.bid_prices.is_empty() {
            market_watch.bid_ask.bid_prices[0].volume as f32
        } else {
            0.0
        },
        bid_volume2: if market_watch.bid_ask.bid_prices.len() > 1 {
            market_watch.bid_ask.bid_prices[1].volume as f32
        } else {
            0.0
        },
        bid_volume3: if market_watch.bid_ask.bid_prices.len() > 2 {
            market_watch.bid_ask.bid_prices[2].volume as f32
        } else {
            0.0
        },
    }
}

pub fn convert_to_stock_data(market_watch: &VCIMarketWatch) -> SlintStockData {
    // Divide all price values by 1000 to get the actual price
    let price = market_watch.match_price.match_price / 1000.0;
    let ref_price = market_watch.listing_info.ref_price / 1000.0;
    let change = price - ref_price;
    let change_percent = if ref_price != 0.0 {
        (change / ref_price) * 100.0
    } else {
        0.0
    };

    // Get bid/ask prices
    let (ask3_price, ask2_price, ask1_price) = if market_watch.bid_ask.ask_prices.len() >= 3 {
        (
            market_watch.bid_ask.ask_prices[2].price / 1000.0,
            market_watch.bid_ask.ask_prices[1].price / 1000.0,
            market_watch.bid_ask.ask_prices[0].price / 1000.0,
        )
    } else {
        (0.0, 0.0, 0.0)
    };

    let (bid1_price, bid2_price, bid3_price) = if market_watch.bid_ask.bid_prices.len() >= 3 {
        (
            market_watch.bid_ask.bid_prices[0].price / 1000.0,
            market_watch.bid_ask.bid_prices[1].price / 1000.0,
            market_watch.bid_ask.bid_prices[2].price / 1000.0,
        )
    } else {
        (0.0, 0.0, 0.0)
    };

    let volume = market_watch.match_price.accumulated_volume as f64;
    let ceil_price = market_watch.listing_info.ceiling / 1000.0;
    let floor_price = market_watch.listing_info.floor / 1000.0;
    let high = market_watch.match_price.highest / 1000.0;
    let low = market_watch.match_price.lowest / 1000.0;

    SlintStockData {
        symbol: market_watch.listing_info.symbol.clone().into(),
        info: market_watch.listing_info.organ_name.clone().into(),
        price: price as f32,
        change: change as f32,
        change_percent: change_percent as f32,
        volume: volume as f32,
        high: high as f32,
        low: low as f32,
        open: 0.0,  // Not available in market watch data
        close: 0.0, // Not available in market watch data
        ref_price: ref_price as f32,
        ceil_price: ceil_price as f32,
        floor_price: floor_price as f32,
        ask_price1: ask1_price as f32,
        ask_price2: ask2_price as f32,
        ask_price3: ask3_price as f32,
        ask_volume1: if !market_watch.bid_ask.ask_prices.is_empty() {
            market_watch.bid_ask.ask_prices[0].volume as f32
        } else {
            0.0
        },
        ask_volume2: if market_watch.bid_ask.ask_prices.len() > 1 {
            market_watch.bid_ask.ask_prices[1].volume as f32
        } else {
            0.0
        },
        ask_volume3: if market_watch.bid_ask.ask_prices.len() > 2 {
            market_watch.bid_ask.ask_prices[2].volume as f32
        } else {
            0.0
        },
        bid_price1: bid1_price as f32,
        bid_price2: bid2_price as f32,
        bid_price3: bid3_price as f32,
        bid_volume1: if !market_watch.bid_ask.bid_prices.is_empty() {
            market_watch.bid_ask.bid_prices[0].volume as f32
        } else {
            0.0
        },
        bid_volume2: if market_watch.bid_ask.bid_prices.len() > 1 {
            market_watch.bid_ask.bid_prices[1].volume as f32
        } else {
            0.0
        },
        bid_volume3: if market_watch.bid_ask.bid_prices.len() > 2 {
            market_watch.bid_ask.bid_prices[2].volume as f32
        } else {
            0.0
        },
        is_changed: 0,
    }
}

pub fn is_trading_hours() -> bool {
    let now = chrono::Utc::now();
    // Convert to Vietnam time (UTC+7)
    let vietnam_time = now + chrono::Duration::hours(7);
    let current_hour = vietnam_time.hour();
    let current_minute = vietnam_time.minute();

    // Trading hours: 9:00 AM to 3:00 PM (15:00)
    let start_time = 9 * 60; // 9:00 AM in minutes
    let end_time = 15 * 60; // 3:00 PM in minutes
    let current_time_minutes = current_hour * 60 + current_minute;

    // Also check if it's a weekday (Monday = 1, Sunday = 7)
    let is_weekday = vietnam_time.weekday().num_days_from_monday() < 5;
    log::info!(
        "Current time: {}:{} - Trading hours: {} to {} - Is weekday: {}",
        current_hour,
        current_minute,
        start_time / 60,
        end_time / 60,
        is_weekday
    );

    is_weekday && current_time_minutes >= start_time && current_time_minutes < end_time
}

/// Create sector-specific stock groups based on watchlist category
fn create_sector_groups(
    sort_type: ShortType,
    stock_data: Vec<SlintStockData>,
    custom_list: Vec<String>,
) -> Vec<SlintStockGroup> {
    let _ = sort_type;
    let sector_stocks = {
        // Default: create all sector groups that have stocks
        let mut all_groups = Vec::new();

        // Manually create each sector group to avoid recursion issues
        let sector_definitions = [
            (
                "MY LIST",
                custom_list
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<&str>>(),
            ),
            (
                "NGÂN HÀNG",
                vec![
                    "VPB", "SHB", "ABB", "CTG", "VAB", "VIB", "TPB", "OCB", "NVB", "STB", "MBB",
                    "BID", "TCB", "MSB", "ACB", "HDB", "VCB", "LPB", "KLB", "EIB", "BVB", "NAB",
                    "BAB", "SSB", "SCB", "PGB", "SGB", "BVF", "GPB",
                ],
            ),
            (
                "VN30",
                vec![
                    "VPB", "CTG", "TPB", "STB", "MBB", "BID", "NVL", "SSI", "TCB", "PNJ", "BCM",
                    "MSN", "ACB", "MWG", "HDB", "SAB", "PDR", "VRE", "VJC", "HPG", "FPT", "GVR",
                    "BVH", "GAS", "VCB", "POW", "VNM", "PLX", "VHM", "VIC",
                ],
            ),
            (
                "DẦU KHÍ",
                vec![
                    "PET", "PVS", "CNG", "PVC", "VIP", "ASP", "PVI", "OIL", "PSH", "PVB", "PVG",
                    "PXS", "VTO", "GAS", "PVD", "PVT", "PLX", "PVP", "BSR", "POW", "PVE", "PVR",
                    "PVX", "PLO",
                ],
            ),
            (
                "BĐS",
                vec![
                    "VHM", "IDC", "SZC", "FCN", "DXG", "L14", "NTL", "TCH", "SCR", "LDG", "AGG",
                    "CEO", "NLG", "NDN", "CII", "TDC", "KBC", "CRE", "NVL", "PAN", "DIG", "HDC",
                    "D2D", "HQC", "PDR", "KDH", "CKG", "GVR", "IJC", "VIC", "VRE", "HDG", "IDI",
                    "ITA", "NHA", "TDH",
                ],
            ),
            (
                "CHỨNG KHOÁN",
                vec![
                    "SSI", "VCI", "VND", "HCM", "SHS", "FTS", "BSI", "VDS", "SBS", "APS", "MBS",
                    "VIX", "BVS", "AGR", "CTS", "ORS", "TVB", "PSI", "IVS",
                ],
            ),
            (
                "NĂNG LƯỢNG",
                vec![
                    "POW", "VSH", "NT2", "REE", "PC1", "HDG", "TV2", "BCG", "GEG", "KHP", "QTP",
                    "ASM", "EVF", "SBA", "GEX", "SEB", "TBC",
                ],
            ),
            (
                "DỆT MAY",
                vec![
                    "TCM", "NDT", "GIL", "TNG", "MSH", "VGG", "BDG", "VGT", "STK", "GTN", "TET",
                    "TTF", "STT", "EVE",
                ],
            ),
            (
                "THÉP",
                vec![
                    "HSG", "VGS", "NKG", "TLH", "SMC", "HPG", "POM", "TNB", "TNS", "TVN", "TIS",
                ],
            ),
            (
                "THỦY SẢN",
                vec![
                    "VHC", "CMX", "FMC", "AAM", "IDI", "MPC", "SSN", "ACL", "ANV", "ICF",
                ],
            ),
            (
                "CẢNG BIỂN",
                vec![
                    "VSC", "TOS", "HAH", "GMD", "VIP", "CDN", "TMS", "TCW", "TCL", "CLL", "DVP",
                    "VTO", "PVT", "PVP", "VOS", "DL1", "SGP", "PHP",
                ],
            ),
            ("PHÂN ĐẠM", vec!["DPM", "DCM", "LAS", "DDV", "BFC"]),
            (
                "BẢO HIỂM",
                vec!["BIC", "MIG", "PVI", "PRE", "BMI", "PGI", "BVH"],
            ),
            ("MÍA ĐƯỜNG", vec!["SBT", "CBS", "KTS", "SLS", "QNS", "LSS"]),
            (
                "DƯỢC",
                vec![
                    "DCL", "DHT", "FIT", "DBD", "DP3", "DMC", "AMV", "LDP", "OPC", "DHG", "TNH",
                    "DVN", "IMP",
                ],
            ),
            (
                "BĐS KCN",
                vec![
                    "VGC", "KBC", "BCM", "TIP", "D2D", "PHR", "LHG", "CCL", "GVR", "ITA", "IDC",
                    "SZC", "SIP", "NTL", "NTC",
                ],
            ),
            ("NHỰA", vec!["AAA", "PLP", "DPR", "RDP", "BMP"]),
            ("VLXD", vec!["HOM", "HT1", "BTS", "VCS", "BCC"]),
            (
                "LƯƠNG THỰC",
                vec!["NAF", "HAG", "DBC", "PAN", "BAF", "AFX", "LTG", "TAR"],
            ),
            ("CAO SU", vec!["SRC", "DPR", "TRC", "GVR", "DRC"]),
        ];

        for (sector_name, symbols) in sector_definitions {
            let sector_stocks: Vec<SlintStockData> = stock_data
                .iter()
                .filter(|&s| symbols.contains(&s.symbol.to_string().as_str()))
                .cloned()
                .collect();

            all_groups.push(SlintStockGroup {
                group_name: sector_name.into(),
                stocks: ModelRc::new(slint::VecModel::from(sector_stocks)),
                is_expanded: true,
            });
        }

        sort_stocks(&all_groups, sort_type)
    };
    sector_stocks
}

pub fn sort_stocks(
    stock_groups: &[SlintStockGroup],
    sort_type: crate::slint_generatedAppWindow::ShortType,
) -> Vec<SlintStockGroup> {
    let mut updated_groups = Vec::new();

    for group in stock_groups.iter() {
        // Convert ModelRc to Vec for sorting
        let stocks: Vec<_> = group.stocks.iter().collect();
        let mut sorted_stocks = stocks;

        // Sort the stocks based on the sort type
        match sort_type {
            crate::slint_generatedAppWindow::ShortType::Alphabet => {
                sorted_stocks.sort_by(|a, b| a.symbol.to_string().cmp(&b.symbol.to_string()));
            }
            crate::slint_generatedAppWindow::ShortType::HighPrice => {
                sorted_stocks.sort_by(|a, b| {
                    b.change_percent
                        .partial_cmp(&a.change_percent)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            crate::slint_generatedAppWindow::ShortType::LowPrice => {
                sorted_stocks.sort_by(|a, b| {
                    a.change_percent
                        .partial_cmp(&b.change_percent)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            crate::slint_generatedAppWindow::ShortType::None => {
                // No sorting needed
            }
        }

        // Create new group with sorted stocks
        let new_group = crate::slint_generatedAppWindow::StockGroup {
            group_name: group.group_name.clone(),
            stocks: slint::ModelRc::new(VecModel::from(sorted_stocks)),
            is_expanded: group.is_expanded,
        };
        updated_groups.push(new_group);
    }

    updated_groups
}

pub fn sort_market_watch(
    market_data: &[SlintMarketWatchData],
    sort_column: i32,
    sort_ascending: bool,
    show_percentage: bool,
) -> Vec<SlintMarketWatchData> {
    let mut sorted_data = market_data.to_vec();

    match sort_column {
        0 => {
            // Sort by symbol (alphabetical)
            if sort_ascending {
                sorted_data.sort_by(|a, b| a.symbol.to_string().cmp(&b.symbol.to_string()));
            } else {
                sorted_data.sort_by(|a, b| b.symbol.to_string().cmp(&a.symbol.to_string()));
            }
        }
        1 => {
            // Sort by ref price
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.ref_price
                        .partial_cmp(&b.ref_price)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.ref_price
                        .partial_cmp(&a.ref_price)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        2 => {
            // Sort by ceil price
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.ceil_price
                        .partial_cmp(&b.ceil_price)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.ceil_price
                        .partial_cmp(&a.ceil_price)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        3 => {
            // Sort by floor price
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.floor_price
                        .partial_cmp(&b.floor_price)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.floor_price
                        .partial_cmp(&a.floor_price)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        4 => {
            // Sort by bid price 3
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.bid_price3
                        .partial_cmp(&b.bid_price3)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.bid_price3
                        .partial_cmp(&a.bid_price3)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        5 => {
            // Sort by bid volume 3
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.bid_volume3
                        .partial_cmp(&b.bid_volume3)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.bid_volume3
                        .partial_cmp(&a.bid_volume3)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        6 => {
            // Sort by bid price 2
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.bid_price2
                        .partial_cmp(&b.bid_price2)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.bid_price2
                        .partial_cmp(&a.bid_price2)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        7 => {
            // Sort by bid volume 2
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.bid_volume2
                        .partial_cmp(&b.bid_volume2)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.bid_volume2
                        .partial_cmp(&a.bid_volume2)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        8 => {
            // Sort by bid price 1
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.bid_price1
                        .partial_cmp(&b.bid_price1)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.bid_price1
                        .partial_cmp(&a.bid_price1)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        9 => {
            // Sort by bid volume 1
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.bid_volume1
                        .partial_cmp(&b.bid_volume1)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.bid_volume1
                        .partial_cmp(&a.bid_volume1)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        10 => {
            // Sort by match price
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.match_price
                        .partial_cmp(&b.match_price)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.match_price
                        .partial_cmp(&a.match_price)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        11 => {
            // Sort by volume
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.match_volume
                        .partial_cmp(&b.match_volume)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.match_volume
                        .partial_cmp(&a.match_volume)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        12 => {
            // Sort by change (percentage or absolute depending on show_percentage mode)
            if show_percentage {
                // Sort by change percentage
                if sort_ascending {
                    sorted_data.sort_by(|a, b| {
                        a.change_percent
                            .partial_cmp(&b.change_percent)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });
                } else {
                    sorted_data.sort_by(|a, b| {
                        b.change_percent
                            .partial_cmp(&a.change_percent)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });
                }
            } else {
                // Sort by absolute change
                if sort_ascending {
                    sorted_data.sort_by(|a, b| {
                        a.change
                            .partial_cmp(&b.change)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });
                } else {
                    sorted_data.sort_by(|a, b| {
                        b.change
                            .partial_cmp(&a.change)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });
                }
            }
        }
        13 => {
            // Sort by ask price 1
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.ask_price1
                        .partial_cmp(&b.ask_price1)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.ask_price1
                        .partial_cmp(&a.ask_price1)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        14 => {
            // Sort by ask volume 1
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.ask_volume1
                        .partial_cmp(&b.ask_volume1)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.ask_volume1
                        .partial_cmp(&a.ask_volume1)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        15 => {
            // Sort by ask price 2
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.ask_price2
                        .partial_cmp(&b.ask_price2)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.ask_price2
                        .partial_cmp(&a.ask_price2)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        16 => {
            // Sort by ask volume 2
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.ask_volume2
                        .partial_cmp(&b.ask_volume2)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.ask_volume2
                        .partial_cmp(&a.ask_volume2)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        17 => {
            // Sort by ask price 3
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.ask_price3
                        .partial_cmp(&b.ask_price3)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.ask_price3
                        .partial_cmp(&a.ask_price3)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        18 => {
            // Sort by ask volume 3
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.ask_volume3
                        .partial_cmp(&b.ask_volume3)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.ask_volume3
                        .partial_cmp(&a.ask_volume3)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        19 => {
            // Sort by total volume
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.volume
                        .partial_cmp(&b.volume)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.volume
                        .partial_cmp(&a.volume)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        20 => {
            // Sort by high
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.high
                        .partial_cmp(&b.high)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.high
                        .partial_cmp(&a.high)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        21 => {
            // Sort by low
            if sort_ascending {
                sorted_data.sort_by(|a, b| {
                    a.low
                        .partial_cmp(&b.low)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            } else {
                sorted_data.sort_by(|a, b| {
                    b.low
                        .partial_cmp(&a.low)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        _ => {
            // No sorting for unknown columns
        }
    }

    sorted_data
}
