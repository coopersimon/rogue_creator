# Rogue Creator

Roguelike engine.

## JSON

RC uses JSON files for config. RC uses a *.hub.json file to load the game.

## Engine Workings

`glob` can be used throughout to access the global object. `.` is used to access individual data inside. Although the components must be declared in the hub file, it is read/write and the values begin uninitialised.

`data` can be used throughout to access the global read-only data. `.` is used to access individual data inside. This is specified in .json files and can only be read from.

In addition to `glob`, `data`, and local variables inside functions, there is always a level stack. The level stack can be accessed with `level` and `.`.

If a script is triggered by an entity function (`init`, `action`, `delete`), `this` can be used to access the current entity. It will equal `null` otherwise.

RC follows a predictable cycle:
* When the game begins the 'start' script is called (specified in hub file). This will probably initialise the `glob` object. It MUST initialise a layout, and usually a level too.
* Next, the `render` function is called (defined by the current layout).
* Next, if in ticking mode, the `tick` function will be called, then input will be awaited for a short time (depending on frame-rate). If in normal mode, input will be awaited depending on the input specified in the layout.
* When either script returns, render is re-called.

`tick` is often used to advance a cycle even when not in ticking mode.

## Scripts

RC extensively makes use of a proprietary scripting language (similar to JS).

Below a brief documentation can be found.

### Types

RC is dynamically typed for in-script variables, however json declared variables are statically typed. The types that exist are:
* Integer
* Coord: `<x,y>`
* Float
* Text
* List(type)
* Entity
* Level (?)
* object (instead of entity and level?)

New variables are declared with `var`. Entities cannot be declared in-script, these must be declared in json.

List components can be accessed with `\[\]` square brackets. Entity components can be accessed with the `.` dot.

`level` can be used to access the current level. Level objects act similarly to entities.

#### Type functions

Certain types have engine functions that can be called with the `->` operator. They are listed for each type below.

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

Functions can be called from a different file by using the `:` specifier: `:path/to/file.scr:function_name();`. The file path is rooted at the location of the .hub.json file.

There are a number of engine functions which can be called. The `glob` keyword can be optionally used as a specifier to access global functions.

#### Engine functions

##### Entity manipulation
* `create_dynamic(integer)`: Creates new "id" instance of entity. Runs its `init` script. Returns a handle to access the instance with. Adds instance to the level's dynamic instance list.
* `create_static(integer)`: As above, but does not add to the dynamic instance list.
* `delete(entity)`: Despawns instance (if spawned), runs its 'delete' script, and then removes altogether. If it was dynamic, it is removed from the dynamic instance list.
* `spawn_entity(entity, coord)`: Spawns instance at coords specified. If not possible, then crashes.
* `despawn_entity(entity)`: Despawns instance, however keeps data so it can be re-spawned.
* `location_of(entity)`: Returns coord containing location of instance.
* `instance_at(coord)`: Returns instance if one exists at coords specified. Returns null if nothing is at those coords.
* `run_dynamic_actions()`: Runs action script for all instances in dynamic instance list.

##### Layout
* `layout(text)`: Changes active layout to "text", as defined in json. This MUST be called before the end of the `start` script.

The following can only be found in the `render` function (or sub-functions):
* `place_print(coord, coord)`: Places print between screen coords specified. Also determines current size of print buffer.
* `show_map(coord, coord)`: Selects what to show with `place_map`. (defaults to the whole map from top left)
* `place_map(coord, coord)`: Places level map at screen coords specified. Level MUST be LOADED before this is called.
* `place_text(text, coord, coord)`: Places text between coords.

More (to do with colouring text, centering text etc) will be coming soon.

##### Level
* `create_level(text)`: Creates level of name "text". Returns integer id to refer to the level with.
* `delete_level(integer)`: Deletes level of id "integer", if it exists.
* `load_level(integer)`: Makes the active level id "integer". Uses it to render the map, etc.
* `clone_level(integer)`: Clones the level id "integer", and returns a new id to refer to the new level with.

##### Flow control
* `tick()`: Run the tick script, as specified in the hub file.
* `await_input()`: Waits for a single input. Returns "text" containing the key inputted.
* `set_input(text)`: Sets input to file specified in "text".

##### Misc
* `print(text)`: Adds text to the print buffer, which can be displayed on screen. If the text is longer than the display length, it is split into multiple entries.
* `next_print()`: Shows the next entry in the print buffer.
* `clear_print()`: Clears entire print buffer and blanks.

* `wait(integer)`: Waits "integer" milliseconds.

* `rand(integer, integer)`: Returns an integer in the range specified (inclusively).

* `exit()`: Exits script execution engine and returns to last JSON call.

* `end_game()`: Runs 'end' script. Once 'end' script returns, the game ends and engine closes.
