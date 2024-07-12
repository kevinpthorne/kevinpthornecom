import * as wasm from "kevinpthornecom_wasm";

document.body.setAttribute("style", "margin: 0;height:100vh; width:100vw;");

var canvasElement = document.createElement("canvas");
canvasElement.id = "theCanvas";
canvasElement.width = window.innerWidth;
canvasElement.height = window.innerHeight;
onresize = (event) => {
    canvasElement.width = window.innerWidth;
    canvasElement.height = window.innerHeight;
};
document.body.appendChild(canvasElement);

wasm.start(canvasElement);
