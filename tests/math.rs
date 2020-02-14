#![cfg(test)]

#[macro_use]
mod macros;

test!(
    percentage_decimal,
    "a {\n  color: percentage(0.2);\n}\n",
    "a {\n  color: 20%;\n}\n"
);
test!(
    percentage_division,
    "a {\n  color: percentage(100px / 50px);\n}\n",
    "a {\n  color: 200%;\n}\n"
);
test!(
    integer_division,
    "a {\n  color: percentage(2);\n}\n",
    "a {\n  color: 200%;\n}\n"
);
test!(
    rounds_down,
    "a {\n  color: round(10.4px);\n}\n",
    "a {\n  color: 10px;\n}\n"
);
test!(
    rounds_up,
    "a {\n  color: round(10.6px);\n}\n",
    "a {\n  color: 11px;\n}\n"
);
test!(
    floor_below_pt_5,
    "a {\n  color: floor(10.4px);\n}\n",
    "a {\n  color: 10px;\n}\n"
);
test!(
    floor_above_pt_5,
    "a {\n  color: floor(10.6px);\n}\n",
    "a {\n  color: 10px;\n}\n"
);
test!(
    ceil_below_pt_5,
    "a {\n  color: ceil(10.4px);\n}\n",
    "a {\n  color: 11px;\n}\n"
);
test!(
    ceil_above_pt_5,
    "a {\n  color: ceil(10.6px);\n}\n",
    "a {\n  color: 11px;\n}\n"
);
test!(
    abs_positive,
    "a {\n  color: abs(10);\n}\n",
    "a {\n  color: 10;\n}\n"
);
test!(
    abs_negative,
    "a {\n  color: abs(-10);\n}\n",
    "a {\n  color: 10;\n}\n"
);
test!(
    abs_unit,
    "a {\n  color: abs(-10px);\n}\n",
    "a {\n  color: 10px;\n}\n"
);