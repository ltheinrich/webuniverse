import { load, api_fetch, login_data } from "../js/common.js";

load(async function (wasm) {
    reload_resources();
    setInterval(reload_resources, 5000);
});

function reload_resources() {
    api_fetch(async function (json) {
        if (json.stats != undefined) {
            const list = document.getElementById("resourceslist");
            list.innerHTML = "";
            for (const key in json.stats) {
                let mem_percent = Math.round((json.stats[key].memused / json.stats[key].memtotal) * 10000.0) / 100.0;
                let mem_used = Math.round(json.stats[key].memused / 10000.0) / 100.0;
                let mem_total = Math.round(json.stats[key].memtotal / 10000.0) / 100.0;
                let disk_percent = Math.round((json.stats[key].diskused / json.stats[key].disktotal) * 10000.0) / 100.0;
                let disk_used = Math.round(json.stats[key].diskused / 10000.0) / 100.0;
                let disk_total = Math.round(json.stats[key].disktotal / 10000.0) / 100.0;
                const a = document.createElement("a");
                a.innerHTML = "<h6>" + key + "</h6>"
                    + "<progress value=\"" + (json.stats[key].cpu == 0.0 ? NaN : json.stats[key].cpu) + "\" max=\"100\"></progress> CPU " + json.stats[key].cpu + "%<br>"
                    + "<progress value=\"" + mem_percent + "\" max=\"100\"></progress> Memory " + mem_percent + "% (" + mem_used + " GB of " + mem_total + " GB)<br>"
                    + "<progress value=\"" + disk_percent + "\" max=\"100\"></progress> Disk space " + disk_percent + "% (" + disk_used + " GB of " + disk_total + " GB)";
                a.classList.add("list-group-item");
                a.classList.add("list-group-item-action");
                list.appendChild(a);
            }
        } else {
            alert("API error: " + json.error);
        }
    }, "server/stats", login_data());
}
