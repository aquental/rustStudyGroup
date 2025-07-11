## Problem Requirements:

- The user should be able to parse different types of files: CSV, XML, and JSON with files named file.csv, file.xml, and file.json, respectively.
- Each file should have specific parsing logic. In real life, it will be a complex parsing logic based on the file type, but in this case you can just print a message indicating the type of file being parsed.
- It should be possible to change the parsing logic without changing the existing codebase.
- The file-processing logic should be easily testable and maintainable.
- You need to choose between two design patterns, Strategy or Factory Method, and then implement the solution.

---

### Strategy Pattern

Strategy Pattern: This pattern is designed to encapsulate a family of interchangeable algorithms (in this case, parsing logic for CSV, XML, JSON) and make them swappable at runtime.

It allows you to define a common interface for parsing and switch the specific parsing logic based on the file type without altering the core codebase.

The Strategy pattern aligns better with the requirement to handle different parsing logics (behaviors) for each file type.
