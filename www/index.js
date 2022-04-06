import { Universe } from "life";

const pre = document.getElementById("life-canvas");
const universe = new Universe();
console.log(universe)

const renderLoop = () => {
    universe.tick();
    pre.innerHTML = universe.toString();

    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);