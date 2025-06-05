# Chain Reaction - Bevy Game Jam 6

This is an entry for the bevy game jam 6.

## Premise

Grow a network of energy dispersing hubs - each causing the others to react.

## Game Loop

- the more hubs active
- the more currency accumlated
- the more modules bought/boosted
- the greater the energy dispersal

## User Stories

- [x] see hubs
- [x] see their currency go up
- [x] grab-buy modules from shop
- [x] drag-n-drop modules onto hubs   (left-click)
- [x] boost module by selecting       (right-click)
- [ ] see module effects
  - [x] wave
  - [ ] field
  - [ ] bolt
  - [ ] beam
- [ ] move around the map and zoom

## Notes

- [ ] crates
  - [x] [`bevy_egui`](https://docs.rs/bevy_egui/latest/bevy_egui/)
  - [ ] [`bevy_simple_subsecond_system`](https://github.com/TheBevyFlock/bevy_simple_subsecond_system)
- [ ] hubs
  - [x] gains currency per pulse
  - [x] hubs multiply charge
    - [x] the further out the hub, the higher the multiplier
    - [x] multiplier result is rounded down
  - [x] hubs loose charge over time
  - [x] hubs gain charge from pulsing effect
  - [x] hubs can fit one module
  - ~~[ ] connected via graph network~~
- [ ] modules
  - [ ] different types of energy dispersal
    - [x] gong:       diamond   | pulses outward a diminishing ring of energy
    - [ ] generator:  hexagon   | pulsing field of evenly distributed energy
    - [ ] cannon:     pentagon  | slowly spinning beam of energy
    - [ ] tesla:      triangle  | randomly dispersed jolt of energy
    - ~~[ ] pump:      surges energy along the network edges~~
  - [x] any module can fit to any hub
  - [x] modules use immediate energy available
  - [x] modules can be boosted for currency (boosting multiplies energy)
- [ ] UI
  - [x] currency
  - [x] shop
    - [x] displays modules at the bottom
    - [x] enabled module type when affordable
- [x] pulse
  - [x] hubs keep track of collisions
  - [x] check pulse timer for end, then remove
  - [x] if no pulse, and dispersal clear, add pulse and give energy
- [ ] dispersal
  - [ ] is the effect itself, spreading energy
    - [x] wave: all within ring are checked
    - [ ] field: all within are updated simultaneously
    - [ ] beam: all within width are checked
    - [ ] bolt: moves on after timer
  - [x] use bloom to visualize

## Limitations

- do not allow for positive feedback loop (growth must only occur from user interaction)
- graph network requires more effort, STS (Shrink The Scope)
- pump requires graph network, STS

## Planning

- [x] Day 1: planning, setup, hub network
- [x] Day 2: crates, module art, drag-n-drop, relationships, currency, shop
- [x] Day 3: pulse masking, currency display
- [x] Day 4: energy dispersal, effects
- [ ] Day 5: energy dispersal
- [ ] Day 6: testing, tweaking, sound
- [ ] Day 7: ship
- [ ] Day 8: SHIP!

## Issues

- [ ] hub mask does not blink if holding module (feature?)
