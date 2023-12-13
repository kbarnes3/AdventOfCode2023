pub enum Direction {
    Left,
    Right,
}

pub struct NodeLink<'a> {
    pub node: &'a str,
    pub left: &'a str,
    pub right: &'a str,
}

pub struct Map<'a, const D: usize, const N: usize> {
    pub directions: [Direction; D],
    pub nodes: [NodeLink<'a>; N],
}

pub const START_NODE: &str = "AAA";
pub const END_NODE: &str = "ZZZ";

pub const START_SUFFIX: &str = "A";
pub const END_SUFFIX: &str = "Z";

// Transform with:
// '<,'>s/L/Direction::Left, /g | '<,'>s/R/Direction::Right, /g
// '<,'>s/\(\w\+\) = (\(\w\+\), \(\w\+\))/{ NodeLink { node: "\1", left: "\2", right: "\3" } },

pub const SAMPLE_DATA: Map<2, 7> = Map {
    directions: [Direction::Right, Direction::Left],

    nodes: [
        {
            NodeLink {
                node: "AAA",
                left: "BBB",
                right: "CCC",
            }
        },
        {
            NodeLink {
                node: "BBB",
                left: "DDD",
                right: "EEE",
            }
        },
        {
            NodeLink {
                node: "CCC",
                left: "ZZZ",
                right: "GGG",
            }
        },
        {
            NodeLink {
                node: "DDD",
                left: "DDD",
                right: "DDD",
            }
        },
        {
            NodeLink {
                node: "EEE",
                left: "EEE",
                right: "EEE",
            }
        },
        {
            NodeLink {
                node: "GGG",
                left: "GGG",
                right: "GGG",
            }
        },
        {
            NodeLink {
                node: "ZZZ",
                left: "ZZZ",
                right: "ZZZ",
            }
        },
    ],
};

pub const SAMPLE_DATA_2: Map<3, 3> = Map {
    directions: [Direction::Left, Direction::Left, Direction::Right],

    nodes: [
        {
            NodeLink {
                node: "AAA",
                left: "BBB",
                right: "BBB",
            }
        },
        {
            NodeLink {
                node: "BBB",
                left: "AAA",
                right: "ZZZ",
            }
        },
        {
            NodeLink {
                node: "ZZZ",
                left: "ZZZ",
                right: "ZZZ",
            }
        },
    ],
};

pub const SAMPLE_DATA_3: Map<2, 8> = Map {
    directions: [Direction::Left, Direction::Right],

    nodes: [
        {
            NodeLink {
                node: "11A",
                left: "11B",
                right: "XXX",
            }
        },
        {
            NodeLink {
                node: "11B",
                left: "XXX",
                right: "11Z",
            }
        },
        {
            NodeLink {
                node: "11Z",
                left: "11B",
                right: "XXX",
            }
        },
        {
            NodeLink {
                node: "22A",
                left: "22B",
                right: "XXX",
            }
        },
        {
            NodeLink {
                node: "22B",
                left: "22C",
                right: "22C",
            }
        },
        {
            NodeLink {
                node: "22C",
                left: "22Z",
                right: "22Z",
            }
        },
        {
            NodeLink {
                node: "22Z",
                left: "22B",
                right: "22B",
            }
        },
        {
            NodeLink {
                node: "XXX",
                left: "XXX",
                right: "XXX",
            }
        },
    ],
};

