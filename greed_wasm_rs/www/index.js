import { Game, set_panic_hook } from "greed_wasm_rs";

set_panic_hook();

let game_field = document.getElementById("greed_field");
let out = document.getElementById("out");
let x_size = document.getElementById("x_size");
let y_size = document.getElementById("y_size");
let seed = document.getElementById("seed");

let save = document.getElementById("save_game");
let load = document.getElementById("load_game");


let game = undefined;
let reset_timeout = undefined;

load.onchange = (e) => {
    e.target.files[0].text().then((text) => {
        try {
            game = Game.from_string(text);
            out.innerHTML = `current seed: <b>${game.seed()}</b>`;
        } catch (e) {
            out.innerHTML = `Error: <b>${e}</b>`;
            game = undefined;
        }
        redraw();
    })
};

save.onclick = (e) => {
    if (game !== undefined) {

        download(game.save(), "greed.json", "application/json")
    }
}
document.getElementById("gen_btn").onclick = generate;

document.addEventListener('keydown', (e) => {

    if (e.code.startsWith("Numpad")) {
        let num = e.code.slice(6);
        if (num != 0 && num != 5) {
            console.log(`move with key ${num}`)
            try {
                game.move_numpad(num);
                redraw();
            } catch (e) {
                show_message(`Error: <b>${e}</b>`)
            }
        }
    } else if (e.code === "KeyU") {
        //console.log("undo move")
        try {
            game.undo();
            redraw();
        } catch (e) {
            show_message(`Error: <b>${e}</b>`)
        }
    }
});

generate();

function show_message(msg) {
    out.innerHTML = msg;
    reset_timeout.cancel()
    reset_timeout = setTimeout(redraw, 2000);
}

function generate() {
    try {
        game = Game.generate(x_size.value, y_size.value, seed.value);
        out.innerHTML = `current seed: <b>${game.seed()}</b>`;
    } catch (e) {
        out.innerHTML = `Error: <b>${e}</b>`;
        game = undefined;
    }
    redraw();
}

function redraw() {
    if (game !== undefined) {
        game_field.innerText = game.print();
        if (game.is_stuck()) {
            out.innerHTML = "<b>No possible moves</b>";
        } else {
            out.innerHTML = `current seed: <b>${game.seed()}</b>`;
        }
    } else {
        game_field.innerText = "Invalid Game"
    }
}

function download(data, filename, type) {
    var file = new Blob([data], { type: type });
    if (window.navigator.msSaveOrOpenBlob) // IE10+
        window.navigator.msSaveOrOpenBlob(file, filename);
    else { // Others
        var a = document.createElement("a"),
            url = URL.createObjectURL(file);
        a.href = url;
        a.download = filename;
        document.body.appendChild(a);
        a.click();
        setTimeout(function () {
            document.body.removeChild(a);
            window.URL.revokeObjectURL(url);
        }, 0);
    }
}