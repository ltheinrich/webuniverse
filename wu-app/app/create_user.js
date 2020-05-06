import { load, api_fetch, login_data } from "../js/common.js";

load(async function (wasm) {
    document.getElementById("createform").onsubmit = function () {
        const username = document.getElementById("username").value;
        const password = document.getElementById("password").value;
        if (username == "") {
            return alert("Empty username") == true;
        } else if (password == "") {
            return alert("Empty password") == true;
        }
        const argon2_hash = wasm.argon2_hash(password, username);
        api_fetch(async function (json) {
            if (json.error == false) {
                alert("User successfuly created");
                location.href = "./users.html";
            } else {
                alert("API error: " + json.error);
            }
        }, "users/create", { user: username, password: argon2_hash, ...login_data() });
        return false;
    };
});
