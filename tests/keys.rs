#![allow(non_snake_case)]

macro_rules! buffer_tests {
    ($mod:ident, $($name:ident: $keymaps:literal => $output:literal,)*) => {
        mod $mod {
            use vim_buffer::Buffer;
            $(
                #[test]
                fn $name() {
                    let mut buffer = Buffer::default();
                    buffer.update_from_string($keymaps).unwrap();
                    assert_eq!(
                        buffer.as_content(),
                        $output,
                        "Keys: \x1b[35m{}\x1b[0m",
                        $keymaps
                    );
                }
            )*
        }
    };
}

buffer_tests!(insert,

backspace: "ia<BS><BS>bcd<BS>" => "bc",
arrows: "iabc<Left>d<Right>e" => "abdce",

);

buffer_tests!(normal,

backspace: "ia<Esc><BS><BS>ibcd<Esc><BS>ie" => "becda",
a: "ia<Esc>ab" => "ab",
i: "ia<Esc>ib" => "ba",
I: "i  ab<Esc>Ic" => "  cab",
I_no_space: "iab<Esc>Ic" => "cab",
I_empty_line: "i   <Esc>Ia" => "   a",
A: "iabc<Esc>Ad" => "abcd",
A_empty_line: "i   <Esc>Aa" => "   a",
arrows: "iabc<Esc><Left>id<Esc><Right>ae" => "adbec",
h_l: "iabc<Esc>hid<Esc>lae" => "adbec",
_0: "i  abc<Esc>0id" => "d  abc",
dollar: "i abc <Esc>0id<Esc>$ae" => "d abc e",
caret: "i  abc<Esc>^id" => "  dabc",
f_not_found: "iabc<Esc>0<Right>fzid" => "adbc",
t_not_found: "iabc<Esc>0<Right>tzid" => "adbc",
f: "iabcabc<Esc>0fcad<Esc>fcae" => "abcdabce",
t: "iabcabc<Esc>0tcad<Esc>ltcae" => "abdcabec",
F_not_found: "iabc<Esc><Left>Fzad" => "abdc",
T_not_found: "iabc<Esc><Left>Tzad" => "abdc",
F: "iabcabc<Esc>Faad<Esc>hFaae" => "aebcadbc",
T: "iabcabc<Esc>Taad<Esc>hhTaae" => "abecabdc",
x: "iabcd<Esc>x<Left>x" => "ac",
x_empty: "x" => "",
X: "iabcd<Esc>X<Left>X" => "bd",
s: "iabcd<Esc>se<Esc>hsf" => "abfe",
S: "iabcdef<Esc>hhhSghij" => "ghij",
r: "iabcd<Esc>Fbre" => "aecd",

w: "ibc   def::(Bl<Esc>0wa.<Esc>lwa.<Esc>lwa.<Esc>lwa." => "bc   d.ef:.:(B.l.",
w_end_space: "iab <Esc>0wa." => "ab .",
w_end_symbol: "i)))<Esc>0wa." => "))).",

W: "i  ab  cd<Esc>0Wiz<Esc>lWiz<Esc>lWaz" => "  zab  zcdz",
W_empty: "i    <Esc>0Waz" => "    z",

b: "i)))) ef<Esc>biz<Esc>biz" => "z)))) zef",
b_word: "iab<Esc>biz" => "zab",
b_word_space: "iab <Esc>biz" => "zab ",
b_leading_space: "i ab <Esc>biz" => " zab ",
b_symbols: "iab(:) <Esc>biz" => "abz(:) ",
b_leading_symbols: "i(:)<Esc>biz" => "z(:)",
b_spaces: "i  <Esc>biz" => "z  ",
b_single_char: "i a b<Esc>biz" => " za b",

B_empty: "i   <Esc>Biz" => "z   ",
B_words: "iab cd  <Esc>Biz<Esc>Biy" => "yab zcd  ",
B_symbols: "iab(:)cd  <Esc>Biz<Esc>Biy" => "yzab(:)cd  ",

dw_middle: "iabc def<Esc>0dw" => "def",
dw_leading_space: "i  abc def<Esc>0dw" => "abc def",
dw_single_char: "ia b<Esc>0dw" => "b",
dw_cursor_middle: "iabc def<Esc>Bldw" => "abc d",
dw_end: "iabc<Esc>0dw" => "",

db_middle: "iabc def<Esc>db" => "abc f",
db_start: "iabc<Esc>0db" => "abc",
db_leading_space: "i  abc<Esc>db" => "  c",
db_single_char: "i a b<Esc>db" => " b",
db_single_char_end: "i a b <Esc>db" => " a  ",

d0_middle: "i abc def ghi<Esc>bbd0" => "def ghi",
d0_start: "iabc<Esc>0d0" => "abc",
d0_end: "iabc def <Esc>d0" => " ",
d0_empty: "d0" => "",

ddollar_start: "iabc def<Esc>0d$" => "",
ddollar_end: "iabc def<Esc>d$" => "abc de",
ddollar_middle: "iabc def<Esc>hhhhd$" => "ab",
ddollar_empty: "d$" => "",

dcaret_end: "i  abc<Esc>d^" => "  c",
dcaret_start: "iabc<Esc>0d^" => "abc",
dcaret_spaces_only: "i    <Esc>d^" => "   ",
dcaret_empty: "d^" => "",
dcaret_middle: "i abc def ghi<Esc>bbld^" => " ef ghi",

dinvalid: "d " => "",

);
