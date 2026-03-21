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
right_end: "iabc<Right>d" => "abcd",
left_start: "iabc<Esc>I<Left>d" => "dabc",

);

buffer_tests!(normal,

backspace: "ia<Esc><BS><BS>ibcd<Esc><BS>ie" => "becda",
arrows: "iabc<Esc><Left>id<Esc><Right>ae" => "adbec",
right_end: "iabc<Esc><Right>ad" => "abcd",
left_start: "iabc<Esc>0<Left>id" => "dabc",

a: "ia<Esc>ab" => "ab",
i: "ia<Esc>ib" => "ba",
I: "i  ab<Esc>Ic" => "  cab",
I_no_space: "iab<Esc>Ic" => "cab",
I_empty_line: "i   <Esc>Ia" => "   a",
A: "iabc<Esc>Ad" => "abcd",
A_empty_line: "i   <Esc>Aa" => "   a",
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
simple_F: "iabc<Esc>Fbid" => "adbc",
T: "iabcabc<Esc>Taad<Esc>hhTaae" => "abecabdc",
x: "iabcd<Esc>x<Left>x" => "ac",
x_empty: "x" => "",
X: "iabcd<Esc>X<Left>X" => "bd",
s: "iabcd<Esc>se<Esc>hsf" => "abfe",
S: "iabcdef<Esc>hhhSghij" => "ghij",
r: "iabcd<Esc>Fbre" => "aecd",
r_empty: "rx" => "",
r_end: "iabc<Esc>rx" => "abx",
r_dollar_end: "iabc<Esc>$rx" => "abx",

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

dnone: "d " => "",
dinvalid: "iabc<Esc>0dAdl" => "bc",

dW: "iab  cd ef<Esc>0dW" => "cd ef",
dW_middle_no_right: "iab  cd ef<Esc>dW" => "ab  cd e",
dW_middle_right: "iab  cd ef<Esc><Right>dW" => "ab  cd e",

dB: "iab  cd ef<Esc>0<Right><Right>dB" => "  cd ef",
dB_space: "i  ab<Esc>0<Right>dB" => " ab",
dB_start: "iab<Esc>0dB" => "ab",

_f: "iabcabc<Esc>0fcaz" => "abczabc",
df: "iabcabc<Esc>0dfc" => "abc",
df_not_found: "iabc<Esc>0dfz" => "abc",
df_invalid: "iabc<Esc>0drib" => "babc",

_t: "iabcabc<Esc>0tcaz" => "abzcabc",
dt: "iabcabc<Esc>0dtc" => "cabc",
dt_not_found: "iabc<Esc>0dtz" => "abc",

_T: "iabcabc<Esc>Tcaz" => "abcazbc",
dT: "iabcabc<Esc>dTc" => "abcc",
dT_not_found: "iabc<Esc>dTz" => "abc",

_F: "iabcabc<Esc>Fcaz" => "abczabc",
dF: "iabcabc<Esc>dFc" => "abc",
dF_not_found: "iabc<Esc>dFz" => "abc",

e: "iabc def<Esc>0eaz" => "abcz def",
e_not_in_word: "i abc def<Esc>0eaz" => " abcz def",
e_symbols: "iabc, def<Esc>0eaz" => "abcz, def",
e_end: "iabc<Esc>eaz" => "abcz",
e_num_under: "ia0b_c2d, <Esc>0eaz" => "a0b_c2dz, ",
e_double_symbols: "iabc!!def <Esc>0ea.<Esc>ea.<Esc>ea." => "abc.!!.def. ",
e_end_of: "ia b<Esc>0eac" => "a bc",

E: "iabc def<Esc>0Eaz" => "abcz def",
E_not_in_word: "i abc def<Esc>0Eaz" => " abcz def",
E_symbols: "iabc, def<Esc>0Eaz" => "abc,z def",
E_end: "iabc<Esc>Eaz" => "abcz",
E_num_under: "ia0b_c2d, <Esc>0Eaz" => "a0b_c2d,z ",
E_double_symbols: "iabc!!def <Esc>0Ea.<Esc>Ea." => "abc!!def. .",
E_end_of: "ia b<Esc>0Eac" => "a bc",

de_space: "iabc def<Esc>0de" => " def",
de_symbol: "iabc!def<Esc>0de" => "!def",
de_symbol_word: "i(:)abc<Esc>0de" => "abc",
de_empty: "i   <Esc>0de" => "",
de_end: "iab(:) de<Esc>0de" => "(:) de",

dE_space: "iabc def<Esc>0dE" => " def",
dE_symbol: "iabc!def<Esc>0dE" => "",
dE_symbol_word: "i(:)abc<Esc>0dE" => "",
dE_empty: "i   <Esc>0dE" => "",
dE_end: "iab(:) de<Esc>0dE" => " de",

tilde: "ia<Esc>~" => "A",
tilde_double: "iabc<Esc>0~~" => "ABc",
tilde_too_many: "iab<Esc>0~~~" => "Ab",
tilde_invalid: "i(<Esc>~" => "(",
tilde_empty: "~" => "",

dd: "iab (pr )'<Esc>bbdd" => "",
D: "iab (pr )'<Esc>bbD" => "ab (",

g_ignore: "iabc<Esc>gxx" => "ab",

ge_empty: "gei." => ".",
ge_only_space: "i     <Esc>gei." => ".     ",
ge_in_word: "iabcd(:)<Esc>gea." => "abcd.(:)",
ge_whitespace: "iabcd   <Esc>gea." => "abcd.   ",
ge_single_whitespace: "iabcd <Esc>gea." => "abcd. ",
ge_words: "iabcd efgh<Esc>gea." => "abcd. efgh",
ge_one_word: "i   word<Esc>gei." => ".   word",
ge_no_space: "iabcdef<Esc>gei." => ".abcdef",

gE_empty: "gEi." => ".",
gE_only_space: "i     <Esc>gEi." => ".     ",
gE_in_word: "iabcd(:)<Esc>gEa." => "a.bcd(:)",
gE_whitespace: "iabcd   <Esc>gEa." => "abcd.   ",
gE_single_whitespace: "iabcd <Esc>gEa." => "abcd. ",
gE_words: "iabcd efgh<Esc>gEa." => "abcd. efgh",
gE_one_word: "i   word<Esc>gEi." => ".   word",
gE_no_space: "iabcdef<Esc>gEi." => ".abcdef",

cw: "iabc. z<Esc>0cwe" => "e. z",
cW: "iabc. z<Esc>0cWe" => "ez",

cc: "iabc<Esc>cc" => "",
cc_insert: "iabc<Esc>ccdef" => "def",

c0_empty: "c0" => "",
c0_start: "iabc<Esc>0c0d" => "dabc",
c0_middle: "i abc def ghi<Esc>bbc0j" => "jdef ghi",
c0_end: "iabc def <Esc>c0g" => "g ",
c0_insert: "iabc<Esc>c0def" => "defc",

cdollar_empty: "c$" => "",
cdollar_start: "iabc def<Esc>0c$" => "",
cdollar_end: "iabc def<Esc>c$" => "abc de",
cdollar_middle: "iabc def<Esc>hhhhc$" => "ab",
cdollar_insert: "iabc def<Esc>0c$ghi" => "ghi",

C_empty: "C" => "",
C_start: "iabc def<Esc>0C" => "",
C_end: "iabc def<Esc>C" => "abc de",
C_middle: "iabc def<Esc>hhhhC" => "ab",
C_insert: "iabc def<Esc>0Cghi" => "ghi",

ccaret_empty: "c^" => "",
ccaret_end: "i  abc<Esc>c^" => "  c",
ccaret_start: "iabc<Esc>0c^" => "abc",
ccaret_spaces_only: "i    <Esc>c^" => "   ",
ccaret_middle: "i abc def ghi<Esc>bblc^" => " ef ghi",

cb_middle: "iabc def<Esc>cb" => "abc f",
cb_start: "iabc<Esc>0cb" => "abc",
cb_leading_space: "i  abc<Esc>cb" => "  c",
cb_single_char: "i a b<Esc>cb" => " b",
cb_single_char_end: "i a b <Esc>cb" => " a  ",
cb_insert: "iabc def<Esc>cbx" => "abc xf",

cB: "iab  cd ef<Esc>0<Right><Right>cB" => "  cd ef",
cB_space: "i  ab<Esc>0<Right>cB" => " ab",
cB_start: "iab<Esc>0cB" => "ab",

ce_space: "iabc def<Esc>0ce" => " def",
ce_symbol: "iabc!def<Esc>0ce" => "!def",
ce_symbol_word: "i(:)abc<Esc>0ce" => "abc",
ce_empty: "i   <Esc>0ce" => "",
ce_end: "iab(:) de<Esc>0ce" => "(:) de",
ce_insert: "iabc def<Esc>0cex" => "x def",

cE_space: "iabc def<Esc>0cE" => " def",
cE_symbol: "iabc!def<Esc>0cE" => "",
cE_symbol_word: "i(:)abc<Esc>0cE" => "",
cE_empty: "i   <Esc>0cE" => "",
cE_end: "iab(:) de<Esc>0cE" => " de",

cf: "iabcabc<Esc>0cfc" => "abc",
cf_not_found: "iabc<Esc>0cfz" => "abc",
cf_insert: "iabcabc<Esc>0cfcx" => "xabc",

ct: "iabcabc<Esc>0ctc" => "cabc",
ct_not_found: "iabc<Esc>0ctz" => "abc",

cT: "iabcabc<Esc>cTc" => "abcc",
cT_not_found: "iabc<Esc>cTz" => "abc",

cF: "iabcabc<Esc>cFc" => "abc",
cF_not_found: "iabc<Esc>cFz" => "abc",

gUU: "iabc<Esc>hgUUiz" => "zABC",
gUw: "iabc.def<Esc>0lgUwiz" => "azBC.def",
gU0: "iabc.def<Esc>gegU0iz" => "zABC.def",
gUf: "iabc.def<Esc>0gUfe" => "ABC.DEf",
gU_fail: "iabc.def<Esc>0gUfz" => "abc.def",
gUb: "iabcde.f<Esc>FcgUbiz" => "zABcde.f",

guu: "iABC<Esc>hguuiZ" => "Zabc",
guW: "iABC.DEF<Esc>0lguwiZ" => "AZbc.DEF",
gu0: "iABC.DEF<Esc>gegu0iZ" => "Zabc.DEF",
guF: "iABC.DEF<Esc>0gufE" => "abc.deF",
gu_fail: "iABC.DEF<Esc>0gufZ" => "ABC.DEF",
guB: "iABCDE.F<Esc>FCgubiZ" => "ZabCDE.F",

g_tilde_w: "iaBc!d<Esc>0lg~wiz" => "azbC!d",
g_tilde_0: "iaBc!D<Esc>g~0iz" => "zAbC!D",
g_tilde_fail: "iaBc!<Esc>g~Fzaz" => "aBc!z",

dp: "iabc<Esc>0dfb$p" => "cab",
dd_: "iabc<Esc>dd" => "",
ddp: "iabc<Esc>ddp" => "abc",
dP: "iabc<Esc>0dfb$P" => "abc",

y: "iabcdef<Esc>0yfcP" => "abcabcdef",
y_fail: "iabcdef<Esc>0yfzP" => "abcdef",
Y: "iabcdef<Esc>FdY0P" => "defabcdef",

dot_i: "iabc def<Esc>." => "abc deabc deff",
dot_dw: "iabc def ghi<Esc>0dw.." => "",
dot_move: "iabc def ghi<Esc>0dww." => "def ",
dot_invalid: "iabcdef<Esc>0dfc." => "def",
dot_replace: "iabcdef<Esc>0Rklm<Esc>." =>"klklmf",

nb_f: "iabc defc ghic<Esc>02fca." => "abc defc. ghic",
nb_f_nb: "iab1 def1 ghi1<Esc>03f1i." => "ab1 def1 ghi.1",
nb_multiple_f: "icccccccccccccccccccc<Esc>013fci." => "ccccccccccccc.ccccccc",
nb_too_many: "iabc abc<Esc>09999wa." => "abc abc.",

);

buffer_tests!(replace,

simple: "ihell(!:zd:tqs. qflk z<Esc>F(Ro world<Esc>lD" => "hello world",
end: "iabcdef<Esc>0Rhello world" => "hello world",
end_r: "iabcdef<Esc>0RHello WORLD!<Esc>r." => "Hello WORLD.",
end_lr: "iabcdef<Esc>0Rhello world!<Esc>lr." => "hello world.",
invalid: "R<C-a><S-CR><CR>" => "",
empty: "Rabc" => "abc",
arrows: "iabcdef<Esc>0Rghi<Left>j<Right>k<Right>l" => "ghjdkfl",
esc_replace: "iabcdef<Esc>0Rklm<Esc>Rklm" =>"klklmf",
bs: "iabcdef<Esc>0fdRghi<BS><BS><BS><BS><BS><BS>jk" => "jkcdef",
bs_start: "iabc<Esc>0Rdef<BS><BS><BS><BS>." => ".bc",
bs_too_far: "iabc<Esc>0Rdefghi<BS><BS>k" => "defgk",
arrows_bs: "iabc<Esc>0Rx<Right>y<BS><BS><BS><BS>" => "xbc",

);
