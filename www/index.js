import { Universe } from "flower-game";

const pre = document.getElementById("flower-game-canvas");
const universe = Universe.new();

// testing

const renderLoop = () => {
  pre.textContent = universe.render();
  universe.tick();

  requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
