def is_balanced(str):
    """
    Checks if a string containing brackets is balanced.

    Args:
      str: The input string.

    Returns:
      True if the string is balanced, False otherwise.
    """
    stack = []
    opening_brackets = '([{'
    closing_brackets = ')]}'
    bracket_pairs = {
        '(': ')',
        '{': '}',
        '[': ']'
    }

    for char in str:
        if char in opening_brackets:
            stack.append(char)
        elif char in closing_brackets:
            if not stack or bracket_pairs[stack.pop()] != char:
                return False

    return len(stack) == 0


def main():
    str1 = "({[]})"
    print(is_balanced(str1))  # Output: True
    str2 = "({[}])"
    print(is_balanced(str2))  # Output: False


if __name__ == "__main__":
    main()
