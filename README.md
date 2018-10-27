# Rogue Creator

Roguelike engine.

Note that all of the below may not be 100% correct.

## JSON

RC uses JSON files for config. RC uses a `*.hub.json` file to load the game.

## Engine Workings

`glob` can be used throughout to access the global object. `.` is used to access individual data inside. Although the components must be declared in the hub file, it is read/write and the values begin uninitialised.

`data` can be used throughout to access the global read-only data. `.` is used to access individual data inside. This is specified in .json files and can only be read from.

In addition to `glob`, `data`, and local variables inside functions, there is always a level stack. The level stack can be accessed with `level` and `.`.

If a script is triggered by an entity function (`init`, `action`, `delete`), `this` can be used to access the current entity. It will equal `null` otherwise.

RC follows a predictable cycle:
* When the game begins the 'init' script is called (specified in hub file). This will probably initialise the `glob` object. It MUST initialise a layout, and usually a level too.
* Next, the `render` function is called (defined by the current layout).
* Next, if in ticking mode, the `tick` function will be called, then input will be awaited for a short time (depending on frame-rate). If in normal mode, input will be awaited depending on the input specified in the layout.
* When either script returns, render is re-called.

`tick` is often used to advance a cycle even when not in ticking mode.

Render can be called at any point, for example if one wants to show text changing mid-tick.

## Making Level:
Tiles must be defined. The level map can be made from these tiles.

Init the level, then create the map from the special functions (in `makemap`). Once the level tiles are placed, entites can be spawned in.

Entities must be initialised, then they can be stored wherever. Most likely, the only persistent entity will be the player and whatever they are holding: store these persistent entities in the global object. When entities are spawned into a level they are stored inside the level too - along with their local coordinates. The level owns an entity's location, and a reference to the rest of the entity's data.

Entities are spawned as active or static, which determines whether they are called in the `tick` command or not.

Once everything is spawned into the level, input is awaited. Once input is put in, it is translated to an input command (i.e., the "up" arrow key is translated to "move_up").

The input command calls a script which does some sort of action. This action may or may not call `tick` which will make all active entities take an action.


## Scripts

RC extensively makes use of a proprietary scripting language (similar to JS).

Below a brief documentation can be found.

### String Object
The string object can be used when visual formatting is desired. It is set up as below:
{
  text: String,
  length: Int/Null,
  colour: String/Null,
  options: String/List(String)/Null
}

#### Colours

#### Options

### Types

RC is dynamically typed for in-script variables, however json declared variables are statically typed. The types that exist are:
* Integer
* Coord (pair): `<x,y>`
* Float
* Text
* Bool
* List(type)
* Entity
* Level (?)
* object (instead of entity and level?)

New variables are declared with `var`. Entities cannot be declared in-script, these must be declared in json.

List components can be accessed with `\[\]` square brackets. Entity components can be accessed with the `.` dot.

`level` can be used to access the current level. Level objects act similarly to entities.

#### Type functions

Certain types have core functions that can be called with the `->` operator. They are listed for each type below.

##### Integer
* `to_text()`: Returns as text. Implicitly called when added to a text variable.
* `to_float()`: Returns as float. Implicitly called when added to a float variable.
* `abs()`: Returns absolute value.

##### Coord
* `x()`: Returns x coord as integer.
* `y()`: Returns y coord as integer.

##### Float
* `to_text()`: Returns as text. Implicitly called when added to a text variable.
* `ceiling()`: Rounds up and returns as integer.
* `floor()`: Rounds down and returns as integer.
* `round()`: Rounds up if fractional part is >= 0.5, rounds down otherwise, and returns as integer.
* `is_nan()`: Returns true if number is NaN.

##### Text
* `to_int()`: Returns as integer if possible. Crashes otherwise.
* `to_float()`: Returns as float if possible. Crashes otherwise.
* `len()`: Returns length of text string.

##### List
* `append(var)`: Appends `var` to the list. Modifies list in-place.
* `concat(list)`: Appends the entirety of `list` to the list. Modifies list in-place.
* `len()`: Returns length of list.
* `type()`: Returns type of internal data as text.

