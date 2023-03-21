# str_id

Fixed length string for legible IDs.

## Features

* **Copy**: Like &str but without the lifetimes.
* **Fast**: Uses SSE4.2 instructions for string comparision.
* **Const**: Create IDs at comile time then simply copy them.
* **Serde**: Names are simply strings in text based data formats.
