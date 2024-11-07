import re


def match_patterns(words, patterns):
    """
    Matches patterns against a list of words and returns matching patterns.

    Args:
      words: A list of words to match against.
      patterns: A list of patterns in string format.

    Returns:
      A list of matching patterns, preserving their original order.
    """

    matching_patterns = []
    for pattern in patterns:
        for word in words:
            if re.match(pattern, word):
                matching_patterns.append(pattern)
                break  # Move to the next pattern if a match is found
    return matching_patterns


def main():
    words = ["apple", "banana", "orange", "pineapple"]
    patterns = ["^a.*", ".*n$", "p.*e"]
    mp = match_patterns(words, patterns)
    print(mp)


if __name__ == "__main__":
    main()
