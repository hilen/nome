# nome

A metronome. One Rust crate on [hilen](https://github.com/hilen/hilen), so the
same code runs on desktop, iOS and Android. The beat plays on the `SoundClock`
of hilen, which starts each click on an exact sample.

## Sounds

A switch above the tempo buttons picks the click, saved in `click.json` in the
data folder.

- Beep, `assets/sounds/beep.wav`. `scripts/render-beep.rs` renders it, a 440 Hz
  tone of 0.05 s, run it with `rust scripts/render-beep.rs`. Beat 1 plays the
  same file 5 semitones up.
- Hi-hat, `assets/sounds/hihat.wav`, a closed hit by Sajmund,
  [Freesound 132415](https://freesound.org/people/Sajmund/sounds/132415/), CC0.
  Beat 1 plays at full volume, the other beats at 0.6.

The bars light from the same clock. They run as a declared loop of 1 bar, which
starts at the beat of the clock on Start and on every tempo change.

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
