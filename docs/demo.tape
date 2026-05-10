# VHS tape for the README demo.
# Render with:  make demos   (or run vhs directly with target/release on PATH)
# Requires `vhs` + `ttyd` + `ffmpeg`.

Output docs/demo.gif

Set FontSize 16
Set Width 1100
Set Height 700
Set Theme "Catppuccin Mocha"
Set Shell "bash"
Set TypingSpeed 60ms
Set PlaybackSpeed 1.0
Set Padding 20

Hide
Type "cd $(mktemp -d) && export PS1='$ ' && clear"
Enter
Sleep 600ms
Show

# 1. init — pick language (Go is the default) and the Queue kata
Type "katac init"
Sleep 400ms
Enter
Sleep 1500ms
Enter
Sleep 1000ms
Down@180ms 5
Space
Sleep 400ms
Down@180ms 1
Space
Sleep 400ms
Down@180ms 1
Space
Sleep 600ms
Enter
Sleep 1800ms

# 2. start today's kata — copies the template into days/day1/
Type "katac Queue"
Sleep 300ms
Enter
Sleep 1200ms

Type "ls days/day1/"
Sleep 300ms
Enter
Sleep 1500ms

# 3. run it — go test fails because Queue is unimplemented. that's the kata.
Type "katac run"
Sleep 300ms
Enter
Sleep 3500ms

# 4. tomorrow: random picks N katas for you
Type "katac random 2"
Sleep 300ms
Enter
Sleep 1500ms

Type "ls days/"
Sleep 300ms
Enter
Sleep 2500ms
