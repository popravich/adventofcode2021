from collections import Counter

SEGMENT_FREQ = {
    'a': 8,
    'b': 6,
    'c': 8,
    'd': 7,
    'e': 4,
    'f': 9,
    'g': 7,
}

SEGMENTS_TO_DIGITS = {
    "abcefg": 0,
    "cf": 1,
    "acdeg": 2,
    "acdfg": 3,
    "bcdf": 4,
    "abdfg": 5,
    "abdefg": 6,
    "acf": 7,
    "abcdefg": 8,
    "abcdfg": 9,
}


def main():
    with open('../src/input/day8.txt') as f:
        data = list(f)

    freq_to_segment = {}
    for s, f in SEGMENT_FREQ.items():
        if f in freq_to_segment:
            freq_to_segment[f] += s
        else:
            freq_to_segment[f] = s

    result = 0
    for line in data:
        indicator_patterns, indications = map(str.strip, line.split(' | '))

        one = four = ""
        freq = Counter()
        for pattern in indicator_patterns.split(' '):
            if len(pattern) == 2:
                assert one == "", "one is alredy set"
                one = pattern
            elif len(pattern) == 4:
                assert four == "", "four is already set"
                four = pattern
            freq.update(pattern)

        transcoding_table = dict()
        for segment, fr in freq.items():
            real_segments = freq_to_segment[fr]
            if segment in transcoding_table:
                transcoding_table[segment] += real_segments
            else:
                transcoding_table[segment] = real_segments

        segment_c = [c for c in one if len(transcoding_table[c]) > 1][0]
        transcoding_table[segment_c] = 'c'
        for k, v in transcoding_table.items():
            if v == 'ac':
                transcoding_table[k] = 'a'

        segment_d = [c for c in four if len(transcoding_table[c]) > 1][0]
        transcoding_table[segment_d] = 'd'
        for k, v in transcoding_table.items():
            if v == 'dg':
                transcoding_table[k] = 'g'

        assert all(len(v) == 1 for v in transcoding_table.values())

        for (i, indication) in enumerate(reversed(indications.split(' '))):
            real = ''.join(sorted(transcoding_table[c] for c in indication))
            digit = SEGMENTS_TO_DIGITS[real]
            result += digit * 10**i
    print("Part2 result:", result)


if __name__ == '__main__':
    main()
