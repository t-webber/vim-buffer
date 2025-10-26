use vim_buffer::Buffer;

macro_rules! buffer_tests {
    ($($name:ident: $keymaps:expr => $output:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let mut buffer = Buffer::default();
                buffer.update_from_string($keymaps).unwrap();
                assert_eq!(buffer.as_content(), $output);
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
    );
