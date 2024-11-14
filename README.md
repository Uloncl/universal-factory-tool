# Universal Factory Tool

This Project is inteded to be a tool similar to the satisfactory modeller tool or the various other satisfactory planning tools available, however this tool is intended to be able to plan factories in any factory game that uses the compatable mechanics of recipes with inputs and outputs.

### This tool should be capable of the following:
1. [ ] the ability to place nodes representing recipes on an area
  - [ ] a node should have these various properties
    - [ ] a name
    - [ ] a machine
    - [ ] an optional input resource(s)
    - [ ] an optional output resources
    - [ ] an optional power requirement
2. [ ] the ability to connect the inputs and output resources of nodes to each other
  - [ ] a connection should have these various properties
    - [ ] a calculated amount based on input required by the target node/recipe
    - [ ] a colour
    - [ ] an optional method (belt/train/truck/etc)
    - [ ] an optional calculated surplus in the case that the output is larger than the input amount in someway
      - [ ] e.g. a byproduct of a recipe cannot be totally consumed by a target node
    - [ ] a display method which should be either smooth or sharp
      - [ ] a smooth conenction would be created as a bezier line where its nodes can be adjusted as well as allowing additional nodes to be created allowing full customisation of where the line goes
      - [ ] a sharp line should connect to its target only through 90<sup>o</sup> (or potentially 45<sup>o</sup> ) corners
3. [ ] customisability/moddability through having each games assets defined externally from the executable
  - [ ] each supported game (at a minimum this would be Satisfactory, Factorio and Dyson Sphere Program) would have its own json file which would define a list of raw recources, recipes, machines, transport methods and special nodes (e.g. Space Elevator for Satisfactory or a Dyson Sphere Vertical Launching Silo)
4. [ ] the ability to assign a group of nodes to a "factory"
  - [ ] a factory would display as a square that encloses the selected nodes
  - [ ] a factory would function similarly to a node and would inherit/sum all of the values of the nodes it contains
  - [ ] a factory would inherit all unconnected inputs and outputs of the nodes it contains, it should display these inherited resources at the edge of its assigned area where they are automatically connected to the matching internal recipes as well as allowing external nodes and factories to connect to them
  - [ ] potentially the ability to define an empty factory and have the factory generate the most efficient production chain automatically based on predefined inputs and outputs
5. [ ] the ability to connect nodes and factories to the electrical grid(s)
  - [ ] power switch nodes allowing grids to be defined and separated from each other and potentially named
  - [ ] ability to calculate the exact amount of power each grid and the overall grid would require
6. [ ] labels should be able to be created anywhere on the page and should provide the following features
  - [ ] basic text editing with text colour, size and style all editable
    - [ ] potentially full markdown with lists, tables, links, checkboxes depends on the egui framework i intend to use
  - [ ] other style options such as the option to have a box around the text as well as the colour of the box edge and background
7. [ ] snapping options with an adjustable and displayable grid
  - [ ] defaults to a global grid except within factories which should have their own local grid that should allow nodes within the factory to be placed snugly within the factory borders