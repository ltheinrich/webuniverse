import { load, api_fetch, login_data } from "../js/common.js";

let data_len = 0;

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
    reload_console(name, consoledata);
    setInterval(function () {
        reload_console(name, consoledata);
    }, 1000);
    document.getElementById("serverconsole").onsubmit = function () {
        const server_command_doc = document.getElementById("servercommand");
        const server_command = server_command_doc.value;
        if (server_command == "") {
            return alert("Empty command") == true;
        }
        server_command_doc.value = "";
        api_fetch(async function (json) {
            if (json.error != false) {
                alert("API error: " + json.error);
            }
        }, "servers/exec", { name, servercommand: server_command, ...login_data() });
        return false;
    };
});

function reload_console(name, consoledata) {
    api_fetch(async function (json) {
        if (json.data != undefined) {
            if (json.data.length > 0) {
                data_len = json.len;
                let temp_data = consoledata.value + json.data;
                if (temp_data.length > 50000) {
                    consoledata.value = temp_data.substring(temp_data.length - 50000);
                } else {
                    consoledata.value = temp_data;
                }
                if (!consoledata.mouseIsOver) {
                    consoledata.scrollTop = consoledata.scrollHeight;
                }
            }
        } else {
            alert("API error: " + json.error);
            if (json.error == "server does not exist" || json.error == "unauthenticated") {
                location.href = "./servers.html";
            }
        }
    }, "servers/data", { name, readlen: data_len, ...login_data() });
}
