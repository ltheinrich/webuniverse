import { load, api_fetch, login_data } from "../js/common.js";

let settings = { "": "" };
const setting_key = document.getElementById("setting_key");
const setting_value = document.getElementById("setting_value");
const change_setting = document.getElementById("change_setting");

load(async function (wasm) {
    reload_settings();
    setting_key.addEventListener("change", function () {
        setting_value.value = settings[setting_key.value];
    });
    change_setting.onsubmit = function () {
        api_fetch(async function (json) {
            if (json.error == false) {
                reload_settings();
                setting_value.value = "";
            } else {
                alert("API error: " + json.error);
            }
        }, "settings/set", { settingkey: setting_key.value, settingvalue: wasm.str_encode(setting_value.value), ...login_data() });
        return false;
    }
});

async function reload_settings() {
    api_fetch(async function (json) {
        if (json.settings != undefined) {
            settings = json.settings;
            setting_key.innerHTML = "<option disabled selected>Setting key</option>"
            for (const key in settings) {
                const option = document.createElement("option");
                option.innerText = key;
                option.value = key;
                setting_key.appendChild(option);
            }
        } else {
            alert("API error: " + json.error);
        }
    }, "settings/all", login_data());
}
