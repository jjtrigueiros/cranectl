(to do: describe installation)

Backend startup:
```
cd backend
cargo run
```

Frontend startup:
```
cd frontend
npx next dev
```


## Implementation details

OpenGL uses a right-handed system with a vertical Y-axis. To simplify and avoid a coordinate conversion (rotation around the X-axis), this system was adopted for the project.

I've made use of Denavit-Hartenberg parameters as they helped me reason about rendering the 3d model, and they can be of further use if we want to integrate with a more complex IK solver (the current IK implementation is pretty simple).

Wherever DH parameters are referenced, consider the variables:
- theta_1: swing angle in relation to the starting position
- d2: height of the arm in relation to the lift base
- theta_3: elbow angle in relation to a stretched arm
- theta_4: wrist angle in relation to the forearm

... and the following constants:
- d3: vertical displacement at the elbow joint (distance between the upper arm reference and the forearm reference)
- d4: same for the wrist joint (same for the forearm and the hand)
- r3: horizontal displacement between the axis of movement for the lift and the axis of rotation for the elbow (informally referred to as "upper arm length" although the physical component can be larger)
- r4: same, between the elbow and wrist rotation axes
