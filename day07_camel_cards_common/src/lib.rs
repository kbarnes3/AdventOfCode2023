pub struct HandBid {
    pub hand: [char; 5],
    pub bid: u64,
}

// Transform with:
// '<,'>s/\(.\)\(.\)\(.\)\(.\)\(.\) \(\d\+\)/{ HandBid { hand: [ '\1', '\2', '\3', '\4', '\5'], bid: \6 } },

pub const SAMPLE_DATA: [HandBid; 5] = [
    {
        HandBid {
            hand: ['3', '2', 'T', '3', 'K'],
            bid: 765,
        }
    },
    {
        HandBid {
            hand: ['T', '5', '5', 'J', '5'],
            bid: 684,
        }
    },
    {
        HandBid {
            hand: ['K', 'K', '6', '7', '7'],
            bid: 28,
        }
    },
    {
        HandBid {
            hand: ['K', 'T', 'J', 'J', 'T'],
            bid: 220,
        }
    },
    {
        HandBid {
            hand: ['Q', 'Q', 'Q', 'J', 'A'],
            bid: 483,
        }
    },
];
