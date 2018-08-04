pub struct Rect {
    pub min: Point,
    pub max: Point,
}

pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub static FONT_MAP: [(f32, Rect); 94] = [
    (
        1.0,
        Rect {
            min: Point {
                x: 0.96484375,
                y: 0.31640625
            },
            max: Point {
                x: 0.98828125,
                y: 0.43359375
            }
        }
    ),
    (
        -17.0,
        Rect {
            min: Point {
                x: 0.078125,
                y: 0.90625
            },
            max: Point {
                x: 0.12890625,
                y: 0.953125
            }
        }
    ),
    (
        0.0,
        Rect {
            min: Point {
                x: 0.22265625,
                y: 0.6875
            },
            max: Point {
                x: 0.3046875,
                y: 0.796875
            }
        }
    ),
    (
        7.0,
        Rect {
            min: Point {
                x: 0.15234375,
                y: 0.00390625
            },
            max: Point {
                x: 0.23046875,
                y: 0.16796875
            }
        }
    ),
    (
        2.0,
        Rect {
            min: Point {
                x: 0.3203125,
                y: 0.1796875
            },
            max: Point {
                x: 0.4453125,
                y: 0.3046875
            }
        }
    ),
    (
        1.0,
        Rect {
            min: Point {
                x: 0.78125,
                y: 0.1796875
            },
            max: Point {
                x: 0.8828125,
                y: 0.30078125
            }
        }
    ),
    (
        -17.0,
        Rect {
            min: Point {
                x: 0.13671875,
                y: 0.90625
            },
            max: Point {
                x: 0.15625,
                y: 0.953125
            }
        }
    ),
    (
        7.0,
        Rect {
            min: Point {
                x: 0.00390625,
                y: 0.00390625
            },
            max: Point {
                x: 0.046875,
                y: 0.171875
            }
        }
    ),
    (
        7.0,
        Rect {
            min: Point {
                x: 0.0546875,
                y: 0.00390625
            },
            max: Point {
                x: 0.09765625,
                y: 0.171875
            }
        }
    ),
    (
        -14.0,
        Rect {
            min: Point {
                x: 0.6953125,
                y: 0.80859375
            },
            max: Point {
                x: 0.765625,
                y: 0.875
            }
        }
    ),
    (
        -6.0,
        Rect {
            min: Point {
                x: 0.7734375,
                y: 0.80859375
            },
            max: Point {
                x: 0.83984375,
                y: 0.87109375
            }
        }
    ),
    (
        7.0,
        Rect {
            min: Point {
                x: 0.93359375,
                y: 0.80859375
            },
            max: Point {
                x: 0.9609375,
                y: 0.859375
            }
        }
    ),
    (
        -11.0,
        Rect {
            min: Point {
                x: 0.32421875,
                y: 0.90625
            },
            max: Point {
                x: 0.375,
                y: 0.921875
            }
        }
    ),
    (
        1.0,
        Rect {
            min: Point {
                x: 0.21484375,
                y: 0.90625
            },
            max: Point {
                x: 0.23828125,
                y: 0.93359375
            }
        }
    ),
    (
        5.0,
        Rect {
            min: Point {
                x: 0.65234375,
                y: 0.00390625
            },
            max: Point {
                x: 0.70703125,
                y: 0.15625
            }
        }
    ),
    (
        1.0,
        Rect {
            min: Point {
                x: 0.79296875,
                y: 0.31640625
            },
            max: Point {
                x: 0.8671875,
                y: 0.43359375
            }
        }
    ),
    (
        0.0,
        Rect {
            min: Point {
                x: 0.1640625,
                y: 0.6875
            },
            max: Point {
                x: 0.21484375,
                y: 0.796875
            }
        }
    ),
    (
        0.0,
        Rect {
            min: Point {
                x: 0.2578125,
                y: 0.4453125
            },
            max: Point {
                x: 0.328125,
                y: 0.55859375
            }
        }
    ),
    (
        1.0,
        Rect {
            min: Point {
                x: 0.62890625,
                y: 0.31640625
            },
            max: Point {
                x: 0.703125,
                y: 0.43359375
            }
        }
    ),
    (
        0.0,
        Rect {
            min: Point {
                x: 0.00390625,
                y: 0.56640625
            },
            max: Point {
                x: 0.08203125,
                y: 0.6796875
            }
        }
    ),
    (
        1.0,
        Rect {
            min: Point {
                x: 0.18359375,
                y: 0.56640625
            },
            max: Point {
                x: 0.2578125,
                y: 0.6796875
            }
        }
    ),
    (
        1.0,
        Rect {
            min: Point {
                x: 0.7109375,
                y: 0.31640625
            },
            max: Point {
                x: 0.78515625,
                y: 0.43359375
            }
        }
    ),
    (
        1.0,
        Rect {
            min: Point {
                x: 0.08984375,
                y: 0.6875
            },
            max: Point {
                x: 0.15625,
                y: 0.80078125
            }
        }
    ),
    (
        1.0,
        Rect {
            min: Point {
                x: 0.875,
                y: 0.31640625
            },
            max: Point {
                x: 0.95703125,
                y: 0.43359375
            }
        }
    ),
    (
        1.0,
        Rect {
            min: Point {
                x: 0.45703125,
                y: 0.31640625
            },
            max: Point {
                x: 0.53125,
                y: 0.43359375
            }
        }
    ),
    (
        1.0,
        Rect {
            min: Point {
                x: 0.23046875,
                y: 0.80859375
            },
            max: Point {
                x: 0.25390625,
                y: 0.89453125
            }
        }
    ),
    (
        7.0,
        Rect {
            min: Point {
                x: 0.3125,
                y: 0.6875
            },
            max: Point {
                x: 0.33984375,
                y: 0.796875
            }
        }
    ),
    (
        -4.0,
        Rect {
            min: Point {
                x: 0.546875,
                y: 0.80859375
            },
            max: Point {
                x: 0.61328125,
                y: 0.8828125
            }
        }
    ),
    (
        -8.0,
        Rect {
            min: Point {
                x: 0.00390625,
                y: 0.90625
            },
            max: Point {
                x: 0.0703125,
                y: 0.953125
            }
        }
    ),
    (
        -4.0,
        Rect {
            min: Point {
                x: 0.62109375,
                y: 0.80859375
            },
            max: Point {
                x: 0.6875,
                y: 0.8828125
            }
        }
    ),
    (
        1.0,
        Rect {
            min: Point {
                x: 0.29296875,
                y: 0.31640625
            },
            max: Point {
                x: 0.359375,
                y: 0.4375
            }
        }
    ),
    (
        9.0,
        Rect {
            min: Point {
                x: 0.390625,
                y: 0.00390625
            },
            max: Point {
                x: 0.5390625,
                y: 0.15625
            }
        }
    ),
    (
        0.0,
        Rect {
            min: Point {
                x: 0.7109375,
                y: 0.56640625
            },
            max: Point {
                x: 0.8046875,
                y: 0.6796875
            }
        }
    ),
    (
        0.0,
        Rect {
            min: Point {
                x: 0.00390625,
                y: 0.6875
            },
            max: Point {
                x: 0.08203125,
                y: 0.80078125
            }
        }
    ),
    (
        1.0,
        Rect {
            min: Point {
                x: 0.109375,
                y: 0.31640625
            },
            max: Point {
                x: 0.19140625,
                y: 0.4375
            }
        }
    ),
    (
        0.0,
        Rect {
            min: Point {
                x: 0.88671875,
                y: 0.56640625
            },
            max: Point {
                x: 0.96875,
                y: 0.6796875
            }
        }
    ),
    (
        0.0,
        Rect {
            min: Point {
                x: 0.8125,
                y: 0.56640625
            },
            max: Point {
                x: 0.87890625,
                y: 0.6796875
            }
        }
    ),
    (
        0.0,
        Rect {
            min: Point {
                x: 0.36328125,
                y: 0.56640625
            },
            max: Point {
                x: 0.42578125,
                y: 0.6796875
            }
        }
    ),
    (
        1.0,
        Rect {
            min: Point {
                x: 0.19921875,
                y: 0.31640625
            },
            max: Point {
                x: 0.28515625,
                y: 0.4375
            }
        }
    ),
    (
        0.0,
        Rect {
            min: Point {
                x: 0.16796875,
                y: 0.4453125
            },
            max: Point {
                x: 0.25,
                y: 0.55859375
            }
        }
    ),
    (
        0.0,
        Rect {
            min: Point {
                x: 0.00390625,
                y: 0.4453125
            },
            max: Point {
                x: 0.0234375,
                y: 0.55859375
            }
        }
    ),
    (
        6.0,
        Rect {
            min: Point {
                x: 0.8515625,
                y: 0.00390625
            },
            max: Point {
                x: 0.88671875,
                y: 0.140625
            }
        }
    ),
    (
        0.0,
        Rect {
            min: Point {
                x: 0.3359375,
                y: 0.4453125
            },
            max: Point {
                x: 0.41796875,
                y: 0.55859375
            }
        }
    ),
    (
        0.0,
        Rect {
            min: Point {
                x: 0.42578125,
                y: 0.4453125
            },
            max: Point {
                x: 0.48828125,
                y: 0.55859375
            }
        }
    ),
    (
        0.0,
        Rect {
            min: Point {
                x: 0.49609375,
                y: 0.4453125
            },
            max: Point {
                x: 0.60546875,
                y: 0.55859375
            }
        }
    ),
    (
        0.0,
        Rect {
            min: Point {
                x: 0.61328125,
                y: 0.4453125
            },
            max: Point {
                x: 0.6953125,
                y: 0.55859375
            }
        }
    ),
    (
        1.0,
        Rect {
            min: Point {
                x: 0.00390625,
                y: 0.31640625
            },
            max: Point {
                x: 0.1015625,
                y: 0.4375
            }
        }
    ),
    (
        0.0,
        Rect {
            min: Point {
                x: 0.703125,
                y: 0.4453125
            },
            max: Point {
                x: 0.77734375,
                y: 0.55859375
            }
        }
    ),
    (
        5.0,
        Rect {
            min: Point {
                x: 0.7421875,
                y: 0.00390625
            },
            max: Point {
                x: 0.84375,
                y: 0.140625
            }
        }
    ),
    (
        0.0,
        Rect {
            min: Point {
                x: 0.85546875,
                y: 0.4453125
            },
            max: Point {
                x: 0.9375,
                y: 0.55859375
            }
        }
    ),
    (
        1.0,
        Rect {
            min: Point {
                x: 0.890625,
                y: 0.1796875
            },
            max: Point {
                x: 0.97265625,
                y: 0.30078125
            }
        }
    ),
    (
        0.0,
        Rect {
            min: Point {
                x: 0.08984375,
                y: 0.56640625
            },
            max: Point {
                x: 0.17578125,
                y: 0.6796875
            }
        }
    ),
    (
        1.0,
        Rect {
            min: Point {
                x: 0.5390625,
                y: 0.31640625
            },
            max: Point {
                x: 0.62109375,
                y: 0.43359375
            }
        }
    ),
    (
        0.0,
        Rect {
            min: Point {
                x: 0.265625,
                y: 0.56640625
            },
            max: Point {
                x: 0.35546875,
                y: 0.6796875
            }
        }
    ),
    (
        0.0,
        Rect {
            min: Point {
                x: 0.03125,
                y: 0.4453125
            },
            max: Point {
                x: 0.16015625,
                y: 0.55859375
            }
        }
    ),
    (
        0.0,
        Rect {
            min: Point {
                x: 0.43359375,
                y: 0.56640625
            },
            max: Point {
                x: 0.5234375,
                y: 0.6796875
            }
        }
    ),
    (
        0.0,
        Rect {
            min: Point {
                x: 0.53125,
                y: 0.56640625
            },
            max: Point {
                x: 0.62109375,
                y: 0.6796875
            }
        }
    ),
    (
        0.0,
        Rect {
            min: Point {
                x: 0.62890625,
                y: 0.56640625
            },
            max: Point {
                x: 0.703125,
                y: 0.6796875
            }
        }
    ),
    (
        5.0,
        Rect {
            min: Point {
                x: 0.34375,
                y: 0.00390625
            },
            max: Point {
                x: 0.3828125,
                y: 0.15625
            }
        }
    ),
    (
        5.0,
        Rect {
            min: Point {
                x: 0.546875,
                y: 0.00390625
            },
            max: Point {
                x: 0.6015625,
                y: 0.15625
            }
        }
    ),
    (
        5.0,
        Rect {
            min: Point {
                x: 0.609375,
                y: 0.00390625
            },
            max: Point {
                x: 0.64453125,
                y: 0.15625
            }
        }
    ),
    (
        -21.0,
        Rect {
            min: Point {
                x: 0.84765625,
                y: 0.80859375
            },
            max: Point {
                x: 0.92578125,
                y: 0.86328125
            }
        }
    ),
    (
        6.0,
        Rect {
            min: Point {
                x: 0.3828125,
                y: 0.90625
            },
            max: Point {
                x: 0.46484375,
                y: 0.921875
            }
        }
    ),
    (
        -26.0,
        Rect {
            min: Point {
                x: 0.1640625,
                y: 0.90625
            },
            max: Point {
                x: 0.20703125,
                y: 0.9375
            }
        }
    ),
    (
        1.0,
        Rect {
            min: Point {
                x: 0.34765625,
                y: 0.6875
            },
            max: Point {
                x: 0.42578125,
                y: 0.78125
            }
        }
    ),
    (
        1.0,
        Rect {
            min: Point {
                x: 0.16015625,
                y: 0.1796875
            },
            max: Point {
                x: 0.23828125,
                y: 0.30859375
            }
        }
    ),
    (
        1.0,
        Rect {
            min: Point {
                x: 0.43359375,
                y: 0.6875
            },
            max: Point {
                x: 0.5,
                y: 0.78125
            }
        }
    ),
    (
        1.0,
        Rect {
            min: Point {
                x: 0.00390625,
                y: 0.1796875
            },
            max: Point {
                x: 0.078125,
                y: 0.30859375
            }
        }
    ),
    (
        1.0,
        Rect {
            min: Point {
                x: 0.59375,
                y: 0.6875
            },
            max: Point {
                x: 0.66796875,
                y: 0.78125
            }
        }
    ),
    (
        0.0,
        Rect {
            min: Point {
                x: 0.24609375,
                y: 0.1796875
            },
            max: Point {
                x: 0.3125,
                y: 0.3046875
            }
        }
    ),
    (
        9.0,
        Rect {
            min: Point {
                x: 0.89453125,
                y: 0.00390625
            },
            max: Point {
                x: 0.9765625,
                y: 0.13671875
            }
        }
    ),
    (
        0.0,
        Rect {
            min: Point {
                x: 0.453125,
                y: 0.1796875
            },
            max: Point {
                x: 0.5234375,
                y: 0.3046875
            }
        }
    ),
    (
        0.0,
        Rect {
            min: Point {
                x: 0.12890625,
                y: 0.1796875
            },
            max: Point {
                x: 0.15234375,
                y: 0.30859375
            }
        }
    ),
    (
        9.0,
        Rect {
            min: Point {
                x: 0.10546875,
                y: 0.00390625
            },
            max: Point {
                x: 0.14453125,
                y: 0.16796875
            }
        }
    ),
    (
        0.0,
        Rect {
            min: Point {
                x: 0.53125,
                y: 0.1796875
            },
            max: Point {
                x: 0.60546875,
                y: 0.3046875
            }
        }
    ),
    (
        1.0,
        Rect {
            min: Point {
                x: 0.0859375,
                y: 0.1796875
            },
            max: Point {
                x: 0.12109375,
                y: 0.30859375
            }
        }
    ),
    (
        0.0,
        Rect {
            min: Point {
                x: 0.83203125,
                y: 0.6875
            },
            max: Point {
                x: 0.9453125,
                y: 0.77734375
            }
        }
    ),
    (
        0.0,
        Rect {
            min: Point {
                x: 0.75390625,
                y: 0.6875
            },
            max: Point {
                x: 0.82421875,
                y: 0.77734375
            }
        }
    ),
    (
        1.0,
        Rect {
            min: Point {
                x: 0.5078125,
                y: 0.6875
            },
            max: Point {
                x: 0.5859375,
                y: 0.78125
            }
        }
    ),
    (
        9.0,
        Rect {
            min: Point {
                x: 0.61328125,
                y: 0.1796875
            },
            max: Point {
                x: 0.69140625,
                y: 0.3046875
            }
        }
    ),
    (
        9.0,
        Rect {
            min: Point {
                x: 0.69921875,
                y: 0.1796875
            },
            max: Point {
                x: 0.7734375,
                y: 0.3046875
            }
        }
    ),
    (
        0.0,
        Rect {
            min: Point {
                x: 0.08203125,
                y: 0.80859375
            },
            max: Point {
                x: 0.1328125,
                y: 0.8984375
            }
        }
    ),
    (
        1.0,
        Rect {
            min: Point {
                x: 0.67578125,
                y: 0.6875
            },
            max: Point {
                x: 0.74609375,
                y: 0.78125
            }
        }
    ),
    (
        1.0,
        Rect {
            min: Point {
                x: 0.78515625,
                y: 0.4453125
            },
            max: Point {
                x: 0.84765625,
                y: 0.55859375
            }
        }
    ),
    (
        1.0,
        Rect {
            min: Point {
                x: 0.00390625,
                y: 0.80859375
            },
            max: Point {
                x: 0.07421875,
                y: 0.8984375
            }
        }
    ),
    (
        0.0,
        Rect {
            min: Point {
                x: 0.26171875,
                y: 0.80859375
            },
            max: Point {
                x: 0.34375,
                y: 0.89453125
            }
        }
    ),
    (
        0.0,
        Rect {
            min: Point {
                x: 0.3515625,
                y: 0.80859375
            },
            max: Point {
                x: 0.46875,
                y: 0.89453125
            }
        }
    ),
    (
        0.0,
        Rect {
            min: Point {
                x: 0.140625,
                y: 0.80859375
            },
            max: Point {
                x: 0.22265625,
                y: 0.89453125
            }
        }
    ),
    (
        9.0,
        Rect {
            min: Point {
                x: 0.3671875,
                y: 0.31640625
            },
            max: Point {
                x: 0.44921875,
                y: 0.4375
            }
        }
    ),
    (
        0.0,
        Rect {
            min: Point {
                x: 0.4765625,
                y: 0.80859375
            },
            max: Point {
                x: 0.5390625,
                y: 0.89453125
            }
        }
    ),
    (
        6.0,
        Rect {
            min: Point {
                x: 0.23828125,
                y: 0.00390625
            },
            max: Point {
                x: 0.28125,
                y: 0.1640625
            }
        }
    ),
    (
        5.0,
        Rect {
            min: Point {
                x: 0.71484375,
                y: 0.00390625
            },
            max: Point {
                x: 0.734375,
                y: 0.15625
            }
        }
    ),
    (
        6.0,
        Rect {
            min: Point {
                x: 0.2890625,
                y: 0.00390625
            },
            max: Point {
                x: 0.3359375,
                y: 0.1640625
            }
        }
    ),
    (
        -10.0,
        Rect {
            min: Point {
                x: 0.24609375,
                y: 0.90625
            },
            max: Point {
                x: 0.31640625,
                y: 0.9296875
            }
        }
    )
];