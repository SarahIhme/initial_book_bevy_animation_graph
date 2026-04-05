# An omni-directional run-walk controller

## Pre-Requisites

This chapter assumes you have forward, backward and sideways walk and run animation. Create animation assets for each of them.

## Create animation graph 

- create blend node with all of them
- add event annotation to it TESTING
- make a scene with it
- use it with bevy_ahoy


# An omni-directional character controller with punch and idle

## Pre-Requisites

This chapter assumes you have omnidirectional motion in an animation graph, as well as idle and punch animation.


## Create FSM

- make FSM to change from Idle/moving/punching
- make a node which send events coming in further (punch for exmaple), and adds idle/moving change based on speed

And that should be it!!!