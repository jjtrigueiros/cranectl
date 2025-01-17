# cranectl

This is a demo project built to accomplish a few personal goals:
- Brush up on some stuff I was already (at least somewhat) familiar with
  - React (and Javascript in general)
  - Rust
  - Robotics (forward/inverse kinematics)
- Learn about and use some new stuff I've been meaning to try:
  - websockets
  - three.js

The backend (Rust) simulates a 4-DOF crane equipped with a gripper (the gripper itself is not implemented). It stores the state of each of the crane's actuators and accepts control inputs: either setpoints for each of its individual actuators, or a single setpoint for the end actuator (in which case, the system will calculate the required position for each of its constituent actuators through inverse kinematics).

The frontend (React, Typescript, three.js) is a simple webpage that allows us to visualize the crane state and send commands to the backend.

The two components communicate via websockets: the backend exposes a server which the frontend will attempt to connect to on page load.

Here's a [demo video](demo.webm) of the project in action.

![demo](demo.gif)

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
