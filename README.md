# text analysis

Take a text file as input and analyze the text, allowing the output of the analysis report as text or JSON format.

# Usage

Usage: text-analysis [OPTIONS] --file <FILE>

Options:
-f, --file <FILE>
-t, --top <TOP> Display top N letters and words, 0 for all [default: 10]
-j, --json
-h, --help Print help

## Normal output

```rust
cargo r -- --file t8.shakespeare.txt
```

```
--- Text Analysis Report ---

Input File: t8.shakespeare.txt
Total Row Characters: 5458199
Total Words: 1418387
Total Letters: 5209313
Total Unique Words: 28122
Total Unique Letters: 26

--- Words (Top: 10) ---
1. the: 27643 (0.53%)
2. and: 26728 (0.51%)
3. i: 20681 (0.40%)
4. to: 19198 (0.37%)
5. of: 18173 (0.35%)
6. a: 14613 (0.28%)
7. you: 13649 (0.26%)
8. my: 12480 (0.24%)
9. that: 11121 (0.21%)
10. in: 10967 (0.21%)

--- Letters (Top: 10) ---
1. e: 447204 (8.58%)
2. t: 329775 (6.33%)
3. o: 314600 (6.04%)
4. a: 289150 (5.55%)
5. i: 253990 (4.88%)
6. s: 248989 (4.78%)
7. n: 243262 (4.67%)
8. r: 237864 (4.57%)
9. h: 236868 (4.55%)
10. l: 170019 (3.26%)

--- Report End ---
```

## JSON Output

```rust
cargo r -- --file t8.shakespeare.txt --json
```

```json
{
    "report": {
        "info": {
            "file": "t8.shakespeare.txt",
            "top": 10,
            "total_letters": 5209313,
            "total_row_characters": 5458199,
            "total_unique_letters": 26,
            "total_unique_words": 28122,
            "total_words": 1418387
        },
        "letters": [
            {
                "count": 447204,
                "percentage": 8.584701538085938,
                "value": "e"
            },
            {
                "count": 329775,
                "percentage": 6.330489158630371,
                "value": "t"
            },
            {
                "count": 314600,
                "percentage": 6.039184093475342,
                "value": "o"
            },
            {
                "count": 289150,
                "percentage": 5.550635814666748,
                "value": "a"
            },
            {
                "count": 253990,
                "percentage": 4.875690937042236,
                "value": "i"
            },
            {
                "count": 248989,
                "percentage": 4.779689788818359,
                "value": "s"
            },
            {
                "count": 243262,
                "percentage": 4.66975212097168,
                "value": "n"
            },
            {
                "count": 237864,
                "percentage": 4.5661301612854,
                "value": "r"
            },
            {
                "count": 236868,
                "percentage": 4.54701042175293,
                "value": "h"
            },
            {
                "count": 170019,
                "percentage": 3.2637510299682617,
                "value": "l"
            }
        ],
        "words": [
            {
                "count": 27643,
                "percentage": 0.5306457877159119,
                "value": "the"
            },
            {
                "count": 26728,
                "percentage": 0.5130811333656311,
                "value": "and"
            },
            {
                "count": 20681,
                "percentage": 0.3970005214214325,
                "value": "i"
            },
            {
                "count": 19198,
                "percentage": 0.3685322701931,
                "value": "to"
            },
            {
                "count": 18173,
                "percentage": 0.34885597229003906,
                "value": "of"
            },
            {
                "count": 14613,
                "percentage": 0.2805168330669403,
                "value": "a"
            },
            {
                "count": 13649,
                "percentage": 0.2620115280151367,
                "value": "you"
            },
            {
                "count": 12480,
                "percentage": 0.23957093060016632,
                "value": "my"
            },
            {
                "count": 11121,
                "percentage": 0.2134830504655838,
                "value": "that"
            },
            {
                "count": 10967,
                "percentage": 0.21052679419517517,
                "value": "in"
            }
        ]
    }
}
```
