[![Pipeline](https://github.com/aldevv/katac/actions/workflows/pipeline.yml/badge.svg)](https://github.com/aldevv/katac/actions/workflows/pipeline.yml) 
[![Releases](https://github.com/aldevv/katac/actions/workflows/release.yml/badge.svg)](https://github.com/aldevv/katac/actions/workflows/release.yml) 

Katac is a simple command-line application designed to streamline the process of practicing coding katas. It allows you to organize your katas by copying them into dedicated day folders and easily run them when you're done

# Features

- **Organized Practice:** Create day folders to neatly store katas for each day of practice.
- **Effortless Copying:** Copy a kata into the designated day folder with a single command.
- **Seamless Execution:** Run katas effortlessly from within their respective day folders.

# Usage
## create a kata
1. create a `katas` folder
2. add the name for a kata you want to create, let's try `hello_world`
```bash
mkdir -p katas/hello_world
```
3. add the skeleton you will begin the kata with each day
```go
// hello.go
func helloWorld() {
}

func main() {
    helloWorld()
}
```

if you want to run the kata after finishing a day, you will also need to add a Makefile like
this:
```make
# Makefile
run:
    go run hello.go
```

## begin a new day
to begin a new day run the katac command with the kata or katas you want to do:
```bash
# katac <kata_name>...
katac hello_world
```
this will create a `days` folder which will contain a `day1` containing your kata

## run your kata
after you are done writing your kata like in this example:
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

# Install
## Releases
you can download the release for your specific OS and put it in your PATH

## Cargo
```bash
cargo install katac
```

# Dependencies
- make


# Contributing

If you have any ideas for improvements or find any issues, feel free to open an issue or submit a pull request.
