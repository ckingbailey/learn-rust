use std::io::{stdin, stdout, Read, Write};

fn main() {
    let nth_gift = [ // TODO: This should be stored in a data file
        [
            "first", // QUESTION: Is there a library that will translate numbers to their adjectives?
            "partridge in a pear tree"
        ],
        [
            "second",
            "turtle doves"
        ],
        [
            "third",
            "french hens"
        ],
        [
            "fourth",
            "calling birds"
        ],
        [
            "fifth",
            "gold rings"
        ],
        [
            "sixth",
            "geese a-laying"
        ],
        [
            "seventh",
            "swans a-swimming"
        ],
        [
            "eigth",
            "maids a-milking"
        ],
        [
            "ninth",
            "ladies dancing"
        ],
        [
            "tenth",
            "lords a-leaping"
        ],
        [
            "eleventh",
            "pipers piping"
        ],
        [
            "twelfth",
            "drummers drumming"
        ]
    ];

    for (i, elem) in nth_gift.iter().enumerate() {
        println!(
            "On the {} day of Christmas\nMy true love gave to me\n{} {}",
            elem[0],
            i + 1,
            elem[1]
        );
        if i > 0 {
            for j in (0..i).rev() {
                println!("{} {}", j + 1, nth_gift[j][1]);
            }
        }
        if i < nth_gift.len() - 1 {
            pause();
        }
    }
}

fn pause() {
    let mut stdout = stdout();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}
