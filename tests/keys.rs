use vim_buffer::Buffer;

macro_rules! buffer_tests {
    ($($name:ident: $keymaps:literal => $output:literal,)*) => {
        $(
            #[test]
            fn $name() {
                let mut buffer = Buffer::default();
                buffer.update_from_string($keymaps).unwrap();
                assert_eq!(buffer.as_content(), $output, "Keys: \x1b[35m{}\x1b[0m", $keymaps);
            }
        )*
    };
}

buffer_tests!(

insert_backspace: "ia<BS><BS>bcd<BS>" => "bc",
normal_backspace: "ia<Esc><BS><BS>ibcd<Esc><BS>ie" => "becda",
normal_a: "ia<Esc>ab" => "ab",
normal_i: "ia<Esc>ib" => "ba",
normal_cap_i: "i  ab<Esc>Ic" => "  cab",
normal_cap_i_no_space: "iab<Esc>Ic" => "cab",
normal_cap_i_empty_line: "i   <Esc>Ia" => "   a",
normal_cap_a: "iabc<Esc>Ad" => "abcd",
normal_cap_a_empty_line: "i   <Esc>Aa" => "   a",
insert_arrows: "iabc<Left>d<Right>e" => "abdce",
normal_arrows: "iabc<Esc><Left>id<Esc><Right>ae" => "adbec",
normal_h_l: "iabc<Esc>hid<Esc>lae" => "adbec",
normal_0: "i  abc<Esc>0id" => "d  abc",
normal_dollar: "i abc <Esc>0id<Esc>$ae" => "d abc e",
normal_caret: "i  abc<Esc>^id" => "  dabc",
normal_f_not_found: "iabc<Esc>0<Right>fzid" => "adbc",
normal_t_not_found: "iabc<Esc>0<Right>tzid" => "adbc",
normal_f: "iabcabc<Esc>0fcad<Esc>fcae" => "abcdabce",
normal_t: "iabcabc<Esc>0tcad<Esc>ltcae" => "abdcabec",
normal_cap_f_not_found: "iabc<Esc><Left>Fzad" => "abdc",
normal_cap_t_not_found: "iabc<Esc><Left>Tzad" => "abdc",
normal_cap_f: "iabcabc<Esc>Faad<Esc>hFaae" => "aebcadbc",
normal_cap_t: "iabcabc<Esc>Taad<Esc>hhTaae" => "abecabdc",
normal_x: "iabcd<Esc>x<Left>x" => "ac",
normal_x_empty: "x" => "",
normal_cap_x: "iabcd<Esc>X<Left>X" => "bd",
normal_s: "iabcd<Esc>se<Esc>hsf" => "abfe",
normal_cap_s: "iabcdef<Esc>hhhSghij" => "ghij",
normal_r: "iabcd<Esc>Fbre" => "aecd",

normal_w: "iabc   def::(Bl<Esc>0wa.<Esc>lwa.<Esc>lwa.<Esc>lwa." => "abc   d.ef:.:(B.l.",
normal_w_end_space: "iab <Esc>0wa." => "ab .",
normal_w_end_symbol: "i)))<Esc>0wa." => "))).",
normal_cap_w: "i  ab  cd<Esc>0Wiz<Esc>lWiz<Esc>lWaz" => "  zab  zcdz",
normal_cap_w_empty: "i    <Esc>0Waz" => "    z",

normal_b: "i)))) ef<Esc>biz<Esc>biz" => "z)))) zef",
normal_b_word: "iab<Esc>biz" => "zab",
normal_b_word_space: "iab <Esc>biz" => "zab ",
normal_b_leading_space: "i ab <Esc>biz" => " zab ",
normal_b_symbols: "iab(:) <Esc>biz" => "abz(:) ",
normal_b_leading_symbols: "i(:)<Esc>biz" => "z(:)",
normal_b_spaces: "i  <Esc>biz" => "z  ",

normal_cap_b_empty: "i   <Esc>Biz" => "z   ",
normal_cap_b_words: "iab cd  <Esc>Biz<Esc>Biy" => "yab zcd  ",
normal_cap_b_symbols: "iab(:)cd  <Esc>Biz<Esc>Biy" => "yzab(:)cd  ",

);
