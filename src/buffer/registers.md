# Register rules

| Name  | Type           | Behaviour                              | Implemented             |
| ----- | -------------- | -------------------------------------- | ----------------------- |
| `"`   | unnamed        | default register for yank/delete/paste | Yes                     |
| `0`   | yank           | last yanked text only                  | Yes                     |
| `1–9` | delete history | linewise deletes; `"1` = newest        | Needs multiline support |
| `-`   | small delete   | deletes within one line                | Yes                     |
| `a–z` | named          | user-defined (overwrite)               | Yes                     |
| `A–Z` | named append   | same as a–z but append                 | No                      |
| `+`   | clipboard      | system clipboard                       | No                      |
| `*`   | primary        | X11 primary selection                  | No                      |
| `%`   | readonly       | current file name                      | Meaningless (no file)   |
| `#`   | readonly       | alternate file name                    | Meaningless (no file)   |
| `:`   | readonly       | last command                           | Needs command support   |
| `/`   | readonly       | last search pattern                    | Needs search support    |
| `.`   | readonly       | last inserted text                     | No                      |
| `=`   | expression     | evaluate Vimscript expression          | No                      |
| `_`   | black hole     | discard content                        | Yes                     |
