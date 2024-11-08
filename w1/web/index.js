import * as sim from "simulation-wasm";
import * as THREE from "three";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls.js";

// Simulation

const ruleConfig = {
  x: 30,
  y: 30,
  z: 30,
  alive: [4],
  born: [4],
  health: 5,
  neighborType: 0,
};

const rules = new sim.Ruleset(
  Uint8Array.from(ruleConfig.alive),
  Uint8Array.from(ruleConfig.born),
  ruleConfig.health,
  ruleConfig.neighborType,
  ruleConfig.x,
  ruleConfig.y,
  ruleConfig.z
);

const ca = new sim.Simulation(rules);

// Initialize Renderer
const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(
  75,
  window.innerWidth / window.innerHeight,
  0.1,
  1000
);

const renderer = new THREE.WebGLRenderer();
renderer.setSize(window.innerWidth, window.innerHeight);
const canvas = document.body.appendChild(renderer.domElement);

// Construct Cells
const cellGeometry = new THREE.BoxGeometry(0.08, 0.08, 0.08);
const cells = [];
for (let x = 0; x < ruleConfig.x; x++) {
  for (let y = 0; y < ruleConfig.y; y++) {
    for (let z = 0; z < ruleConfig.z; z++) {
      const cellMaterial = new THREE.MeshBasicMaterial({
        color: new THREE.Color().setRGB(
          Math.abs(x / ruleConfig.x - 0.5) * 2,
          Math.abs(y / ruleConfig.y - 0.5) * 2,
          Math.abs(z / ruleConfig.z - 0.5) * 2
        ),
      });
      const cell = new THREE.Mesh(cellGeometry, cellMaterial);
      cell.position.set(x / 10, y / 10, z / 10);
      scene.add(cell);
      cells.push(cell);
    }
  }
}

// Camera Controls
camera.position.x = ruleConfig.x / 10;
camera.position.y = ruleConfig.y / 10;
camera.position.z = ruleConfig.z / 10;
camera.lookAt(
  (0.5 * ruleConfig.x) / 10,
  (0.5 * ruleConfig.y) / 10,
  (0.5 * ruleConfig.z) / 10
);

const controls = new OrbitControls(camera, canvas);
controls.enableDamping = true;
controls.target.set(
  (0.5 * ruleConfig.x) / 10,
  (0.5 * ruleConfig.y) / 10,
  (0.5 * ruleConfig.z) / 10
);

// Render Loop
const clock = new THREE.Clock();
let lastTime = 0;
function animate() {
  const elapsedTime = clock.getElapsedTime();
  if (elapsedTime - lastTime > 1 / 20) {
    lastTime = elapsedTime;
    ca.step();
    let caCells = ca.cells();
    for (const cell in cells) {
      if (caCells[cell] != 0) {
        cells[cell].visible = true;
      } else {
        cells[cell].visible = false;
      }
    }
  }
  controls.update();
  renderer.render(scene, camera);
}
renderer.setAnimationLoop(animate);
