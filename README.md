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
- [ ] see their currency go up
- [ ] see module effects
- [ ] grab-buy modules from shop
- [ ] move around the map and zoom
- [ ] buy modules
- [ ] drag-n-drop modules onto hubs   (left-click)
- [ ] boost module by selecting       (right-click)

## Notes

- [ ] crates
  - [x] [`bevy_egui`](https://docs.rs/bevy_egui/latest/bevy_egui/)
  - [ ] [`bevy_simple_subsecond_system`](https://github.com/TheBevyFlock/bevy_simple_subsecond_system)
- [ ] hubs
  - [ ] gains currency per pulse
  - [ ] hubs multiply charge
    - [ ] the further out the hub, the higher the multiplier
    - [ ] multiplier result is rounded down
  - [ ] hubs loose charge over time
  - [ ] hubs gain charge from pulsing effect
  - [ ] hubs can fit one module
  - ~~[ ] connected via graph network~~
- [ ] modules
  - [ ] different types of energy dispersal
    - [ ] gong:       diamond   | pulses outward a diminishing ring of energy
    - [ ] generator:  hexagon   | pulsing field of evenly distributed energy
    - [ ] cannon:     pentagon  | slowly spinning beam of energy
    - [ ] tesla:      triangle  | randomly dispersed jolt of energy
    - ~~[ ] pump:      surges energy along the network edges~~
  - [ ] any module can fit to any hub
  - [ ] modules use immediate energy available
  - [ ] modules can be boosted for currency (boosting multiplies energy)
- [ ] UI
  - [ ] currency
  - [ ] shop
    - [ ] displays modules at the bottom
    - [ ] enabled module type when affordable
- [ ] pulse
  - [ ] pulse stores reciever entity and timer
  - [ ] disperser keeps track of pulses
  - [ ] check pulse timer for end, then remove
  - [ ] if no pulse, and dispersal clear, add pulse and give energy
- [ ] dispersal
  - [ ] is the effect itself, spreading energy
    - [ ] field: all within are updated simultaneously
    - [ ] beam: all within width are checked
    - [ ] wave: all within ring are checked
    - [ ] bolt: moves on after timer
  - [ ] use bloom to visualize

## Limitations

- do not allow for positive feedback loop (growth must only occur from user interaction)
- graph network requires more effort, STS (Shrink The Scope)
- pump requires graph network, STS

## Planning

- [x] Day 1: planning, setup, hub network
- [x] Day 2: crates, module art, drag-n-drop, relationships, currency, shop
- [x] Day 3: pulse masking, currency display
- [ ] Day 4: energy dispersal, effects, sound
- [ ] Day 5: cicd, publish
- [ ] Day 6: testing, tweaking
- [ ] Day 7: ship
- [ ] Day 8: SHIP!
