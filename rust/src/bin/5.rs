use std::cmp::max;
use std::collections::HashMap;

#[derive(Clone)]
struct Pos {
    x: isize,
    y: isize,
}

#[derive(Clone)]
struct Line {
    from: Pos,
    to: Pos,
}

fn main() {
    println!("part one: {}", part_one(LINES.to_vec()));
    println!("part two: {}", part_two(LINES.to_vec()));
}

fn part_one(lines: Vec<Line>) -> usize {
    let washed = lines
        .into_iter()
        .filter(|line| line.from.y == line.to.y || line.from.x == line.to.x)
        .collect();

    find_overlaps(washed)
}

fn part_two(lines: Vec<Line>) -> usize {
    find_overlaps(lines)
}

fn find_overlaps(lines: Vec<Line>) -> usize {
    let mut points = HashMap::new();

    for line in lines {
        let dx = (line.to.x - line.from.x).signum();
        let dy = (line.to.y - line.from.y).signum();
        let steps = max(
            (line.from.y - line.to.y).abs(),
            (line.from.x - line.to.x).abs(),
        );

        for n in 0..steps + 1 {
            let x = line.from.x + n * dx;
            let y = line.from.y + n * dy;
            *points.entry((x, y)).or_insert(0) += 1;
        }
    }

    points.values().filter(|&&x| x > 1).count()
}

mod tests {
    #[test]
    fn test_part_one() {
        assert_eq!(7142, super::part_one(super::LINES.to_vec()));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(20012, super::part_two(super::LINES.to_vec()));
    }
}

