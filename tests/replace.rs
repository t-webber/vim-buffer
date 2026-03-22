mod common;

buffer_tests!(

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
