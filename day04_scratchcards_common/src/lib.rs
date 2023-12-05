pub struct ScratchCard<const W: usize, const N: usize> {
    pub number: u64,
    pub winning_numbers: [u64; W],
    pub numbers: [u64; N],
}

// Transform with:
// '<,'>s/\(\d\) /\1, /g | '<,'>s/:/, winning_numbers: [/ | '<,'>s/|/], numbers: [/ | '<,'>s/Card/{ ScratchCard { number: / | '<,'>s/$/] } },

pub const SAMPLE_DATA: [ScratchCard<5, 8>; 6] = [
    {
        ScratchCard {
            number: 1,
            winning_numbers: [41, 48, 83, 86, 17],
            numbers: [83, 86, 6, 31, 17, 9, 48, 53],
        }
    },
    {
        ScratchCard {
            number: 2,
            winning_numbers: [13, 32, 20, 16, 61],
            numbers: [61, 30, 68, 82, 17, 32, 24, 19],
        }
    },
    {
        ScratchCard {
            number: 3,
            winning_numbers: [1, 21, 53, 59, 44],
            numbers: [69, 82, 63, 72, 16, 21, 14, 1],
        }
    },
    {
        ScratchCard {
            number: 4,
            winning_numbers: [41, 92, 73, 84, 69],
            numbers: [59, 84, 76, 51, 58, 5, 54, 83],
        }
    },
    {
        ScratchCard {
            number: 5,
            winning_numbers: [87, 83, 26, 28, 32],
            numbers: [88, 30, 70, 12, 93, 22, 82, 36],
        }
    },
    {
        ScratchCard {
            number: 6,
            winning_numbers: [31, 18, 13, 56, 72],
            numbers: [74, 77, 10, 23, 35, 67, 36, 11],
        }
    },
];
