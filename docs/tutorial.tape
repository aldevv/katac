# VHS tape for the docs/tutorial.md GIF.
# Story: scaffold a custom Add kata, fill it in with nano, see the test fail,
# fix the implementation with nano, see it pass.
# Render with:  make demos   (or run vhs directly with target/release on PATH)
# Requires `vhs` + `ttyd` + `ffmpeg` + `nano` + `python3`.
#
# `nano -I` skips ~/.nanorc so the recording is reproducible regardless of
# the renderer's local editor config (autoindent, tab-expand, etc.).

Output docs/tutorial.gif

Set FontSize 16
Set Width 1100
Set Height 750
Set Theme "Catppuccin Mocha"
Set Shell "bash"
Set TypingSpeed 75ms
Set PlaybackSpeed 0.85
Set Padding 20

Hide
Type "cd $(mktemp -d) && export PS1='$ ' && clear"
Enter
Sleep 600ms
Show

# 1. scaffold an empty kata — see what katac creates
Type "katac new Add"
Enter
Sleep 1200ms

Type "ls katas/Add/"
Enter
Sleep 1500ms

# only a Makefile — show its placeholder contents
Type "cat katas/Add/Makefile"
Enter
Sleep 2800ms

# 2. wire the Makefile up to python3
Hide
Type "clear"
Enter
Sleep 200ms
Show

Type "nano -I katas/Add/Makefile"
Enter
Sleep 1500ms
# nano opens on the 2-line placeholder. Cut both lines.
Ctrl+K
Sleep 200ms
Ctrl+K
Sleep 400ms
# type the real run target (Tab for the recipe indent)
Type "run:"
Enter
Tab
Type "python3 test_add.py"
Sleep 400ms
Ctrl+O
Sleep 500ms
Enter
Sleep 500ms
Ctrl+X
Sleep 1000ms

# 3. write the implementation skeleton in nano
Type "nano -I katas/Add/add.py"
Enter
Sleep 1500ms
Type "def add(a, b):"
Enter
Type "    return 0  # TODO"
Sleep 500ms
Ctrl+O
Sleep 500ms
Enter
Sleep 500ms
Ctrl+X
Sleep 1000ms

# 4. write the test in nano
Type "nano -I katas/Add/test_add.py"
Enter
Sleep 1500ms
Type "from add import add"
Enter
Type "assert add(2, 3) == 5"
Enter
Type "print('PASS')"
Sleep 500ms
Ctrl+O
Sleep 500ms
Enter
Sleep 500ms
Ctrl+X
Sleep 1000ms

# 5. peek at everything we wrote
Hide
Type "clear"
Enter
Sleep 200ms
Show

Type "cat katas/Add/Makefile katas/Add/add.py katas/Add/test_add.py"
Enter
Sleep 3500ms

# 6. start the kata and run — fails because add() returns 0
Hide
Type "clear"
Enter
Sleep 200ms
Show

Type "katac Add"
Enter
Sleep 1200ms

Type "katac run"
Enter
Sleep 3800ms

# 7. open the file in nano and fix it
Hide
Type "clear"
Enter
Sleep 200ms
Show

Type "nano -I days/day1/Add/add.py"
Enter
Sleep 1500ms
# cursor opens at line 1, col 0. Move to end of line 2 ("    return 0  # TODO"),
# delete the 9 trailing chars ("0  # TODO"), type "a + b".
Down 1
Sleep 300ms
Ctrl+E
Sleep 300ms
Backspace@40ms 9
Sleep 400ms
Type "a + b"
Sleep 400ms
Ctrl+O
Sleep 500ms
Enter
Sleep 500ms
Ctrl+X
Sleep 1000ms

# 8. confirm the fix
Type "cat days/day1/Add/add.py"
Enter
Sleep 2200ms

# 9. run again — PASS
Type "katac run"
Enter
Sleep 3000ms
