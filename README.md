# teefriends
A program that finds online Teeworlds friends by querying every server.
The result gets stored in the data dir, and can be output from there.

The program uses the request_info_6_ex request, and can parse Info6, Info6Ex and Info6ExMore answers.

## help

```console
Usage: teefriends [OPTIONS]

Options:
  -c, --count    Print friend count
  -n, --names    Print friend names
  -f, --fetch    Update friend storage
  -h, --help     Print help
  -V, --version  Print version
```
