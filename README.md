# Chain Reaction - Bevy Game Jam 6

This is an entry for the bevy game jam 6.

## Premise

Grow a network of energy dispersing nodes - each causing the others to react.

## Game Loop

- the more nodes active
- the more points accumlated
- the more modules bought/boosted
- the greater the energy dispersal

## Notes

- [ ] nodes
  - [ ] ~connected via graph network~
  - [ ] gains points per pulse
  - [ ] nodes multiple charge
    - [ ] the further out the node, the higher the multiplier
    - [ ] multiplier result is rounded down
  - [ ] nodes loose charge over time
  - [ ] nodes gain charge from pulsing effect
  - [ ] nodes can fit one module
- [ ] modules
  - [ ] different types of energy dispersal
    - [ ] ~pump:       surges energy along the network edges~
    - [ ] gong:       pulses outward a diminishing ring of energy
    - [ ] generator:  pulsing field of evenly distributed energy
    - [ ] beam:       slowly spinning beam of energy
    - [ ] tesla:      randomly dispersed jolt of energy
  - [ ] any module can fit to any node
  - [ ] modules use immediate energy available
  - [ ] modules can be boosted for points (boosting multiplies energy)

## Limitations

- do not allow for positive feedback loop (growth must only occur from user interaction)
- graph network requires more effort, STS (Shrink The Scope)
- pump requires graph network, STS
