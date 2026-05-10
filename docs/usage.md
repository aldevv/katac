# Usage

## Create a kata

1. Create a folder named `katas`.
2. Add a folder for the kata you want to create:
   ```bash
   mkdir -p katas/hello_world
   ```
3. Add the skeleton — this is the entrypoint for each day:
   ```go
   // hello.go
   func helloWorld() {
   }

   func main() {
       helloWorld()
   }
   ```

## Begin a new day

To begin a new day, run `katac` with the kata or katas you want to do
(arguments may also be paths):

```bash
# katac <kata_name>...
katac hello_world
```

This creates a `days` folder containing `day1/` with your kata.

## Run your kata

You can run your kata if it has a `Makefile` (and `make` is on `PATH`),
or a `run.sh` (`run.bat` on Windows):

```make
# Makefile
run:
	go run hello.go
```

After you finish writing the kata, e.g.:

```go
import "fmt"
func helloWorld() {
    fmt.Println("hello world")
}

func main() {
    helloWorld()
}
```

run it with:

```bash
# katac run [kata_name]...
katac run
```

## Change `katas` and `days` folders permanently

Create a `katac.toml` file:

```toml
[katas]
katas_dir = "go-katas"
days_dir = "go-days"
```

## Run random katas

Pick N random katas from your `katas` directory:

```bash
# copies 4 randomly selected katas from your katas directory into days/
katac random 4
```

### Restrict the random pool

To control which katas `random` picks from, add this to `katac.toml`:

```toml
[katas]
random = ["Map", "LRU", "Trie", "Stack"]
```

## Initialize from examples

Interactively select and copy example katas (uses the templates baked
into the binary):

```bash
katac init
```

## Upgrade

Update to the latest release:

```bash
katac upgrade
```
