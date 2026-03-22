mod common;

buffer_tests!(

backspace: "ia<BS><BS>bcd<BS>" => "bc",
arrows: "iabc<Left>d<Right>e" => "abdce",
right_end: "iabc<Right>d" => "abcd",
left_start: "iabc<Esc>I<Left>d" => "dabc",

);
