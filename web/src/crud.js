const API_URL = new URL(process.env.API_URL || "http://localhost:8000");
function fetchSafe() {
    return new Promise((resolve, reject) => {
        fetch(...arguments)
            .then((resp) => {
                if (resp.ok) {
                    resolve(resp);
                } else {
                    reject(resp.statusText);
                }
            })
            .catch(reject);
    });
}
function fetchSafeJson() {
    return fetchSafe(...arguments).then((d) => d.json());
}
export function getTargets() {
    return fetchSafeJson(new URL("targets", API_URL));
}
export function createTarget(targ) {
    return fetchSafeJson(new URL("targets", API_URL), {
        method: "POST",
        body: JSON.stringify(targ),
    });
}
// I think update target is bad idea, see crud.rs
// export function updateTarget(targ) {
//     return fetchSafeJson(new URL("targets", API_URL), {
//         method: "PATCh",
//         body: JSON.stringify(targ),
//     });
// }
export function getConfig() {
    return fetchSafeJson(new URL("config", API_URL));
}
export function updateConfig(conf) {
    return fetchSafeJson(new URL("config", API_URL), {
        method: "PATCH",
        body: JSON.stringify(conf),
    });
}
export function getMappings() {
    return fetchSafeJson(new URL("mappings", API_URL), {
        method: "GET",
    });
}
