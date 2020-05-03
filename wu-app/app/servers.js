import { load, api_fetch, login_data } from "../js/common.js";

load(async function (wasm) {
    api_fetch(async function (json) {
        if (json.servers != undefined) {
            const servers = document.getElementById("serverslist");
            for (let i = 0; i < json.servers.length; i++) {
                const a = document.createElement("a");
                a.innerText = json.servers[i];
                a.classList.add("list-group-item");
                a.classList.add("list-group-item-action");
                a.href = "./server.html?name=" + json.servers[i];
                servers.appendChild(a);
            }
        } else {
            alert("API error: " + json.error);
        }
    }, "servers/list", login_data());
});
