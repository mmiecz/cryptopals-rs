use phf;
use phf::phf_map;
///Letter frequencies in english language, taken from https://www3.nd.edu/~busiforc/handouts/cryptography/letterfrequencies.html
/// TODO: Add other chars, change source to:http://www.viviancook.uk/SpellStats/SingleLetterFrequencies.htm
pub static ENGLISH_LETTERS_FREQ: phf::Map<char, u32> = phf_map! {
    'e' => 557_331,
    't' => 401_449,
    'a' => 362_559,
    'o' => 340_157,
    'n' => 317_669,
    'i' => 311_560,
    's' => 286_499,
    'h' => 271_880,
    'r' => 257_079,
    'd' => 190_472,
    'l' => 189_582,
    'u' => 130_678,
    'c' => 117_936,
    'm' => 108_900,
    'g' => 101_871,
    'w' => 100_294,
    'f' => 96_431,
    'y' => 84_729,
    'p' => 80_953,
    'b' => 62_140,
};
