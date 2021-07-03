// @ts-check
import init, { step, set, initialize } from "./pkg/conway_wasm.js";

// @ts-ignore
window.console_log = console.log;

const toggle = (row, col) => {
    let node = document.querySelector("#grid-container div[data-row='" + row + "'][data-col='" + col + "']");
    set(row, col, !node.classList.contains("active"));
    node.classList.toggle("active");
}

const build = (/** @type {number[]} */ grid, /** @type {number} */ size) => {
	const container = document.getElementById("grid-container");
    container.innerHTML = "";
	for (let row = 0; row < size; row++) {
		let rowElement = document.createElement("div");
		rowElement.classList.add("row");
		for (let col = 0; col < size; col++) {
			let value = grid[(row * size) + col];
			let node = document.createElement("div");
			node.classList.add("element");
            node.dataset.row = row.toString();
            node.dataset.col = col.toString();
            node.addEventListener('click', () => {
                toggle(row, col);
            })
			if (value == 1) {
				node.classList.add("active");
			}
			rowElement.appendChild(node);
		}
		container.appendChild(rowElement);
	}
}

const update = (/** @type {number[]} */ grid, /** @type {number} */ size) => {
    console.log("updating");
    for (let row = 0; row < size; row++) {
        for (let col = 0; col < size; col++) {
            let node = document.querySelector("#grid-container div[data-row='" + row + "'][data-col='" + col + "']");
            let value = grid[(row * size) + col];
            node.classList.toggle("active", value == 1);
        }
    }
}

// @ts-ignore
window.render = (/** @type {number[]} */ grid, /** @type {number} */ size) => {
	const container = document.getElementById("grid-container");
    if(container.innerHTML == "") {
        build(grid, size);
    } else {
        update(grid, size);
    }

}

let /** @type {number} */ interval = null;

init()
	.then(() => {
		initialize();
        document.getElementById("step").addEventListener('click', () => {
            step();
        });
        let runButton = document.getElementById("run");
        runButton.addEventListener('click', () => {
            if(interval == null) {
                interval = setInterval(step, 100);
                runButton.innerHTML = "Pause";
            } else {
                clearInterval(interval);
                interval = null;
                runButton.innerHTML = "Run";
            }
        });
	});