const LINES: [Line; 500] = [
    Line {
        from: Pos { x: 223, y: 805 },
        to: Pos { x: 223, y: 548 },
    },
    Line {
        from: Pos { x: 609, y: 164 },
        to: Pos { x: 609, y: 503 },
    },
    Line {
        from: Pos { x: 461, y: 552 },
        to: Pos { x: 796, y: 552 },
    },
    Line {
        from: Pos { x: 207, y: 361 },
        to: Pos { x: 207, y: 34 },
    },
    Line {
        from: Pos { x: 503, y: 879 },
        to: Pos { x: 503, y: 946 },
    },
    Line {
        from: Pos { x: 937, y: 52 },
        to: Pos { x: 937, y: 268 },
    },
    Line {
        from: Pos { x: 560, y: 652 },
        to: Pos { x: 118, y: 652 },
    },
    Line {
        from: Pos { x: 771, y: 103 },
        to: Pos { x: 85, y: 789 },
    },
    Line {
        from: Pos { x: 119, y: 156 },
        to: Pos { x: 947, y: 984 },
    },
    Line {
        from: Pos { x: 356, y: 634 },
        to: Pos { x: 607, y: 634 },
    },
    Line {
        from: Pos { x: 348, y: 812 },
        to: Pos { x: 873, y: 287 },
    },
    Line {
        from: Pos { x: 409, y: 490 },
        to: Pos { x: 726, y: 490 },
    },
    Line {
        from: Pos { x: 298, y: 790 },
        to: Pos { x: 298, y: 454 },
    },
    Line {
        from: Pos { x: 407, y: 543 },
        to: Pos { x: 820, y: 130 },
    },
    Line {
        from: Pos { x: 206, y: 89 },
        to: Pos { x: 591, y: 89 },
    },
    Line {
        from: Pos { x: 164, y: 709 },
        to: Pos { x: 976, y: 709 },
    },
    Line {
        from: Pos { x: 208, y: 921 },
        to: Pos { x: 208, y: 131 },
    },
    Line {
        from: Pos { x: 515, y: 209 },
        to: Pos { x: 515, y: 745 },
    },
    Line {
        from: Pos { x: 876, y: 639 },
        to: Pos { x: 281, y: 44 },
    },
    Line {
        from: Pos { x: 270, y: 453 },
        to: Pos { x: 727, y: 910 },
    },
    Line {
        from: Pos { x: 190, y: 417 },
        to: Pos { x: 190, y: 755 },
    },
    Line {
        from: Pos { x: 522, y: 726 },
        to: Pos { x: 903, y: 726 },
    },
    Line {
        from: Pos { x: 390, y: 651 },
        to: Pos { x: 603, y: 864 },
    },
    Line {
        from: Pos { x: 707, y: 549 },
        to: Pos { x: 926, y: 330 },
    },
    Line {
        from: Pos { x: 471, y: 869 },
        to: Pos { x: 471, y: 561 },
    },
    Line {
        from: Pos { x: 970, y: 735 },
        to: Pos { x: 401, y: 735 },
    },
    Line {
        from: Pos { x: 612, y: 624 },
        to: Pos { x: 612, y: 88 },
    },
    Line {
        from: Pos { x: 844, y: 879 },
        to: Pos { x: 844, y: 453 },
    },
    Line {
        from: Pos { x: 400, y: 38 },
        to: Pos { x: 400, y: 350 },
    },
    Line {
        from: Pos { x: 832, y: 225 },
        to: Pos { x: 984, y: 225 },
    },
    Line {
        from: Pos { x: 971, y: 642 },
        to: Pos { x: 42, y: 642 },
    },
    Line {
        from: Pos { x: 70, y: 862 },
        to: Pos { x: 447, y: 485 },
    },
    Line {
        from: Pos { x: 183, y: 79 },
        to: Pos { x: 183, y: 708 },
    },
    Line {
        from: Pos { x: 598, y: 700 },
        to: Pos { x: 598, y: 287 },
    },
    Line {
        from: Pos { x: 635, y: 195 },
        to: Pos { x: 39, y: 195 },
    },
    Line {
        from: Pos { x: 587, y: 362 },
        to: Pos { x: 349, y: 362 },
    },
    Line {
        from: Pos { x: 108, y: 88 },
        to: Pos { x: 965, y: 945 },
    },
    Line {
        from: Pos { x: 700, y: 299 },
        to: Pos { x: 165, y: 299 },
    },
    Line {
        from: Pos { x: 295, y: 824 },
        to: Pos { x: 785, y: 334 },
    },
    Line {
        from: Pos { x: 211, y: 284 },
        to: Pos { x: 390, y: 105 },
    },
    Line {
        from: Pos { x: 288, y: 326 },
        to: Pos { x: 672, y: 710 },
    },
    Line {
        from: Pos { x: 595, y: 231 },
        to: Pos { x: 595, y: 679 },
    },
    Line {
        from: Pos { x: 671, y: 576 },
        to: Pos { x: 813, y: 718 },
    },
    Line {
        from: Pos { x: 14, y: 845 },
        to: Pos { x: 784, y: 75 },
    },
    Line {
        from: Pos { x: 700, y: 129 },
        to: Pos { x: 43, y: 129 },
    },
    Line {
        from: Pos { x: 83, y: 913 },
        to: Pos { x: 889, y: 107 },
    },
    Line {
        from: Pos { x: 830, y: 596 },
        to: Pos { x: 322, y: 596 },
    },
    Line {
        from: Pos { x: 391, y: 450 },
        to: Pos { x: 391, y: 779 },
    },
    Line {
        from: Pos { x: 384, y: 32 },
        to: Pos { x: 384, y: 430 },
    },
    Line {
        from: Pos { x: 311, y: 948 },
        to: Pos { x: 938, y: 321 },
    },
    Line {
        from: Pos { x: 460, y: 288 },
        to: Pos { x: 460, y: 392 },
    },
    Line {
        from: Pos { x: 924, y: 602 },
        to: Pos { x: 924, y: 595 },
    },
    Line {
        from: Pos { x: 703, y: 458 },
        to: Pos { x: 703, y: 475 },
    },
    Line {
        from: Pos { x: 335, y: 953 },
        to: Pos { x: 335, y: 195 },
    },
    Line {
        from: Pos { x: 692, y: 314 },
        to: Pos { x: 927, y: 314 },
    },
    Line {
        from: Pos { x: 131, y: 433 },
        to: Pos { x: 131, y: 737 },
    },
    Line {
        from: Pos { x: 590, y: 771 },
        to: Pos { x: 965, y: 771 },
    },
    Line {
        from: Pos { x: 650, y: 13 },
        to: Pos { x: 963, y: 13 },
    },
    Line {
        from: Pos { x: 586, y: 904 },
        to: Pos { x: 658, y: 976 },
    },
    Line {
        from: Pos { x: 238, y: 824 },
        to: Pos { x: 782, y: 824 },
    },
    Line {
        from: Pos { x: 366, y: 45 },
        to: Pos { x: 691, y: 370 },
    },
    Line {
        from: Pos { x: 428, y: 758 },
        to: Pos { x: 201, y: 758 },
    },
    Line {
        from: Pos { x: 240, y: 545 },
        to: Pos { x: 30, y: 545 },
    },
    Line {
        from: Pos { x: 396, y: 154 },
        to: Pos { x: 332, y: 154 },
    },
    Line {
        from: Pos { x: 549, y: 307 },
        to: Pos { x: 233, y: 307 },
    },
    Line {
        from: Pos { x: 187, y: 240 },
        to: Pos { x: 851, y: 904 },
    },
    Line {
        from: Pos { x: 151, y: 135 },
        to: Pos { x: 937, y: 921 },
    },
    Line {
        from: Pos { x: 342, y: 850 },
        to: Pos { x: 342, y: 156 },
    },
    Line {
        from: Pos { x: 695, y: 200 },
        to: Pos { x: 695, y: 754 },
    },
    Line {
        from: Pos { x: 385, y: 880 },
        to: Pos { x: 893, y: 372 },
    },
    Line {
        from: Pos { x: 986, y: 966 },
        to: Pos { x: 813, y: 966 },
    },
    Line {
        from: Pos { x: 727, y: 661 },
        to: Pos { x: 727, y: 402 },
    },
    Line {
        from: Pos { x: 316, y: 937 },
        to: Pos { x: 316, y: 797 },
    },
    Line {
        from: Pos { x: 422, y: 235 },
        to: Pos { x: 422, y: 282 },
    },
    Line {
        from: Pos { x: 965, y: 684 },
        to: Pos { x: 882, y: 684 },
    },
    Line {
        from: Pos { x: 266, y: 222 },
        to: Pos { x: 419, y: 69 },
    },
    Line {
        from: Pos { x: 649, y: 843 },
        to: Pos { x: 635, y: 857 },
    },
    Line {
        from: Pos { x: 618, y: 84 },
        to: Pos { x: 126, y: 576 },
    },
    Line {
        from: Pos { x: 588, y: 822 },
        to: Pos { x: 588, y: 636 },
    },
    Line {
        from: Pos { x: 569, y: 142 },
        to: Pos { x: 569, y: 607 },
    },
    Line {
        from: Pos { x: 899, y: 479 },
        to: Pos { x: 488, y: 890 },
    },
    Line {
        from: Pos { x: 986, y: 52 },
        to: Pos { x: 369, y: 52 },
    },
    Line {
        from: Pos { x: 987, y: 478 },
        to: Pos { x: 551, y: 914 },
    },
    Line {
        from: Pos { x: 867, y: 951 },
        to: Pos { x: 973, y: 845 },
    },
    Line {
        from: Pos { x: 90, y: 401 },
        to: Pos { x: 304, y: 401 },
    },
    Line {
        from: Pos { x: 60, y: 836 },
        to: Pos { x: 798, y: 836 },
    },
    Line {
        from: Pos { x: 143, y: 675 },
        to: Pos { x: 686, y: 675 },
    },
    Line {
        from: Pos { x: 743, y: 974 },
        to: Pos { x: 743, y: 305 },
    },
    Line {
        from: Pos { x: 981, y: 899 },
        to: Pos { x: 551, y: 469 },
    },
    Line {
        from: Pos { x: 705, y: 430 },
        to: Pos { x: 493, y: 430 },
    },
    Line {
        from: Pos { x: 301, y: 366 },
        to: Pos { x: 823, y: 366 },
    },
    Line {
        from: Pos { x: 978, y: 712 },
        to: Pos { x: 617, y: 712 },
    },
    Line {
        from: Pos { x: 426, y: 805 },
        to: Pos { x: 426, y: 345 },
    },
    Line {
        from: Pos { x: 532, y: 855 },
        to: Pos { x: 532, y: 54 },
    },
    Line {
        from: Pos { x: 612, y: 143 },
        to: Pos { x: 612, y: 133 },
    },
    Line {
        from: Pos { x: 57, y: 52 },
        to: Pos { x: 955, y: 950 },
    },
    Line {
        from: Pos { x: 880, y: 50 },
        to: Pos { x: 16, y: 914 },
    },
    Line {
        from: Pos { x: 89, y: 908 },
        to: Pos { x: 89, y: 214 },
    },
    Line {
        from: Pos { x: 487, y: 867 },
        to: Pos { x: 586, y: 867 },
    },
    Line {
        from: Pos { x: 181, y: 285 },
        to: Pos { x: 181, y: 470 },
    },
    Line {
        from: Pos { x: 526, y: 666 },
        to: Pos { x: 86, y: 226 },
    },
    Line {
        from: Pos { x: 117, y: 704 },
        to: Pos { x: 117, y: 961 },
    },
    Line {
        from: Pos { x: 289, y: 101 },
        to: Pos { x: 983, y: 795 },
    },
    Line {
        from: Pos { x: 586, y: 429 },
        to: Pos { x: 442, y: 429 },
    },
    Line {
        from: Pos { x: 442, y: 869 },
        to: Pos { x: 734, y: 869 },
    },
    Line {
        from: Pos { x: 564, y: 479 },
        to: Pos { x: 564, y: 382 },
    },
    Line {
        from: Pos { x: 447, y: 486 },
        to: Pos { x: 62, y: 101 },
    },
    Line {
        from: Pos { x: 42, y: 218 },
        to: Pos { x: 509, y: 218 },
    },
    Line {
        from: Pos { x: 21, y: 890 },
        to: Pos { x: 843, y: 68 },
    },
    Line {
        from: Pos { x: 84, y: 978 },
        to: Pos { x: 921, y: 141 },
    },
    Line {
        from: Pos { x: 590, y: 960 },
        to: Pos { x: 590, y: 934 },
    },
    Line {
        from: Pos { x: 54, y: 949 },
        to: Pos { x: 967, y: 36 },
    },
    Line {
        from: Pos { x: 799, y: 39 },
        to: Pos { x: 767, y: 39 },
    },
    Line {
        from: Pos { x: 979, y: 232 },
        to: Pos { x: 979, y: 628 },
    },
    Line {
        from: Pos { x: 489, y: 482 },
        to: Pos { x: 339, y: 482 },
    },
    Line {
        from: Pos { x: 759, y: 473 },
        to: Pos { x: 290, y: 942 },
    },
    Line {
        from: Pos { x: 960, y: 958 },
        to: Pos { x: 32, y: 30 },
    },
    Line {
        from: Pos { x: 134, y: 180 },
        to: Pos { x: 134, y: 864 },
    },
    Line {
        from: Pos { x: 972, y: 981 },
        to: Pos { x: 13, y: 22 },
    },
    Line {
        from: Pos { x: 106, y: 385 },
        to: Pos { x: 11, y: 385 },
    },
    Line {
        from: Pos { x: 849, y: 454 },
        to: Pos { x: 447, y: 454 },
    },
    Line {
        from: Pos { x: 477, y: 385 },
        to: Pos { x: 955, y: 863 },
    },
    Line {
        from: Pos { x: 853, y: 180 },
        to: Pos { x: 922, y: 180 },
    },
    Line {
        from: Pos { x: 509, y: 274 },
        to: Pos { x: 751, y: 32 },
    },
    Line {
        from: Pos { x: 905, y: 295 },
        to: Pos { x: 779, y: 295 },
    },
    Line {
        from: Pos { x: 661, y: 629 },
        to: Pos { x: 104, y: 629 },
    },
    Line {
        from: Pos { x: 935, y: 117 },
        to: Pos { x: 93, y: 959 },
    },
    Line {
        from: Pos { x: 165, y: 372 },
        to: Pos { x: 746, y: 953 },
    },
    Line {
        from: Pos { x: 988, y: 141 },
        to: Pos { x: 122, y: 141 },
    },
    Line {
        from: Pos { x: 625, y: 621 },
        to: Pos { x: 625, y: 406 },
    },
    Line {
        from: Pos { x: 24, y: 710 },
        to: Pos { x: 465, y: 710 },
    },
    Line {
        from: Pos { x: 417, y: 468 },
        to: Pos { x: 851, y: 34 },
    },
    Line {
        from: Pos { x: 365, y: 285 },
        to: Pos { x: 572, y: 285 },
    },
    Line {
        from: Pos { x: 217, y: 164 },
        to: Pos { x: 217, y: 214 },
    },
    Line {
        from: Pos { x: 943, y: 439 },
        to: Pos { x: 465, y: 439 },
    },
    Line {
        from: Pos { x: 80, y: 102 },
        to: Pos { x: 80, y: 717 },
    },
    Line {
        from: Pos { x: 869, y: 19 },
        to: Pos { x: 54, y: 834 },
    },
    Line {
        from: Pos { x: 399, y: 480 },
        to: Pos { x: 399, y: 458 },
    },
    Line {
        from: Pos { x: 644, y: 826 },
        to: Pos { x: 644, y: 911 },
    },
    Line {
        from: Pos { x: 651, y: 189 },
        to: Pos { x: 651, y: 687 },
    },
    Line {
        from: Pos { x: 671, y: 946 },
        to: Pos { x: 332, y: 607 },
    },
    Line {
        from: Pos { x: 531, y: 417 },
        to: Pos { x: 657, y: 417 },
    },
    Line {
        from: Pos { x: 847, y: 350 },
        to: Pos { x: 847, y: 112 },
    },
    Line {
        from: Pos { x: 315, y: 733 },
        to: Pos { x: 871, y: 177 },
    },
    Line {
        from: Pos { x: 749, y: 118 },
        to: Pos { x: 692, y: 118 },
    },
    Line {
        from: Pos { x: 55, y: 616 },
        to: Pos { x: 55, y: 894 },
    },
    Line {
        from: Pos { x: 570, y: 307 },
        to: Pos { x: 633, y: 307 },
    },
    Line {
        from: Pos { x: 12, y: 964 },
        to: Pos { x: 883, y: 93 },
    },
    Line {
        from: Pos { x: 84, y: 299 },
        to: Pos { x: 84, y: 185 },
    },
    Line {
        from: Pos { x: 49, y: 187 },
        to: Pos { x: 903, y: 187 },
    },
    Line {
        from: Pos { x: 592, y: 40 },
        to: Pos { x: 842, y: 40 },
    },
    Line {
        from: Pos { x: 639, y: 381 },
        to: Pos { x: 802, y: 544 },
    },
    Line {
        from: Pos { x: 59, y: 61 },
        to: Pos { x: 836, y: 61 },
    },
    Line {
        from: Pos { x: 968, y: 51 },
        to: Pos { x: 266, y: 753 },
    },
    Line {
        from: Pos { x: 883, y: 373 },
        to: Pos { x: 883, y: 130 },
    },
    Line {
        from: Pos { x: 612, y: 45 },
        to: Pos { x: 406, y: 45 },
    },
    Line {
        from: Pos { x: 206, y: 698 },
        to: Pos { x: 206, y: 823 },
    },
    Line {
        from: Pos { x: 385, y: 685 },
        to: Pos { x: 385, y: 46 },
    },
    Line {
        from: Pos { x: 656, y: 338 },
        to: Pos { x: 73, y: 921 },
    },
    Line {
        from: Pos { x: 256, y: 794 },
        to: Pos { x: 365, y: 903 },
    },
    Line {
        from: Pos { x: 671, y: 247 },
        to: Pos { x: 248, y: 247 },
    },
    Line {
        from: Pos { x: 722, y: 509 },
        to: Pos { x: 635, y: 422 },
    },
    Line {
        from: Pos { x: 460, y: 783 },
        to: Pos { x: 615, y: 783 },
    },
    Line {
        from: Pos { x: 946, y: 980 },
        to: Pos { x: 946, y: 129 },
    },
    Line {
        from: Pos { x: 343, y: 780 },
        to: Pos { x: 343, y: 723 },
    },
    Line {
        from: Pos { x: 218, y: 371 },
        to: Pos { x: 218, y: 856 },
    },
    Line {
        from: Pos { x: 363, y: 809 },
        to: Pos { x: 143, y: 589 },
    },
    Line {
        from: Pos { x: 434, y: 739 },
        to: Pos { x: 889, y: 739 },
    },
    Line {
        from: Pos { x: 75, y: 71 },
        to: Pos { x: 975, y: 971 },
    },
    Line {
        from: Pos { x: 57, y: 253 },
        to: Pos { x: 582, y: 778 },
    },
    Line {
        from: Pos { x: 976, y: 237 },
        to: Pos { x: 976, y: 148 },
    },
    Line {
        from: Pos { x: 386, y: 866 },
        to: Pos { x: 386, y: 544 },
    },
    Line {
        from: Pos { x: 901, y: 797 },
        to: Pos { x: 901, y: 630 },
    },
    Line {
        from: Pos { x: 976, y: 706 },
        to: Pos { x: 195, y: 706 },
    },
    Line {
        from: Pos { x: 264, y: 420 },
        to: Pos { x: 272, y: 428 },
    },
    Line {
        from: Pos { x: 693, y: 72 },
        to: Pos { x: 693, y: 379 },
    },
    Line {
        from: Pos { x: 888, y: 832 },
        to: Pos { x: 888, y: 490 },
    },
    Line {
        from: Pos { x: 363, y: 900 },
        to: Pos { x: 363, y: 350 },
    },
    Line {
        from: Pos { x: 25, y: 312 },
        to: Pos { x: 25, y: 58 },
    },
    Line {
        from: Pos { x: 292, y: 307 },
        to: Pos { x: 481, y: 307 },
    },
    Line {
        from: Pos { x: 715, y: 393 },
        to: Pos { x: 976, y: 132 },
    },
    Line {
        from: Pos { x: 641, y: 450 },
        to: Pos { x: 96, y: 450 },
    },
    Line {
        from: Pos { x: 650, y: 38 },
        to: Pos { x: 432, y: 38 },
    },
    Line {
        from: Pos { x: 339, y: 97 },
        to: Pos { x: 476, y: 97 },
    },
    Line {
        from: Pos { x: 916, y: 24 },
        to: Pos { x: 13, y: 927 },
    },
    Line {
        from: Pos { x: 933, y: 934 },
        to: Pos { x: 34, y: 35 },
    },
    Line {
        from: Pos { x: 971, y: 367 },
        to: Pos { x: 971, y: 919 },
    },
    Line {
        from: Pos { x: 726, y: 310 },
        to: Pos { x: 477, y: 559 },
    },
    Line {
        from: Pos { x: 12, y: 984 },
        to: Pos { x: 986, y: 10 },
    },
    Line {
        from: Pos { x: 318, y: 531 },
        to: Pos { x: 318, y: 72 },
    },
    Line {
        from: Pos { x: 604, y: 979 },
        to: Pos { x: 12, y: 387 },
    },
    Line {
        from: Pos { x: 890, y: 39 },
        to: Pos { x: 890, y: 213 },
    },
    Line {
        from: Pos { x: 944, y: 954 },
        to: Pos { x: 33, y: 43 },
    },
    Line {
        from: Pos { x: 507, y: 830 },
        to: Pos { x: 284, y: 607 },
    },
    Line {
        from: Pos { x: 724, y: 111 },
        to: Pos { x: 724, y: 242 },
    },
    Line {
        from: Pos { x: 425, y: 912 },
        to: Pos { x: 425, y: 445 },
    },
    Line {
        from: Pos { x: 371, y: 903 },
        to: Pos { x: 371, y: 634 },
    },
    Line {
        from: Pos { x: 415, y: 314 },
        to: Pos { x: 415, y: 509 },
    },
    Line {
        from: Pos { x: 884, y: 849 },
        to: Pos { x: 884, y: 454 },
    },
    Line {
        from: Pos { x: 726, y: 647 },
        to: Pos { x: 447, y: 926 },
    },
    Line {
        from: Pos { x: 588, y: 463 },
        to: Pos { x: 588, y: 426 },
    },
    Line {
        from: Pos { x: 807, y: 453 },
        to: Pos { x: 807, y: 593 },
    },
    Line {
        from: Pos { x: 32, y: 449 },
        to: Pos { x: 975, y: 449 },
    },
    Line {
        from: Pos { x: 593, y: 757 },
        to: Pos { x: 593, y: 607 },
    },
    Line {
        from: Pos { x: 521, y: 850 },
        to: Pos { x: 521, y: 139 },
    },
    Line {
        from: Pos { x: 843, y: 478 },
        to: Pos { x: 843, y: 317 },
    },
    Line {
        from: Pos { x: 408, y: 834 },
        to: Pos { x: 408, y: 455 },
    },
    Line {
        from: Pos { x: 65, y: 241 },
        to: Pos { x: 864, y: 241 },
    },
    Line {
        from: Pos { x: 532, y: 138 },
        to: Pos { x: 613, y: 138 },
    },
    Line {
        from: Pos { x: 477, y: 239 },
        to: Pos { x: 477, y: 676 },
    },
    Line {
        from: Pos { x: 92, y: 400 },
        to: Pos { x: 92, y: 935 },
    },
    Line {
        from: Pos { x: 268, y: 104 },
        to: Pos { x: 300, y: 104 },
    },
    Line {
        from: Pos { x: 942, y: 20 },
        to: Pos { x: 93, y: 869 },
    },
    Line {
        from: Pos { x: 294, y: 134 },
        to: Pos { x: 695, y: 134 },
    },
    Line {
        from: Pos { x: 748, y: 477 },
        to: Pos { x: 748, y: 311 },
    },
    Line {
        from: Pos { x: 581, y: 879 },
        to: Pos { x: 481, y: 879 },
    },
    Line {
        from: Pos { x: 292, y: 57 },
        to: Pos { x: 874, y: 639 },
    },
    Line {
        from: Pos { x: 829, y: 787 },
        to: Pos { x: 944, y: 787 },
    },
    Line {
        from: Pos { x: 130, y: 780 },
        to: Pos { x: 442, y: 780 },
    },
    Line {
        from: Pos { x: 754, y: 435 },
        to: Pos { x: 956, y: 435 },
    },
    Line {
        from: Pos { x: 306, y: 659 },
        to: Pos { x: 306, y: 491 },
    },
    Line {
        from: Pos { x: 252, y: 612 },
        to: Pos { x: 646, y: 612 },
    },
    Line {
        from: Pos { x: 846, y: 949 },
        to: Pos { x: 846, y: 924 },
    },
    Line {
        from: Pos { x: 197, y: 888 },
        to: Pos { x: 145, y: 836 },
    },
    Line {
        from: Pos { x: 156, y: 790 },
        to: Pos { x: 151, y: 790 },
    },
    Line {
        from: Pos { x: 903, y: 305 },
        to: Pos { x: 671, y: 73 },
    },
    Line {
        from: Pos { x: 195, y: 79 },
        to: Pos { x: 195, y: 40 },
    },
    Line {
        from: Pos { x: 781, y: 67 },
        to: Pos { x: 781, y: 635 },
    },
    Line {
        from: Pos { x: 742, y: 743 },
        to: Pos { x: 742, y: 280 },
    },
    Line {
        from: Pos { x: 297, y: 42 },
        to: Pos { x: 618, y: 42 },
    },
    Line {
        from: Pos { x: 237, y: 151 },
        to: Pos { x: 156, y: 151 },
    },
    Line {
        from: Pos { x: 851, y: 930 },
        to: Pos { x: 47, y: 126 },
    },
    Line {
        from: Pos { x: 425, y: 368 },
        to: Pos { x: 659, y: 134 },
    },
    Line {
        from: Pos { x: 57, y: 890 },
        to: Pos { x: 898, y: 49 },
    },
    Line {
        from: Pos { x: 86, y: 62 },
        to: Pos { x: 86, y: 445 },
    },
    Line {
        from: Pos { x: 133, y: 499 },
        to: Pos { x: 133, y: 604 },
    },
    Line {
        from: Pos { x: 202, y: 567 },
        to: Pos { x: 872, y: 567 },
    },
    Line {
        from: Pos { x: 749, y: 578 },
        to: Pos { x: 749, y: 804 },
    },
    Line {
        from: Pos { x: 379, y: 379 },
        to: Pos { x: 147, y: 379 },
    },
    Line {
        from: Pos { x: 510, y: 474 },
        to: Pos { x: 510, y: 388 },
    },
    Line {
        from: Pos { x: 184, y: 115 },
        to: Pos { x: 738, y: 115 },
    },
    Line {
        from: Pos { x: 904, y: 613 },
        to: Pos { x: 550, y: 613 },
    },
    Line {
        from: Pos { x: 755, y: 649 },
        to: Pos { x: 755, y: 305 },
    },
    Line {
        from: Pos { x: 461, y: 306 },
        to: Pos { x: 461, y: 547 },
    },
    Line {
        from: Pos { x: 469, y: 124 },
        to: Pos { x: 542, y: 124 },
    },
    Line {
        from: Pos { x: 736, y: 218 },
        to: Pos { x: 736, y: 968 },
    },
    Line {
        from: Pos { x: 307, y: 662 },
        to: Pos { x: 307, y: 188 },
    },
    Line {
        from: Pos { x: 360, y: 970 },
        to: Pos { x: 58, y: 668 },
    },
    Line {
        from: Pos { x: 36, y: 267 },
        to: Pos { x: 214, y: 267 },
    },
    Line {
        from: Pos { x: 980, y: 330 },
        to: Pos { x: 22, y: 330 },
    },
    Line {
        from: Pos { x: 222, y: 972 },
        to: Pos { x: 222, y: 178 },
    },
    Line {
        from: Pos { x: 846, y: 774 },
        to: Pos { x: 714, y: 774 },
    },
    Line {
        from: Pos { x: 798, y: 837 },
        to: Pos { x: 789, y: 837 },
    },
    Line {
        from: Pos { x: 567, y: 258 },
        to: Pos { x: 567, y: 502 },
    },
    Line {
        from: Pos { x: 325, y: 582 },
        to: Pos { x: 325, y: 976 },
    },
    Line {
        from: Pos { x: 138, y: 386 },
        to: Pos { x: 138, y: 691 },
    },
    Line {
        from: Pos { x: 326, y: 878 },
        to: Pos { x: 326, y: 386 },
    },
    Line {
        from: Pos { x: 790, y: 276 },
        to: Pos { x: 811, y: 276 },
    },
    Line {
        from: Pos { x: 517, y: 522 },
        to: Pos { x: 81, y: 86 },
    },
    Line {
        from: Pos { x: 493, y: 567 },
        to: Pos { x: 406, y: 567 },
    },
    Line {
        from: Pos { x: 522, y: 370 },
        to: Pos { x: 13, y: 370 },
    },
    Line {
        from: Pos { x: 31, y: 697 },
        to: Pos { x: 607, y: 121 },
    },
    Line {
        from: Pos { x: 297, y: 524 },
        to: Pos { x: 297, y: 320 },
    },
    Line {
        from: Pos { x: 790, y: 681 },
        to: Pos { x: 753, y: 681 },
    },
    Line {
        from: Pos { x: 90, y: 961 },
        to: Pos { x: 901, y: 150 },
    },
    Line {
        from: Pos { x: 262, y: 46 },
        to: Pos { x: 262, y: 68 },
    },
    Line {
        from: Pos { x: 18, y: 26 },
        to: Pos { x: 977, y: 985 },
    },
    Line {
        from: Pos { x: 782, y: 381 },
        to: Pos { x: 956, y: 381 },
    },
    Line {
        from: Pos { x: 353, y: 740 },
        to: Pos { x: 353, y: 595 },
    },
    Line {
        from: Pos { x: 32, y: 448 },
        to: Pos { x: 941, y: 448 },
    },
    Line {
        from: Pos { x: 405, y: 254 },
        to: Pos { x: 686, y: 254 },
    },
    Line {
        from: Pos { x: 853, y: 57 },
        to: Pos { x: 297, y: 613 },
    },
    Line {
        from: Pos { x: 555, y: 209 },
        to: Pos { x: 439, y: 209 },
    },
    Line {
        from: Pos { x: 765, y: 679 },
        to: Pos { x: 142, y: 56 },
    },
    Line {
        from: Pos { x: 175, y: 903 },
        to: Pos { x: 175, y: 685 },
    },
    Line {
        from: Pos { x: 693, y: 653 },
        to: Pos { x: 845, y: 653 },
    },
    Line {
        from: Pos { x: 394, y: 108 },
        to: Pos { x: 394, y: 901 },
    },
    Line {
        from: Pos { x: 351, y: 108 },
        to: Pos { x: 335, y: 108 },
    },
    Line {
        from: Pos { x: 841, y: 83 },
        to: Pos { x: 841, y: 716 },
    },
    Line {
        from: Pos { x: 525, y: 608 },
        to: Pos { x: 525, y: 496 },
    },
    Line {
        from: Pos { x: 874, y: 32 },
        to: Pos { x: 874, y: 214 },
    },
    Line {
        from: Pos { x: 354, y: 760 },
        to: Pos { x: 44, y: 760 },
    },
    Line {
        from: Pos { x: 249, y: 330 },
        to: Pos { x: 864, y: 945 },
    },
    Line {
        from: Pos { x: 553, y: 377 },
        to: Pos { x: 553, y: 944 },
    },
    Line {
        from: Pos { x: 903, y: 374 },
        to: Pos { x: 335, y: 374 },
    },
    Line {
        from: Pos { x: 387, y: 34 },
        to: Pos { x: 387, y: 86 },
    },
    Line {
        from: Pos { x: 380, y: 331 },
        to: Pos { x: 380, y: 124 },
    },
    Line {
        from: Pos { x: 618, y: 520 },
        to: Pos { x: 797, y: 520 },
    },
    Line {
        from: Pos { x: 718, y: 169 },
        to: Pos { x: 703, y: 169 },
    },
    Line {
        from: Pos { x: 355, y: 184 },
        to: Pos { x: 851, y: 184 },
    },
    Line {
        from: Pos { x: 582, y: 570 },
        to: Pos { x: 582, y: 313 },
    },
    Line {
        from: Pos { x: 312, y: 952 },
        to: Pos { x: 312, y: 460 },
    },
    Line {
        from: Pos { x: 269, y: 70 },
        to: Pos { x: 269, y: 197 },
    },
    Line {
        from: Pos { x: 701, y: 907 },
        to: Pos { x: 701, y: 768 },
    },
    Line {
        from: Pos { x: 645, y: 417 },
        to: Pos { x: 645, y: 548 },
    },
    Line {
        from: Pos { x: 931, y: 532 },
        to: Pos { x: 367, y: 532 },
    },
    Line {
        from: Pos { x: 497, y: 361 },
        to: Pos { x: 497, y: 348 },
    },
    Line {
        from: Pos { x: 563, y: 642 },
        to: Pos { x: 976, y: 642 },
    },
    Line {
        from: Pos { x: 376, y: 504 },
        to: Pos { x: 376, y: 448 },
    },
    Line {
        from: Pos { x: 538, y: 945 },
        to: Pos { x: 538, y: 773 },
    },
    Line {
        from: Pos { x: 594, y: 886 },
        to: Pos { x: 594, y: 281 },
    },
    Line {
        from: Pos { x: 879, y: 558 },
        to: Pos { x: 192, y: 558 },
    },
    Line {
        from: Pos { x: 985, y: 68 },
        to: Pos { x: 66, y: 987 },
    },
    Line {
        from: Pos { x: 599, y: 420 },
        to: Pos { x: 599, y: 41 },
    },
    Line {
        from: Pos { x: 296, y: 318 },
        to: Pos { x: 296, y: 132 },
    },
    Line {
        from: Pos { x: 330, y: 619 },
        to: Pos { x: 302, y: 619 },
    },
    Line {
        from: Pos { x: 245, y: 137 },
        to: Pos { x: 918, y: 810 },
    },
    Line {
        from: Pos { x: 823, y: 798 },
        to: Pos { x: 556, y: 531 },
    },
    Line {
        from: Pos { x: 64, y: 201 },
        to: Pos { x: 723, y: 860 },
    },
    Line {
        from: Pos { x: 955, y: 365 },
        to: Pos { x: 955, y: 829 },
    },
    Line {
        from: Pos { x: 372, y: 976 },
        to: Pos { x: 255, y: 859 },
    },
    Line {
        from: Pos { x: 804, y: 962 },
        to: Pos { x: 168, y: 962 },
    },
    Line {
        from: Pos { x: 200, y: 442 },
        to: Pos { x: 200, y: 97 },
    },
    Line {
        from: Pos { x: 965, y: 964 },
        to: Pos { x: 870, y: 869 },
    },
    Line {
        from: Pos { x: 534, y: 158 },
        to: Pos { x: 128, y: 564 },
    },
    Line {
        from: Pos { x: 380, y: 739 },
        to: Pos { x: 577, y: 542 },
    },
    Line {
        from: Pos { x: 740, y: 391 },
        to: Pos { x: 740, y: 651 },
    },
    Line {
        from: Pos { x: 167, y: 177 },
        to: Pos { x: 619, y: 177 },
    },
    Line {
        from: Pos { x: 215, y: 449 },
        to: Pos { x: 215, y: 330 },
    },
    Line {
        from: Pos { x: 494, y: 612 },
        to: Pos { x: 19, y: 137 },
    },
    Line {
        from: Pos { x: 458, y: 634 },
        to: Pos { x: 458, y: 257 },
    },
    Line {
        from: Pos { x: 884, y: 817 },
        to: Pos { x: 393, y: 326 },
    },
    Line {
        from: Pos { x: 407, y: 291 },
        to: Pos { x: 19, y: 679 },
    },
    Line {
        from: Pos { x: 627, y: 173 },
        to: Pos { x: 627, y: 570 },
    },
    Line {
        from: Pos { x: 53, y: 93 },
        to: Pos { x: 552, y: 592 },
    },
    Line {
        from: Pos { x: 809, y: 363 },
        to: Pos { x: 119, y: 363 },
    },
    Line {
        from: Pos { x: 588, y: 418 },
        to: Pos { x: 588, y: 764 },
    },
    Line {
        from: Pos { x: 807, y: 131 },
        to: Pos { x: 807, y: 834 },
    },
    Line {
        from: Pos { x: 616, y: 61 },
        to: Pos { x: 514, y: 61 },
    },
    Line {
        from: Pos { x: 553, y: 642 },
        to: Pos { x: 236, y: 325 },
    },
    Line {
        from: Pos { x: 959, y: 553 },
        to: Pos { x: 683, y: 553 },
    },
    Line {
        from: Pos { x: 36, y: 754 },
        to: Pos { x: 36, y: 830 },
    },
    Line {
        from: Pos { x: 533, y: 293 },
        to: Pos { x: 144, y: 293 },
    },
    Line {
        from: Pos { x: 950, y: 780 },
        to: Pos { x: 396, y: 780 },
    },
    Line {
        from: Pos { x: 949, y: 878 },
        to: Pos { x: 14, y: 878 },
    },
    Line {
        from: Pos { x: 453, y: 180 },
        to: Pos { x: 989, y: 180 },
    },
    Line {
        from: Pos { x: 22, y: 46 },
        to: Pos { x: 670, y: 694 },
    },
    Line {
        from: Pos { x: 479, y: 206 },
        to: Pos { x: 479, y: 552 },
    },
    Line {
        from: Pos { x: 22, y: 53 },
        to: Pos { x: 599, y: 53 },
    },
    Line {
        from: Pos { x: 254, y: 964 },
        to: Pos { x: 884, y: 334 },
    },
    Line {
        from: Pos { x: 578, y: 813 },
        to: Pos { x: 100, y: 813 },
    },
    Line {
        from: Pos { x: 945, y: 247 },
        to: Pos { x: 778, y: 80 },
    },
    Line {
        from: Pos { x: 312, y: 978 },
        to: Pos { x: 312, y: 518 },
    },
    Line {
        from: Pos { x: 882, y: 225 },
        to: Pos { x: 882, y: 967 },
    },
    Line {
        from: Pos { x: 581, y: 683 },
        to: Pos { x: 293, y: 395 },
    },
    Line {
        from: Pos { x: 107, y: 540 },
        to: Pos { x: 534, y: 967 },
    },
    Line {
        from: Pos { x: 382, y: 946 },
        to: Pos { x: 28, y: 946 },
    },
    Line {
        from: Pos { x: 864, y: 743 },
        to: Pos { x: 246, y: 743 },
    },
    Line {
        from: Pos { x: 538, y: 558 },
        to: Pos { x: 733, y: 753 },
    },
    Line {
        from: Pos { x: 811, y: 436 },
        to: Pos { x: 814, y: 436 },
    },
    Line {
        from: Pos { x: 982, y: 33 },
        to: Pos { x: 65, y: 950 },
    },
    Line {
        from: Pos { x: 785, y: 829 },
        to: Pos { x: 945, y: 829 },
    },
    Line {
        from: Pos { x: 322, y: 471 },
        to: Pos { x: 346, y: 471 },
    },
    Line {
        from: Pos { x: 904, y: 528 },
        to: Pos { x: 904, y: 669 },
    },
    Line {
        from: Pos { x: 231, y: 471 },
        to: Pos { x: 772, y: 471 },
    },
    Line {
        from: Pos { x: 773, y: 490 },
        to: Pos { x: 669, y: 386 },
    },
    Line {
        from: Pos { x: 867, y: 482 },
        to: Pos { x: 417, y: 32 },
    },
    Line {
        from: Pos { x: 352, y: 856 },
        to: Pos { x: 352, y: 478 },
    },
    Line {
        from: Pos { x: 723, y: 355 },
        to: Pos { x: 619, y: 355 },
    },
    Line {
        from: Pos { x: 667, y: 922 },
        to: Pos { x: 667, y: 247 },
    },
    Line {
        from: Pos { x: 642, y: 386 },
        to: Pos { x: 241, y: 386 },
    },
    Line {
        from: Pos { x: 594, y: 35 },
        to: Pos { x: 594, y: 580 },
    },
    Line {
        from: Pos { x: 916, y: 723 },
        to: Pos { x: 793, y: 723 },
    },
    Line {
        from: Pos { x: 73, y: 774 },
        to: Pos { x: 269, y: 970 },
    },
    Line {
        from: Pos { x: 43, y: 273 },
        to: Pos { x: 148, y: 168 },
    },
    Line {
        from: Pos { x: 744, y: 637 },
        to: Pos { x: 825, y: 637 },
    },
    Line {
        from: Pos { x: 98, y: 30 },
        to: Pos { x: 98, y: 383 },
    },
    Line {
        from: Pos { x: 130, y: 277 },
        to: Pos { x: 802, y: 277 },
    },
    Line {
        from: Pos { x: 167, y: 122 },
        to: Pos { x: 672, y: 627 },
    },
    Line {
        from: Pos { x: 871, y: 866 },
        to: Pos { x: 564, y: 559 },
    },
    Line {
        from: Pos { x: 923, y: 475 },
        to: Pos { x: 539, y: 859 },
    },
    Line {
        from: Pos { x: 422, y: 714 },
        to: Pos { x: 422, y: 946 },
    },
    Line {
        from: Pos { x: 667, y: 950 },
        to: Pos { x: 667, y: 640 },
    },
    Line {
        from: Pos { x: 758, y: 181 },
        to: Pos { x: 12, y: 927 },
    },
    Line {
        from: Pos { x: 129, y: 927 },
        to: Pos { x: 129, y: 288 },
    },
    Line {
        from: Pos { x: 485, y: 661 },
        to: Pos { x: 402, y: 661 },
    },
    Line {
        from: Pos { x: 114, y: 573 },
        to: Pos { x: 974, y: 573 },
    },
    Line {
        from: Pos { x: 674, y: 779 },
        to: Pos { x: 851, y: 779 },
    },
    Line {
        from: Pos { x: 977, y: 184 },
        to: Pos { x: 977, y: 143 },
    },
    Line {
        from: Pos { x: 229, y: 937 },
        to: Pos { x: 229, y: 138 },
    },
    Line {
        from: Pos { x: 520, y: 887 },
        to: Pos { x: 520, y: 512 },
    },
    Line {
        from: Pos { x: 918, y: 329 },
        to: Pos { x: 918, y: 990 },
    },
    Line {
        from: Pos { x: 732, y: 41 },
        to: Pos { x: 521, y: 41 },
    },
    Line {
        from: Pos { x: 399, y: 245 },
        to: Pos { x: 883, y: 729 },
    },
    Line {
        from: Pos { x: 824, y: 618 },
        to: Pos { x: 356, y: 618 },
    },
    Line {
        from: Pos { x: 215, y: 218 },
        to: Pos { x: 845, y: 848 },
    },
    Line {
        from: Pos { x: 704, y: 34 },
        to: Pos { x: 307, y: 431 },
    },
    Line {
        from: Pos { x: 124, y: 166 },
        to: Pos { x: 696, y: 738 },
    },
    Line {
        from: Pos { x: 692, y: 749 },
        to: Pos { x: 839, y: 749 },
    },
    Line {
        from: Pos { x: 790, y: 637 },
        to: Pos { x: 790, y: 598 },
    },
    Line {
        from: Pos { x: 697, y: 396 },
        to: Pos { x: 669, y: 396 },
    },
    Line {
        from: Pos { x: 419, y: 140 },
        to: Pos { x: 113, y: 446 },
    },
    Line {
        from: Pos { x: 426, y: 738 },
        to: Pos { x: 171, y: 738 },
    },
    Line {
        from: Pos { x: 489, y: 494 },
        to: Pos { x: 190, y: 793 },
    },
    Line {
        from: Pos { x: 320, y: 301 },
        to: Pos { x: 320, y: 398 },
    },
    Line {
        from: Pos { x: 275, y: 809 },
        to: Pos { x: 275, y: 717 },
    },
    Line {
        from: Pos { x: 537, y: 703 },
        to: Pos { x: 465, y: 703 },
    },
    Line {
        from: Pos { x: 536, y: 450 },
        to: Pos { x: 560, y: 450 },
    },
    Line {
        from: Pos { x: 153, y: 927 },
        to: Pos { x: 914, y: 166 },
    },
    Line {
        from: Pos { x: 246, y: 692 },
        to: Pos { x: 485, y: 453 },
    },
    Line {
        from: Pos { x: 26, y: 179 },
        to: Pos { x: 26, y: 554 },
    },
    Line {
        from: Pos { x: 487, y: 678 },
        to: Pos { x: 487, y: 696 },
    },
    Line {
        from: Pos { x: 807, y: 719 },
        to: Pos { x: 224, y: 719 },
    },
    Line {
        from: Pos { x: 605, y: 920 },
        to: Pos { x: 899, y: 920 },
    },
    Line {
        from: Pos { x: 112, y: 262 },
        to: Pos { x: 112, y: 765 },
    },
    Line {
        from: Pos { x: 752, y: 898 },
        to: Pos { x: 752, y: 429 },
    },
    Line {
        from: Pos { x: 861, y: 103 },
        to: Pos { x: 861, y: 477 },
    },
    Line {
        from: Pos { x: 628, y: 505 },
        to: Pos { x: 628, y: 248 },
    },
    Line {
        from: Pos { x: 556, y: 293 },
        to: Pos { x: 556, y: 276 },
    },
    Line {
        from: Pos { x: 826, y: 682 },
        to: Pos { x: 273, y: 129 },
    },
    Line {
        from: Pos { x: 685, y: 324 },
        to: Pos { x: 685, y: 692 },
    },
    Line {
        from: Pos { x: 544, y: 410 },
        to: Pos { x: 544, y: 678 },
    },
    Line {
        from: Pos { x: 796, y: 633 },
        to: Pos { x: 796, y: 950 },
    },
    Line {
        from: Pos { x: 753, y: 843 },
        to: Pos { x: 753, y: 936 },
    },
    Line {
        from: Pos { x: 817, y: 40 },
        to: Pos { x: 817, y: 600 },
    },
    Line {
        from: Pos { x: 137, y: 941 },
        to: Pos { x: 677, y: 401 },
    },
    Line {
        from: Pos { x: 563, y: 457 },
        to: Pos { x: 599, y: 457 },
    },
    Line {
        from: Pos { x: 251, y: 644 },
        to: Pos { x: 251, y: 67 },
    },
    Line {
        from: Pos { x: 170, y: 792 },
        to: Pos { x: 805, y: 792 },
    },
    Line {
        from: Pos { x: 171, y: 486 },
        to: Pos { x: 171, y: 877 },
    },
    Line {
        from: Pos { x: 337, y: 481 },
        to: Pos { x: 268, y: 412 },
    },
    Line {
        from: Pos { x: 43, y: 158 },
        to: Pos { x: 44, y: 158 },
    },
    Line {
        from: Pos { x: 148, y: 610 },
        to: Pos { x: 863, y: 610 },
    },
    Line {
        from: Pos { x: 332, y: 765 },
        to: Pos { x: 202, y: 765 },
    },
    Line {
        from: Pos { x: 307, y: 637 },
        to: Pos { x: 334, y: 637 },
    },
    Line {
        from: Pos { x: 557, y: 380 },
        to: Pos { x: 231, y: 54 },
    },
    Line {
        from: Pos { x: 858, y: 76 },
        to: Pos { x: 150, y: 784 },
    },
    Line {
        from: Pos { x: 477, y: 329 },
        to: Pos { x: 319, y: 329 },
    },
    Line {
        from: Pos { x: 306, y: 608 },
        to: Pos { x: 306, y: 38 },
    },
    Line {
        from: Pos { x: 245, y: 42 },
        to: Pos { x: 245, y: 929 },
    },
    Line {
        from: Pos { x: 15, y: 786 },
        to: Pos { x: 745, y: 786 },
    },
    Line {
        from: Pos { x: 946, y: 321 },
        to: Pos { x: 841, y: 321 },
    },
    Line {
        from: Pos { x: 837, y: 281 },
        to: Pos { x: 837, y: 762 },
    },
    Line {
        from: Pos { x: 847, y: 391 },
        to: Pos { x: 847, y: 840 },
    },
    Line {
        from: Pos { x: 304, y: 52 },
        to: Pos { x: 304, y: 299 },
    },
    Line {
        from: Pos { x: 938, y: 122 },
        to: Pos { x: 877, y: 122 },
    },
    Line {
        from: Pos { x: 214, y: 347 },
        to: Pos { x: 862, y: 347 },
    },
    Line {
        from: Pos { x: 494, y: 540 },
        to: Pos { x: 751, y: 540 },
    },
    Line {
        from: Pos { x: 184, y: 29 },
        to: Pos { x: 913, y: 758 },
    },
    Line {
        from: Pos { x: 904, y: 12 },
        to: Pos { x: 15, y: 901 },
    },
    Line {
        from: Pos { x: 573, y: 23 },
        to: Pos { x: 158, y: 23 },
    },
    Line {
        from: Pos { x: 926, y: 603 },
        to: Pos { x: 643, y: 603 },
    },
    Line {
        from: Pos { x: 105, y: 506 },
        to: Pos { x: 518, y: 506 },
    },
    Line {
        from: Pos { x: 551, y: 917 },
        to: Pos { x: 983, y: 917 },
    },
    Line {
        from: Pos { x: 708, y: 33 },
        to: Pos { x: 831, y: 33 },
    },
    Line {
        from: Pos { x: 347, y: 173 },
        to: Pos { x: 218, y: 44 },
    },
    Line {
        from: Pos { x: 933, y: 175 },
        to: Pos { x: 933, y: 781 },
    },
    Line {
        from: Pos { x: 902, y: 556 },
        to: Pos { x: 902, y: 812 },
    },
    Line {
        from: Pos { x: 556, y: 485 },
        to: Pos { x: 252, y: 789 },
    },
    Line {
        from: Pos { x: 823, y: 807 },
        to: Pos { x: 368, y: 352 },
    },
    Line {
        from: Pos { x: 217, y: 744 },
        to: Pos { x: 217, y: 470 },
    },
    Line {
        from: Pos { x: 795, y: 455 },
        to: Pos { x: 795, y: 783 },
    },
    Line {
        from: Pos { x: 170, y: 944 },
        to: Pos { x: 926, y: 188 },
    },
    Line {
        from: Pos { x: 55, y: 655 },
        to: Pos { x: 258, y: 655 },
    },
    Line {
        from: Pos { x: 158, y: 57 },
        to: Pos { x: 959, y: 858 },
    },
    Line {
        from: Pos { x: 714, y: 823 },
        to: Pos { x: 714, y: 550 },
    },
    Line {
        from: Pos { x: 238, y: 18 },
        to: Pos { x: 388, y: 18 },
    },
    Line {
        from: Pos { x: 980, y: 985 },
        to: Pos { x: 12, y: 17 },
    },
    Line {
        from: Pos { x: 360, y: 596 },
        to: Pos { x: 770, y: 596 },
    },
    Line {
        from: Pos { x: 846, y: 684 },
        to: Pos { x: 220, y: 58 },
    },
    Line {
        from: Pos { x: 552, y: 107 },
        to: Pos { x: 552, y: 974 },
    },
    Line {
        from: Pos { x: 228, y: 552 },
        to: Pos { x: 354, y: 552 },
    },
    Line {
        from: Pos { x: 421, y: 41 },
        to: Pos { x: 421, y: 103 },
    },
    Line {
        from: Pos { x: 674, y: 475 },
        to: Pos { x: 912, y: 475 },
    },
    Line {
        from: Pos { x: 455, y: 626 },
        to: Pos { x: 455, y: 683 },
    },
    Line {
        from: Pos { x: 952, y: 841 },
        to: Pos { x: 946, y: 841 },
    },
    Line {
        from: Pos { x: 920, y: 792 },
        to: Pos { x: 381, y: 253 },
    },
    Line {
        from: Pos { x: 786, y: 918 },
        to: Pos { x: 786, y: 175 },
    },
    Line {
        from: Pos { x: 889, y: 859 },
        to: Pos { x: 889, y: 24 },
    },
    Line {
        from: Pos { x: 178, y: 604 },
        to: Pos { x: 573, y: 209 },
    },
    Line {
        from: Pos { x: 71, y: 621 },
        to: Pos { x: 550, y: 621 },
    },
    Line {
        from: Pos { x: 38, y: 36 },
        to: Pos { x: 922, y: 920 },
    },
    Line {
        from: Pos { x: 104, y: 690 },
        to: Pos { x: 575, y: 690 },
    },
    Line {
        from: Pos { x: 252, y: 883 },
        to: Pos { x: 894, y: 241 },
    },
    Line {
        from: Pos { x: 627, y: 107 },
        to: Pos { x: 417, y: 107 },
    },
    Line {
        from: Pos { x: 768, y: 913 },
        to: Pos { x: 13, y: 158 },
    },
    Line {
        from: Pos { x: 708, y: 337 },
        to: Pos { x: 708, y: 407 },
    },
    Line {
        from: Pos { x: 156, y: 941 },
        to: Pos { x: 156, y: 297 },
    },
    Line {
        from: Pos { x: 814, y: 653 },
        to: Pos { x: 814, y: 829 },
    },
    Line {
        from: Pos { x: 234, y: 920 },
        to: Pos { x: 896, y: 920 },
    },
    Line {
        from: Pos { x: 652, y: 170 },
        to: Pos { x: 128, y: 170 },
    },
    Line {
        from: Pos { x: 765, y: 825 },
        to: Pos { x: 347, y: 825 },
    },
    Line {
        from: Pos { x: 681, y: 901 },
        to: Pos { x: 681, y: 112 },
    },
    Line {
        from: Pos { x: 410, y: 301 },
        to: Pos { x: 979, y: 301 },
    },
    Line {
        from: Pos { x: 462, y: 681 },
        to: Pos { x: 462, y: 726 },
    },
    Line {
        from: Pos { x: 117, y: 957 },
        to: Pos { x: 117, y: 693 },
    },
    Line {
        from: Pos { x: 479, y: 948 },
        to: Pos { x: 698, y: 948 },
    },
    Line {
        from: Pos { x: 839, y: 965 },
        to: Pos { x: 97, y: 223 },
    },
    Line {
        from: Pos { x: 102, y: 189 },
        to: Pos { x: 102, y: 366 },
    },
    Line {
        from: Pos { x: 93, y: 798 },
        to: Pos { x: 819, y: 72 },
    },
    Line {
        from: Pos { x: 27, y: 336 },
        to: Pos { x: 27, y: 655 },
    },
    Line {
        from: Pos { x: 161, y: 635 },
        to: Pos { x: 527, y: 269 },
    },
    Line {
        from: Pos { x: 140, y: 272 },
        to: Pos { x: 140, y: 336 },
    },
    Line {
        from: Pos { x: 884, y: 915 },
        to: Pos { x: 41, y: 72 },
    },
    Line {
        from: Pos { x: 575, y: 563 },
        to: Pos { x: 155, y: 563 },
    },
    Line {
        from: Pos { x: 387, y: 601 },
        to: Pos { x: 387, y: 597 },
    },
    Line {
        from: Pos { x: 355, y: 186 },
        to: Pos { x: 782, y: 613 },
    },
    Line {
        from: Pos { x: 866, y: 435 },
        to: Pos { x: 816, y: 435 },
    },
    Line {
        from: Pos { x: 96, y: 161 },
        to: Pos { x: 764, y: 161 },
    },
    Line {
        from: Pos { x: 971, y: 29 },
        to: Pos { x: 21, y: 979 },
    },
];
