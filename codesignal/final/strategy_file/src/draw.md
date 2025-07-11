# Composite

This pattern is designed to handle part-whole hierarchies, where individual objects (leaf nodes, like Circle or Square) and compositions of objects (composite nodes, like a NestedWindow) are treated uniformly.  
It’s ideal for scenarios where a container (the nested window) needs to manage a collection of components (shapes) and perform operations (like draw) on them as if they were a single entity.

System where different shapes can be added and combined. Each shape will have its own draw method to show it is being drawn.

Here's what you need to do:

- Create simple shapes like circles and squares using individual components.
- Create a container (a nested window) that can hold multiple shapes.
- Ensure each shape has a draw method that prints a message to the console indicating which shape is being drawn.
- Create the nested window, add the shapes to it, and call the draw method to draw all the shapes at once.

---
