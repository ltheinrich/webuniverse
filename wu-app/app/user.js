import { load, api_fetch, login_data } from "../js/common.js";

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
                    alert("User successfuly deleted");
                    location.href = "./users.html";
                } else {
                    alert("API error: " + json.error);
                }
            }, "user/delete_user", { user_username: user, ...login_data() });
        }
    });
});
