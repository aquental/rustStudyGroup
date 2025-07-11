# Smart Home

system where various smart home devices can be controlled with different actions using a remote control.
The system should be easily extensible and maintainable.

## Problem Requirements:

- Each device should have specific actions such as turning on or turning off. For simplicity, print a message indicating the action taken.
- It should be possible to change the actions of a device without altering the existing codebase.
- The device-control logic should be easily testable and maintainable.
- You need to choose between two design patterns: Command or Strategy depending on the requirements, and implement the system accordingly.

---

## Command Pattern:

This pattern encapsulates a request (e.g., turning a device on or off) as an object, allowing you to parameterize clients with different requests, queue or log requests, and support undoable operations.

It’s ideal for scenarios where actions are discrete commands that can be issued, stored, or modified independently.

The Command pattern is the better choice for this use case because it:

- Encapsulates individual actions (turn on, turn off) as command objects, making them modular and reusable.
- Allows adding new actions (e.g., restart) without modifying existing code, adhering to the Open/Closed Principle.
- Supports easy testing of individual commands and the controller, as shown in the test cases.
- Provides flexibility for runtime command management (e.g., queuing or executing commands dynamically).
- Keeps the codebase maintainable by isolating action logic in separate command structs.
