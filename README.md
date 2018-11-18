# Rogue Creator

Roguelike engine.

Note that all of the below may not be 100% correct.

## Dev notes and TODOs:
Short term
* Clean up state funcs
* Chain calls in library funcs

Longer term:
* Input mapping
* Text object: implement colours & effects
* Global database
* Sub objects
* More map display functions / reading map functions
* Actions (and and post- actions) - closure for ordering
* More map drawing functions (fill area?)
* Tick function
* Cleanup and finish rendering functions
* Cleanup of MainCommand enum
* Proper error handling in engine - returning errors, lots of critical errors, using custom error object

### Longer term:
* Real time ticking option
* Tile based 2d graphics
* Multithreading
* Sounds
* Events


## JSON

RC uses JSON files for config. RC uses a `hub.json` file to load the game.

Throughout there are multiple places where either data or script snippets are defined. Script snippets are an expression which returns a value. These values that are returned may or may not be used (more details below).

## Engine Workings
The cycle:
* First, `init` in the `hub.json` file is called. This **must** be defined. This **must** call the `set_layout` function to set the initial layout. It _should_ setup a level and load it. The value that is returned from this is used as the global object, which can be accessed via the `glob::obj` function at any time later.
* After this, the `render` script in the chosen layout is called. Every layout **must** define a `render` function. It doesn't have to do anything, but it _should_ call functions from the `txtrend` package.
* Next, an input key is read from the user. The chosen layout maps input keys to scripts. A `default` can be set for any key that isn't explicitly defined. These scripts _can_ call the `control::tick` function, which runs the `tick` script defined in the hub file after the key script returns.
* The `tick` script runs if necessary, then the game is re-rendered and then awaits more input. This loop continues until `control::terminate_game` is called.

### Making Level:
TODO: Write about levels

### Layouts:
Layouts define a `render` function and feature `inputs`, which map to script expressions. Inputs can be single characters or special keys, listed below. A "default" can also be defined, which matches any other key input.
* enter
* backspace
* space
* tab
* arrow keys: left, up, right, down


## Scripts

RC extensively makes use of my [modscript](https://github.com/coopersimon/modscript) scripting language (similar to JS).

Some object formats are used for engine functions. These are listed below.

### String Object
The string object can be used when visual formatting is desired. It is set up as below:
{
  text: String,
  len: Int/Null,
  colour: String/Null,
  options: String/List(String)/Null
}

#### Colours

#### Options


### Functions

RC allows the definition and calling of functions. Functions can take a variable number of arguments, and return data. None of these values are explicitly typed. They are defined as below:

```
func function_name(argument_1, argument_2) {
    ...
    return x;
}
```

Functions can be called from a different package by adding the following statement to the top of a file or function: `import package_name as id;`. Then, functions can be called as follows: `id::function_name()`. `package_name` can be an engine function, or a string which contains the file location, e.g. `"path/to/file.scr"`. The file path is rooted at the location of the .hub.json file.

There are a number of engine functions which can be called. They exist in different packages which must be `import`ed.

#### Engine functions
Each of the following functions are found inside the package noted. They must be imported before use.

`coord` below can be represented by a few types. `[x,y]` in a list, or `{x: int, y: int}` as an object.

##### Entity manipulation: `entity`
* `create_global(string)`: Creates new instance of entity. Runs its `init` script. Returns id. Adds ID to the global instance list.
* `create(string)`: Creates new instance of entity. Runs its `init` script. Returns id. Adds ID to the level's instance list.
* `delete(integer)`: Despawns instance (if spawned), runs its 'delete' script, and then removes altogether.
* `clone(integer)`: Clones the entity id "integer", and returns a new id to refer to the new entity with.
* `data(integer)`: Gets a mutable reference to entity data (returned by init script).
* `run_actions()`: Runs pre, action and post scripts for all instances in level instance list.

##### Layout: `txtrend`
The following should only be found in the `render` function (or sub-functions):
* `place_print(coord, coord)`: Places print between screen coords specified. Also determines current size of print buffer.
* `place_map(coord, coord)`: Places level map at screen coords specified. Level MUST be LOADED before this is called.
* `place_text(text, coord, coord)`: Places text between coords.

More (to do with colouring text, centering text etc) will be coming soon.

##### Print box: `pbox`
* `print(text)`: Adds text to the print buffer, which can be displayed on screen. If the text is longer than the display length, it is split into multiple entries.
* `next()`: Shows the next entry in the print buffer.
* `clear()`: Clears entire print buffer and blanks.

##### Map: `map`
* `display(coord)`: Selects what to show with `place_map`, specifying the top left coordinate (defaults to (0,0))
* `show_all()`: reveals entire map for rendering.
* `hide_all()`: hides entire map so it isn't rendered.
* `show_surround(coord)`: reveals the tiles around and including coord.
* `hide_surround(coord)`: hides the tiles around and including coord.

##### Level interaction: `level`
* `create(string)`: Creates level of name "text". Returns integer id to refer to the level with.
* `delete(integer)`: Deletes level of id "integer", if it exists.
* `load(integer)`: Makes the active level id "integer". Uses it to render the map, etc.
* `clone(integer)`: Clones the level id "integer", and returns a new id to refer to the new level with.

* `data()`: Gets a mutable reference to level data (returned by init script).
* `instance_at(coord)`: Returns instance if one exists at coords specified. Returns null if nothing is at those coords.
* `location_of(integer)`: Returns coord containing location of instance.

##### Map creation: `makemap`
* `fill_tile(string, coord, <coord>)`: Draw tiles between coords, or at first coord if second is not specified.
* `draw_line(string, coord, coord)`: Draws a line between the coords.
* `spawn(integer, coord)`: Spawns instance at coords specified. If not possible, then returns false. If successful, returns true.
* `despawn(integer)`: Despawns instance, however keeps data so it can be re-spawned.
* `move_entity(integer, coord)`: Moves an entity from its existing position to a new position specified.
* `set_entity_display(integer, text)`: Sets visuals for entity to the text object specified. Text contained must be a single character.

##### Flow control: `control`
* `wait(integer)`: Waits "integer" milliseconds.
* `exit()`: Exits script execution engine and returns to last JSON call.
* `tick()`: Calls the tick script after the key input has returned.
* `end_game()`: Runs 'end' script.
* `terminate_game()`: Closes the engine.

##### Global data access: `glob`
* `obj()`: gets a mutable reference to the global object.
* `data()`: gets a reference to the global database.
* `set_layout(text)`: Changes active layout to "text", as defined in json. This MUST be called before the end of the `init` script.
* `last_key()`: Gets the last key pressed as text.

##### Mathematical: `math`
* `sin(n)`: Runs sin function on number.
* `cos(n)`: Runs cos function on number.
* `pow(b,e)`: Returns b raised to the power of e.
* `sqrt(n)`: Returns square root of number.
* `rand(integer, integer)`: Returns an integer in the range specified (inclusively).
