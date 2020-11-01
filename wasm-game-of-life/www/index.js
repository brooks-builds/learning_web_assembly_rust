import {Universe} from "wasm-game-of-life";

const preElement = document.querySelector('#game-of-live-canvas');
const universe = Universe.new(32);
universe.randomize();

requestAnimationFrame(renderLoop);

function renderLoop() {
    preElement.textContent = universe.render();
    universe.tick();
    requestAnimationFrame(renderLoop);
}

setInterval(() => universe.randomize(), 10000);