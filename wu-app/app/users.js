import { load, api_fetch, login_data } from "../js/common.js";

load(async function (wasm) {
    api_fetch(async function (json) {
        if (json.users != undefined) {
            const users = document.getElementById("userslist");
            for (let i = 0; i < json.users.length; i++) {
                const a = document.createElement("a");
                a.innerText = json.users[i];
                a.classList.add("list-group-item");
                a.classList.add("list-group-item-action");
                a.href = "./user.html?name=" + json.users[i];
                users.appendChild(a);
            }

        } else {
            alert("API error: " + json.error);
        }
    }, "user/list", login_data());
});
