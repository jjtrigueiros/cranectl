'use client';

import React, { useRef } from 'react';
import * as THREE from 'three';
import { Group, Mesh, BoxGeometry, Material } from 'three';
import { Canvas, useFrame } from '@react-three/fiber';
import { OrbitControls } from '@react-three/drei';


interface CraneProps {
  swing_deg: number,
  lift_mm: number,
  elbow_deg: number,
  wrist_deg: number,
  gripper_mm: number,
}

const Crane: React.FC<CraneProps> = ({
  swing_deg,
  lift_mm,
  elbow_deg,
  wrist_deg,
  gripper_mm,
}) => {
  const th1 = THREE.MathUtils.degToRad(swing_deg); // theta_1
  const th3 = THREE.MathUtils.degToRad(elbow_deg); // theta_3
  const th4 = THREE.MathUtils.degToRad(wrist_deg); // theta_4

  const d2 = lift_mm / 1000; // d2

  const gripper = gripper_mm / 1000;

  // Model reference
  const REF_X = 0
  const REF_Y = 0
  const REF_Z = 0

  // We will map out the actuator references picked following the DH convention,
  // and display them for debugging purposes.
  // Using Dn/Rn/Tn for variable names may seem confusing here,
  // but personally it helped me not make mistakes in the render code.

  const HELPER_AXES_LENGTH = 0.3

  // Actuator parameters in meters (these should match in the backend for IK to work)
  // d1 = 0
  const D2_MAX = 2 // PILLAR_HEIGHT
  const D3 = -0.10 // ELBOW_DISPLACEMENT
  const D4 = -0.5 // WRIST_DISPLACEMENT
  // all thetas are variable or zero
  // r1 = 0
  // r2 = 0
  const R3 = 0.6 // UPPER_ARM_LENGTH
  const R4 = 0.6 // FOREARM_LENGTH
  // all alphas are zero

  // to do
  const GRIPPER_DISPLACEMENT = 0.2

  // cosmetic, these don't affect IK
  const BASE_WIDTH = 0.2
  const BASE_HEIGHT = 0.15
  const PILLAR_WIDTH = 0.1
  const UPPER_ARM_WIDTH = 0.15
  const FOREARM_WIDTH = 0.10
  const WRIST_1_WIDTH = 0.15
  const WRIST_2_WIDTH = 0.15

  // For a basic crane representation, since we have a lot of freedom picking our
  // actuator references (parallel z-axes), we can use the DH params to draw our
  // basic crane blocks.

  // three.js uses a right-handed reference, but with Y pointing up.
  // We can use the <group> element to represent the next actuator ref. as:
  // <group position={[r, d, 0]}, rotation={[0, theta, alpha]}>
  return (
    <group position={[REF_X, REF_Y, REF_Z]}>
      {/* Crane base reference (x0, y0, z0) */}
      <mesh position={[0, -BASE_HEIGHT/2, 0]}>
        {/* Simple reference box */}
        <boxGeometry args={[BASE_WIDTH, BASE_HEIGHT, BASE_WIDTH]} />
        <meshStandardMaterial color="grey" />
      </mesh>

      {/* Crane proper (x1, y1, z1) */}
      <group rotation={[0, th1, 0]}>
        <axesHelper args={[HELPER_AXES_LENGTH]} />
        <mesh position={[0, D2_MAX/2, 0]}>
          <boxGeometry args={[PILLAR_WIDTH, D2_MAX, PILLAR_WIDTH]} />
          <meshStandardMaterial color="lightgrey" />
        </mesh>

        {/* Upper Arm (x2, y2, z2)*/}
        <group position={[0, d2, 0]}>
          <axesHelper args={[HELPER_AXES_LENGTH]} />
          <mesh position={[0, 0, R3/2]}>
            <boxGeometry args={[UPPER_ARM_WIDTH, UPPER_ARM_WIDTH, R3]} />
            <meshStandardMaterial color="blue" />
          </mesh>

          {/* Forearm (x3, y3, z3)*/}
          <group position={[0, D3, R3]} rotation={[0, th3, 0]}>
            <axesHelper args={[HELPER_AXES_LENGTH]} />
            <mesh position={[0, 0, R4/2]}>
              <boxGeometry args={[FOREARM_WIDTH, FOREARM_WIDTH, R4]} />
              <meshStandardMaterial color="lightgreen" />
            </mesh>

            {/* Hand (x4, y4, z4*/}
            <group position={[0, D4, R4]} rotation={[0, th4, 0]}>
              <axesHelper args={[HELPER_AXES_LENGTH]} />
              <mesh position={[0, -D4/2, 0]}>
                <boxGeometry args={[WRIST_1_WIDTH, D4, WRIST_1_WIDTH]} />
                <meshStandardMaterial color="white" />
              </mesh>
              <mesh position={[0, WRIST_2_WIDTH/2, GRIPPER_DISPLACEMENT/2]}>
                <boxGeometry args={[WRIST_2_WIDTH, WRIST_2_WIDTH, GRIPPER_DISPLACEMENT]} />
                <meshStandardMaterial color="white" />
              </mesh>
            </group>
          </group>
        </group>
      </group>
    </group>
  );
};

const ThreeCanvas: React.FC<CraneProps> = (crane_props) => (
  <div className="relative w-full h-full">
    <Canvas className="absolute inset-0" camera={{ position: [2, 1, 2] }}>
      <ambientLight intensity={1} />
      {/* <pointLight position={[10, 10, 10]} intensity={1000} /> */}
      <Crane {...crane_props} />
      <OrbitControls />
      <axesHelper args={[5]} />
    </Canvas></div>
);

export default ThreeCanvas;

