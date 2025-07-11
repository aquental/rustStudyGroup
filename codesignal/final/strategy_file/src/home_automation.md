# Adapter

This pattern is designed to make two incompatible interfaces work together by converting the interface of one system (e.g., OldLightSystem with switch_on) to match the interface expected by another system (e.g., a trait with turn_on).  
It acts as a bridge between the old system and the new requirement without modifying the original code.

Build a home automation system where an existing lighting system needs to be controlled using a new method name.

Here’s the scenario:

- You have a struct OldLightSystem with a method switch_on to switch on the light.
- There's a new requirement to integrate this with a system that expects to control the light through a trait with a method turn_on.
- Your task is to ensure that the existing OldLightSystem can work with the new trait. Implement this solution and test it in the main function to verify that the old lighting system can be controlled using the new method.

What pattern is the best fit: Decorator or Adapter, and why?
