import * as wasm from "kevinpthornecom_wasm";

document.body.setAttribute(
    "style", 
    "margin: 0; padding 0; height: 100%; width: 100%;"
);

var canvasElement = document.createElement("canvas");
canvasElement.id = "theCanvas";
canvasElement.width = window.innerWidth;
canvasElement.height = window.innerHeight;
canvasElement.setAttribute(
    "style",
    "display: block;"
);
document.body.appendChild(canvasElement);

var app = wasm.init(canvasElement);
canvasElement.onclick = (event) => {
    app.on_click(event);
};
onresize = (event) => {
    canvasElement.width = window.innerWidth;
    canvasElement.height = window.innerHeight;
    app.on_resize();
};
function render(timeStamp) {
    app.on_frame(timeStamp);
    window.requestAnimationFrame(render);
}
window.requestAnimationFrame(render);
