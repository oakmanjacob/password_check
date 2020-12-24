# The goal of this project is to create a program which takes a password string as an input and outputs the difficulty to type the string on different mobile keyboards

Jacob Oakman

## Keyboards will consist of

- Android Standard
- IOS Standard

## This program will need to take into account

- Switching Screens
- Number of buttons clicked
- Capitalization and Caps Lock toggle

## In the future this could also measure

- Distance between buttons
- Swipe gestures
- Password complexity

Finally, this could be adapted into a password generator for passwords that meet complexity requirements but aren't difficult to input.

## Capitalization

### Android

Currently, on my Galaxy S7 Edge, the capitalization button can exist in three states
lowercase -> capitalize -> caps lock

These can be transitioned through via a tap. You can also go back to lowercase from capitalize by typing a character.
For strings of capital letters, caps lock is only made efficient when the capitalized substring is three or more characters.
ie caps lock will always take two or three taps, two to initiate and if the substring isn't at the end of the string, one more to end the caps lock if there is more text after the substring.

### Lets try this with EBNF

```EBNF
whole_number = 0-9;
lowercase = a-z;
uppercase = A-Z;
extra = "," | "." | " ";
special_1 = "+" | "=" | "/" | "_" | "<" | ">" | "[" | "]" | "!" | "@" | "#" | "$" | "%" | "^" | "&" | "*" | "(" | ")" | "-" | "'" | "\" | "\"" | ":" | ";" | "," | "?";
special_2 = "`" | "~" | "\\" | "|" | "{" | "}";

keyboard_lower = (lowercase | whole_number | extra) , keyboard_lower | keyboard_lower, keyboard_upper | e;
keyboard_upper = (uppercase | whole_number | extra) , keyboard_lower | keyboard_caps;
keyboard_caps = (uppercase | whole_number | extra) , (keyboard_caps | keyboard_lower) | e;
keyboard_math = (special_1 | whole_number | extra) , keyboard_math;
```

EBNF is likely a bad plan due to looping definitions.