pub const REAL_DATA: Map<269, 758> = Map {
    directions: [
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Left,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Left,
        Direction::Left,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Left,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Left,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Left,
        Direction::Right,
        Direction::Left,
        Direction::Left,
        Direction::Right,
        Direction::Left,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Left,
        Direction::Left,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Left,
        Direction::Left,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Left,
        Direction::Left,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Left,
        Direction::Left,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Left,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Left,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Left,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Right,
        Direction::Left,
        Direction::Left,
        Direction::Left,
        Direction::Right,
        Direction::Left,
        Direction::Right,
        Direction::Right,
        Direction::Right,
        Direction::Right,
    ],

    nodes: [
        {
            NodeLink {
                node: "PND",
                left: "LHJ",
                right: "NLX",
            }
        },
        {
            NodeLink {
                node: "KPD",
                left: "HMV",
                right: "DLD",
            }
        },
        {
            NodeLink {
                node: "GBD",
                left: "QFL",
                right: "XNG",
            }
        },
        {
            NodeLink {
                node: "QGL",
                left: "BPR",
                right: "XTJ",
            }
        },
        {
            NodeLink {
                node: "XTJ",
                left: "VCQ",
                right: "TBJ",
            }
        },
        {
            NodeLink {
                node: "BQT",
                left: "NXD",
                right: "SCD",
            }
        },
        {
            NodeLink {
                node: "QJK",
                left: "TKT",
                right: "CTC",
            }
        },
        {
            NodeLink {
                node: "MMP",
                left: "VMB",
                right: "NGD",
            }
        },
        {
            NodeLink {
                node: "DDN",
                left: "JRC",
                right: "JRC",
            }
        },
        {
            NodeLink {
                node: "CFM",
                left: "HCK",
                right: "CCB",
            }
        },
        {
            NodeLink {
                node: "FNX",
                left: "XHK",
                right: "GCP",
            }
        },
        {
            NodeLink {
                node: "JFP",
                left: "XMD",
                right: "QTS",
            }
        },
        {
            NodeLink {
                node: "LJL",
                left: "DCT",
                right: "PGF",
            }
        },
        {
            NodeLink {
                node: "TPH",
                left: "GLJ",
                right: "DBV",
            }
        },
        {
            NodeLink {
                node: "BNH",
                left: "JCB",
                right: "KBQ",
            }
        },
        {
            NodeLink {
                node: "KJM",
                left: "GBD",
                right: "JDP",
            }
        },
        {
            NodeLink {
                node: "CGX",
                left: "VGJ",
                right: "VCP",
            }
        },
        {
            NodeLink {
                node: "LFK",
                left: "XSM",
                right: "SHM",
            }
        },
        {
            NodeLink {
                node: "RSP",
                left: "GRS",
                right: "SRF",
            }
        },
        {
            NodeLink {
                node: "CMH",
                left: "TRC",
                right: "JXX",
            }
        },
        {
            NodeLink {
                node: "FXX",
                left: "FRM",
                right: "QVN",
            }
        },
        {
            NodeLink {
                node: "MSS",
                left: "TCG",
                right: "CQC",
            }
        },
        {
            NodeLink {
                node: "DFC",
                left: "MSS",
                right: "FQM",
            }
        },
        {
            NodeLink {
                node: "LVJ",
                left: "JJG",
                right: "TFC",
            }
        },
        {
            NodeLink {
                node: "FRK",
                left: "NDQ",
                right: "CDN",
            }
        },
        {
            NodeLink {
                node: "BCF",
                left: "TCF",
                right: "MFR",
            }
        },
        {
            NodeLink {
                node: "KGJ",
                left: "MVN",
                right: "VKK",
            }
        },
        {
            NodeLink {
                node: "FHK",
                left: "MLK",
                right: "PCS",
            }
        },
        {
            NodeLink {
                node: "VFP",
                left: "QGX",
                right: "NHF",
            }
        },
        {
            NodeLink {
                node: "FMR",
                left: "MKH",
                right: "QKN",
            }
        },
        {
            NodeLink {
                node: "XRG",
                left: "DJB",
                right: "MXG",
            }
        },
        {
            NodeLink {
                node: "VMG",
                left: "JNC",
                right: "XCN",
            }
        },
        {
            NodeLink {
                node: "MSC",
                left: "KBR",
                right: "HPP",
            }
        },
        {
            NodeLink {
                node: "TFC",
                left: "GXJ",
                right: "JSF",
            }
        },
        {
            NodeLink {
                node: "PSV",
                left: "BPN",
                right: "QTG",
            }
        },
        {
            NodeLink {
                node: "CPK",
                left: "HMH",
                right: "MRQ",
            }
        },
        {
            NodeLink {
                node: "SRF",
                left: "KMV",
                right: "BSM",
            }
        },
        {
            NodeLink {
                node: "KVV",
                left: "DHF",
                right: "PGX",
            }
        },
        {
            NodeLink {
                node: "PJL",
                left: "JHL",
                right: "CPK",
            }
        },
        {
            NodeLink {
                node: "MVS",
                left: "VRJ",
                right: "NTP",
            }
        },
        {
            NodeLink {
                node: "VMB",
                left: "XBQ",
                right: "PVS",
            }
        },
        {
            NodeLink {
                node: "DXS",
                left: "RKB",
                right: "KPD",
            }
        },
        {
            NodeLink {
                node: "BJX",
                left: "MCK",
                right: "LCL",
            }
        },
        {
            NodeLink {
                node: "BNZ",
                left: "QRF",
                right: "PQL",
            }
        },
        {
            NodeLink {
                node: "SJT",
                left: "DDD",
                right: "RMR",
            }
        },
        {
            NodeLink {
                node: "BBS",
                left: "MTL",
                right: "MTL",
            }
        },
        {
            NodeLink {
                node: "RMG",
                left: "CDN",
                right: "NDQ",
            }
        },
        {
            NodeLink {
                node: "KFG",
                left: "XNS",
                right: "HGP",
            }
        },
        {
            NodeLink {
                node: "BPN",
                left: "RKQ",
                right: "GVC",
            }
        },
        {
            NodeLink {
                node: "KCN",
                left: "CVH",
                right: "XCH",
            }
        },
        {
            NodeLink {
                node: "JPS",
                left: "VMG",
                right: "RTR",
            }
        },
        {
            NodeLink {
                node: "LDX",
                left: "LVF",
                right: "BSD",
            }
        },
        {
            NodeLink {
                node: "PSQ",
                left: "BKM",
                right: "MGZ",
            }
        },
        {
            NodeLink {
                node: "GXJ",
                left: "JHM",
                right: "LKQ",
            }
        },
        {
            NodeLink {
                node: "VPC",
                left: "LBJ",
                right: "HDD",
            }
        },
        {
            NodeLink {
                node: "DDB",
                left: "PNL",
                right: "NFH",
            }
        },
        {
            NodeLink {
                node: "TVD",
                left: "PJL",
                right: "FMJ",
            }
        },
        {
            NodeLink {
                node: "TQH",
                left: "PCX",
                right: "JSQ",
            }
        },
        {
            NodeLink {
                node: "MPP",
                left: "CHR",
                right: "XCP",
            }
        },
        {
            NodeLink {
                node: "CRQ",
                left: "FKG",
                right: "JQJ",
            }
        },
        {
            NodeLink {
                node: "PVT",
                left: "NMK",
                right: "HKH",
            }
        },
        {
            NodeLink {
                node: "DSH",
                left: "LDN",
                right: "BMR",
            }
        },
        {
            NodeLink {
                node: "XND",
                left: "NPQ",
                right: "BKV",
            }
        },
        {
            NodeLink {
                node: "SHM",
                left: "MSC",
                right: "JNS",
            }
        },
        {
            NodeLink {
                node: "FLV",
                left: "TVT",
                right: "HSM",
            }
        },
        {
            NodeLink {
                node: "GJN",
                left: "FTC",
                right: "XFX",
            }
        },
        {
            NodeLink {
                node: "HKX",
                left: "BCF",
                right: "FHJ",
            }
        },
        {
            NodeLink {
                node: "DBJ",
                left: "TLN",
                right: "CVP",
            }
        },
        {
            NodeLink {
                node: "CVR",
                left: "KFC",
                right: "LQP",
            }
        },
        {
            NodeLink {
                node: "VRJ",
                left: "QMB",
                right: "XLV",
            }
        },
        {
            NodeLink {
                node: "LSS",
                left: "MCK",
                right: "LCL",
            }
        },
        {
            NodeLink {
                node: "GPH",
                left: "TSV",
                right: "FKN",
            }
        },
        {
            NodeLink {
                node: "MLF",
                left: "BBS",
                right: "BBS",
            }
        },
        {
            NodeLink {
                node: "CTN",
                left: "MML",
                right: "SXC",
            }
        },
        {
            NodeLink {
                node: "NMR",
                left: "DCS",
                right: "XQN",
            }
        },
        {
            NodeLink {
                node: "VNP",
                left: "SFF",
                right: "QSD",
            }
        },
        {
            NodeLink {
                node: "TBJ",
                left: "XPG",
                right: "XHV",
            }
        },
        {
            NodeLink {
                node: "VCP",
                left: "SCV",
                right: "TDB",
            }
        },
        {
            NodeLink {
                node: "KMK",
                left: "MLK",
                right: "PCS",
            }
        },
        {
            NodeLink {
                node: "CSJ",
                left: "PNK",
                right: "JQP",
            }
        },
        {
            NodeLink {
                node: "RSN",
                left: "JRC",
                right: "PSQ",
            }
        },
        {
            NodeLink {
                node: "HLL",
                left: "KVJ",
                right: "QTX",
            }
        },
        {
            NodeLink {
                node: "KXM",
                left: "RJC",
                right: "BKQ",
            }
        },
        {
            NodeLink {
                node: "KCR",
                left: "MSM",
                right: "SPN",
            }
        },
        {
            NodeLink {
                node: "BFA",
                left: "CMB",
                right: "RSV",
            }
        },
        {
            NodeLink {
                node: "KHB",
                left: "SFH",
                right: "MVS",
            }
        },
        {
            NodeLink {
                node: "PXT",
                left: "KLV",
                right: "NRC",
            }
        },
        {
            NodeLink {
                node: "VSF",
                left: "XLG",
                right: "XLG",
            }
        },
        {
            NodeLink {
                node: "XJK",
                left: "LRN",
                right: "HXB",
            }
        },
        {
            NodeLink {
                node: "LCR",
                left: "RMJ",
                right: "QLD",
            }
        },
        {
            NodeLink {
                node: "FCR",
                left: "QFF",
                right: "CVG",
            }
        },
        {
            NodeLink {
                node: "DPM",
                left: "SDM",
                right: "DGQ",
            }
        },
        {
            NodeLink {
                node: "TXQ",
                left: "LPK",
                right: "JML",
            }
        },
        {
            NodeLink {
                node: "RLK",
                left: "DMM",
                right: "KBS",
            }
        },
        {
            NodeLink {
                node: "DXQ",
                left: "KPD",
                right: "RKB",
            }
        },
        {
            NodeLink {
                node: "TSV",
                left: "KGJ",
                right: "HCH",
            }
        },
        {
            NodeLink {
                node: "GXQ",
                left: "SXC",
                right: "MML",
            }
        },
        {
            NodeLink {
                node: "BMC",
                left: "XND",
                right: "CLT",
            }
        },
        {
            NodeLink {
                node: "LBL",
                left: "TKM",
                right: "HSD",
            }
        },
        {
            NodeLink {
                node: "FQS",
                left: "DSH",
                right: "HML",
            }
        },
        {
            NodeLink {
                node: "MJM",
                left: "CQK",
                right: "DFC",
            }
        },
        {
            NodeLink {
                node: "CJH",
                left: "XHK",
                right: "XHK",
            }
        },
        {
            NodeLink {
                node: "HHM",
                left: "HJK",
                right: "RKF",
            }
        },
        {
            NodeLink {
                node: "SDM",
                left: "MDD",
                right: "DVL",
            }
        },
        {
            NodeLink {
                node: "NDR",
                left: "CTH",
                right: "HVN",
            }
        },
        {
            NodeLink {
                node: "XVF",
                left: "PSV",
                right: "KFK",
            }
        },
        {
            NodeLink {
                node: "FTV",
                left: "HFF",
                right: "VXH",
            }
        },
        {
            NodeLink {
                node: "KDK",
                left: "BNK",
                right: "JTB",
            }
        },
        {
            NodeLink {
                node: "AAA",
                left: "FTV",
                right: "NDS",
            }
        },
        {
            NodeLink {
                node: "RDQ",
                left: "FTC",
                right: "XFX",
            }
        },
        {
            NodeLink {
                node: "GSC",
                left: "BHJ",
                right: "JJD",
            }
        },
        {
            NodeLink {
                node: "QBV",
                left: "BMC",
                right: "JVX",
            }
        },
        {
            NodeLink {
                node: "SRJ",
                left: "RQS",
                right: "BXQ",
            }
        },
        {
            NodeLink {
                node: "TRC",
                left: "JVM",
                right: "QNL",
            }
        },
        {
            NodeLink {
                node: "RMR",
                left: "BSB",
                right: "HBV",
            }
        },
        {
            NodeLink {
                node: "DCM",
                left: "RRF",
                right: "RFJ",
            }
        },
        {
            NodeLink {
                node: "KGF",
                left: "NHF",
                right: "QGX",
            }
        },
        {
            NodeLink {
                node: "PBF",
                left: "RCT",
                right: "JBQ",
            }
        },
        {
            NodeLink {
                node: "XRQ",
                left: "DGP",
                right: "DKR",
            }
        },
        {
            NodeLink {
                node: "CND",
                left: "DSF",
                right: "VTJ",
            }
        },
        {
            NodeLink {
                node: "CCB",
                left: "SLH",
                right: "HRF",
            }
        },
        {
            NodeLink {
                node: "SMS",
                left: "NDR",
                right: "PTT",
            }
        },
        {
            NodeLink {
                node: "DCT",
                left: "JXQ",
                right: "KQP",
            }
        },
        {
            NodeLink {
                node: "QTX",
                left: "JPH",
                right: "MPK",
            }
        },
        {
            NodeLink {
                node: "MRP",
                left: "CGG",
                right: "JPS",
            }
        },
        {
            NodeLink {
                node: "KQP",
                left: "BNL",
                right: "VDV",
            }
        },
        {
            NodeLink {
                node: "XDR",
                left: "DRQ",
                right: "FXV",
            }
        },
        {
            NodeLink {
                node: "RRL",
                left: "JVT",
                right: "PGM",
            }
        },
        {
            NodeLink {
                node: "LKQ",
                left: "MTN",
                right: "JRG",
            }
        },
        {
            NodeLink {
                node: "NBF",
                left: "RBB",
                right: "CNK",
            }
        },
        {
            NodeLink {
                node: "CQL",
                left: "XCG",
                right: "GNB",
            }
        },
        {
            NodeLink {
                node: "GLT",
                left: "JQM",
                right: "KCN",
            }
        },
        {
            NodeLink {
                node: "FFN",
                left: "FLV",
                right: "FVS",
            }
        },
        {
            NodeLink {
                node: "BPR",
                left: "TBJ",
                right: "VCQ",
            }
        },
        {
            NodeLink {
                node: "JBN",
                left: "DXQ",
                right: "DXS",
            }
        },
        {
            NodeLink {
                node: "KKN",
                left: "PQF",
                right: "VGK",
            }
        },
        {
            NodeLink {
                node: "RCT",
                left: "RSB",
                right: "KDK",
            }
        },
        {
            NodeLink {
                node: "MKL",
                left: "RVB",
                right: "LBL",
            }
        },
        {
            NodeLink {
                node: "DDD",
                left: "BSB",
                right: "HBV",
            }
        },
        {
            NodeLink {
                node: "RHX",
                left: "SQB",
                right: "SHB",
            }
        },
        {
            NodeLink {
                node: "NSV",
                left: "KLF",
                right: "KQG",
            }
        },
        {
            NodeLink {
                node: "MNC",
                left: "HRJ",
                right: "DBS",
            }
        },
        {
            NodeLink {
                node: "QGJ",
                left: "XVL",
                right: "RKD",
            }
        },
        {
            NodeLink {
                node: "KMN",
                left: "FHJ",
                right: "BCF",
            }
        },
        {
            NodeLink {
                node: "VHL",
                left: "MGD",
                right: "CXM",
            }
        },
        {
            NodeLink {
                node: "RVJ",
                left: "BNH",
                right: "GRQ",
            }
        },
        {
            NodeLink {
                node: "QHD",
                left: "FLK",
                right: "QGB",
            }
        },
        {
            NodeLink {
                node: "CQK",
                left: "MSS",
                right: "FQM",
            }
        },
        {
            NodeLink {
                node: "GQT",
                left: "FXX",
                right: "HVC",
            }
        },
        {
            NodeLink {
                node: "JNC",
                left: "TFD",
                right: "NCS",
            }
        },
        {
            NodeLink {
                node: "SFL",
                left: "FFN",
                right: "DMD",
            }
        },
        {
            NodeLink {
                node: "CTC",
                left: "CSJ",
                right: "HPQ",
            }
        },
        {
            NodeLink {
                node: "TCG",
                left: "MTD",
                right: "JTS",
            }
        },
        {
            NodeLink {
                node: "RTN",
                left: "QGJ",
                right: "QGJ",
            }
        },
        {
            NodeLink {
                node: "VGF",
                left: "KBS",
                right: "DMM",
            }
        },
        {
            NodeLink {
                node: "JNL",
                left: "RGC",
                right: "CLB",
            }
        },
        {
            NodeLink {
                node: "THG",
                left: "GSV",
                right: "VSJ",
            }
        },
        {
            NodeLink {
                node: "VDV",
                left: "HCL",
                right: "FLT",
            }
        },
        {
            NodeLink {
                node: "QGB",
                left: "DRN",
                right: "CBG",
            }
        },
        {
            NodeLink {
                node: "VPP",
                left: "RTD",
                right: "GQT",
            }
        },
        {
            NodeLink {
                node: "DHF",
                left: "FQS",
                right: "GNH",
            }
        },
        {
            NodeLink {
                node: "PTR",
                left: "KJN",
                right: "XRG",
            }
        },
        {
            NodeLink {
                node: "DQF",
                left: "CTN",
                right: "GXQ",
            }
        },
        {
            NodeLink {
                node: "VLN",
                left: "GNF",
                right: "TKV",
            }
        },
        {
            NodeLink {
                node: "JBC",
                left: "SFL",
                right: "CVK",
            }
        },
        {
            NodeLink {
                node: "LQR",
                left: "JML",
                right: "LPK",
            }
        },
        {
            NodeLink {
                node: "JPH",
                left: "THG",
                right: "SVP",
            }
        },
        {
            NodeLink {
                node: "CVP",
                left: "JFM",
                right: "VLN",
            }
        },
        {
            NodeLink {
                node: "VXT",
                left: "HMF",
                right: "FFR",
            }
        },
        {
            NodeLink {
                node: "HHK",
                left: "QGJ",
                right: "SSZ",
            }
        },
        {
            NodeLink {
                node: "MLK",
                left: "LVD",
                right: "KPB",
            }
        },
        {
            NodeLink {
                node: "QJF",
                left: "QVC",
                right: "FBM",
            }
        },
        {
            NodeLink {
                node: "KTS",
                left: "XRC",
                right: "MQS",
            }
        },
        {
            NodeLink {
                node: "HSM",
                left: "SGR",
                right: "BKG",
            }
        },
        {
            NodeLink {
                node: "DBN",
                left: "TKB",
                right: "XJK",
            }
        },
        {
            NodeLink {
                node: "QRF",
                left: "HHX",
                right: "TSH",
            }
        },
        {
            NodeLink {
                node: "FMJ",
                left: "CPK",
                right: "JHL",
            }
        },
        {
            NodeLink {
                node: "HLK",
                left: "PJV",
                right: "SBH",
            }
        },
        {
            NodeLink {
                node: "FNB",
                left: "MCV",
                right: "NHG",
            }
        },
        {
            NodeLink {
                node: "HPQ",
                left: "JQP",
                right: "PNK",
            }
        },
        {
            NodeLink {
                node: "RDN",
                left: "QXH",
                right: "VGC",
            }
        },
        {
            NodeLink {
                node: "SHB",
                left: "KXM",
                right: "TFJ",
            }
        },
        {
            NodeLink {
                node: "MDD",
                left: "QGD",
                right: "VRM",
            }
        },
        {
            NodeLink {
                node: "NTP",
                left: "QMB",
                right: "XLV",
            }
        },
        {
            NodeLink {
                node: "MKH",
                left: "VCX",
                right: "FRC",
            }
        },
        {
            NodeLink {
                node: "DGK",
                left: "LCH",
                right: "MGJ",
            }
        },
        {
            NodeLink {
                node: "TKV",
                left: "VFH",
                right: "DXK",
            }
        },
        {
            NodeLink {
                node: "NHX",
                left: "GPH",
                right: "KRT",
            }
        },
        {
            NodeLink {
                node: "GVC",
                left: "MGQ",
                right: "JNH",
            }
        },
        {
            NodeLink {
                node: "MQS",
                left: "VXT",
                right: "KSH",
            }
        },
        {
            NodeLink {
                node: "XFM",
                left: "SMX",
                right: "PTR",
            }
        },
        {
            NodeLink {
                node: "KCH",
                left: "KVR",
                right: "PXF",
            }
        },
        {
            NodeLink {
                node: "RKB",
                left: "HMV",
                right: "DLD",
            }
        },
        {
            NodeLink {
                node: "PTN",
                left: "TVJ",
                right: "PXC",
            }
        },
        {
            NodeLink {
                node: "LQP",
                left: "PKV",
                right: "DBJ",
            }
        },
        {
            NodeLink {
                node: "MGJ",
                left: "JTG",
                right: "SMG",
            }
        },
        {
            NodeLink {
                node: "FNS",
                left: "QLD",
                right: "RMJ",
            }
        },
        {
            NodeLink {
                node: "KVR",
                left: "VNF",
                right: "CDH",
            }
        },
        {
            NodeLink {
                node: "RXV",
                left: "XSF",
                right: "XXL",
            }
        },
        {
            NodeLink {
                node: "TFJ",
                left: "BKQ",
                right: "RJC",
            }
        },
        {
            NodeLink {
                node: "KNK",
                left: "JBN",
                right: "VTC",
            }
        },
        {
            NodeLink {
                node: "QKK",
                left: "CHR",
                right: "XCP",
            }
        },
        {
            NodeLink {
                node: "HXB",
                left: "VBD",
                right: "HLP",
            }
        },
        {
            NodeLink {
                node: "KGL",
                left: "PCX",
                right: "JSQ",
            }
        },
        {
            NodeLink {
                node: "KSR",
                left: "CMH",
                right: "JLT",
            }
        },
        {
            NodeLink {
                node: "PJT",
                left: "DRQ",
                right: "FXV",
            }
        },
        {
            NodeLink {
                node: "DSS",
                left: "SKK",
                right: "XQQ",
            }
        },
        {
            NodeLink {
                node: "VXH",
                left: "CVR",
                right: "TMC",
            }
        },
        {
            NodeLink {
                node: "RGJ",
                left: "TKT",
                right: "CTC",
            }
        },
        {
            NodeLink {
                node: "FVK",
                left: "SFQ",
                right: "TVD",
            }
        },
        {
            NodeLink {
                node: "BKD",
                left: "QTX",
                right: "KVJ",
            }
        },
        {
            NodeLink {
                node: "HHX",
                left: "DVD",
                right: "GTG",
            }
        },
        {
            NodeLink {
                node: "NFH",
                left: "PLM",
                right: "PDC",
            }
        },
        {
            NodeLink {
                node: "DBS",
                left: "KJM",
                right: "RVG",
            }
        },
        {
            NodeLink {
                node: "QFN",
                left: "CMX",
                right: "JSR",
            }
        },
        {
            NodeLink {
                node: "GVK",
                left: "CMH",
                right: "JLT",
            }
        },
        {
            NodeLink {
                node: "VGJ",
                left: "SCV",
                right: "TDB",
            }
        },
        {
            NodeLink {
                node: "VVD",
                left: "HBX",
                right: "GTC",
            }
        },
        {
            NodeLink {
                node: "HNX",
                left: "PLF",
                right: "RBG",
            }
        },
        {
            NodeLink {
                node: "HFF",
                left: "CVR",
                right: "TMC",
            }
        },
        {
            NodeLink {
                node: "JSR",
                left: "DFV",
                right: "FDT",
            }
        },
        {
            NodeLink {
                node: "HMH",
                left: "CFN",
                right: "FCT",
            }
        },
        {
            NodeLink {
                node: "XFF",
                left: "HHM",
                right: "DRV",
            }
        },
        {
            NodeLink {
                node: "CML",
                left: "RVJ",
                right: "VPD",
            }
        },
        {
            NodeLink {
                node: "HFL",
                left: "HVB",
                right: "GML",
            }
        },
        {
            NodeLink {
                node: "HST",
                left: "RMG",
                right: "FRK",
            }
        },
        {
            NodeLink {
                node: "JVX",
                left: "XND",
                right: "CLT",
            }
        },
        {
            NodeLink {
                node: "DMD",
                left: "FVS",
                right: "FLV",
            }
        },
        {
            NodeLink {
                node: "XLV",
                left: "VHL",
                right: "NBP",
            }
        },
        {
            NodeLink {
                node: "HJK",
                left: "QHD",
                right: "GHG",
            }
        },
        {
            NodeLink {
                node: "HCH",
                left: "MVN",
                right: "VKK",
            }
        },
        {
            NodeLink {
                node: "JTF",
                left: "HVB",
                right: "GML",
            }
        },
        {
            NodeLink {
                node: "PLM",
                left: "BMM",
                right: "DST",
            }
        },
        {
            NodeLink {
                node: "FKD",
                left: "FBL",
                right: "KNK",
            }
        },
        {
            NodeLink {
                node: "FVS",
                left: "HSM",
                right: "TVT",
            }
        },
        {
            NodeLink {
                node: "GNH",
                left: "DSH",
                right: "HML",
            }
        },
        {
            NodeLink {
                node: "CBL",
                left: "GLQ",
                right: "XBC",
            }
        },
        {
            NodeLink {
                node: "KBK",
                left: "QJG",
                right: "GGJ",
            }
        },
        {
            NodeLink {
                node: "PXF",
                left: "CDH",
                right: "VNF",
            }
        },
        {
            NodeLink {
                node: "VRM",
                left: "NPL",
                right: "CVT",
            }
        },
        {
            NodeLink {
                node: "LKR",
                left: "XCG",
                right: "XCG",
            }
        },
        {
            NodeLink {
                node: "VXV",
                left: "DVN",
                right: "KHM",
            }
        },
        {
            NodeLink {
                node: "VNF",
                left: "HST",
                right: "RMC",
            }
        },
        {
            NodeLink {
                node: "FKG",
                left: "XFG",
                right: "LCQ",
            }
        },
        {
            NodeLink {
                node: "QGX",
                left: "RXJ",
                right: "LXN",
            }
        },
        {
            NodeLink {
                node: "PNL",
                left: "PDC",
                right: "PLM",
            }
        },
        {
            NodeLink {
                node: "DVL",
                left: "VRM",
                right: "QGD",
            }
        },
        {
            NodeLink {
                node: "SQB",
                left: "TFJ",
                right: "KXM",
            }
        },
        {
            NodeLink {
                node: "PCX",
                left: "TTS",
                right: "MKL",
            }
        },
        {
            NodeLink {
                node: "FDT",
                left: "SJT",
                right: "KQN",
            }
        },
        {
            NodeLink {
                node: "QTS",
                left: "CRQ",
                right: "DKM",
            }
        },
        {
            NodeLink {
                node: "MJG",
                left: "CVK",
                right: "SFL",
            }
        },
        {
            NodeLink {
                node: "BXP",
                left: "HGM",
                right: "QJF",
            }
        },
        {
            NodeLink {
                node: "MCK",
                left: "NJM",
                right: "MMP",
            }
        },
        {
            NodeLink {
                node: "KLV",
                left: "PJT",
                right: "XDR",
            }
        },
        {
            NodeLink {
                node: "RKD",
                left: "JNB",
                right: "KTS",
            }
        },
        {
            NodeLink {
                node: "FLT",
                left: "FCR",
                right: "RHH",
            }
        },
        {
            NodeLink {
                node: "QGS",
                left: "DPM",
                right: "PHJ",
            }
        },
        {
            NodeLink {
                node: "LVB",
                left: "RSF",
                right: "HJV",
            }
        },
        {
            NodeLink {
                node: "XCH",
                left: "XBX",
                right: "XFM",
            }
        },
        {
            NodeLink {
                node: "FTG",
                left: "DPM",
                right: "PHJ",
            }
        },
        {
            NodeLink {
                node: "RGC",
                left: "SNM",
                right: "LJJ",
            }
        },
        {
            NodeLink {
                node: "QCC",
                left: "NRD",
                right: "HNK",
            }
        },
        {
            NodeLink {
                node: "LJM",
                left: "SRJ",
                right: "CCP",
            }
        },
        {
            NodeLink {
                node: "SFQ",
                left: "PJL",
                right: "FMJ",
            }
        },
        {
            NodeLink {
                node: "NRX",
                left: "MVP",
                right: "RSP",
            }
        },
        {
            NodeLink {
                node: "PKC",
                left: "XFD",
                right: "NMR",
            }
        },
        {
            NodeLink {
                node: "SPB",
                left: "JTF",
                right: "HFL",
            }
        },
        {
            NodeLink {
                node: "BVM",
                left: "MCS",
                right: "FVK",
            }
        },
        {
            NodeLink {
                node: "DRQ",
                left: "LKR",
                right: "CQL",
            }
        },
        {
            NodeLink {
                node: "PCS",
                left: "KPB",
                right: "LVD",
            }
        },
        {
            NodeLink {
                node: "HLP",
                left: "GPP",
                right: "PND",
            }
        },
        {
            NodeLink {
                node: "HVN",
                left: "KHB",
                right: "TBT",
            }
        },
        {
            NodeLink {
                node: "SCD",
                left: "GQH",
                right: "NKP",
            }
        },
        {
            NodeLink {
                node: "NPB",
                left: "PJV",
                right: "SBH",
            }
        },
        {
            NodeLink {
                node: "XQQ",
                left: "CJP",
                right: "JLG",
            }
        },
        {
            NodeLink {
                node: "CHX",
                left: "KSJ",
                right: "GJD",
            }
        },
        {
            NodeLink {
                node: "RGQ",
                left: "GLV",
                right: "GLV",
            }
        },
        {
            NodeLink {
                node: "DCS",
                left: "KHQ",
                right: "DTB",
            }
        },
        {
            NodeLink {
                node: "MCV",
                left: "SFK",
                right: "CFM",
            }
        },
        {
            NodeLink {
                node: "SCS",
                left: "RXV",
                right: "MGV",
            }
        },
        {
            NodeLink {
                node: "TKM",
                left: "XFF",
                right: "TMV",
            }
        },
        {
            NodeLink {
                node: "BSD",
                left: "GTB",
                right: "VPC",
            }
        },
        {
            NodeLink {
                node: "RPC",
                left: "SRJ",
                right: "CCP",
            }
        },
        {
            NodeLink {
                node: "TLN",
                left: "JFM",
                right: "VLN",
            }
        },
        {
            NodeLink {
                node: "MTN",
                left: "GFP",
                right: "JFP",
            }
        },
        {
            NodeLink {
                node: "FVP",
                left: "HGP",
                right: "XNS",
            }
        },
        {
            NodeLink {
                node: "JQX",
                left: "TQH",
                right: "KGL",
            }
        },
        {
            NodeLink {
                node: "LQG",
                left: "CSV",
                right: "DFR",
            }
        },
        {
            NodeLink {
                node: "SKK",
                left: "JLG",
                right: "CJP",
            }
        },
        {
            NodeLink {
                node: "MVN",
                left: "MJG",
                right: "JBC",
            }
        },
        {
            NodeLink {
                node: "RKT",
                left: "GLT",
                right: "QRM",
            }
        },
        {
            NodeLink {
                node: "STV",
                left: "FBL",
                right: "KNK",
            }
        },
        {
            NodeLink {
                node: "MQC",
                left: "BBS",
                right: "XGG",
            }
        },
        {
            NodeLink {
                node: "FFR",
                left: "LJL",
                right: "DQP",
            }
        },
        {
            NodeLink {
                node: "BDF",
                left: "MSM",
                right: "SPN",
            }
        },
        {
            NodeLink {
                node: "FQM",
                left: "TCG",
                right: "CQC",
            }
        },
        {
            NodeLink {
                node: "HNK",
                left: "NRF",
                right: "CPT",
            }
        },
        {
            NodeLink {
                node: "VKL",
                left: "DCM",
                right: "DTS",
            }
        },
        {
            NodeLink {
                node: "GDF",
                left: "JQX",
                right: "PJX",
            }
        },
        {
            NodeLink {
                node: "FBL",
                left: "VTC",
                right: "JBN",
            }
        },
        {
            NodeLink {
                node: "JDP",
                left: "XNG",
                right: "QFL",
            }
        },
        {
            NodeLink {
                node: "NLX",
                left: "CLM",
                right: "DGK",
            }
        },
        {
            NodeLink {
                node: "NRD",
                left: "CPT",
                right: "NRF",
            }
        },
        {
            NodeLink {
                node: "HKH",
                left: "FNS",
                right: "LCR",
            }
        },
        {
            NodeLink {
                node: "TRR",
                left: "DTM",
                right: "DTM",
            }
        },
        {
            NodeLink {
                node: "SGR",
                left: "LSS",
                right: "BJX",
            }
        },
        {
            NodeLink {
                node: "DSN",
                left: "DDB",
                right: "TQZ",
            }
        },
        {
            NodeLink {
                node: "KPB",
                left: "FXT",
                right: "PBF",
            }
        },
        {
            NodeLink {
                node: "DQP",
                left: "PGF",
                right: "DCT",
            }
        },
        {
            NodeLink {
                node: "SMX",
                left: "KJN",
                right: "XRG",
            }
        },
        {
            NodeLink {
                node: "GDG",
                left: "VPP",
                right: "PXQ",
            }
        },
        {
            NodeLink {
                node: "KKT",
                left: "CSV",
                right: "DFR",
            }
        },
        {
            NodeLink {
                node: "GPP",
                left: "NLX",
                right: "LHJ",
            }
        },
        {
            NodeLink {
                node: "DTS",
                left: "RRF",
                right: "RFJ",
            }
        },
        {
            NodeLink {
                node: "HXV",
                left: "JVX",
                right: "BMC",
            }
        },
        {
            NodeLink {
                node: "JCB",
                left: "VDK",
                right: "QLF",
            }
        },
        {
            NodeLink {
                node: "PRR",
                left: "FQH",
                right: "DSN",
            }
        },
        {
            NodeLink {
                node: "TFD",
                left: "RTV",
                right: "TGX",
            }
        },
        {
            NodeLink {
                node: "DMM",
                left: "FKD",
                right: "STV",
            }
        },
        {
            NodeLink {
                node: "KMV",
                left: "PPH",
                right: "RSJ",
            }
        },
        {
            NodeLink {
                node: "HGP",
                left: "QMM",
                right: "DCC",
            }
        },
        {
            NodeLink {
                node: "NCS",
                left: "RTV",
                right: "TGX",
            }
        },
        {
            NodeLink {
                node: "VCQ",
                left: "XPG",
                right: "XHV",
            }
        },
        {
            NodeLink {
                node: "QSD",
                left: "RFX",
                right: "XHH",
            }
        },
        {
            NodeLink {
                node: "SPN",
                left: "HND",
                right: "DJN",
            }
        },
        {
            NodeLink {
                node: "DBR",
                left: "CSD",
                right: "LFG",
            }
        },
        {
            NodeLink {
                node: "DST",
                left: "GDG",
                right: "KKR",
            }
        },
        {
            NodeLink {
                node: "BGV",
                left: "NRD",
                right: "HNK",
            }
        },
        {
            NodeLink {
                node: "KQN",
                left: "DDD",
                right: "RMR",
            }
        },
        {
            NodeLink {
                node: "CNP",
                left: "HGM",
                right: "QJF",
            }
        },
        {
            NodeLink {
                node: "MFR",
                left: "DHV",
                right: "NSV",
            }
        },
        {
            NodeLink {
                node: "VSJ",
                left: "JJJ",
                right: "RFR",
            }
        },
        {
            NodeLink {
                node: "CXF",
                left: "CXQ",
                right: "SRV",
            }
        },
        {
            NodeLink {
                node: "SCF",
                left: "CTN",
                right: "GXQ",
            }
        },
        {
            NodeLink {
                node: "HVB",
                left: "CML",
                right: "SQT",
            }
        },
        {
            NodeLink {
                node: "QHN",
                left: "MVP",
                right: "RSP",
            }
        },
        {
            NodeLink {
                node: "HGM",
                left: "FBM",
                right: "QVC",
            }
        },
        {
            NodeLink {
                node: "XGG",
                left: "MTL",
                right: "BNZ",
            }
        },
        {
            NodeLink {
                node: "JMT",
                left: "DKH",
                right: "PNP",
            }
        },
        {
            NodeLink {
                node: "MVK",
                left: "KRT",
                right: "GPH",
            }
        },
        {
            NodeLink {
                node: "BTC",
                left: "PNH",
                right: "PRJ",
            }
        },
        {
            NodeLink {
                node: "MLL",
                left: "TRR",
                right: "RVQ",
            }
        },
        {
            NodeLink {
                node: "VPD",
                left: "GRQ",
                right: "BNH",
            }
        },
        {
            NodeLink {
                node: "XBC",
                left: "FSC",
                right: "KGX",
            }
        },
        {
            NodeLink {
                node: "SFK",
                left: "HCK",
                right: "CCB",
            }
        },
        {
            NodeLink {
                node: "PNK",
                left: "QFN",
                right: "VHD",
            }
        },
        {
            NodeLink {
                node: "SQT",
                left: "VPD",
                right: "RVJ",
            }
        },
        {
            NodeLink {
                node: "QXH",
                left: "KCR",
                right: "BDF",
            }
        },
        {
            NodeLink {
                node: "JQM",
                left: "CVH",
                right: "XCH",
            }
        },
        {
            NodeLink {
                node: "QGD",
                left: "CVT",
                right: "NPL",
            }
        },
        {
            NodeLink {
                node: "JXX",
                left: "JVM",
                right: "QNL",
            }
        },
        {
            NodeLink {
                node: "RTR",
                left: "XCN",
                right: "JNC",
            }
        },
        {
            NodeLink {
                node: "BMM",
                left: "GDG",
                right: "KKR",
            }
        },
        {
            NodeLink {
                node: "MTF",
                left: "FVK",
                right: "MCS",
            }
        },
        {
            NodeLink {
                node: "DJB",
                left: "DDN",
                right: "DDN",
            }
        },
        {
            NodeLink {
                node: "GDQ",
                left: "SKK",
                right: "XQQ",
            }
        },
        {
            NodeLink {
                node: "NRF",
                left: "QKK",
                right: "MPP",
            }
        },
        {
            NodeLink {
                node: "CFN",
                left: "RCG",
                right: "SRH",
            }
        },
        {
            NodeLink {
                node: "BSB",
                left: "LQG",
                right: "KKT",
            }
        },
        {
            NodeLink {
                node: "FJX",
                left: "DSF",
                right: "VTJ",
            }
        },
        {
            NodeLink {
                node: "RMJ",
                left: "VQG",
                right: "CPJ",
            }
        },
        {
            NodeLink {
                node: "LFG",
                left: "VNC",
                right: "RKT",
            }
        },
        {
            NodeLink {
                node: "TKT",
                left: "HPQ",
                right: "CSJ",
            }
        },
        {
            NodeLink {
                node: "JRC",
                left: "BKM",
                right: "BKM",
            }
        },
        {
            NodeLink {
                node: "XNG",
                left: "FTS",
                right: "RDK",
            }
        },
        {
            NodeLink {
                node: "RRR",
                left: "JJD",
                right: "BHJ",
            }
        },
        {
            NodeLink {
                node: "CVK",
                left: "FFN",
                right: "DMD",
            }
        },
        {
            NodeLink {
                node: "TVT",
                left: "SGR",
                right: "BKG",
            }
        },
        {
            NodeLink {
                node: "KSJ",
                left: "LDF",
                right: "SPB",
            }
        },
        {
            NodeLink {
                node: "XSM",
                left: "JNS",
                right: "MSC",
            }
        },
        {
            NodeLink {
                node: "PRJ",
                left: "SSF",
                right: "HNG",
            }
        },
        {
            NodeLink {
                node: "MSG",
                left: "CSD",
                right: "LFG",
            }
        },
        {
            NodeLink {
                node: "TMV",
                left: "DRV",
                right: "HHM",
            }
        },
        {
            NodeLink {
                node: "BKQ",
                left: "VFP",
                right: "KGF",
            }
        },
        {
            NodeLink {
                node: "SQR",
                left: "NML",
                right: "HNX",
            }
        },
        {
            NodeLink {
                node: "SRH",
                left: "FNB",
                right: "XRL",
            }
        },
        {
            NodeLink {
                node: "CBG",
                left: "PXT",
                right: "JSM",
            }
        },
        {
            NodeLink {
                node: "MSM",
                left: "HND",
                right: "DJN",
            }
        },
        {
            NodeLink {
                node: "SCV",
                left: "SML",
                right: "CLH",
            }
        },
        {
            NodeLink {
                node: "QFS",
                left: "TKB",
                right: "XJK",
            }
        },
        {
            NodeLink {
                node: "CLB",
                left: "LJJ",
                right: "SNM",
            }
        },
        {
            NodeLink {
                node: "FRM",
                left: "QGS",
                right: "FTG",
            }
        },
        {
            NodeLink {
                node: "KVX",
                left: "CQK",
                right: "DFC",
            }
        },
        {
            NodeLink {
                node: "QFL",
                left: "RDK",
                right: "FTS",
            }
        },
        {
            NodeLink {
                node: "DSF",
                left: "CDG",
                right: "XLF",
            }
        },
        {
            NodeLink {
                node: "HML",
                left: "BMR",
                right: "LDN",
            }
        },
        {
            NodeLink {
                node: "LVF",
                left: "GTB",
                right: "VPC",
            }
        },
        {
            NodeLink {
                node: "XLF",
                left: "CNP",
                right: "BXP",
            }
        },
        {
            NodeLink {
                node: "HMF",
                left: "DQP",
                right: "LJL",
            }
        },
        {
            NodeLink {
                node: "MGZ",
                left: "RSV",
                right: "CMB",
            }
        },
        {
            NodeLink {
                node: "TGX",
                left: "XRQ",
                right: "XGL",
            }
        },
        {
            NodeLink {
                node: "RMC",
                left: "RMG",
                right: "FRK",
            }
        },
        {
            NodeLink {
                node: "PNH",
                left: "SSF",
                right: "HNG",
            }
        },
        {
            NodeLink {
                node: "PDC",
                left: "DST",
                right: "BMM",
            }
        },
        {
            NodeLink {
                node: "PGF",
                left: "JXQ",
                right: "KQP",
            }
        },
        {
            NodeLink {
                node: "BHJ",
                left: "DSS",
                right: "GDQ",
            }
        },
        {
            NodeLink {
                node: "VNC",
                left: "QRM",
                right: "GLT",
            }
        },
        {
            NodeLink {
                node: "MTL",
                left: "PQL",
                right: "QRF",
            }
        },
        {
            NodeLink {
                node: "XFJ",
                left: "NRX",
                right: "QHN",
            }
        },
        {
            NodeLink {
                node: "TBT",
                left: "SFH",
                right: "MVS",
            }
        },
        {
            NodeLink {
                node: "HSD",
                left: "TMV",
                right: "XFF",
            }
        },
        {
            NodeLink {
                node: "SSJ",
                left: "PSV",
                right: "KFK",
            }
        },
        {
            NodeLink {
                node: "HXP",
                left: "GJN",
                right: "RDQ",
            }
        },
        {
            NodeLink {
                node: "GRV",
                left: "GFN",
                right: "CBL",
            }
        },
        {
            NodeLink {
                node: "BNL",
                left: "HCL",
                right: "FLT",
            }
        },
        {
            NodeLink {
                node: "FTS",
                left: "NPB",
                right: "HLK",
            }
        },
        {
            NodeLink {
                node: "PXQ",
                left: "GQT",
                right: "RTD",
            }
        },
        {
            NodeLink {
                node: "RJM",
                left: "HMS",
                right: "RDN",
            }
        },
        {
            NodeLink {
                node: "SPV",
                left: "JJG",
                right: "TFC",
            }
        },
        {
            NodeLink {
                node: "SVP",
                left: "GSV",
                right: "VSJ",
            }
        },
        {
            NodeLink {
                node: "LCL",
                left: "NJM",
                right: "MMP",
            }
        },
        {
            NodeLink {
                node: "CTH",
                left: "KHB",
                right: "TBT",
            }
        },
        {
            NodeLink {
                node: "JBQ",
                left: "KDK",
                right: "RSB",
            }
        },
        {
            NodeLink {
                node: "XGL",
                left: "DGP",
                right: "DKR",
            }
        },
        {
            NodeLink {
                node: "RRF",
                left: "QFS",
                right: "DBN",
            }
        },
        {
            NodeLink {
                node: "SFF",
                left: "XHH",
                right: "RFX",
            }
        },
        {
            NodeLink {
                node: "GLQ",
                left: "KGX",
                right: "FSC",
            }
        },
        {
            NodeLink {
                node: "RKQ",
                left: "JNH",
                right: "MGQ",
            }
        },
        {
            NodeLink {
                node: "TMF",
                left: "LQR",
                right: "TXQ",
            }
        },
        {
            NodeLink {
                node: "DRN",
                left: "PXT",
                right: "JSM",
            }
        },
        {
            NodeLink {
                node: "TVJ",
                left: "PLL",
                right: "SQR",
            }
        },
        {
            NodeLink {
                node: "GRS",
                left: "KMV",
                right: "BSM",
            }
        },
        {
            NodeLink {
                node: "FRC",
                left: "BTL",
                right: "DVB",
            }
        },
        {
            NodeLink {
                node: "PLF",
                left: "FRQ",
                right: "LCF",
            }
        },
        {
            NodeLink {
                node: "NMK",
                left: "FNS",
                right: "LCR",
            }
        },
        {
            NodeLink {
                node: "XCN",
                left: "NCS",
                right: "TFD",
            }
        },
        {
            NodeLink {
                node: "JTB",
                left: "MQN",
                right: "LDX",
            }
        },
        {
            NodeLink {
                node: "CSD",
                left: "VNC",
                right: "RKT",
            }
        },
        {
            NodeLink {
                node: "MGD",
                left: "HRN",
                right: "LHP",
            }
        },
        {
            NodeLink {
                node: "RJG",
                left: "HXP",
                right: "RQH",
            }
        },
        {
            NodeLink {
                node: "HQV",
                left: "QBV",
                right: "HXV",
            }
        },
        {
            NodeLink {
                node: "NJM",
                left: "NGD",
                right: "VMB",
            }
        },
        {
            NodeLink {
                node: "CBR",
                left: "GGJ",
                right: "QJG",
            }
        },
        {
            NodeLink {
                node: "TMC",
                left: "LQP",
                right: "KFC",
            }
        },
        {
            NodeLink {
                node: "JRH",
                left: "KVR",
                right: "PXF",
            }
        },
        {
            NodeLink {
                node: "KBQ",
                left: "VDK",
                right: "QLF",
            }
        },
        {
            NodeLink {
                node: "HBV",
                left: "KKT",
                right: "LQG",
            }
        },
        {
            NodeLink {
                node: "RFJ",
                left: "QFS",
                right: "DBN",
            }
        },
        {
            NodeLink {
                node: "CFS",
                left: "JQX",
                right: "PJX",
            }
        },
        {
            NodeLink {
                node: "LNP",
                left: "QSD",
                right: "SFF",
            }
        },
        {
            NodeLink {
                node: "DKM",
                left: "JQJ",
                right: "FKG",
            }
        },
        {
            NodeLink {
                node: "CXR",
                left: "PKL",
                right: "CHX",
            }
        },
        {
            NodeLink {
                node: "KBR",
                left: "LGJ",
                right: "PTN",
            }
        },
        {
            NodeLink {
                node: "GNF",
                left: "DXK",
                right: "VFH",
            }
        },
        {
            NodeLink {
                node: "DKR",
                left: "KMK",
                right: "FHK",
            }
        },
        {
            NodeLink {
                node: "XQG",
                left: "CFS",
                right: "GDF",
            }
        },
        {
            NodeLink {
                node: "DFA",
                left: "PNL",
                right: "NFH",
            }
        },
        {
            NodeLink {
                node: "CXM",
                left: "HRN",
                right: "LHP",
            }
        },
        {
            NodeLink {
                node: "XLG",
                left: "TRR",
                right: "TRR",
            }
        },
        {
            NodeLink {
                node: "TTS",
                left: "RVB",
                right: "LBL",
            }
        },
        {
            NodeLink {
                node: "PVS",
                left: "KVX",
                right: "MJM",
            }
        },
        {
            NodeLink {
                node: "VDK",
                left: "PBX",
                right: "KML",
            }
        },
        {
            NodeLink {
                node: "XHH",
                left: "FJX",
                right: "CND",
            }
        },
        {
            NodeLink {
                node: "PKL",
                left: "KSJ",
                right: "GJD",
            }
        },
        {
            NodeLink {
                node: "LJJ",
                left: "BVB",
                right: "PRL",
            }
        },
        {
            NodeLink {
                node: "DTB",
                left: "RGP",
                right: "RJG",
            }
        },
        {
            NodeLink {
                node: "HND",
                left: "MLF",
                right: "MLF",
            }
        },
        {
            NodeLink {
                node: "FLK",
                left: "DRN",
                right: "CBG",
            }
        },
        {
            NodeLink {
                node: "BKZ",
                left: "BLG",
                right: "LVB",
            }
        },
        {
            NodeLink {
                node: "RDK",
                left: "HLK",
                right: "NPB",
            }
        },
        {
            NodeLink {
                node: "MCS",
                left: "SFQ",
                right: "TVD",
            }
        },
        {
            NodeLink {
                node: "HNG",
                left: "RLK",
                right: "VGF",
            }
        },
        {
            NodeLink {
                node: "LGJ",
                left: "TVJ",
                right: "PXC",
            }
        },
        {
            NodeLink {
                node: "LNV",
                left: "QGL",
                right: "NQN",
            }
        },
        {
            NodeLink {
                node: "PKG",
                left: "XLG",
                right: "MLL",
            }
        },
        {
            NodeLink {
                node: "FTC",
                left: "BKD",
                right: "HLL",
            }
        },
        {
            NodeLink {
                node: "HRN",
                left: "VSF",
                right: "PKG",
            }
        },
        {
            NodeLink {
                node: "MGV",
                left: "XSF",
                right: "XXL",
            }
        },
        {
            NodeLink {
                node: "VTJ",
                left: "CDG",
                right: "XLF",
            }
        },
        {
            NodeLink {
                node: "KSH",
                left: "FFR",
                right: "HMF",
            }
        },
        {
            NodeLink {
                node: "SBH",
                left: "SPV",
                right: "LVJ",
            }
        },
        {
            NodeLink {
                node: "RVG",
                left: "GBD",
                right: "JDP",
            }
        },
        {
            NodeLink {
                node: "PGM",
                left: "MSG",
                right: "DBR",
            }
        },
        {
            NodeLink {
                node: "JTJ",
                left: "SRV",
                right: "CXQ",
            }
        },
        {
            NodeLink {
                node: "XFA",
                left: "LVB",
                right: "BLG",
            }
        },
        {
            NodeLink {
                node: "BLN",
                left: "PRJ",
                right: "PNH",
            }
        },
        {
            NodeLink {
                node: "QVN",
                left: "FTG",
                right: "QGS",
            }
        },
        {
            NodeLink {
                node: "PRH",
                left: "FQH",
                right: "FQH",
            }
        },
        {
            NodeLink {
                node: "HDD",
                left: "PRH",
                right: "PRR",
            }
        },
        {
            NodeLink {
                node: "KLF",
                left: "XVF",
                right: "SSJ",
            }
        },
        {
            NodeLink {
                node: "LXN",
                left: "XNL",
                right: "KKN",
            }
        },
        {
            NodeLink {
                node: "QJA",
                left: "PQL",
                right: "QRF",
            }
        },
        {
            NodeLink {
                node: "KVJ",
                left: "MPK",
                right: "JPH",
            }
        },
        {
            NodeLink {
                node: "GRD",
                left: "CFS",
                right: "GDF",
            }
        },
        {
            NodeLink {
                node: "LPK",
                left: "TKS",
                right: "CXR",
            }
        },
        {
            NodeLink {
                node: "KRT",
                left: "TSV",
                right: "FKN",
            }
        },
        {
            NodeLink {
                node: "XRL",
                left: "MCV",
                right: "NHG",
            }
        },
        {
            NodeLink {
                node: "CGG",
                left: "RTR",
                right: "VMG",
            }
        },
        {
            NodeLink {
                node: "DTM",
                left: "FTV",
                right: "NDS",
            }
        },
        {
            NodeLink {
                node: "CCP",
                left: "BXQ",
                right: "RQS",
            }
        },
        {
            NodeLink {
                node: "JHM",
                left: "JRG",
                right: "MTN",
            }
        },
        {
            NodeLink {
                node: "RFX",
                left: "CND",
                right: "FJX",
            }
        },
        {
            NodeLink {
                node: "HCL",
                left: "FCR",
                right: "RHH",
            }
        },
        {
            NodeLink {
                node: "RTV",
                left: "XGL",
                right: "XRQ",
            }
        },
        {
            NodeLink {
                node: "PPL",
                left: "CBL",
                right: "GFN",
            }
        },
        {
            NodeLink {
                node: "ZZZ",
                left: "NDS",
                right: "FTV",
            }
        },
        {
            NodeLink {
                node: "HNR",
                left: "KHM",
                right: "DVN",
            }
        },
        {
            NodeLink {
                node: "NHF",
                left: "RXJ",
                right: "LXN",
            }
        },
        {
            NodeLink {
                node: "JLT",
                left: "JXX",
                right: "TRC",
            }
        },
        {
            NodeLink {
                node: "JNS",
                left: "HPP",
                right: "KBR",
            }
        },
        {
            NodeLink {
                node: "VGC",
                left: "BDF",
                right: "KCR",
            }
        },
        {
            NodeLink {
                node: "LCH",
                left: "SMG",
                right: "JTG",
            }
        },
        {
            NodeLink {
                node: "LHJ",
                left: "CLM",
                right: "DGK",
            }
        },
        {
            NodeLink {
                node: "NNC",
                left: "QGL",
                right: "NQN",
            }
        },
        {
            NodeLink {
                node: "GSV",
                left: "RFR",
                right: "JJJ",
            }
        },
        {
            NodeLink {
                node: "TDB",
                left: "SML",
                right: "CLH",
            }
        },
        {
            NodeLink {
                node: "LBJ",
                left: "PRH",
                right: "PRH",
            }
        },
        {
            NodeLink {
                node: "MKG",
                left: "HBX",
                right: "GTC",
            }
        },
        {
            NodeLink {
                node: "VLB",
                left: "QKN",
                right: "MKH",
            }
        },
        {
            NodeLink {
                node: "GQH",
                left: "QLG",
                right: "RHX",
            }
        },
        {
            NodeLink {
                node: "KGQ",
                left: "SCS",
                right: "XKL",
            }
        },
        {
            NodeLink {
                node: "RSJ",
                left: "GRD",
                right: "XQG",
            }
        },
        {
            NodeLink {
                node: "CVT",
                left: "RRR",
                right: "GSC",
            }
        },
        {
            NodeLink {
                node: "FCT",
                left: "RCG",
                right: "SRH",
            }
        },
        {
            NodeLink {
                node: "XFX",
                left: "HLL",
                right: "BKD",
            }
        },
        {
            NodeLink {
                node: "CPT",
                left: "QKK",
                right: "MPP",
            }
        },
        {
            NodeLink {
                node: "DVN",
                left: "MCT",
                right: "RRL",
            }
        },
        {
            NodeLink {
                node: "PHJ",
                left: "DGQ",
                right: "SDM",
            }
        },
        {
            NodeLink {
                node: "HVC",
                left: "FRM",
                right: "QVN",
            }
        },
        {
            NodeLink {
                node: "RCG",
                left: "FNB",
                right: "XRL",
            }
        },
        {
            NodeLink {
                node: "FXT",
                left: "JBQ",
                right: "RCT",
            }
        },
        {
            NodeLink {
                node: "JVT",
                left: "MSG",
                right: "DBR",
            }
        },
        {
            NodeLink {
                node: "NGD",
                left: "XBQ",
                right: "PVS",
            }
        },
        {
            NodeLink {
                node: "BMR",
                left: "MTF",
                right: "BVM",
            }
        },
        {
            NodeLink {
                node: "CSV",
                left: "XFJ",
                right: "KMM",
            }
        },
        {
            NodeLink {
                node: "SLH",
                left: "HKX",
                right: "KMN",
            }
        },
        {
            NodeLink {
                node: "JPM",
                left: "RPC",
                right: "LJM",
            }
        },
        {
            NodeLink {
                node: "QLG",
                left: "SQB",
                right: "SHB",
            }
        },
        {
            NodeLink {
                node: "QVC",
                left: "JVQ",
                right: "MNC",
            }
        },
        {
            NodeLink {
                node: "VGK",
                left: "RJM",
                right: "BTM",
            }
        },
        {
            NodeLink {
                node: "NDQ",
                left: "QKM",
                right: "KGQ",
            }
        },
        {
            NodeLink {
                node: "GCP",
                left: "RGQ",
                right: "SKJ",
            }
        },
        {
            NodeLink {
                node: "DLD",
                left: "CGX",
                right: "LXG",
            }
        },
        {
            NodeLink {
                node: "SFH",
                left: "NTP",
                right: "VRJ",
            }
        },
        {
            NodeLink {
                node: "CVH",
                left: "XFM",
                right: "XBX",
            }
        },
        {
            NodeLink {
                node: "RJC",
                left: "KGF",
                right: "VFP",
            }
        },
        {
            NodeLink {
                node: "VQG",
                left: "CBR",
                right: "KBK",
            }
        },
        {
            NodeLink {
                node: "CMX",
                left: "DFV",
                right: "FDT",
            }
        },
        {
            NodeLink {
                node: "RHH",
                left: "CVG",
                right: "QFF",
            }
        },
        {
            NodeLink {
                node: "RFR",
                left: "MKG",
                right: "VVD",
            }
        },
        {
            NodeLink {
                node: "HRJ",
                left: "KJM",
                right: "RVG",
            }
        },
        {
            NodeLink {
                node: "RBB",
                left: "NHX",
                right: "MVK",
            }
        },
        {
            NodeLink {
                node: "JXQ",
                left: "VDV",
                right: "BNL",
            }
        },
        {
            NodeLink {
                node: "PLB",
                left: "MLM",
                right: "SMS",
            }
        },
        {
            NodeLink {
                node: "HMV",
                left: "CGX",
                right: "LXG",
            }
        },
        {
            NodeLink {
                node: "RVB",
                left: "TKM",
                right: "HSD",
            }
        },
        {
            NodeLink {
                node: "XBQ",
                left: "MJM",
                right: "KVX",
            }
        },
        {
            NodeLink {
                node: "FBM",
                left: "MNC",
                right: "JVQ",
            }
        },
        {
            NodeLink {
                node: "XFD",
                left: "DCS",
                right: "XQN",
            }
        },
        {
            NodeLink {
                node: "TSH",
                left: "GTG",
                right: "DVD",
            }
        },
        {
            NodeLink {
                node: "GHG",
                left: "QGB",
                right: "FLK",
            }
        },
        {
            NodeLink {
                node: "LHP",
                left: "VSF",
                right: "PKG",
            }
        },
        {
            NodeLink {
                node: "RVV",
                left: "RGJ",
                right: "QJK",
            }
        },
        {
            NodeLink {
                node: "FKN",
                left: "KGJ",
                right: "HCH",
            }
        },
        {
            NodeLink {
                node: "LCQ",
                left: "CJH",
                right: "FNX",
            }
        },
        {
            NodeLink {
                node: "PNP",
                left: "LFK",
                right: "HTB",
            }
        },
        {
            NodeLink {
                node: "MCT",
                left: "JVT",
                right: "PGM",
            }
        },
        {
            NodeLink {
                node: "GFP",
                left: "XMD",
                right: "QTS",
            }
        },
        {
            NodeLink {
                node: "SNM",
                left: "BVB",
                right: "PRL",
            }
        },
        {
            NodeLink {
                node: "DRV",
                left: "HJK",
                right: "RKF",
            }
        },
        {
            NodeLink {
                node: "VKK",
                left: "JBC",
                right: "MJG",
            }
        },
        {
            NodeLink {
                node: "BTL",
                left: "PLB",
                right: "PNB",
            }
        },
        {
            NodeLink {
                node: "SSZ",
                left: "RKD",
                right: "XVL",
            }
        },
        {
            NodeLink {
                node: "XQN",
                left: "DTB",
                right: "KHQ",
            }
        },
        {
            NodeLink {
                node: "RSV",
                left: "HQV",
                right: "NNP",
            }
        },
        {
            NodeLink {
                node: "XNP",
                left: "GLJ",
                right: "DBV",
            }
        },
        {
            NodeLink {
                node: "BVB",
                left: "PPL",
                right: "GRV",
            }
        },
        {
            NodeLink {
                node: "QJG",
                left: "MGT",
                right: "PKC",
            }
        },
        {
            NodeLink {
                node: "TSJ",
                left: "RGC",
                right: "CLB",
            }
        },
        {
            NodeLink {
                node: "HPP",
                left: "LGJ",
                right: "PTN",
            }
        },
        {
            NodeLink {
                node: "LVD",
                left: "FXT",
                right: "PBF",
            }
        },
        {
            NodeLink {
                node: "DFR",
                left: "XFJ",
                right: "KMM",
            }
        },
        {
            NodeLink {
                node: "GNB",
                left: "RTN",
                right: "HHK",
            }
        },
        {
            NodeLink {
                node: "NKP",
                left: "RHX",
                right: "QLG",
            }
        },
        {
            NodeLink {
                node: "CQC",
                left: "MTD",
                right: "JTS",
            }
        },
        {
            NodeLink {
                node: "FRQ",
                left: "GVK",
                right: "KSR",
            }
        },
        {
            NodeLink {
                node: "GLV",
                left: "LVB",
                right: "BLG",
            }
        },
        {
            NodeLink {
                node: "MGT",
                left: "XFD",
                right: "NMR",
            }
        },
        {
            NodeLink {
                node: "JJG",
                left: "GXJ",
                right: "JSF",
            }
        },
        {
            NodeLink {
                node: "GJD",
                left: "LDF",
                right: "SPB",
            }
        },
        {
            NodeLink {
                node: "DBV",
                left: "BLN",
                right: "BTC",
            }
        },
        {
            NodeLink {
                node: "RXJ",
                left: "KKN",
                right: "XNL",
            }
        },
        {
            NodeLink {
                node: "NRC",
                left: "PJT",
                right: "XDR",
            }
        },
        {
            NodeLink {
                node: "SBA",
                left: "XVL",
                right: "RKD",
            }
        },
        {
            NodeLink {
                node: "LDF",
                left: "JTF",
                right: "HFL",
            }
        },
        {
            NodeLink {
                node: "KGX",
                left: "QNS",
                right: "TMF",
            }
        },
        {
            NodeLink {
                node: "QFF",
                left: "RVV",
                right: "QRG",
            }
        },
        {
            NodeLink {
                node: "HRF",
                left: "HKX",
                right: "KMN",
            }
        },
        {
            NodeLink {
                node: "JRG",
                left: "JFP",
                right: "GFP",
            }
        },
        {
            NodeLink {
                node: "SSF",
                left: "RLK",
                right: "VGF",
            }
        },
        {
            NodeLink {
                node: "QMM",
                left: "HDP",
                right: "PVT",
            }
        },
        {
            NodeLink {
                node: "PKV",
                left: "TLN",
                right: "CVP",
            }
        },
        {
            NodeLink {
                node: "XPG",
                left: "BGV",
                right: "QCC",
            }
        },
        {
            NodeLink {
                node: "JML",
                left: "CXR",
                right: "TKS",
            }
        },
        {
            NodeLink {
                node: "XKL",
                left: "MGV",
                right: "RXV",
            }
        },
        {
            NodeLink {
                node: "MXG",
                left: "DDN",
                right: "RSN",
            }
        },
        {
            NodeLink {
                node: "GLJ",
                left: "BTC",
                right: "BLN",
            }
        },
        {
            NodeLink {
                node: "NML",
                left: "RBG",
                right: "PLF",
            }
        },
        {
            NodeLink {
                node: "RQS",
                left: "CXF",
                right: "JTJ",
            }
        },
        {
            NodeLink {
                node: "KMM",
                left: "QHN",
                right: "NRX",
            }
        },
        {
            NodeLink {
                node: "NQN",
                left: "BPR",
                right: "XTJ",
            }
        },
        {
            NodeLink {
                node: "XNL",
                left: "PQF",
                right: "VGK",
            }
        },
        {
            NodeLink {
                node: "JJJ",
                left: "VVD",
                right: "MKG",
            }
        },
        {
            NodeLink {
                node: "CDG",
                left: "CNP",
                right: "BXP",
            }
        },
        {
            NodeLink {
                node: "PBX",
                left: "TPH",
                right: "XNP",
            }
        },
        {
            NodeLink {
                node: "XNS",
                left: "DCC",
                right: "QMM",
            }
        },
        {
            NodeLink {
                node: "RQH",
                left: "RDQ",
                right: "GJN",
            }
        },
        {
            NodeLink {
                node: "BLG",
                left: "HJV",
                right: "RSF",
            }
        },
        {
            NodeLink {
                node: "BTM",
                left: "RDN",
                right: "HMS",
            }
        },
        {
            NodeLink {
                node: "GRQ",
                left: "KBQ",
                right: "JCB",
            }
        },
        {
            NodeLink {
                node: "JHL",
                left: "MRQ",
                right: "HMH",
            }
        },
        {
            NodeLink {
                node: "DGQ",
                left: "MDD",
                right: "DVL",
            }
        },
        {
            NodeLink {
                node: "GGJ",
                left: "PKC",
                right: "MGT",
            }
        },
        {
            NodeLink {
                node: "QNS",
                left: "TXQ",
                right: "LQR",
            }
        },
        {
            NodeLink {
                node: "XFG",
                left: "CJH",
                right: "FNX",
            }
        },
        {
            NodeLink {
                node: "QMB",
                left: "VHL",
                right: "NBP",
            }
        },
        {
            NodeLink {
                node: "PPH",
                left: "GRD",
                right: "XQG",
            }
        },
        {
            NodeLink {
                node: "DVB",
                left: "PNB",
                right: "PLB",
            }
        },
        {
            NodeLink {
                node: "NXD",
                left: "NKP",
                right: "GQH",
            }
        },
        {
            NodeLink {
                node: "LDN",
                left: "BVM",
                right: "MTF",
            }
        },
        {
            NodeLink {
                node: "XBX",
                left: "SMX",
                right: "PTR",
            }
        },
        {
            NodeLink {
                node: "DGP",
                left: "KMK",
                right: "FHK",
            }
        },
        {
            NodeLink {
                node: "JJD",
                left: "GDQ",
                right: "DSS",
            }
        },
        {
            NodeLink {
                node: "CJP",
                left: "NMD",
                right: "VKL",
            }
        },
        {
            NodeLink {
                node: "CVG",
                left: "QRG",
                right: "RVV",
            }
        },
        {
            NodeLink {
                node: "QKM",
                left: "XKL",
                right: "SCS",
            }
        },
        {
            NodeLink {
                node: "HMS",
                left: "VGC",
                right: "QXH",
            }
        },
        {
            NodeLink {
                node: "CMB",
                left: "NNP",
                right: "HQV",
            }
        },
        {
            NodeLink {
                node: "GTB",
                left: "LBJ",
                right: "HDD",
            }
        },
        {
            NodeLink {
                node: "QKN",
                left: "VCX",
                right: "FRC",
            }
        },
        {
            NodeLink {
                node: "GML",
                left: "CML",
                right: "SQT",
            }
        },
        {
            NodeLink {
                node: "DVD",
                left: "DQF",
                right: "SCF",
            }
        },
        {
            NodeLink {
                node: "MLM",
                left: "NDR",
                right: "PTT",
            }
        },
        {
            NodeLink {
                node: "BNK",
                left: "LDX",
                right: "MQN",
            }
        },
        {
            NodeLink {
                node: "MQN",
                left: "BSD",
                right: "LVF",
            }
        },
        {
            NodeLink {
                node: "MPK",
                left: "THG",
                right: "SVP",
            }
        },
        {
            NodeLink {
                node: "DJN",
                left: "MLF",
                right: "MQC",
            }
        },
        {
            NodeLink {
                node: "SXC",
                left: "JMT",
                right: "LNK",
            }
        },
        {
            NodeLink {
                node: "MVP",
                left: "SRF",
                right: "GRS",
            }
        },
        {
            NodeLink {
                node: "CLM",
                left: "MGJ",
                right: "LCH",
            }
        },
        {
            NodeLink {
                node: "JVM",
                left: "BQT",
                right: "KTX",
            }
        },
        {
            NodeLink {
                node: "DCC",
                left: "HDP",
                right: "PVT",
            }
        },
        {
            NodeLink {
                node: "JFM",
                left: "TKV",
                right: "GNF",
            }
        },
        {
            NodeLink {
                node: "JSF",
                left: "JHM",
                right: "LKQ",
            }
        },
        {
            NodeLink {
                node: "CDN",
                left: "QKM",
                right: "KGQ",
            }
        },
        {
            NodeLink {
                node: "KQG",
                left: "XVF",
                right: "SSJ",
            }
        },
        {
            NodeLink {
                node: "CLH",
                left: "LNV",
                right: "NNC",
            }
        },
        {
            NodeLink {
                node: "KTX",
                left: "SCD",
                right: "NXD",
            }
        },
        {
            NodeLink {
                node: "HBX",
                left: "MPG",
                right: "MRP",
            }
        },
        {
            NodeLink {
                node: "JTG",
                left: "FVP",
                right: "KFG",
            }
        },
        {
            NodeLink {
                node: "RSF",
                left: "LKL",
                right: "KVV",
            }
        },
        {
            NodeLink {
                node: "MRQ",
                left: "FCT",
                right: "CFN",
            }
        },
        {
            NodeLink {
                node: "JSM",
                left: "KLV",
                right: "NRC",
            }
        },
        {
            NodeLink {
                node: "QLF",
                left: "PBX",
                right: "KML",
            }
        },
        {
            NodeLink {
                node: "MTD",
                left: "KRB",
                right: "JPM",
            }
        },
        {
            NodeLink {
                node: "RBG",
                left: "FRQ",
                right: "LCF",
            }
        },
        {
            NodeLink {
                node: "QTG",
                left: "RKQ",
                right: "GVC",
            }
        },
        {
            NodeLink {
                node: "NPL",
                left: "GSC",
                right: "RRR",
            }
        },
        {
            NodeLink {
                node: "XVL",
                left: "KTS",
                right: "JNB",
            }
        },
        {
            NodeLink {
                node: "PQL",
                left: "HHX",
                right: "TSH",
            }
        },
        {
            NodeLink {
                node: "CDH",
                left: "RMC",
                right: "HST",
            }
        },
        {
            NodeLink {
                node: "XSF",
                left: "JDV",
                right: "NBF",
            }
        },
        {
            NodeLink {
                node: "RKF",
                left: "GHG",
                right: "QHD",
            }
        },
        {
            NodeLink {
                node: "XMD",
                left: "DKM",
                right: "CRQ",
            }
        },
        {
            NodeLink {
                node: "RVQ",
                left: "DTM",
                right: "ZZZ",
            }
        },
        {
            NodeLink {
                node: "VHD",
                left: "CMX",
                right: "JSR",
            }
        },
        {
            NodeLink {
                node: "RTD",
                left: "HVC",
                right: "FXX",
            }
        },
        {
            NodeLink {
                node: "KFC",
                left: "PKV",
                right: "DBJ",
            }
        },
        {
            NodeLink {
                node: "NHG",
                left: "CFM",
                right: "SFK",
            }
        },
        {
            NodeLink {
                node: "JTS",
                left: "KRB",
                right: "JPM",
            }
        },
        {
            NodeLink {
                node: "BSM",
                left: "RSJ",
                right: "PPH",
            }
        },
        {
            NodeLink {
                node: "VBD",
                left: "PND",
                right: "GPP",
            }
        },
        {
            NodeLink {
                node: "TKS",
                left: "PKL",
                right: "CHX",
            }
        },
        {
            NodeLink {
                node: "TQZ",
                left: "NFH",
                right: "PNL",
            }
        },
        {
            NodeLink {
                node: "SML",
                left: "LNV",
                right: "NNC",
            }
        },
        {
            NodeLink {
                node: "HCK",
                left: "HRF",
                right: "SLH",
            }
        },
        {
            NodeLink {
                node: "PXC",
                left: "SQR",
                right: "PLL",
            }
        },
        {
            NodeLink {
                node: "JLG",
                left: "VKL",
                right: "NMD",
            }
        },
        {
            NodeLink {
                node: "SRV",
                left: "VLB",
                right: "FMR",
            }
        },
        {
            NodeLink {
                node: "PTT",
                left: "CTH",
                right: "HVN",
            }
        },
        {
            NodeLink {
                node: "PLL",
                left: "HNX",
                right: "NML",
            }
        },
        {
            NodeLink {
                node: "TKB",
                left: "HXB",
                right: "LRN",
            }
        },
        {
            NodeLink {
                node: "CXQ",
                left: "VLB",
                right: "FMR",
            }
        },
        {
            NodeLink {
                node: "NBP",
                left: "CXM",
                right: "MGD",
            }
        },
        {
            NodeLink {
                node: "HDP",
                left: "NMK",
                right: "HKH",
            }
        },
        {
            NodeLink {
                node: "LCF",
                left: "GVK",
                right: "KSR",
            }
        },
        {
            NodeLink {
                node: "DKH",
                left: "LFK",
                right: "HTB",
            }
        },
        {
            NodeLink {
                node: "LXG",
                left: "VGJ",
                right: "VCP",
            }
        },
        {
            NodeLink {
                node: "QRG",
                left: "RGJ",
                right: "QJK",
            }
        },
        {
            NodeLink {
                node: "KRB",
                left: "LJM",
                right: "RPC",
            }
        },
        {
            NodeLink {
                node: "JDV",
                left: "RBB",
                right: "CNK",
            }
        },
        {
            NodeLink {
                node: "PGX",
                left: "GNH",
                right: "FQS",
            }
        },
        {
            NodeLink {
                node: "DHV",
                left: "KQG",
                right: "KLF",
            }
        },
        {
            NodeLink {
                node: "QNL",
                left: "BQT",
                right: "KTX",
            }
        },
        {
            NodeLink {
                node: "XCP",
                left: "HNR",
                right: "VXV",
            }
        },
        {
            NodeLink {
                node: "JVQ",
                left: "HRJ",
                right: "DBS",
            }
        },
        {
            NodeLink {
                node: "QLD",
                left: "CPJ",
                right: "VQG",
            }
        },
        {
            NodeLink {
                node: "CLT",
                left: "BKV",
                right: "NPQ",
            }
        },
        {
            NodeLink {
                node: "QRM",
                left: "JQM",
                right: "KCN",
            }
        },
        {
            NodeLink {
                node: "RGP",
                left: "RQH",
                right: "HXP",
            }
        },
        {
            NodeLink {
                node: "PRL",
                left: "GRV",
                right: "PPL",
            }
        },
        {
            NodeLink {
                node: "KBS",
                left: "FKD",
                right: "STV",
            }
        },
        {
            NodeLink {
                node: "XXL",
                left: "NBF",
                right: "JDV",
            }
        },
        {
            NodeLink {
                node: "MML",
                left: "LNK",
                right: "JMT",
            }
        },
        {
            NodeLink {
                node: "VFH",
                left: "JRH",
                right: "KCH",
            }
        },
        {
            NodeLink {
                node: "PQF",
                left: "RJM",
                right: "BTM",
            }
        },
        {
            NodeLink {
                node: "JNB",
                left: "MQS",
                right: "XRC",
            }
        },
        {
            NodeLink {
                node: "VTC",
                left: "DXQ",
                right: "DXS",
            }
        },
        {
            NodeLink {
                node: "XCG",
                left: "RTN",
                right: "RTN",
            }
        },
        {
            NodeLink {
                node: "MGQ",
                left: "LNP",
                right: "VNP",
            }
        },
        {
            NodeLink {
                node: "BXQ",
                left: "CXF",
                right: "JTJ",
            }
        },
        {
            NodeLink {
                node: "JNH",
                left: "LNP",
                right: "VNP",
            }
        },
        {
            NodeLink {
                node: "BKM",
                left: "CMB",
                right: "RSV",
            }
        },
        {
            NodeLink {
                node: "PJX",
                left: "KGL",
                right: "TQH",
            }
        },
        {
            NodeLink {
                node: "SMG",
                left: "FVP",
                right: "KFG",
            }
        },
        {
            NodeLink {
                node: "LRN",
                left: "HLP",
                right: "VBD",
            }
        },
        {
            NodeLink {
                node: "XRC",
                left: "KSH",
                right: "VXT",
            }
        },
        {
            NodeLink {
                node: "MPG",
                left: "JPS",
                right: "CGG",
            }
        },
        {
            NodeLink {
                node: "XHV",
                left: "QCC",
                right: "BGV",
            }
        },
        {
            NodeLink {
                node: "KHM",
                left: "RRL",
                right: "MCT",
            }
        },
        {
            NodeLink {
                node: "RSB",
                left: "BNK",
                right: "JTB",
            }
        },
        {
            NodeLink {
                node: "PNB",
                left: "MLM",
                right: "SMS",
            }
        },
        {
            NodeLink {
                node: "XHK",
                left: "RGQ",
                right: "RGQ",
            }
        },
        {
            NodeLink {
                node: "NNP",
                left: "HXV",
                right: "QBV",
            }
        },
        {
            NodeLink {
                node: "GTC",
                left: "MPG",
                right: "MRP",
            }
        },
        {
            NodeLink {
                node: "LNK",
                left: "PNP",
                right: "DKH",
            }
        },
        {
            NodeLink {
                node: "FHJ",
                left: "TCF",
                right: "MFR",
            }
        },
        {
            NodeLink {
                node: "HJV",
                left: "KVV",
                right: "LKL",
            }
        },
        {
            NodeLink {
                node: "GFN",
                left: "XBC",
                right: "GLQ",
            }
        },
        {
            NodeLink {
                node: "JQP",
                left: "VHD",
                right: "QFN",
            }
        },
        {
            NodeLink {
                node: "CPJ",
                left: "CBR",
                right: "KBK",
            }
        },
        {
            NodeLink {
                node: "NMD",
                left: "DCM",
                right: "DTS",
            }
        },
        {
            NodeLink {
                node: "KML",
                left: "TPH",
                right: "XNP",
            }
        },
        {
            NodeLink {
                node: "CHR",
                left: "HNR",
                right: "VXV",
            }
        },
        {
            NodeLink {
                node: "JSQ",
                left: "MKL",
                right: "TTS",
            }
        },
        {
            NodeLink {
                node: "FQH",
                left: "DDB",
                right: "DDB",
            }
        },
        {
            NodeLink {
                node: "CNK",
                left: "MVK",
                right: "NHX",
            }
        },
        {
            NodeLink {
                node: "VCX",
                left: "BTL",
                right: "DVB",
            }
        },
        {
            NodeLink {
                node: "KHQ",
                left: "RGP",
                right: "RJG",
            }
        },
        {
            NodeLink {
                node: "BKG",
                left: "LSS",
                right: "BJX",
            }
        },
        {
            NodeLink {
                node: "KKR",
                left: "PXQ",
                right: "VPP",
            }
        },
        {
            NodeLink {
                node: "LKL",
                left: "DHF",
                right: "PGX",
            }
        },
        {
            NodeLink {
                node: "PJV",
                left: "SPV",
                right: "LVJ",
            }
        },
        {
            NodeLink {
                node: "SKJ",
                left: "GLV",
                right: "BKZ",
            }
        },
        {
            NodeLink {
                node: "TCF",
                left: "DHV",
                right: "NSV",
            }
        },
        {
            NodeLink {
                node: "DXK",
                left: "JRH",
                right: "KCH",
            }
        },
        {
            NodeLink {
                node: "DFV",
                left: "KQN",
                right: "SJT",
            }
        },
        {
            NodeLink {
                node: "KJN",
                left: "DJB",
                right: "MXG",
            }
        },
        {
            NodeLink {
                node: "BKV",
                left: "TSJ",
                right: "JNL",
            }
        },
        {
            NodeLink {
                node: "JQJ",
                left: "XFG",
                right: "LCQ",
            }
        },
        {
            NodeLink {
                node: "HTB",
                left: "XSM",
                right: "SHM",
            }
        },
        {
            NodeLink {
                node: "NPQ",
                left: "TSJ",
                right: "JNL",
            }
        },
        {
            NodeLink {
                node: "FXV",
                left: "LKR",
                right: "CQL",
            }
        },
        {
            NodeLink {
                node: "NDS",
                left: "HFF",
                right: "VXH",
            }
        },
        {
            NodeLink {
                node: "KFK",
                left: "BPN",
                right: "QTG",
            }
        },
        {
            NodeLink {
                node: "GTG",
                left: "DQF",
                right: "SCF",
            }
        },
        {
            NodeLink {
                node: "FSC",
                left: "QNS",
                right: "TMF",
            }
        },
    ],
};
