[![Pipeline](https://github.com/aldevv/katac/actions/workflows/pipeline.yml/badge.svg)](https://github.com/aldevv/katac/actions/workflows/pipeline.yml) 

Katac is a simple command-line application designed to streamline the process of practicing coding katas. It allows you to organize your katas by copying them into dedicated day folders and easily run them when you're done

# Features

- **Organized Practice:** Create day folders to neatly store katas for each day of practice.
- **Effortless Copying:** Copy a kata into the designated day folder with a single command.
- **Seamless Execution:** Run katas effortlessly from within their respective day folders.

# Install
## Releases
you can download the release for your specific OS and put it in your PATH

## Cargo
```bash
cargo install katac
```

# Dependencies
- make

# Usage
## create a kata
1. create a folder named `katas` 
2. add the name for a kata you want to create
```bash
mkdir -p katas/hello_world
```
3. add the skeleton, this is the entrypoint for each day
```go
// hello.go
func helloWorld() {
}

func main() {
    helloWorld()
}
```

## begin a new day
to begin a new day run the katac command with the kata or katas you want to do:
(it can also be a path)
```bash
# katac <kata_name>...
katac hello_world
```
this will create a `days` folder which will contain a `day1` containing your kata

## run your kata
you can run your kata if the kata has a Makefile (if you have make), or a run.sh (run.bat for windows)
```make
# Makefile
run:
	go run hello.go
```

after you are done writing the kata like in this example:
```go
import "fmt"
func helloWorld() {
    fmt.Println("hello world")
}

func main() {
    helloWorld()
}
```
you can run it by doing this:
```bash
# katac run [kata_name]...
katac run
```

## change katas folder and days folder permanently
you can create a katac.toml file that looks like this:
```toml
[katas]
katas_dir = "go-katas"
days_dir = "go-days"
```

## run random katas
you can run random katas by using the random command and giving the number of random katas
you want to do, like this
```bash
# this will copy 4 randomly selected katas from your katas directory to your days directory
katac random 4
```
### randomly select a subset of katas
if you want to choose the katas the random command will work on, you can add this property to
the katac.toml file
```toml
[katas]
random = ["Map", "LRU", "Trie", "Stack"]
```

# Contributing
If you have any ideas for improvements or find any issues, feel free to open an issue or submit a pull request.
