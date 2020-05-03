import { load, api_fetch, login_data } from "../js/common.js";

load(async function (wasm) {
    const get_params = new URLSearchParams(window.location.search);
    const name = get_params.get("name");
    if (name == undefined) {
        alert("No server name defined");
        location.href = "./servers.html";
        return;
    }
    const consoledata = document.getElementById("consoledata");
    consoledata.mouseIsOver = false;
    consoledata.onmouseover = function () {
        this.mouseIsOver = true;
    };
    consoledata.onmouseout = function () {
        this.mouseIsOver = false;
    }
    setInterval(function () {
        reload_console(name, consoledata);
    }, 1000);
    document.getElementById("serverconsole").onsubmit = function () {
        const server_command = document.getElementById("servercommand");
        if (server_command.value == "") {
            return alert("Empty command") == true;
        }
        api_fetch(async function (json) {
            if (json.error == false) {
                reload_console();
                server_command.value = "";
            } else {
                alert("API error: " + json.error);
            }
        }, "servers/exec", { name, servercommand: server_command.value, ...login_data() });
        return false;
    };
});

function reload_console(name, consoledata) {
    api_fetch(async function (json) {
        if (json.data != undefined) {
            consoledata.value += json.data;
            if (!consoledata.mouseIsOver) {
                consoledata.scrollTop = consoledata.scrollHeight;
            }
        } else {
            alert("API error: " + json.error);
            if (json.error == "server does not exist") {
                location.href = "./servers.html";
            }
        }
    }, "servers/data", { name, readlen: consoledata.value.length, ...login_data() });
}
