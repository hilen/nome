# nome

A metronome. One Rust crate on [hilen](https://github.com/hilen/hilen), so the
same code runs on desktop, iOS and Android. The beat playback comes from
[mnomer](https://github.com/VladasZ/mnomer).

On iOS and Android the screen stays awake while the metronome is visible and
ticking. Locking the phone or leaving the app stops playback. Returning leaves
it stopped until Start is pressed again.

## Run

```bash
make run        # desktop
make ios        # generate the Xcode project and build
make android    # android build
```

## Checks

```bash
make ci         # typos, fmt, clippy, machete
make lint       # clippy only
```
