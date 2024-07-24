
## Running the project
### Development
Backend startup:
```bash
cd backend
cargo run
```

Frontend setup/startup:
```bash
# tested using node v18.20.0 and npm 10.5.0
cd frontend
npm install
npx next dev
```

### Production
Backend:
```bash
cd backend
cargo build -r
# run the binary/executable according to your system
./target/release/backend
```

Frontend:
```bash
cd frontend
npx next build
npx next start
```


## Implementation details

OpenGL uses a right-handed system with a vertical Y-axis. To simplify and avoid a coordinate conversion (rotation around the X-axis), this convention was adopted for the project.

I've made use of Denavit-Hartenberg parameters as they helped me reason about rendering the 3d model, and they can be of further use if we want to integrate with a more complex IK solver (the current IK implementation is pretty simple).

Wherever DH parameters are referenced, consider the variables:
- theta_1: swing angle relative to the starting position
- d2: arm height relative to the lift base
- theta_3: elbow angle relative to a stretched arm
- theta_4: wrist angle relative to the forearm

... and the following constants:
- d3: vertical displacement at the elbow joint (distance between the upper arm ref. and the forearm ref.)
- d4: vertical displacement at the wrist joint (same for the forearm and the hand)
- r3: horizontal displacement between the lift axis and the elbow rotation axis
- r4: horizontal displacement between the elbow and wrist rotation axes

## Demo

[Demo video](demo.webm)