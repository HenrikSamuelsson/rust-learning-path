# Enum

Enums makes it possible to have an element that is one of a set of possible values. For example the days of the week.

```Rust
enum WeekDay {
  Monday,
  Tuesday,
  Wednesday,
  Thursday,
  Friday,
  Saturday,
  Sunday,
}
```

Enums can in Rust have data that goes along with an particular enum entry.

```Rust
enum Location {
  Unknown,
  Known(Latitude, Longitude),
}
```
