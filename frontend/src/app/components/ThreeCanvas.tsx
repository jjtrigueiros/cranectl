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
  const pillarRef = useRef<Group>(null);
  const armRef = useRef<Group>(null);
  const endEffectorRef = useRef<Mesh<BoxGeometry, Material>>(null);

  useFrame(() => {
    if (pillarRef.current) {
      pillarRef.current.rotation.y = THREE.MathUtils.degToRad(swing_deg);
    }
    if (armRef.current) {
      armRef.current.position.y = lift_mm / 1000; // Convert mm to meters for Three.js units
    }
    if (endEffectorRef.current) {
      endEffectorRef.current.position.x = gripper_mm / 1000; // Convert mm to meters for Three.js units
    }
  });

  // Base reference
  const BASE_X = 0
  const BASE_Y = 0
  const BASE_Z = 0
  // Crane measurements
  const BASE_WIDTH = 0.2
  const BASE_HEIGHT = 0.15
  const PILLAR_WIDTH = 0.1
  const PILLAR_HEIGHT = 2
  // to do:
  const UPPER_ARM_WIDTH = 0.3
  const UPPER_ARM_LENGTH = 0.1
  const FOREARM_WIDTH = 10
  const FOREARM_LENGTH = 10
  const WRIST_WIDTH = 1
  const WRIST_HEIGHT = 2

  return (
    <group position={[BASE_X, BASE_Y, BASE_Z]}>
      {/* Static Base */}
      <mesh>
        <boxGeometry args={[BASE_WIDTH, BASE_HEIGHT, BASE_WIDTH]} />
        <meshStandardMaterial color="grey" />
      </mesh>
      {/* Pillar */}
      <group ref={pillarRef}>
        <mesh position={[0, PILLAR_HEIGHT/2, 0]}>
          <boxGeometry args={[PILLAR_WIDTH, PILLAR_HEIGHT, PILLAR_WIDTH]} />
          <meshStandardMaterial color="lightgrey" />
        </mesh>

        {/* Arm */}
        <group ref={armRef} position={[0, lift_mm / 1000, 0]}>
          <mesh position={[2, 0, 0]}>
            <boxGeometry args={[4, 0.2, 0.2]} />
            <meshStandardMaterial color="blue" />
          </mesh>

          {/* End Effector */}
          <mesh ref={endEffectorRef} position={[4, 0, 0]}>
            <boxGeometry args={[0.5, 0.5, 0.5]} />
            <meshStandardMaterial color="red" />
          </mesh>
        </group>
      </group>
    </group>
  );
};
const ThreeCanvas: React.FC = () => {
  return (
    <div className="relative w-full h-full">
    <Canvas className="absolute inset-0" camera={{ position: [2, 1, 2]}}>
      <ambientLight intensity={1} />
      {/* <pointLight position={[10, 10, 10]} intensity={1000} /> */}
      <Crane
        swing_deg={0}
        lift_mm={1500}
        elbow_deg={0}
        wrist_deg={0}
        gripper_mm={1000}
      />
      <OrbitControls />
      <axesHelper args={[5]} />
    </Canvas></div>
  );
};

export default ThreeCanvas;