##### Entity
* `action()`: Runs pre-action, then action, then post-action scripts as defined in json.
* `init()`: Re-initialises entity.
* `set_action(text)`: Sets the action function, using "text" as a script.
* `clone()`: Clones the entity and returns a handle to the new entity. Adds instance to dynamic instance list if dynamic.
* `type()`: Returns type of entity as text.
* `name()`: Returns entity name as text.

### Functions

RC allows the definition and calling of functions. Functions can take a variable number of arguments, and return data. None of these values are explicitly typed. They are defined as below:

```
func function_name(argument_1, argument_2) {
    ...
    return x;
}
```

Functions can be called from a different package by adding the following statement to the top of a file or function: `import package_name as id;`. Then, functions can be called as follows: `id::function_name()`. `package_name` can be an engine function, or a string which contains the file location, e.g. `"path/to/file.scr"`. The file path is rooted at the location of the .hub.json file.

Alternatively, they can be called from an engine package by using the `:` specifier: `:lib_name:function_name();`.

There are a number of engine functions which can be called. They exist in different packages which must be `import`ed.

#### Engine functions
Each of the following functions are found inside the package noted. They must be imported before use.

##### Entity manipulation: entity
* `create_entity(string)`: Creates new instance of entity. Runs its `init` script. Returns id. Adds ID to the level's instance list.
* `delete(entity)`: Despawns instance (if spawned), runs its 'delete' script, and then removes altogether.
* `run_actions()`: Runs pre, action and post scripts for all instances in level instance list.

##### Layout: txtrend
The following should only be found in the `render` function (or sub-functions):
* `place_print(coord, coord)`: Places print between screen coords specified. Also determines current size of print buffer.
* `place_map(coord, coord)`: Places level map at screen coords specified. Level MUST be LOADED before this is called.
* `place_text(text, coord, coord)`: Places text between coords.

More (to do with colouring text, centering text etc) will be coming soon.

##### Layout and map display: layout

* `print(text)`: Adds text to the print buffer, which can be displayed on screen. If the text is longer than the display length, it is split into multiple entries.
* `next_print()`: Shows the next entry in the print buffer.
* `clear_print()`: Clears entire print buffer and blanks.

* `show_map()`: reveals entire map for rendering.
* `hide_map()`: hides entire map so it isn't rendered.
* `map_display(coord, coord)`: Selects what to show with `place_map`. (defaults to the whole map from top left)
* `show_tiles(coord)`: reveals all connected tiles of the same type from coord (if possible).
* `hide_tiles(coord)`: hides all connected tiles of the same type from coord (if possible).
* `show_surround(coord)`: reveals the tiles around and including coord.
* `hide_surround(coord)`: hides the tiles around and including coord.

##### Level interaction: level
* `create(text)`: Creates level of name "text". Returns integer id to refer to the level with.
* `delete(integer)`: Deletes level of id "integer", if it exists.
* `load(integer)`: Makes the active level id "integer". Uses it to render the map, etc.
* `clone(integer)`: Clones the level id "integer", and returns a new id to refer to the new level with.

* `level_data()`: Gets a mutable reference to level data.
* `instance_at(coord)`: Returns instance if one exists at coords specified. Returns null if nothing is at those coords.
* `location_of(entity)`: Returns coord containing location of instance.

##### Map creation: makemap
* `fill_tile(string, coord, <coord>)`: Draw tiles between coords, or at first coord if second is not specified.
* `draw_line(string, coord, coord)`: Draws a line between the coords.
* `spawn_entity(entity, coord)`: Spawns instance at coords specified. If not possible, then returns false. If successful, returns true.
* `despawn_entity(entity)`: Despawns instance, however keeps data so it can be re-spawned.

##### Flow control: control
* `wait(integer)`: Waits "integer" milliseconds.

* `exit()`: Exits script execution engine and returns to last JSON call.

* `end_game()`: Runs 'end' script. Once 'end' script returns, the game ends and engine closes.

##### Global data access: global
* `get()`: gets a mutable reference to the global object.
* `data()`: gets a reference to the global data object.
* `set_layout(text)`: Changes active layout to "text", as defined in json. This MUST be called before the end of the `init` script.

##### Mathematical: math
* `sin(n)`: Runs sin function on number.
* `cos(n)`: Runs cos function on number.
* `pow(b,e)`: Returns b raised to the power of e.
* `sqrt(n)`: Returns square root of number.
* `rand(integer, integer)`: Returns an integer in the range specified (inclusively).
