import { load, api_fetch, login_data, username } from "../js/common.js";

load(async function (wasm) {
    const get_params = new URLSearchParams(window.location.search);
    const user = get_params.get("name");
    if (user == undefined) {
        alert("No username defined");
        location.href = "./users.html";
        return;
    }
    document.getElementById("deleteuser").addEventListener("click", function () {
        if (confirm("Delete user?")) {
            api_fetch(async function (json) {
                if (json.error == false) {
                    alert("User successfully deleted");
                    location.href = "./users.html";
                } else {
                    alert("API error: " + json.error);
                }
            }, "users/delete", { user: user, ...login_data() });
        }
    });
    document.getElementById("changeform").onsubmit = function () {
        const new_username = document.getElementById("newusername");
        const new_password = document.getElementById("newpassword");
        if (new_username.value == "" && new_password.value == "") {
            return alert("Username and password empty") == true;
        } else if (new_password.value == "") {
            return alert("Password must be changed when changing the username") == true;
        }
        const argon2_hash = wasm.argon2_hash(new_password.value, new_username.value != "" ? new_username.value : user);
        api_fetch(async function (json) {
            if (json.error == false) {
                if (user == username()) {
                    sessionStorage.setItem("username", new_username.value);
                }
                alert("User successfully changed");
                location.href = "./users.html";
            } else {
                alert("API error: " + json.error);
            }
        }, "users/change", new_username.value != "" && new_password.value != "" ? { newusername: new_username.value, password: argon2_hash, user, ...login_data() } : { password: argon2_hash, user, ...login_data() });
        return false;
    };
});
