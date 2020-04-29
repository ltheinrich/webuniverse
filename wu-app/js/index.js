import { load, api_fetch } from "./common.js";

load(async function (wasm) {
    document.getElementById("loginform").onsubmit = function () {
        const username = document.getElementById("username").value;
        const password = wasm.hash_password(document.getElementById("password").value, username);
        api_fetch(async function (json) {
            if ("token" in json) {
                sessionStorage.setItem("username", username);
                sessionStorage.setItem("token", json.token);
                location.href = "./app/";
            } else {
                alert("API error: " + json.error);
            }
        }, "user/login", { username, password });
        return false;
    };
}, false);
